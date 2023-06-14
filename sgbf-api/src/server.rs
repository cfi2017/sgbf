use std::borrow::Cow;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use anyhow::Context;
use axum::{BoxError, Router};
use axum::error_handling::HandleErrorLayer;
use axum::extract::State;
use axum::headers::HeaderName;
use axum::http::{Method, StatusCode};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::middleware::from_fn_with_state;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum_client_ip::SecureClientIpSource;
use firestore::FirestoreDb;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::catch_panic::CatchPanicLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use sgbf_client::client::axum::AuthCache;
use crate::cache::Cache;
use crate::config::Config;
use crate::routes;
use crate::state::{AppState, SharedState};
use crate::store::with_uid;

pub async fn init_default_server() -> anyhow::Result<()> {
    let config = Config::load().context("could not load config")?;
    let _guard = crate::tracing::init_tracing(&config.tracing)?;

    let db = FirestoreDb::new(&config.firebase.project).await?;

    let auth_cache = AuthCache::new();
    let cache = Arc::new(Cache::new(db.clone(), &config.cache.username, &config.cache.password));
    let cache_handle = {
        let cache = cache.clone();
        info!("starting cache polling");
        tokio::spawn(async move {
            cache.start_polling().await
        })
    };
    let auth_cache_handle = {
        let auth_cache = auth_cache.clone();
        info!("starting auth cache polling");
        tokio::spawn(async move {
            auth_cache.start_polling().await
        })
    };
    let state = SharedState::build(AppState {
        auth_cache,
        config: config.clone(),
        cache: cache.clone(),
        db: db.clone()
    });
    _ = init_server(&config, state).await;
    info!("shutting down cache polling");
    cache_handle.abort();
    info!("shutting down auth cache polling");
    auth_cache_handle.abort();
    Ok(())
}

pub async fn init_server(cfg: &Config, state: SharedState) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION])
        .allow_origin(Any);
    let auth_service = ServiceBuilder::new()
        .layer(from_fn_with_state(state.clone(), sgbf_client::client::axum::auth::<_, SharedState>))
        .layer(from_fn_with_state(state.clone(), with_uid::<_, SharedState>));
    let server = Router::new()
        .route("/status", get(routes::status))
        .route("/reservation/login", post(routes::reservation::login))
        .route("/reservation/calendar", get(routes::reservation::get_calendar)
            .layer(auth_service.to_owned())
        )
        .route("/reservation/day", get(routes::reservation::get_day).post(routes::reservation::update_day)
            .layer(auth_service.to_owned())
        )
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(CatchPanicLayer::new())
                .layer(TraceLayer::new_for_http()
                    .on_response(DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Millis))
                    .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                )
                .layer(cors)
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(20))
        )
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .with_state(state.clone());

    let addr = SocketAddr::from((cfg.server.host.to_owned(), cfg.server.port));
    hyper::Server::bind(&addr)
        .serve(server.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await.context("error running server")
}

async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (StatusCode::REQUEST_TIMEOUT, Cow::from("request timed out"));
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Cow::from("service is overloaded, try again later"),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Cow::from(format!("Unhandled internal error: {}", error)),
    )
}

pub enum ServerError {
    InvalidToken,
    Unknown(UnknownServerError),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token").into_response(),
            Self::Unknown(err) => err.into_response(),
        }
    }
}

pub struct UnknownServerError(anyhow::Error);

impl IntoResponse for UnknownServerError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ServerError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self::Unknown(err.into().into())
    }
}

impl<E> From<E> for UnknownServerError
    where
        E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}

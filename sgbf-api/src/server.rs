use std::borrow::Cow;
use std::net::SocketAddr;
use std::time::Duration;
use anyhow::Context;
use axum::{BoxError, Router};
use axum::error_handling::HandleErrorLayer;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum_client_ip::SecureClientIpSource;
use tokio::signal;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;
use crate::config::Config;
use crate::routes;
use crate::state::{AppState, SharedState};

pub async fn init_default_server() -> anyhow::Result<()> {
    let config = Config::load().context("could not load config")?;
    let state = SharedState::build(AppState::new(config.to_owned()));
    let _guard = crate::tracing::init_tracing(&config.tracing)?;
    init_server(&config, state).await
}

pub async fn init_server(cfg: &Config, state: SharedState) -> anyhow::Result<()> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    let server = Router::new()
        .route("/status", get(routes::status))
        .route("/reservation/login", post(routes::reservation::login))
        .route("/reservation/calendar", get(routes::reservation::get_calendar))
        .route("/reservation/day", get(routes::reservation::get_day))
        .layer(
            ServiceBuilder::new()
                // Handle errors from middleware
                .layer(cors)
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http()
                    .on_response(DefaultOnResponse::new().level(Level::INFO).latency_unit(LatencyUnit::Millis))
                    .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                ),
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

pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
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

    println!("signal received, starting graceful shutdown");
}

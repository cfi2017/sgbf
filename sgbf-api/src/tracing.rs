use std::str::FromStr;

use anyhow::Context;
use opentelemetry_otlp::WithExportConfig;
use sentry::types::Dsn;
use sentry::ClientInitGuard;
use serde::Deserialize;
use tracing_error::ErrorLayer;
use tracing_log::LogTracer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorReportingConfig {
    sentry_dsn: Option<String>,
    sampling_rate: f32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TracingConfig {
    error_reporting: ErrorReportingConfig,
    logger: LoggerConfig,
}

pub fn init_tracing(cfg: &TracingConfig) -> anyhow::Result<ClientInitGuard> {
    if std::env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=todos=debug` to see debug logs,
        // this only shows access logs.
        std::env::set_var("RUST_LOG", &cfg.logger.level);
    }
    // log interoperability layer
    LogTracer::init().context("could not initialise log tracer")?;

    // sentry initialisation
    let _guard = sentry::init(sentry::ClientOptions {
        // Set this a to lower value in production
        dsn: cfg
            .error_reporting
            .sentry_dsn
            .as_ref()
            .map(|dsn| Dsn::from_str(dsn).unwrap()),
        release: sentry::release_name!(),
        traces_sample_rate: cfg.error_reporting.sampling_rate,
        ..sentry::ClientOptions::default()
    });

    // opentelemetry tracing exporter
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic().with_endpoint("http://localhost:4317"))
        .install_batch(opentelemetry::runtime::Tokio)
        .context("could not create otel tracing exporter")?;

    // opentelemetry metrics exporter
    let meter = opentelemetry_otlp::new_pipeline()
        .metrics(
            opentelemetry_sdk::metrics::selectors::simple::inexpensive(),
            opentelemetry_sdk::export::metrics::aggregation::stateless_temporality_selector(),
            opentelemetry::runtime::Tokio,
        )
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .build()
        .context("could not create otel metrics exporter")?;

    // opentelemetry tracing integration layers
    let tracer = tracing_opentelemetry::layer().with_tracer(tracer);
    let meter = tracing_opentelemetry::MetricsLayer::new(meter);

    #[cfg(not(debug_assertions))]
        let log_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());
    #[cfg(debug_assertions)]
        let log_layer = tracing_subscriber::fmt::layer()
        .with_filter(tracing_subscriber::filter::EnvFilter::from_default_env());

    // tracing subscriber
    let subscriber = tracing_subscriber::registry()
        .with(log_layer)
        .with(ErrorLayer::default())
        .with(sentry::integrations::tracing::layer())
        .with(tracer)
        .with(meter);

    tracing::subscriber::set_global_default(subscriber).expect("unable to initialize tracing");
    opentelemetry::global::shutdown_tracer_provider();
    Ok(_guard)
}

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Initialise default Non-JSON tracing.
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer())
        .init()
}

/// Initialise default JSON `Barter` logging.
///
/// Note that this filters out duplicate logs produced by the `AuditManager` updating its replica
/// `EngineState`.
pub fn init_json_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::filter::EnvFilter::builder()
                .with_default_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with(tracing_subscriber::fmt::layer().json().flatten_event(true))
        .init()
}

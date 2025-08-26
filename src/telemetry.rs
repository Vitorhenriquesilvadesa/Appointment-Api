use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::ApplicationError;

pub fn init() -> Result<WorkerGuard, ApplicationError> {
    let env_filter = EnvFilter::from_default_env();

    let file_appender = rolling::daily("logs", "api.log");
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_writer)
        .pretty()
        .json();

    let console_layer = tracing_subscriber::fmt::layer().pretty();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    Ok(guard)
}

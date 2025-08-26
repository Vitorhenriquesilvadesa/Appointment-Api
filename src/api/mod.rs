use rocket::http::Status;
use thiserror::Error;

pub mod catchers;
pub mod domain;
pub mod dto;
pub mod error;
pub mod infra;
pub mod repository;
pub mod routes;
pub mod service;

pub use catchers::*;

pub type ApiResult<T, E> = Result<T, (Status, E)>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Error to connect database: {0}")]
    DatabaseConnection(String),

    #[error("Error to parse configuration file: {0}")]
    Settings(String),

    #[error("Error to launch application: {0}")]
    LaunchError(String),

    #[error("Error to setup telemetry system: {0}")]
    TelemetryInitialization(String),

    #[error("Error to setup environment variables: {0}")]
    EnvironmentLoading(String),
}

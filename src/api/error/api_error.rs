use rocket::http::Status;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum ApiError {
    #[error("Failed to create entity in database: {0}")]
    EntityCreation(String),

    #[error("Cannot update entity: {0}")]
    EntityUpdate(String),

    #[error("Cannot delete entity: {0}")]
    EntityDelete(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    #[error("{{{status},{details}}}")]
    Custom {
        status: Status,
        details: serde_json::Value,
    },
}

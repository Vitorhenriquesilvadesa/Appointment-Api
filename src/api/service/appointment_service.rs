use std::sync::Arc;

use rocket::http::Status;
use validator::Validate;

use crate::api::{
    domain::appointment::Appointment,
    dto::format_validation_errors,
    error::ApiError,
    repository::{AppointmentRepository, NewAppointment, UpdateAppointment},
};

pub struct AppointmentService {
    appointment_repo: Arc<dyn AppointmentRepository + Send + Sync>,
}

impl AppointmentService {
    pub fn new(appointment_repo: Arc<dyn AppointmentRepository + Send + Sync>) -> Self {
        Self { appointment_repo }
    }

    pub async fn create(&self, spec: NewAppointment) -> Result<Appointment, ApiError> {
        spec.validate().map_err(|e| {
            let json_error = format_validation_errors(e);
            ApiError::Custom {
                status: Status::BadRequest,
                details: serde_json::to_value(&json_error).unwrap(),
            }
        })?;

        let appointment = self
            .appointment_repo
            .create(spec)
            .await
            .map_err(|e| ApiError::EntityCreation(e.to_string()))?;

        Ok(appointment)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Appointment, ApiError> {
        let appointment = self
            .appointment_repo
            .get_by_id(id.into())
            .await
            .map_err(|e| ApiError::EntityNotFound(e.to_string()))?;

        Ok(appointment)
    }

    pub async fn get_all(&self) -> Result<Vec<Appointment>, ApiError> {
        let appointments = self
            .appointment_repo
            .get_all()
            .await
            .map_err(|e| ApiError::EntityNotFound(e.to_string()))?;

        Ok(appointments)
    }

    pub async fn update(&self, id: &str, spec: UpdateAppointment) -> Result<Appointment, ApiError> {
        let appointment = self
            .appointment_repo
            .update(id.into(), spec)
            .await
            .map_err(|e| match e {
                crate::api::repository::RepositoryError::NotFound => {
                    ApiError::EntityNotFound(e.to_string())
                }
                _ => ApiError::EntityUpdate(e.to_string()),
            })?;

        Ok(appointment)
    }

    pub async fn delete_by_id(&self, id: &str) -> Result<Appointment, ApiError> {
        let appointment = self
            .appointment_repo
            .delete_by_id(id.into())
            .await
            .map_err(|e| ApiError::EntityDelete(e.to_string()))?;

        Ok(appointment)
    }
}

use chrono::{DateTime, Utc};
use rocket::async_trait;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::api::{domain::appointment::Appointment, repository::CrudRepository};

#[derive(Debug, Clone, Validate, Serialize, Deserialize)]
pub struct NewAppointment {
    #[validate(length(min = 1, message = "Title should not be empty"))]
    pub title: String,

    pub description: Option<String>,

    #[validate(custom(
        function = validate_dates,
    ))]
    pub start_time: DateTime<Utc>,

    pub end_time: DateTime<Utc>,

    #[validate(length(min = 1, message = "User ID is required"))]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateAppointment {
    pub title: Option<String>,
    pub description: Option<String>,
    pub end_time: Option<DateTime<Utc>>,
}

#[async_trait]
pub trait AppointmentRepository:
    CrudRepository<Entity = Appointment, NewEntity = NewAppointment, UpdateEntity = UpdateAppointment>
{
}

pub fn validate_dates(_date: &DateTime<Utc>) -> Result<(), ValidationError> {
    Ok(())
}

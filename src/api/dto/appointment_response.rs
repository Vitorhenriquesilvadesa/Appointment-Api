use chrono::Utc;
use serde::Serialize;

use crate::api::domain::appointment::Appointment;

#[derive(Serialize)]
pub struct CreateAppointmentResponse {
    pub title: String,
    pub description: Option<String>,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub user_id: String,
}

impl From<Appointment> for CreateAppointmentResponse {
    fn from(val: Appointment) -> CreateAppointmentResponse {
        CreateAppointmentResponse {
            title: val.title,
            description: val.description,
            start_time: val.start_time,
            end_time: val.end_time,
            user_id: val.user_id,
        }
    }
}

#[derive(Serialize)]
pub struct EditAppointmentResponse {
    pub title: String,
    pub description: Option<String>,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub user_id: String,
}

impl From<Appointment> for EditAppointmentResponse {
    fn from(val: Appointment) -> EditAppointmentResponse {
        EditAppointmentResponse {
            title: val.title,
            description: val.description,
            start_time: val.start_time,
            end_time: val.end_time,
            user_id: val.user_id,
        }
    }
}

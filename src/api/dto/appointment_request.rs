use chrono::Utc;
use serde::Deserialize;

use crate::api::repository::{NewAppointment, UpdateAppointment};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateAppointmentRequest {
    pub title: String,
    pub description: Option<String>,
    pub start_time: chrono::DateTime<Utc>,
    pub end_time: chrono::DateTime<Utc>,
    pub user_id: String,
}

impl From<CreateAppointmentRequest> for NewAppointment {
    fn from(val: CreateAppointmentRequest) -> NewAppointment {
        NewAppointment {
            title: val.title,
            description: val.description,
            start_time: val.start_time,
            end_time: val.end_time,
            user_id: val.user_id,
        }
    }
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EditAppointmentRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub end_time: Option<chrono::DateTime<Utc>>,
}

impl From<EditAppointmentRequest> for UpdateAppointment {
    fn from(val: EditAppointmentRequest) -> UpdateAppointment {
        UpdateAppointment {
            title: val.title,
            description: val.description,
            end_time: val.end_time,
        }
    }
}

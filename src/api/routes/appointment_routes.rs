use std::sync::Arc;

use rocket::{Route, State, delete, get, http::Status, post, put, routes, serde::json::Json};
use tracing::{info, instrument};

use crate::api::{
    ApiResult,
    domain::appointment::Appointment,
    dto::{
        appointment_request::{CreateAppointmentRequest, EditAppointmentRequest},
        appointment_response::{CreateAppointmentResponse, EditAppointmentResponse},
    },
    error::ApiError,
    service::appointment_service::AppointmentService,
};

pub fn routes() -> Vec<Route> {
    routes![
        get_all_appointments,
        new_appointment,
        edit_appointment,
        delete_appointment_by_id,
        get_appointment_by_id,
    ]
}

#[instrument(name = "New Appointment", skip(appointment_service, appointment))]
#[post("/appointment", format = "json", data = "<appointment>")]
async fn new_appointment(
    appointment: Json<CreateAppointmentRequest>,
    appointment_service: &State<Arc<AppointmentService>>,
) -> Result<Json<CreateAppointmentResponse>, (Status, Json<serde_json::Value>)> {
    match appointment_service.create(appointment.0.into()).await {
        Ok(result) => Ok(Json(result.into())),
        Err(e) => match e {
            ApiError::Custom { status, details } => Err((status, Json(details))),
            _ => Err((
                Status::BadRequest,
                Json(serde_json::json!({ "error": e.to_string() })),
            )),
        },
    }
}

#[instrument(name = "Edit Appointment", skip(appointment, appointment_service))]
#[put("/appointment/<id>", format = "json", data = "<appointment>")]
async fn edit_appointment(
    id: &str,
    appointment: Json<EditAppointmentRequest>,
    appointment_service: &State<Arc<AppointmentService>>,
) -> ApiResult<Json<EditAppointmentResponse>, ()> {
    info!(
        "Trying to update user with id {} with [title: {:?}, description: {:?}, end_date: {:?}",
        id, appointment.title, appointment.description, appointment.end_time
    );

    match appointment_service.update(id, appointment.0.into()).await {
        Ok(result) => {
            info!("Appointment updated successful");
            Ok(Json(result.into()))
        }
        Err(e) => match e {
            ApiError::EntityNotFound(_) => Err((Status::NotFound, ())),
            _ => Err((Status::BadRequest, ())),
        },
    }
}

#[instrument(name = "Get All Appointments", skip(appointment_service))]
#[get("/appointment")]
async fn get_all_appointments(
    appointment_service: &State<Arc<AppointmentService>>,
) -> ApiResult<Json<Vec<Appointment>>, Json<String>> {
    info!("Trying to get");

    match appointment_service.get_all().await {
        Ok(result) => {
            info!("Get all successfull");
            Ok(result.into())
        }
        Err(e) => Err((Status::BadRequest, Json(e.to_string()))),
    }
}

#[instrument(skip(appointment_service))]
#[get("/appointment/<id>")]
async fn get_appointment_by_id(
    id: &str,
    appointment_service: &State<Arc<AppointmentService>>,
) -> ApiResult<Json<Appointment>, Json<String>> {
    info!("Getting appointment with id {id}");

    match appointment_service.get_by_id(id).await {
        Ok(result) => {
            info!("Entity found");
            Ok(result.into())
        }
        Err(e) => match e {
            ApiError::EntityNotFound(_) => Err((Status::NotFound, Json(e.to_string()))),
            _ => Err((Status::BadRequest, Json(e.to_string()))),
        },
    }
}

#[delete("/appointment/<id>")]
async fn delete_appointment_by_id(
    id: &str,
    appointment_service: &State<Arc<AppointmentService>>,
) -> ApiResult<Json<Appointment>, ()> {
    info!("Trying to delete appointment with id {id}");

    match appointment_service.delete_by_id(id).await {
        Ok(result) => {
            info!("Deleted successful");
            Ok(result.into())
        }
        Err(_) => Err((Status::NotFound, ())),
    }
}

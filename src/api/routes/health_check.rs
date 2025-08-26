use rocket::{Route, get, http::Status, routes};

pub fn routes() -> Vec<Route> {
    routes![health_check]
}

#[get("/health_check")]
pub fn health_check() -> Status {
    Status::Ok
}

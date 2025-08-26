use rocket::Route;

pub mod appointment_routes;
pub mod health_check;

pub fn app_routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.append(&mut health_check::routes());
    routes.append(&mut appointment_routes::routes());

    routes
}

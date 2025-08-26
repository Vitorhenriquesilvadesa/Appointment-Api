use std::sync::Arc;

use rocket::Config;

use crate::{
    ApplicationContext,
    api::{
        self, ApplicationError,
        infra::{
            db::surreal_client::create_surreal_client,
            surreal::appointment_repo::SurrealAppointRepository,
        },
        routes::app_routes,
        service::appointment_service::AppointmentService,
    },
    load_config, telemetry,
};

pub async fn run_app() -> Result<(), ApplicationError> {
    dotenv::dotenv().map_err(|e| ApplicationError::EnvironmentLoading(e.to_string()))?;

    let guard = telemetry::init()?;

    let settings = load_config()?;
    let surreal_settings = settings.surrealdb.clone();
    let server_settings = settings.server.clone();

    let db = create_surreal_client(surreal_settings).await?;

    let appointment_repo = SurrealAppointRepository::new(Arc::clone(&db));
    let appointment_service = Arc::new(AppointmentService::new(Arc::new(appointment_repo)));

    rocket::build()
        .configure(Config {
            address: server_settings.address,
            port: server_settings.port,
            ..Default::default()
        })
        .manage(ApplicationContext { db, settings })
        .manage(appointment_service)
        .register("/api", api::global_catchers())
        .mount("/api", app_routes())
        .launch()
        .await
        .map_err(|e| ApplicationError::LaunchError(e.pretty_print().into()))?;

    drop(guard);

    Ok(())
}

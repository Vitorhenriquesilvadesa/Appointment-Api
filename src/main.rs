use schedule_api::{api::ApplicationError, app::run_app};

#[rocket::main]
async fn main() -> Result<(), ApplicationError> {
    run_app().await
}

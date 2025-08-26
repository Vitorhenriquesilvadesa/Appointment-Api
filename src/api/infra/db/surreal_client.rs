use std::sync::Arc;

use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{SurrealDbConfig, api::ApplicationError};

pub async fn create_surreal_client(
    config: SurrealDbConfig,
) -> Result<Arc<Surreal<Client>>, ApplicationError> {
    let client = Surreal::new::<Ws>(config.host)
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    client
        .signin(Root {
            username: &config.username,
            password: &config.password,
        })
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    client
        .use_ns(config.namespace)
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;
    client
        .use_db(config.database)
        .await
        .map_err(|e| ApplicationError::DatabaseConnection(e.to_string()))?;

    Ok(Arc::new(client))
}

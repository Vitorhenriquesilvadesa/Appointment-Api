use std::{net::IpAddr, sync::Arc};

use config::{Config, File};
use serde::Deserialize;
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::api::ApplicationError;

pub mod api;
pub mod app;
pub mod telemetry;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub surrealdb: SurrealDbConfig,
    pub server: ServerConfig,
    // pub jwt: JwtSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SurrealDbConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub database: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub address: IpAddr,
    // pub allowed_origins: Vec<String>,
    // pub allowed_methods: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtSettings {
    pub secret: String,
    pub expiration: i64,
}

pub struct ApplicationContext {
    pub db: Arc<Surreal<Client>>,
    pub settings: Settings,
}

pub fn load_config() -> Result<Settings, ApplicationError> {
    let config: Settings = Config::builder()
        .add_source(File::with_name("Config"))
        .build()
        .map_err(|e| ApplicationError::Settings(e.to_string()))?
        .try_deserialize()
        .map_err(|e| ApplicationError::Settings(e.to_string()))?;

    Ok(config)
}

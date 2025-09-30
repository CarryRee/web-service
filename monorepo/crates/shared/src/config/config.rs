use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use tracing::{error, info};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub database: DatabaseConfig,
    pub service: ServiceConfig,
    pub log: LogConfig,
    pub jwt: JWT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub idle_timeout: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub port: u16,
    pub worker_id: u32,
    pub worker_id_bit_len: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogConfig {
    pub level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JWT {
    pub secret: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        // let base_path = get_project_root()?.join("config");

        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("./"));

        let base_path = manifest_dir.join("config");

        let env = std::env::var("APP_ENV").unwrap_or("dev".to_string());

        let config = Config::builder()
            .add_source(File::from(base_path.join("default.yaml")).required(true))
            .add_source(File::from(base_path.join(format!("{}.yaml", env))).required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()
            .map_err(|e| {
                error!("Failed to build config: {}", e);
                Box::new(e) as Box<dyn Error>
            })?;

        let app_config = config.try_deserialize().map_err(|e| {
            error!("Failed to deserialize config: {}", e);
            Box::new(e) as Box<dyn Error>
        })?;

        info!("Loaded config: {:?}", app_config);
        Ok(app_config)
    }
}

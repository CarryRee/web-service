use axum::Router;
use idgenerator::*;
use shared::{config, logger};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use crate::handlers::auth_handler;
use crate::repositories::{pg_user_repo, user_repo};
use crate::services::user_service;
use crate::models::claims::JwtSecret;

// 使用子模块文件的方式
pub mod handlers {
    pub mod auth_handler;
}

pub mod services {
    pub mod user_service;
}

pub mod repositories {
    pub mod pg_user_repo;
    pub mod user_repo;
}

pub mod models {
    pub mod claims;
    pub mod user;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    let config = config::config::AppConfig::load()?;
    logger::logger::init_logging(&config.log.level);
    tracing::info!("Service1 Config: {:?}", config);

    let port = config.service.port;
    let url = config.database.url;
    let max_connect = config.database.max_connections;
    let timeout = config.database.idle_timeout as u64;
    let work_id = config.service.worker_id;
    let worker_id_bit_len = config.service.worker_id_bit_len;

    let secret = JwtSecret {
        access_secret: config.jwt.access_secret,
        access_validity_period: config.jwt.access_validity_period,
        refresh_secret: config.jwt.refresh_secret,
        refresh_validity_period: config.jwt.refresh_validity_period,
    };
    let jwt_secret = Arc::new(secret);

    let pool = PgPoolOptions::new()
        .max_connections(max_connect)
        .idle_timeout(Duration::from_secs(timeout))
        .connect(&url)
        .await?;

    // Setup the option for the id generator instance.
    let options = IdGeneratorOptions::new()
        .worker_id(work_id)
        .worker_id_bit_len(worker_id_bit_len);
    // Initialize the id generator instance with the option.
    // Other options not set will be given the default value.
    let _ = IdInstance::init(options)?;

    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    let repo: Arc<dyn user_repo::UserRepo> = Arc::new(pg_user_repo::PgUserRepo::new(pool.clone()));
    let service = Arc::new(user_service::UserService::new(repo, pool, jwt_secret));

    // build our application with a route
    let auth_router = auth_handler::create_router(service);

    // main router
    let app = Router::new().nest("/api/v1", auth_router);

    // run our app with hyper, listening globally on port
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

use crate::models::user::{LoginUserRequest, RegisterUserRequest};
use crate::services::user_service::UserService;
use axum::{
    Json, Router,
    extract::{State, rejection::JsonRejection},
    http::StatusCode,
    routing::{get, post},
};
use shared::{constants::constants, reply::reply::Reply};
use std::{string, sync::Arc};
use validator::Validate;

pub fn create_router(service: Arc<UserService>) -> Router {
    let auth_router = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(service);

    Router::new().nest("/auth", auth_router)
}

pub async fn register(
    State(service): State<Arc<UserService>>,
    payload: Result<Json<RegisterUserRequest>, JsonRejection>,
) -> Result<Json<Reply<()>>, (StatusCode, Json<Reply<()>>)> {
    let Json(req) = payload.map_err(|e| {
        tracing::error!("json error: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(Reply::error(constants::CODE_PARAMETER_ERROR)),
        )
    })?;

    req.validate().map_err(|e| {
        tracing::error!("validate error: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(Reply::error(constants::CODE_PARAMETER_ERROR)),
        )
    })?;

    service
        .register(req)
        .await
        .map_err(|code: u16| (StatusCode::INTERNAL_SERVER_ERROR, Json(Reply::error(code))))?;

    Ok(Json(Reply::success(())))
}

pub async fn login(
    State(service): State<Arc<UserService>>,
    payload: Result<Json<LoginUserRequest>, JsonRejection>,
) -> Result<Json<Reply<()>>, (StatusCode, Json<Reply<()>>)> {
    let Json(req) = payload.map_err(|e| {
        tracing::error!("json error: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(Reply::error(constants::CODE_PARAMETER_ERROR)),
        )
    })?;

    req.validate().map_err(|e| {
        tracing::error!("validate error: {}", e);
        (
            StatusCode::BAD_REQUEST,
            Json(Reply::error(constants::CODE_PARAMETER_ERROR)),
        )
    })?;

    service
        .login(req)
        .await
        .map_err(|code: u16| (StatusCode::INTERNAL_SERVER_ERROR, Json(Reply::error(code))))?;

    Ok(Json(Reply::success(())))
}

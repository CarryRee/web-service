use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub is_active: bool,
    pub role: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Validate)]
pub struct LoginUserReply {
    pub username: String,
    pub email: String,
    pub role: String,
    pub access_token: String,
    pub refresh_token: String,
    pub access_expire_time: i64,
    pub refresh_expire_time: i64,
}

#[derive(Debug, Serialize)]
pub struct RefreshTokenReply {
    pub access_token: String,
    pub access_expire_time: i64,
}

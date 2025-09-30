use sqlx::PgPool;
use std::sync::Arc;

use shared::constants::constants;

use crate::models::user::{LoginUserRequest, RegisterUserRequest};
use crate::repositories::user_repo::UserRepo;
use idgenerator::*;

extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct UserService {
    repo: Arc<dyn UserRepo>,
    pool: PgPool,
    jwt_secret: String,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepo>, pool: PgPool, jwt_secret: String) -> Self {
        UserService {
            repo,
            pool,
            jwt_secret,
        }
    }

    pub async fn register(&self, user: RegisterUserRequest) -> Result<(), u16> {
        // Check if the user exists
        let existing_user = self.repo.find_by_email(&user.email).await.map_err(|e| {
            tracing::error!("database find email error: {}", e);
            constants::CODE_DATE_OPERATION_ERROR
        })?;
        if existing_user.is_some() {
            return Err(constants::CODE_ACCOUNT_ALREADY_EXISTS);
        }

        // Encrypted password
        let hashed = hash(user.password, DEFAULT_COST).map_err(|e| {
            tracing::error!("hash error: {}", e);
            constants::CODE_DATE_OPERATION_ERROR
        });

        // Call `next_id` to generate a new unique id.
        let id = IdInstance::next_id();

        // insert user
        self.repo
            .create(id, user.username, user.email, hashed.unwrap())
            .await
            .map_err(|e| {
                tracing::error!("database insert error: {}", e);
                constants::CODE_DATE_OPERATION_ERROR
            })
    }

    pub async fn login(&self, user: LoginUserRequest) -> Result<(), u16> {
        // Check if the user exists
        let existing_user: Option<crate::models::user::User> =
            self.repo.find_by_email(&user.email).await.map_err(|e| {
                tracing::error!("database find email error: {}", e);
                constants::CODE_DATE_OPERATION_ERROR
            })?;
        if !existing_user.is_some() {
            return Err(constants::CODE_ACCOUNT_NOT_EXISTS);
        }

        if !verify(user.password, &existing_user.unwrap().password_hash).unwrap() {
            return Err(constants::CODE_WRONG_ACCOUNT_OR_PASSWORD);
        }

        Ok(())
    }
}

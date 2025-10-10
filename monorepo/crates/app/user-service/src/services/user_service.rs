use sqlx::PgPool;
use std::sync::Arc;

use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use shared::constants::constants;

use time::{Duration, OffsetDateTime};

use crate::models::{
    claims::{AccessTokenClaims, JwtSecret, RefreshTokenClaims},
    user::{LoginUserReply, LoginUserRequest, RegisterUserRequest, RefreshTokenRequest, RefreshTokenReply},
};
use crate::repositories::user_repo::UserRepo;
use idgenerator::*;

extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};

pub struct UserService {
    repo: Arc<dyn UserRepo>,
    pool: PgPool,
    jwt_secret: Arc<JwtSecret>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepo>, pool: PgPool, jwt_secret: Arc<JwtSecret>) -> Self {
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

    pub async fn login(&self, user: LoginUserRequest) -> Result<LoginUserReply, u16> {
        // Check if the user exists
        let existing_user: Option<crate::models::user::User> =
            self.repo.find_by_email(&user.email).await.map_err(|e| {
                tracing::error!("database find email error: {}", e);
                constants::CODE_DATE_OPERATION_ERROR
            })?;
        if !existing_user.is_some() {
            return Err(constants::CODE_ACCOUNT_NOT_EXISTS);
        }

        if !verify(
            user.password,
            &existing_user.as_ref().unwrap().password_hash,
        )
        .unwrap()
        {
            return Err(constants::CODE_WRONG_ACCOUNT_OR_PASSWORD);
        }

        // access token
        let now = OffsetDateTime::now_utc();
        let access_exp = now + Duration::seconds(self.jwt_secret.access_validity_period);

        let access_claims = AccessTokenClaims {
            sub: existing_user.as_ref().unwrap().email.clone(),
            exp: access_exp.unix_timestamp(),
            iat: now.unix_timestamp(),
            role: existing_user.as_ref().unwrap().role.clone(),
        };

        let access_token = encode(
            &Header::default(), // default use HS256
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.access_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("jwt encode error: {}", e);
            constants::CODE_INTERNAL_SERVER_ERROR
        })?;

        // refresh token
        let refresh_exp = now + Duration::seconds(self.jwt_secret.refresh_validity_period);

        let refresh_claims = RefreshTokenClaims {
            sub: existing_user.as_ref().unwrap().email.clone(),
            exp: refresh_exp.unix_timestamp(),
            token_type: "refresh".to_string(),
        };

        let refresh_token = encode(
            &Header::default(), // default use HS256
            &refresh_claims,
            &EncodingKey::from_secret(self.jwt_secret.refresh_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("jwt encode error: {}", e);
            constants::CODE_INTERNAL_SERVER_ERROR
        })?;

        let reply = LoginUserReply {
            username: existing_user.as_ref().unwrap().username.clone(),
            email: existing_user.as_ref().unwrap().email.clone(),
            role: existing_user.as_ref().unwrap().role.clone(),
            access_token: access_token,
            refresh_token: refresh_token,
            access_expire_time: access_exp.unix_timestamp(),
            refresh_expire_time: refresh_exp.unix_timestamp(),
        };

        Ok(reply)
    }

    pub async fn refresh_token(&self, req: RefreshTokenRequest) -> Result<RefreshTokenReply, u16> {
        // validate refresh token
        let mut validation = Validation::new(Algorithm::HS256);
        // validate token_type
        validation.set_required_spec_claims(&["exp", "sub", "token_type"]);

        // decode & check token expiry
        let token_data = decode::<RefreshTokenClaims>(
            &req.refresh_token,
            &DecodingKey::from_secret(self.jwt_secret.refresh_secret.as_ref()),
            &validation,
        )
        .map_err(|e| {
            tracing::error!("jwt decode error: {}", e);
            constants::CODE_PARAMETER_ERROR
        })?;

        // check token type
        if token_data.claims.token_type != "refresh" {
            return Err(constants::CODE_PARAMETER_ERROR);
        }

        let existing_user: Option<crate::models::user::User> = self
            .repo
            .find_by_email(&token_data.claims.sub)
            .await
            .map_err(|e| {
                tracing::error!("database find email error: {}", e);
                constants::CODE_DATE_OPERATION_ERROR
            })?;

        if !existing_user.is_some() {
            return Err(constants::CODE_ACCOUNT_NOT_EXISTS);
        }

        // access token
        let now = OffsetDateTime::now_utc();
        let access_exp = now + Duration::seconds(self.jwt_secret.access_validity_period);

        let access_claims = AccessTokenClaims {
            sub: existing_user.as_ref().unwrap().email.clone(),
            exp: access_exp.unix_timestamp(),
            iat: now.unix_timestamp(),
            role: existing_user.as_ref().unwrap().role.clone(),
        };

        let access_token = encode(
            &Header::default(), // default use HS256
            &access_claims,
            &EncodingKey::from_secret(self.jwt_secret.access_secret.as_ref()),
        )
        .map_err(|e| {
            tracing::error!("jwt encode error: {}", e);
            constants::CODE_INTERNAL_SERVER_ERROR
        })?;

        let reply = RefreshTokenReply {
            access_token: access_token,
            access_expire_time: access_exp.unix_timestamp(),
        };
        Ok(reply)
    }
}

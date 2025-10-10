use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: String,  // email
    pub exp: i64,     // exp
    pub iat: i64,     // iat
    pub role: String, // role
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: String,        // email
    pub exp: i64,           // exp
    pub token_type: String, // refresh token
}

pub struct JwtSecret {
    pub access_secret: String,
    pub access_validity_period: i64,
    pub refresh_secret: String,
    pub refresh_validity_period: i64,
}

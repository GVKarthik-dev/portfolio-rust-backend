use crate::errors::{ApiError, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
}

pub fn hash_password(password: &str) -> Result<String> {
    bcrypt::hash(password, 12).map_err(|_| {
        ApiError::InternalServerError("Failed to hash password".to_string())
    })
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    bcrypt::verify(password, hash).map_err(|_| {
        ApiError::InternalServerError("Failed to verify password".to_string())
    })
}

pub fn create_jwt(user_id: &str) -> Result<String> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let now = chrono::Utc::now().timestamp();
    let expires_in = now + 86400 * 7; // 7 days

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expires_in,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| ApiError::InternalServerError("Failed to create JWT".to_string()))
}

#[allow(dead_code)]
pub fn verify_jwt(token: &str) -> Result<Claims> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))
}

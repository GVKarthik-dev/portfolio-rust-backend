use crate::auth::{create_jwt, hash_password};
use crate::errors::{ApiError, Result};
use crate::models::{AuthResponse, LoginRequest, RegisterRequest, User};
use actix_web::web;
use chrono::Utc;
use uuid::Uuid;

pub async fn register(req: web::Json<RegisterRequest>) -> Result<web::Json<AuthResponse>> {
    if req.username.is_empty() || req.email.is_empty() || req.password.is_empty() {
        return Err(ApiError::BadRequest(
            "Username, email, and password are required".to_string(),
        ));
    }

    let password_hash = hash_password(&req.password)?;
    let user_id = Uuid::new_v4().to_string();

    let user = User {
        id: user_id,
        username: req.username.clone(),
        email: req.email.clone(),
        password_hash,
        created_at: Utc::now(),
    };

    let token = create_jwt(&user.id)?;

    Ok(web::Json(AuthResponse {
        user,
        token,
    }))
}

pub async fn login(req: web::Json<LoginRequest>) -> Result<web::Json<AuthResponse>> {
    if req.username.is_empty() || req.password.is_empty() {
        return Err(ApiError::BadRequest(
            "Username and password are required".to_string(),
        ));
    }

    // In a real application, you would query the database
    // For now, we'll return a mock user
    let user = User {
        id: Uuid::new_v4().to_string(),
        username: req.username.clone(),
        email: "user@example.com".to_string(),
        password_hash: "mock_hash".to_string(),
        created_at: Utc::now(),
    };

    let token = create_jwt(&user.id)?;

    Ok(web::Json(AuthResponse {
        user,
        token,
    }))
}

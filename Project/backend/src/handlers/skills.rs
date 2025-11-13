use crate::errors::{ApiError, Result};
use crate::models::{CreateSkillRequest, Skill, UpdateSkillRequest};
use actix_web::web;
use chrono::Utc;
use uuid::Uuid;

pub async fn list_skills() -> Result<web::Json<Vec<Skill>>> {
    // In a real application, you would fetch from database
    Ok(web::Json(Vec::new()))
}

pub async fn get_skill(skill_id: web::Path<String>) -> Result<web::Json<Skill>> {
    Err(ApiError::NotFound(format!(
        "Skill {} not found",
        skill_id
    )))
}

pub async fn create_skill(req: web::Json<CreateSkillRequest>) -> Result<web::Json<Skill>> {
    if req.name.is_empty() || req.category.is_empty() {
        return Err(ApiError::BadRequest(
            "Name and category are required".to_string(),
        ));
    }

    let skill = Skill {
        id: Uuid::new_v4().to_string(),
        user_id: "user_id".to_string(), // Would come from JWT token
        name: req.name.clone(),
        category: req.category.clone(),
        proficiency: req.proficiency.clone(),
        created_at: Utc::now(),
    };

    Ok(web::Json(skill))
}

pub async fn update_skill(
    skill_id: web::Path<String>,
    _req: web::Json<UpdateSkillRequest>,
) -> Result<web::Json<Skill>> {
    Err(ApiError::NotFound(format!(
        "Skill {} not found",
        skill_id
    )))
}

pub async fn delete_skill(skill_id: web::Path<String>) -> Result<web::Json<serde_json::Value>> {
    Ok(web::Json(serde_json::json!({
        "message": format!("Skill {} deleted", skill_id)
    })))
}

use crate::errors::{ApiError, Result};
use crate::models::{CreateProjectRequest, Project, UpdateProjectRequest};
use actix_web::web;
use chrono::Utc;
use uuid::Uuid;

pub async fn list_projects() -> Result<web::Json<Vec<Project>>> {
    // In a real application, you would fetch from database
    Ok(web::Json(Vec::new()))
}

pub async fn get_project(project_id: web::Path<String>) -> Result<web::Json<Project>> {
    Err(ApiError::NotFound(format!(
        "Project {} not found",
        project_id
    )))
}

pub async fn create_project(
    req: web::Json<CreateProjectRequest>,
) -> Result<web::Json<Project>> {
    if req.title.is_empty() || req.description.is_empty() {
        return Err(ApiError::BadRequest(
            "Title and description are required".to_string(),
        ));
    }

    let project = Project {
        id: Uuid::new_v4().to_string(),
        user_id: "user_id".to_string(), // Would come from JWT token
        title: req.title.clone(),
        description: req.description.clone(),
        technologies: req.technologies.clone(),
        repository_url: req.repository_url.clone(),
        live_url: req.live_url.clone(),
        image_url: req.image_url.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    Ok(web::Json(project))
}

pub async fn update_project(
    project_id: web::Path<String>,
    _req: web::Json<UpdateProjectRequest>,
) -> Result<web::Json<Project>> {
    Err(ApiError::NotFound(format!(
        "Project {} not found",
        project_id
    )))
}

pub async fn delete_project(project_id: web::Path<String>) -> Result<web::Json<serde_json::Value>> {
    Ok(web::Json(serde_json::json!({
        "message": format!("Project {} deleted", project_id)
    })))
}

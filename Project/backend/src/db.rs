use crate::models::{User, Project, Skill};
use crate::errors::{ApiError, Result};
use std::sync::Mutex;
use std::collections::HashMap;

pub struct Database {
    users: Mutex<HashMap<String, User>>,
    projects: Mutex<HashMap<String, Project>>,
    skills: Mutex<HashMap<String, Skill>>,
}

#[allow(dead_code)]
impl Database {
    pub fn new() -> Self {
        Database {
            users: Mutex::new(HashMap::new()),
            projects: Mutex::new(HashMap::new()),
            skills: Mutex::new(HashMap::new()),
        }
    }

    // User operations
    pub fn create_user(&self, user: User) -> Result<User> {
        let mut users = self.users.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock users".to_string())
        })?;
        users.insert(user.id.clone(), user.clone());
        Ok(user)
    }

    pub fn get_user(&self, id: &str) -> Result<User> {
        let users = self.users.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock users".to_string())
        })?;
        users
            .get(id)
            .cloned()
            .ok_or_else(|| ApiError::NotFound("User not found".to_string()))
    }

    pub fn find_user_by_username(&self, username: &str) -> Result<User> {
        let users = self.users.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock users".to_string())
        })?;
        users
            .values()
            .find(|u| u.username == username)
            .cloned()
            .ok_or_else(|| ApiError::NotFound("User not found".to_string()))
    }

    // Project operations
    pub fn create_project(&self, project: Project) -> Result<Project> {
        let mut projects = self.projects.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock projects".to_string())
        })?;
        projects.insert(project.id.clone(), project.clone());
        Ok(project)
    }

    pub fn get_project(&self, id: &str) -> Result<Project> {
        let projects = self.projects.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock projects".to_string())
        })?;
        projects
            .get(id)
            .cloned()
            .ok_or_else(|| ApiError::NotFound("Project not found".to_string()))
    }

    pub fn list_projects(&self, user_id: &str) -> Result<Vec<Project>> {
        let projects = self.projects.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock projects".to_string())
        })?;
        let result: Vec<Project> = projects
            .values()
            .filter(|p| p.user_id == user_id)
            .cloned()
            .collect();
        Ok(result)
    }

    pub fn update_project(&self, project: Project) -> Result<Project> {
        let mut projects = self.projects.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock projects".to_string())
        })?;
        projects.insert(project.id.clone(), project.clone());
        Ok(project)
    }

    pub fn delete_project(&self, id: &str) -> Result<()> {
        let mut projects = self.projects.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock projects".to_string())
        })?;
        projects.remove(id).ok_or_else(|| ApiError::NotFound("Project not found".to_string()))?;
        Ok(())
    }

    // Skill operations
    pub fn create_skill(&self, skill: Skill) -> Result<Skill> {
        let mut skills = self.skills.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock skills".to_string())
        })?;
        skills.insert(skill.id.clone(), skill.clone());
        Ok(skill)
    }

    pub fn get_skill(&self, id: &str) -> Result<Skill> {
        let skills = self.skills.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock skills".to_string())
        })?;
        skills
            .get(id)
            .cloned()
            .ok_or_else(|| ApiError::NotFound("Skill not found".to_string()))
    }

    pub fn list_skills(&self, user_id: &str) -> Result<Vec<Skill>> {
        let skills = self.skills.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock skills".to_string())
        })?;
        let result: Vec<Skill> = skills
            .values()
            .filter(|s| s.user_id == user_id)
            .cloned()
            .collect();
        Ok(result)
    }

    pub fn update_skill(&self, skill: Skill) -> Result<Skill> {
        let mut skills = self.skills.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock skills".to_string())
        })?;
        skills.insert(skill.id.clone(), skill.clone());
        Ok(skill)
    }

    pub fn delete_skill(&self, id: &str) -> Result<()> {
        let mut skills = self.skills.lock().map_err(|_| {
            ApiError::InternalServerError("Failed to lock skills".to_string())
        })?;
        skills.remove(id).ok_or_else(|| ApiError::NotFound("Skill not found".to_string()))?;
        Ok(())
    }
}

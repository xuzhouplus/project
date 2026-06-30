use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Environment {
    pub id: i64,
    pub project_id: i64,
    pub name: String,
    pub url: Option<String>,
    pub git_branch: Option<String>,
    pub ssh_host: Option<String>,
    pub ssh_password: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateEnvironment {
    pub project_id: i64,
    pub name: String,
    pub url: Option<String>,
    pub git_branch: Option<String>,
    pub ssh_host: Option<String>,
    pub ssh_password: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateEnvironment {
    pub id: i64,
    pub name: String,
    pub url: Option<String>,
    pub git_branch: Option<String>,
    pub ssh_host: Option<String>,
    pub ssh_password: Option<String>,
    pub db_host: Option<String>,
    pub db_port: Option<i32>,
    pub db_name: Option<String>,
    pub db_username: Option<String>,
    pub db_password: Option<String>,
    pub description: Option<String>,
}

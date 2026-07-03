use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub git_url: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateProject {
    pub name: String,
    pub git_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateProject {
    pub id: i64,
    pub name: String,
    pub git_url: String,
    pub description: Option<String>,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListProjectsQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
    pub keyword: Option<String>,
}

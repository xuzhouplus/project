use crate::api::{ApiResponse, IdResult};
use crate::db::DbState;
use crate::models::{CreateProject, Project, UpdateProject};
use tauri::State;

#[tauri::command]
pub async fn create_project(
    db: State<'_, DbState>,
    input: CreateProject,
) -> Result<ApiResponse<Project>, String> {
    Ok(ApiResponse::from_result(
        sqlx::query_as::<_, Project>(
            "INSERT INTO projects (name, git_url, description)
             VALUES (?, ?, ?)
             RETURNING *",
        )
        .bind(&input.name)
        .bind(&input.git_url)
        .bind(&input.description)
        .fetch_one(db.pool())
        .await
        .map_err(|e| e.to_string()),
    ))
}

#[tauri::command]
pub async fn get_project(
    db: State<'_, DbState>,
    id: i64,
) -> Result<ApiResponse<Project>, String> {
    Ok(match sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE id = ?")
        .bind(id)
        .fetch_optional(db.pool())
        .await
    {
        Ok(Some(project)) => ApiResponse::success(project),
        Ok(None) => ApiResponse::not_found("project", id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

#[tauri::command]
pub async fn list_projects(db: State<'_, DbState>) -> Result<ApiResponse<Vec<Project>>, String> {
    Ok(ApiResponse::from_result(
        sqlx::query_as::<_, Project>("SELECT * FROM projects ORDER BY name ASC")
            .fetch_all(db.pool())
            .await
            .map_err(|e| e.to_string()),
    ))
}

#[tauri::command]
pub async fn update_project(
    db: State<'_, DbState>,
    input: UpdateProject,
) -> Result<ApiResponse<Project>, String> {
    Ok(match sqlx::query_as::<_, Project>(
        "UPDATE projects
         SET name = ?, git_url = ?, description = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.git_url)
    .bind(&input.description)
    .bind(input.id)
    .fetch_optional(db.pool())
    .await
    {
        Ok(Some(project)) => ApiResponse::success(project),
        Ok(None) => ApiResponse::not_found("project", input.id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

#[tauri::command]
pub async fn delete_project(
    db: State<'_, DbState>,
    id: i64,
) -> Result<ApiResponse<IdResult>, String> {
    Ok(match sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(id)
        .execute(db.pool())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => ApiResponse::success(IdResult { id }),
        Ok(_) => ApiResponse::not_found("project", id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

use crate::api::{ApiResponse, IdResult, PaginatedList};
use crate::db::DbState;
use crate::models::{CreateProject, ListProjectsQuery, Project, UpdateProject};
use tauri::State;

fn normalized_keyword(keyword: Option<String>) -> Option<String> {
    keyword
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
        .map(|k| format!("%{k}%"))
}

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
pub async fn list_projects(
    db: State<'_, DbState>,
    query: ListProjectsQuery,
) -> Result<ApiResponse<PaginatedList<Project>>, String> {
    let page = query.page.max(1);
    let page_size = query.page_size.clamp(1, 100);
    let offset = (page - 1) * page_size;
    let like_pattern = normalized_keyword(query.keyword);

    let result = if let Some(pattern) = like_pattern {
        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM projects
             WHERE name LIKE ? OR IFNULL(description, '') LIKE ?",
        )
        .bind(&pattern)
        .bind(&pattern)
        .fetch_one(db.pool())
        .await
        .map_err(|e| e.to_string())?;

        let items = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects
             WHERE name LIKE ? OR IFNULL(description, '') LIKE ?
             ORDER BY LOWER(name), name COLLATE BINARY DESC
             LIMIT ? OFFSET ?",
        )
        .bind(&pattern)
        .bind(&pattern)
        .bind(page_size)
        .bind(offset)
        .fetch_all(db.pool())
        .await
        .map_err(|e| e.to_string())?;

        (items, total)
    } else {
        let total = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(db.pool())
            .await
            .map_err(|e| e.to_string())?;

        let items = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects ORDER BY LOWER(name), name COLLATE BINARY DESC ASC LIMIT ? OFFSET ?",
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(db.pool())
        .await
        .map_err(|e| e.to_string())?;

        (items, total)
    };

    let has_more = page * page_size < result.1;

    Ok(ApiResponse::success(PaginatedList {
        items: result.0,
        total: result.1,
        page,
        page_size,
        has_more,
    }))
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

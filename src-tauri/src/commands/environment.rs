use crate::api::{ApiResponse, IdResult};
use crate::crypto::CryptoState;
use crate::db::DbState;
use crate::models::{CreateEnvironment, Environment, UpdateEnvironment};
use tauri::State;

fn decrypt_environment(env: Environment, crypto: &CryptoState) -> Result<Environment, String> {
    Ok(Environment {
        ssh_password: crypto.decrypt_optional(env.ssh_password)?,
        db_password: crypto.decrypt_optional(env.db_password)?,
        ..env
    })
}

#[tauri::command]
pub async fn create_environment(
    db: State<'_, DbState>,
    crypto: State<'_, CryptoState>,
    input: CreateEnvironment,
) -> Result<ApiResponse<Environment>, String> {
    let ssh_password = match crypto.encrypt_optional(input.ssh_password) {
        Ok(value) => value,
        Err(error) => return Ok(ApiResponse::internal(error)),
    };
    let db_password = match crypto.encrypt_optional(input.db_password) {
        Ok(value) => value,
        Err(error) => return Ok(ApiResponse::internal(error)),
    };

    Ok(match sqlx::query_as::<_, Environment>(
        "INSERT INTO environments (
            project_id, name, url, git_branch,
            ssh_host, ssh_password,
            db_host, db_port, db_name, db_username, db_password,
            description
         )
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
         RETURNING *",
    )
    .bind(input.project_id)
    .bind(&input.name)
    .bind(&input.url)
    .bind(&input.git_branch)
    .bind(&input.ssh_host)
    .bind(&ssh_password)
    .bind(&input.db_host)
    .bind(input.db_port)
    .bind(&input.db_name)
    .bind(&input.db_username)
    .bind(&db_password)
    .bind(&input.description)
    .fetch_one(db.pool())
    .await
    {
        Ok(env) => match decrypt_environment(env, &crypto) {
            Ok(env) => ApiResponse::success(env),
            Err(error) => ApiResponse::internal(error),
        },
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

#[tauri::command]
pub async fn get_environment(
    db: State<'_, DbState>,
    crypto: State<'_, CryptoState>,
    id: i64,
) -> Result<ApiResponse<Environment>, String> {
    Ok(match sqlx::query_as::<_, Environment>("SELECT * FROM environments WHERE id = ?")
        .bind(id)
        .fetch_optional(db.pool())
        .await
    {
        Ok(Some(env)) => match decrypt_environment(env, &crypto) {
            Ok(env) => ApiResponse::success(env),
            Err(error) => ApiResponse::internal(error),
        },
        Ok(None) => ApiResponse::not_found("environment", id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

#[tauri::command]
pub async fn list_environments(
    db: State<'_, DbState>,
    crypto: State<'_, CryptoState>,
    project_id: i64,
) -> Result<ApiResponse<Vec<Environment>>, String> {
    let envs = match sqlx::query_as::<_, Environment>(
        "SELECT * FROM environments
         WHERE project_id = ?
         ORDER BY id",
    )
    .bind(project_id)
    .fetch_all(db.pool())
    .await
    {
        Ok(envs) => envs,
        Err(error) => return Ok(ApiResponse::internal(error.to_string())),
    };

    let mut decrypted = Vec::with_capacity(envs.len());
    for env in envs {
        match decrypt_environment(env, &crypto) {
            Ok(env) => decrypted.push(env),
            Err(error) => return Ok(ApiResponse::internal(error)),
        }
    }

    Ok(ApiResponse::success(decrypted))
}

#[tauri::command]
pub async fn update_environment(
    db: State<'_, DbState>,
    crypto: State<'_, CryptoState>,
    input: UpdateEnvironment,
) -> Result<ApiResponse<Environment>, String> {
    let ssh_password = match crypto.encrypt_optional(input.ssh_password) {
        Ok(value) => value,
        Err(error) => return Ok(ApiResponse::internal(error)),
    };
    let db_password = match crypto.encrypt_optional(input.db_password) {
        Ok(value) => value,
        Err(error) => return Ok(ApiResponse::internal(error)),
    };

    Ok(match sqlx::query_as::<_, Environment>(
        "UPDATE environments
         SET name = ?, url = ?, git_branch = ?,
             ssh_host = ?, ssh_password = ?,
             db_host = ?, db_port = ?, db_name = ?, db_username = ?,
             db_password = ?, description = ?
         WHERE id = ?
         RETURNING *",
    )
    .bind(&input.name)
    .bind(&input.url)
    .bind(&input.git_branch)
    .bind(&input.ssh_host)
    .bind(&ssh_password)
    .bind(&input.db_host)
    .bind(input.db_port)
    .bind(&input.db_name)
    .bind(&input.db_username)
    .bind(&db_password)
    .bind(&input.description)
    .bind(input.id)
    .fetch_optional(db.pool())
    .await
    {
        Ok(Some(env)) => match decrypt_environment(env, &crypto) {
            Ok(env) => ApiResponse::success(env),
            Err(error) => ApiResponse::internal(error),
        },
        Ok(None) => ApiResponse::not_found("environment", input.id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

#[tauri::command]
pub async fn delete_environment(
    db: State<'_, DbState>,
    id: i64,
) -> Result<ApiResponse<IdResult>, String> {
    Ok(match sqlx::query("DELETE FROM environments WHERE id = ?")
        .bind(id)
        .execute(db.pool())
        .await
    {
        Ok(result) if result.rows_affected() > 0 => ApiResponse::success(IdResult { id }),
        Ok(_) => ApiResponse::not_found("environment", id),
        Err(error) => ApiResponse::internal(error.to_string()),
    })
}

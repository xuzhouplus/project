use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::path::{Path, PathBuf};
use tauri::{App, AppHandle, Manager};

pub struct DbState {
    pool: SqlitePool,
}

impl DbState {
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

pub fn db_path(app: &AppHandle) -> tauri::Result<PathBuf> {
    let dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("app.db"))
}

pub async fn create_pool(db_path: &Path) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true)
        .foreign_keys(true);

    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}

pub fn setup_db(app: &App) -> tauri::Result<()> {
    let handle = app.handle().clone();
    let path = db_path(&handle)?;

    let pool = tauri::async_runtime::block_on(create_pool(&path)).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("failed to connect database: {e}"),
        )
    })?;

    app.manage(DbState { pool });
    Ok(())
}

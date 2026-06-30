use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migration() -> Migration {
    Migration {
        version: 3,
        description: "add git_branch column to environments",
        sql: "ALTER TABLE environments ADD COLUMN git_branch TEXT;",
        kind: MigrationKind::Up,
    }
}

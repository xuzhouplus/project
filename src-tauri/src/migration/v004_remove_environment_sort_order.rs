use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migration() -> Migration {
    Migration {
        version: 4,
        description: "remove sort_order column from environments",
        sql: "
            DROP INDEX IF EXISTS idx_environments_project_sort;
            ALTER TABLE environments DROP COLUMN sort_order;
        ",
        kind: MigrationKind::Up,
    }
}

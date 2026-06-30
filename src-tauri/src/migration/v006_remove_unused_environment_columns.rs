use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migration() -> Migration {
    Migration {
        version: 6,
        description: "remove unused ssh and db columns from environments",
        sql: "
            ALTER TABLE environments DROP COLUMN ssh_port;
            ALTER TABLE environments DROP COLUMN ssh_username;
            ALTER TABLE environments DROP COLUMN ssh_private_key;
            ALTER TABLE environments DROP COLUMN db_type;
        ",
        kind: MigrationKind::Up,
    }
}

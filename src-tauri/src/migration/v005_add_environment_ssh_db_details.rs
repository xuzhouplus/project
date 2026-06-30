use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migration() -> Migration {
    Migration {
        version: 5,
        description: "add ssh and db detail columns to environments",
        sql: "
            ALTER TABLE environments ADD COLUMN ssh_port INTEGER;
            ALTER TABLE environments ADD COLUMN ssh_username TEXT;
            ALTER TABLE environments ADD COLUMN ssh_private_key TEXT;
            ALTER TABLE environments ADD COLUMN db_type TEXT;
            ALTER TABLE environments ADD COLUMN db_port INTEGER;
            ALTER TABLE environments ADD COLUMN db_username TEXT;
        ",
        kind: MigrationKind::Up,
    }
}

use tauri_plugin_sql::{Migration, MigrationKind};

pub fn migration() -> Migration {
    Migration {
        version: 2,
        description: "create projects and environments tables",
        sql: "
            CREATE TABLE IF NOT EXISTS projects (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT    NOT NULL UNIQUE,
                git_url     TEXT    NOT NULL,
                description TEXT,
                created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
                updated_at  TEXT    NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS environments (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                project_id    INTEGER NOT NULL,
                name          TEXT    NOT NULL,
                url           TEXT,
                ssh_host      TEXT,
                ssh_password  TEXT,
                db_host       TEXT,
                db_name       TEXT,
                db_password   TEXT,
                description   TEXT,
                sort_order    INTEGER NOT NULL DEFAULT 0,
                created_at    TEXT    NOT NULL DEFAULT (datetime('now')),
                updated_at    TEXT    NOT NULL DEFAULT (datetime('now')),
                FOREIGN KEY (project_id)
                    REFERENCES projects(id)
                    ON DELETE CASCADE
                    ON UPDATE CASCADE,
                UNIQUE (project_id, name)
            );

            CREATE INDEX IF NOT EXISTS idx_environments_project_id
                ON environments (project_id);

            CREATE INDEX IF NOT EXISTS idx_environments_project_sort
                ON environments (project_id, sort_order);

            CREATE TRIGGER IF NOT EXISTS trg_projects_updated_at
            AFTER UPDATE ON projects
            FOR EACH ROW
            BEGIN
                UPDATE projects
                SET updated_at = datetime('now')
                WHERE id = OLD.id;
            END;

            CREATE TRIGGER IF NOT EXISTS trg_environments_updated_at
            AFTER UPDATE ON environments
            FOR EACH ROW
            BEGIN
                UPDATE environments
                SET updated_at = datetime('now')
                WHERE id = OLD.id;
            END;
        ",
        kind: MigrationKind::Up,
    }
}

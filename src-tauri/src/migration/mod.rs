mod v001_create_notes;
mod v002_create_projects_and_environments;
mod v003_add_git_branch_to_environments;
mod v004_remove_environment_sort_order;
mod v005_add_environment_ssh_db_details;
mod v006_remove_unused_environment_columns;

use tauri_plugin_sql::Migration;

pub fn migrations() -> Vec<Migration> {
    vec![
        v001_create_notes::migration(),
        v002_create_projects_and_environments::migration(),
        v003_add_git_branch_to_environments::migration(),
        v004_remove_environment_sort_order::migration(),
        v005_add_environment_ssh_db_details::migration(),
        v006_remove_unused_environment_columns::migration(),
    ]
}

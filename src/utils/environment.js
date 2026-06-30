/**
 * @param {import("../api/environment.js").Environment} environment
 */
export function mapEnvironmentFromApi(environment) {
  return {
    id: environment.id,
    project_id: environment.project_id,
    key: String(environment.id),
    name: environment.name,
    branch: environment.git_branch,
    url: environment.url,
    description: environment.description,
    git_branch: environment.git_branch,
    ssh_host: environment.ssh_host,
    ssh_password: environment.ssh_password,
    db_host: environment.db_host,
    db_port: environment.db_port,
    db_name: environment.db_name,
    db_username: environment.db_username,
    db_password: environment.db_password,
    created_at: environment.created_at,
    updated_at: environment.updated_at,
  };
}

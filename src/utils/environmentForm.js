export function createDefaultEnvironmentForm() {
  return {
    ssh: {
      host: "",
      password: "",
    },
    db: {
      host: "",
      port: null,
      database: "",
      username: "",
      password: "",
    },
  };
}

/**
 * @param {ReturnType<typeof createDefaultEnvironmentForm>} formData
 */
export function mapEnvironmentFormToApi(formData) {
  const empty = (value) => {
    const trimmed = value?.trim?.() ?? value;
    return trimmed === "" || trimmed === undefined ? null : trimmed;
  };

  return {
    ssh_host: empty(formData.ssh.host),
    ssh_password: empty(formData.ssh.password),
    db_host: empty(formData.db.host),
    db_port: formData.db.port ?? null,
    db_name: empty(formData.db.database),
    db_username: empty(formData.db.username),
    db_password: empty(formData.db.password),
  };
}

/**
 * @param {import("../api/environment.js").Environment | Record<string, unknown>} environment
 */
export function mapEnvironmentToForm(environment) {
  return {
    name: environment.name ?? "",
    git_branch: environment.git_branch ?? environment.branch ?? "",
    url: environment.url ?? "",
    description: environment.description ?? "",
    envForm: {
      ssh: {
        host: environment.ssh_host ?? "",
        password: environment.ssh_password ?? "",
      },
      db: {
        host: environment.db_host ?? "",
        port: environment.db_port ?? 3306,
        database: environment.db_name ?? "",
        username: environment.db_username ?? "",
        password: environment.db_password ?? "",
      },
    },
  };
}

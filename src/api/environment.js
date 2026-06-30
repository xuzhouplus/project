import { invokeApi } from "./client.js";

/**
 * @typedef {Object} Environment
 * @property {number} id
 * @property {number} project_id
 * @property {string} name
 * @property {string | null} url
 * @property {string | null} git_branch
 * @property {string | null} ssh_host
 * @property {string | null} ssh_password
 * @property {string | null} db_host
 * @property {number | null} db_port
 * @property {string | null} db_name
 * @property {string | null} db_username
 * @property {string | null} db_password
 * @property {string | null} description
 * @property {string} created_at
 * @property {string} updated_at
 */

/**
 * @typedef {Object} EnvironmentConnectionInput
 * @property {string | null} [ssh_host]
 * @property {string | null} [ssh_password]
 * @property {string | null} [db_host]
 * @property {number | null} [db_port]
 * @property {string | null} [db_name]
 * @property {string | null} [db_username]
 * @property {string | null} [db_password]
 */

/**
 * @typedef {EnvironmentConnectionInput & {
 *   id: number,
 *   name: string,
 *   url?: string | null,
 *   git_branch?: string | null,
 *   description?: string | null,
 * }} UpdateEnvironmentInput
 */

/**
 * @typedef {EnvironmentConnectionInput & {
 *   project_id: number,
 *   name: string,
 *   url?: string | null,
 *   git_branch?: string | null,
 *   description?: string | null,
 * }} CreateEnvironmentInput
 */

/**
 * @param {number} projectId
 * @returns {Promise<Environment[]>}
 */
export function get_environments(projectId) {
  return invokeApi("list_environments", { projectId });
}

/**
 * @param {CreateEnvironmentInput} input
 * @returns {Promise<Environment>}
 */
export function create_environment(input) {
  return invokeApi("create_environment", { input });
}

/**
 * @param {UpdateEnvironmentInput} input
 * @returns {Promise<Environment>}
 */
export function update_environment(input) {
  return invokeApi("update_environment", { input });
}

/**
 * @param {number} id
 * @returns {Promise<{ id: number }>}
 */
export function delete_environment(id) {
  return invokeApi("delete_environment", { id });
}

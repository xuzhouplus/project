import { invokeApi } from "./client.js";

/**
 * @typedef {Object} Project
 * @property {number} id
 * @property {string} name
 * @property {string} git_url
 * @property {string | null} description
 * @property {string} created_at
 * @property {string} updated_at
 */

/** @returns {Promise<Project[]>} */
export function get_projects() {
  return invokeApi("list_projects");
}

/**
 * @typedef {Object} CreateProjectInput
 * @property {string} name
 * @property {string} git_url
 * @property {string | null} [description]
 */

/**
 * @param {CreateProjectInput} input
 * @returns {Promise<Project>}
 */
export function create_project(input) {
  return invokeApi("create_project", { input });
}

/**
 * @typedef {Object} UpdateProjectInput
 * @property {number} id
 * @property {string} name
 * @property {string} git_url
 * @property {string | null} [description]
 */

/**
 * @param {UpdateProjectInput} input
 * @returns {Promise<Project>}
 */
export function update_project(input) {
  return invokeApi("update_project", { input });
}

/**
 * @param {number} id
 * @returns {Promise<{ id: number }>}
 */
export function delete_project(id) {
  return invokeApi("delete_project", { id });
}

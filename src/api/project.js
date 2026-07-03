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

/**
 * @typedef {Object} ListProjectsQuery
 * @property {number} [page]
 * @property {number} [page_size]
 * @property {string | null} [keyword]
 */

/**
 * @typedef {Object} PaginatedProjects
 * @property {Project[]} items
 * @property {number} total
 * @property {number} page
 * @property {number} page_size
 * @property {boolean} has_more
 */

/**
 * @param {ListProjectsQuery} [query]
 * @returns {Promise<PaginatedProjects>}
 */
export function get_projects(query = {}) {
  const keyword = query.keyword?.trim() || null;
  return invokeApi("list_projects", {
    query: {
      page: query.page ?? 1,
      page_size: query.page_size ?? 20,
      keyword,
    },
  });
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

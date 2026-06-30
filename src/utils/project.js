/**
 * @param {import("./project.js").Project} project
 */
export function mapProjectFromApi(project) {
  return {
    id: project.id,
    name: project.name,
    git: project.git_url,
    description: project.description,
    environments: [],
  };
}

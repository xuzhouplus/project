import { ref, computed, provide, inject } from "vue";

const PROJECT_SELECTION_KEY = Symbol("projectSelection");

export function provideProjectSelection(initialProjects = []) {
  const projects = ref(initialProjects);
  const selectedId = ref(initialProjects[0]?.id ?? null);

  const selectedProject = computed(
    () => projects.value.find((p) => p.id === selectedId.value) ?? null
  );

  function selectProject(id) {
    selectedId.value = id;
  }

  const state = {
    projects,
    selectedId,
    selectedProject,
    selectProject,
  };

  provide(PROJECT_SELECTION_KEY, state);
  return state;
}

export function useProjectSelection() {
  const state = inject(PROJECT_SELECTION_KEY);
  if (!state) {
    throw new Error("useProjectSelection must be used within ProjectManager");
  }
  return state;
}

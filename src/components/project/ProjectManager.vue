<script setup>
import { ref, onMounted } from "vue";
import { ElMessage, ElMessageBox } from "element-plus";
import { provideProjectSelection } from "../../composables/useProjectSelection.js";
import {
  get_projects,
  create_project,
  update_project,
  delete_project,
  ApiError,
} from "../../api/index.js";
import { mapProjectFromApi } from "../../utils/project.js";
import ProjectSidebar from "./ProjectSidebar.vue";
import ProjectDetail from "./ProjectDetail.vue";
import ProjectCreateDialog from "./ProjectCreateDialog.vue";

const showProjectDialog = ref(false);
const savingProject = ref(false);
const editingProject = ref(null);

const { projects, selectedId, selectProject } = provideProjectSelection([]);

async function loadProjects() {
  const list = await get_projects();
  projects.value = list.map(mapProjectFromApi);
}

async function handleSelectProject(project) {
  selectProject(project.id);
}

function handleAddProject() {
  editingProject.value = null;
  showProjectDialog.value = true;
}

function handleEditProject(project) {
  editingProject.value = project;
  showProjectDialog.value = true;
}

async function handleSubmitProject(form) {
  savingProject.value = true;
  try {
    if (editingProject.value) {
      await update_project({
        id: editingProject.value.id,
        name: form.name,
        git_url: form.git_url,
        description: form.description,
      });
      await loadProjects();
      selectProject(editingProject.value.id);
      ElMessage.success("项目更新成功");
    } else {
      const created = await create_project(form);
      await loadProjects();
      selectProject(created.id);
      ElMessage.success("项目创建成功");
    }
    showProjectDialog.value = false;
    editingProject.value = null;
  } catch (error) {
    const message =
      error instanceof ApiError
        ? error.message
        : editingProject.value
          ? "项目更新失败"
          : "项目创建失败";
    ElMessage.error(message);
  } finally {
    savingProject.value = false;
  }
}

async function handleDeleteProject(project) {
  try {
    await ElMessageBox.confirm(
      `确定删除项目「${project.name}」吗？关联的环境配置将一并删除。`,
      "删除项目",
      {
        type: "warning",
        confirmButtonText: "删除",
        cancelButtonText: "取消",
        confirmButtonClass: "el-button--danger",
      }
    );
  } catch {
    return;
  }

  try {
    await delete_project(project.id);
    await loadProjects();

    if (projects.value.length > 0) {
      selectProject(projects.value[0].id);
    } else {
      selectProject(null);
    }

    ElMessage.success("项目已删除");
  } catch (error) {
    const message =
      error instanceof ApiError ? error.message : "项目删除失败";
    ElMessage.error(message);
  }
}

onMounted(async () => {
  try {
    await loadProjects();
    if (projects.value.length > 0) {
      selectProject(projects.value[0].id);
    }
  } catch (error) {
    const message =
      error instanceof ApiError ? error.message : "加载项目列表失败";
    ElMessage.error(message);
  }
});
</script>

<template>
  <div class="project-manager">
    <ProjectSidebar
      :projects="projects"
      :selected-id="selectedId"
      @select-project="handleSelectProject"
      @add-project="handleAddProject"
    />
    <ProjectDetail
      @edit-project="handleEditProject"
      @delete-project="handleDeleteProject"
    />
    <ProjectCreateDialog
      v-model="showProjectDialog"
      :project="editingProject"
      :loading="savingProject"
      @submit="handleSubmitProject"
    />
  </div>
</template>

<style scoped>
.project-manager {
  display: flex;
  width: 100%;
  height: 100%;
  overflow: hidden;
}
</style>

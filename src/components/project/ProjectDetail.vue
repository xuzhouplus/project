<script setup>
import {ref, watch} from "vue";
import {Link, Plus, Edit, Delete, CopyDocument} from "@element-plus/icons-vue";
import {ElMessage, ElMessageBox} from "element-plus";
import {useProjectSelection} from "../../composables/useProjectSelection.js";
import {
  get_environments,
  create_environment,
  update_environment,
  delete_environment,
  ApiError,
} from "../../api/index.js";
import {mapEnvironmentFromApi} from "../../utils/environment.js";
import {buildMysqlConnectionCommand, copyToClipboard} from "../../utils/mysql.js";
import EnvironmentCreateDialog from "./EnvironmentCreateDialog.vue";
import CopyableValue from "./CopyableValue.vue";

const emit = defineEmits(["editProject", "deleteProject"]);

const {selectedProject, projects} = useProjectSelection();

const environments = ref([]);
const activeEnvironment = ref("");
const loadingEnvironments = ref(false);
const showEnvironmentDialog = ref(false);
const savingEnvironment = ref(false);
const editingEnvironment = ref(null);

function syncEnvironmentsToProject(projectId, envs) {
  const project = projects.value.find((item) => item.id === projectId);
  if (project) {
    project.environments = envs;
  }
}

async function loadEnvironments(projectId) {
  loadingEnvironments.value = true;
  try {
    const list = await get_environments(projectId);
    environments.value = list.map(mapEnvironmentFromApi);
    syncEnvironmentsToProject(projectId, environments.value);

    if (
        !environments.value.some((env) => env.key === activeEnvironment.value)
    ) {
      activeEnvironment.value = environments.value[0]?.key ?? "";
    }
  } catch (error) {
    environments.value = [];
    activeEnvironment.value = "";
    const message =
        error instanceof ApiError ? error.message : "加载环境列表失败";
    ElMessage.error(message);
  } finally {
    loadingEnvironments.value = false;
  }
}

function handleAddEnvironment() {
  editingEnvironment.value = null;
  showEnvironmentDialog.value = true;
}

function handleEditEnvironment(env) {
  editingEnvironment.value = env;
  showEnvironmentDialog.value = true;
}

async function handleSubmitEnvironment(form) {
  const projectId = selectedProject.value?.id;
  if (!projectId) return;

  savingEnvironment.value = true;
  try {
    if (editingEnvironment.value) {
      const updated = await update_environment({
        id: editingEnvironment.value.id,
        ...form,
      });
      await loadEnvironments(projectId);
      activeEnvironment.value = String(updated.id);
      ElMessage.success("环境更新成功");
    } else {
      const created = await create_environment({
        project_id: projectId,
        ...form,
      });
      await loadEnvironments(projectId);
      activeEnvironment.value = String(created.id);
      ElMessage.success("环境创建成功");
    }
    showEnvironmentDialog.value = false;
    editingEnvironment.value = null;
  } catch (error) {
    const message =
        error instanceof ApiError
            ? error.message
            : editingEnvironment.value
                ? "环境更新失败"
                : "环境创建失败";
    ElMessage.error(message);
  } finally {
    savingEnvironment.value = false;
  }
}

async function handleCopyMysqlCommand(env) {
  try {
    const command = buildMysqlConnectionCommand(env);
    await copyToClipboard(command);
    ElMessage.success("MySQL 连接命令已复制");
  } catch (error) {
    ElMessage.error(error instanceof Error ? error.message : "复制失败");
  }
}

async function handleDeleteEnvironment(env) {
  try {
    await ElMessageBox.confirm(
        `确定删除环境「${env.name}」吗？`,
        "删除环境",
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

  const projectId = selectedProject.value?.id;
  if (!projectId) return;

  try {
    await delete_environment(env.id);
    await loadEnvironments(projectId);
    ElMessage.success("环境已删除");
  } catch (error) {
    const message =
        error instanceof ApiError ? error.message : "环境删除失败";
    ElMessage.error(message);
  }
}

watch(
    () => selectedProject.value?.id,
    (projectId) => {
      if (!projectId) {
        environments.value = [];
        activeEnvironment.value = "";
        return;
      }
      loadEnvironments(projectId);
    },
    {immediate: true}
);
</script>

<template>
  <section v-loading="loadingEnvironments" class="project-detail">
    <template v-if="selectedProject">
      <div class="detail-header">
        <div class="detail-title-row">
          <h2 class="detail-title">{{ selectedProject.name }}</h2>
          <div class="detail-actions">
            <el-button
                :icon="Edit"
                size="small"
                @click="emit('editProject', selectedProject)"
            >
              编辑
            </el-button>
            <el-button
                :icon="Delete"
                size="small"
                type="danger"
                plain
                @click="emit('deleteProject', selectedProject)"
            >
              删除
            </el-button>
          </div>
        </div>

        <div v-if="selectedProject.git" class="detail-git">
          <el-icon>
            <Link/>
          </el-icon>
          <CopyableValue :value="selectedProject.git"/>
        </div>
        <p class="detail-description">
          {{ selectedProject.description }}
        </p>
      </div>

      <div class="env-section">
        <div class="env-tabs-bar">
          <el-tabs
              v-if="environments.length"
              v-model="activeEnvironment"
              class="env-tabs"
              type="border-card"
          >
            <el-tab-pane
                v-for="env in environments"
                :key="env.key"
                :label="env.name"
                :name="env.key"
            >
              <el-scrollbar class="env-container">
                <div class="env-content">
                  <div class="env-actions">
                    <el-button
                        :icon="Edit"
                        size="small"
                        @click="handleEditEnvironment(env)"
                    >
                      编辑
                    </el-button>
                    <el-button
                        :icon="Delete"
                        size="small"
                        type="danger"
                        plain
                        @click="handleDeleteEnvironment(env)"
                    >
                      删除
                    </el-button>
                  </div>

                  <el-descriptions :column="1" border class="env-desc-block" label-width="100px">
                    <el-descriptions-item label="环境名称">
                      {{ env.name }}
                    </el-descriptions-item>
                    <el-descriptions-item label="Git 分支">
                      <el-tag v-if="env.branch" size="small">{{ env.branch }}</el-tag>
                      <span v-else>&nbsp;</span>
                    </el-descriptions-item>
                    <el-descriptions-item label="部署地址">
                      <el-link
                          v-if="env.url"
                          :href="env.url"
                          target="_blank"
                          type="primary"
                      >
                        {{ env.url }}
                      </el-link>
                      <span v-else>—</span>
                    </el-descriptions-item>
                    <el-descriptions-item label="环境说明">
                      {{ env.description }}
                    </el-descriptions-item>
                  </el-descriptions>

                  <div class="env-section-title">SSH 配置</div>
                  <el-descriptions :column="1" border class="env-desc-block" label-width="100px">
                    <el-descriptions-item label="主机">
                      <CopyableValue :value="env.ssh_host"/>
                    </el-descriptions-item>
                    <el-descriptions-item label="密码">
                      <CopyableValue :value="env.ssh_password"/>
                    </el-descriptions-item>
                  </el-descriptions>

                  <div class="env-section-header">
                    <div class="env-section-title">数据库配置</div>
                    <el-button
                        :icon="CopyDocument"
                        size="small"
                        text
                        type="primary"
                        @click="handleCopyMysqlCommand(env)"
                    >
                      复制连接命令
                    </el-button>
                  </div>
                  <el-descriptions :column="1" border class="env-desc-block" label-width="100px">
                    <el-descriptions-item label="主机">
                      <CopyableValue :value="env.db_host"/>
                    </el-descriptions-item>
                    <el-descriptions-item label="端口">
                      <CopyableValue :value="env.db_port"/>
                    </el-descriptions-item>
                    <el-descriptions-item label="数据库名">
                      <CopyableValue :value="env.db_name"/>
                    </el-descriptions-item>
                    <el-descriptions-item label="用户名">
                      <CopyableValue :value="env.db_username"/>
                    </el-descriptions-item>
                    <el-descriptions-item label="密码">
                      <CopyableValue :value="env.db_password"/>
                    </el-descriptions-item>
                  </el-descriptions>
                </div>
              </el-scrollbar>
            </el-tab-pane>
          </el-tabs>

          <span v-else class="env-tabs-placeholder">环境配置</span>

          <el-button
              type="primary"
              :icon="Plus"
              circle
              size="small"
              class="env-add-btn"
              @click="handleAddEnvironment"
          />
        </div>

        <el-empty
            v-if="!environments.length && !loadingEnvironments"
            description="暂无环境配置，点击 + 新增"
        />
      </div>

      <EnvironmentCreateDialog
          v-model="showEnvironmentDialog"
          :environment="editingEnvironment"
          :loading="savingEnvironment"
          @submit="handleSubmitEnvironment"
      />
    </template>

    <el-empty v-else description="请从左侧选择一个项目"/>
  </section>
</template>

<style scoped>
.project-detail {
  display: flex;
  flex-direction: column;
  flex: 1;
  min-width: 0;
  height: 100%;
  padding: 24px;
  background-color: var(--el-bg-color-page);
  box-sizing: border-box;
}

.detail-header {
  margin-bottom: 5px;
  flex-shrink: 0;
  height: 90px;
}

.detail-title-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 8px;
}

.detail-title {
  margin: 0;
  font-size: 22px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.detail-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.detail-git {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.detail-description {
  margin: 6px 0 0;
  font-size: 12px;
  line-height: 1.5;
  color: var(--el-text-color-secondary);
}

.env-section {
  flex: 1;
  border-radius: var(--el-border-radius-base);
  overflow: hidden;
}

.env-tabs-bar {
  display: flex;
  align-items: flex-start;
  gap: 8px;
  position: relative;
  border-radius: var(--el-border-radius-base);
}

.env-tabs {
  flex: 1;
  min-width: 0;
}

.env-tabs :deep(.el-tabs__header) {
  margin-bottom: 0;
}

.env-tabs-placeholder {
  flex: 1;
  padding-top: 10px;
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-secondary);
}

.env-add-btn {
  position: absolute;
  z-index: 999;
  right: 10px;
  top: calc((39px - 24px) / 2);
}

.env-container {
  padding: 8px;
}

.env-actions {
  margin-bottom: 16px;
}

.env-section-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.env-section-header {
  display: flex;
  align-items: center;
  gap: 12px;
}

.env-section-header .env-section-title {
  margin: 0;
}

.env-desc-block + .env-section-header,
.env-desc-block + .env-section-title {
  margin-top: 20px;
}
</style>

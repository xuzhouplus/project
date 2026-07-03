<script setup>
import { Menu, Plus, Search, Loading } from "@element-plus/icons-vue";

const props = defineProps({
  projects: {
    type: Array,
    default: () => [],
  },
  selectedId: {
    type: [Number, String],
    default: null,
  },
  loading: {
    type: Boolean,
    default: false,
  },
  hasMore: {
    type: Boolean,
    default: false,
  },
});

const searchKeyword = defineModel("searchKeyword", { type: String, default: "" });

const emit = defineEmits(["selectProject", "addProject", "endReached"]);

function handleSelect(project) {
  emit("selectProject", project);
}

function handleEndReached() {
  if (props.loading || !props.hasMore) return;
  emit("endReached");
}
</script>

<template>
  <aside class="project-sidebar">
    <div class="sidebar-header">
      <span class="sidebar-title">项目列表</span>
      <el-button
        type="primary"
        :icon="Plus"
        circle
        size="small"
        @click="emit('addProject')"
      />
    </div>

    <div class="sidebar-search">
      <el-input
        v-model="searchKeyword"
        placeholder="搜索项目"
        clearable
        :prefix-icon="Search"
      />
    </div>

    <el-scrollbar class="sidebar-list" @end-reached="handleEndReached">
      <ul v-if="projects.length" class="project-list">
        <li
          v-for="project in projects"
          :key="project.id"
          class="project-item"
          :class="{ active: project.id === selectedId }"
          @click="handleSelect(project)"
        >
          <el-icon class="project-icon"><Menu /></el-icon>
          <div class="project-info">
            <span class="project-name">{{ project.name }}</span>
          </div>
        </li>
      </ul>
      <el-empty
        v-else-if="!loading"
        :image-size="64"
        description="未找到匹配项目"
        class="sidebar-empty"
      />
      <div v-if="loading" class="sidebar-loading">
        <el-icon class="is-loading"><Loading /></el-icon>
        <span>加载中...</span>
      </div>
    </el-scrollbar>
  </aside>
</template>

<style scoped>
.project-sidebar {
  display: flex;
  flex-direction: column;
  width: 280px;
  min-width: 280px;
  height: 100%;
  border-right: 1px solid var(--el-border-color-light);
  background-color: var(--el-bg-color);
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.sidebar-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.sidebar-search {
  padding: 12px 16px;
  border-bottom: 1px solid var(--el-border-color-lighter);
}

.sidebar-list {
  flex: 1;
}

.sidebar-empty {
  padding: 32px 0;
}

.sidebar-loading {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 0 16px;
  font-size: 13px;
  color: var(--el-text-color-secondary);
}

.project-list {
  list-style: none;
  margin: 0;
  padding: 8px;
}

.project-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  border-radius: var(--el-border-radius-base);
  cursor: pointer;
  transition: background-color 0.2s;
}

.project-item:hover {
  background-color: var(--el-fill-color-light);
}

.project-item.active {
  background-color: var(--el-color-primary-light-9);
}

.project-item.active .project-name {
  color: var(--el-color-primary);
}

.project-icon {
  margin-top: 2px;
  font-size: 18px;
  color: var(--el-color-primary);
}

.project-info {
  display: flex;
  flex-direction: column;
  gap: 4px;
  min-width: 0;
}

.project-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--el-text-color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.project-desc {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>

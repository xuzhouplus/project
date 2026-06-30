<script setup>
import { createDefaultEnvironmentForm } from "../../utils/environmentForm.js";

const formData = defineModel({
  type: Object,
  default: () => createDefaultEnvironmentForm(),
});
</script>

<template>
  <el-form :model="formData" label-width="100px" label-position="right">
    <div class="form-section">
      <div class="form-section-title">SSH 配置</div>

      <el-form-item label="主机">
        <el-input v-model="formData.ssh.host" placeholder="例如 192.168.1.100" />
      </el-form-item>

      <el-form-item label="密码">
        <el-input
          v-model="formData.ssh.password"
          type="text"
          show-password
          placeholder="SSH 密码（可选）"
        />
      </el-form-item>
    </div>

    <el-divider />

    <div class="form-section">
      <div class="form-section-title">数据库配置</div>

      <el-form-item label="主机">
        <el-input v-model="formData.db.host" placeholder="例如 127.0.0.1" />
      </el-form-item>

      <el-form-item label="端口">
        <el-input-number
          v-model="formData.db.port"
          :min="1"
          :max="65535"
          controls-position="right"
        />
      </el-form-item>

      <el-form-item label="数据库名">
        <el-input v-model="formData.db.database" placeholder="数据库名称" />
      </el-form-item>

      <el-form-item label="用户名">
        <el-input v-model="formData.db.username" placeholder="数据库用户名" />
      </el-form-item>

      <el-form-item label="密码">
        <el-input
          v-model="formData.db.password"
          type="text"
          show-password
          placeholder="数据库密码"
        />
      </el-form-item>
    </div>
  </el-form>
</template>

<style scoped>
.form-section-title {
  margin-bottom: 16px;
  font-size: 15px;
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.form-section :deep(.el-input) {
  width: 100%;
}

.form-section :deep(.el-input-number) {
  width: 160px;
}
</style>

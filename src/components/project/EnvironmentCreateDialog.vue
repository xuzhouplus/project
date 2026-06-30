<script setup>
import { ref, reactive, watch, computed } from "vue";
import EnvironmentForm from "./EnvironmentForm.vue";
import {
  createDefaultEnvironmentForm,
  mapEnvironmentFormToApi,
  mapEnvironmentToForm,
} from "../../utils/environmentForm.js";

const visible = defineModel({ type: Boolean, default: false });

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  environment: {
    type: Object,
    default: null,
  },
});

const emit = defineEmits(["submit"]);

const formRef = ref();
const envForm = ref(createDefaultEnvironmentForm());

const form = reactive({
  name: "",
  git_branch: "",
  url: "",
  description: "",
});

const isEdit = computed(() => !!props.environment);
const dialogTitle = computed(() => (isEdit.value ? "编辑环境" : "新建环境"));

const rules = {
  name: [{ required: true, message: "请输入环境名称", trigger: "blur" }],
};

function resetForm() {
  form.name = "";
  form.git_branch = "";
  form.url = "";
  form.description = "";
  envForm.value = createDefaultEnvironmentForm();
  formRef.value?.resetFields();
}

function fillForm(environment) {
  const mapped = mapEnvironmentToForm(environment);
  form.name = mapped.name;
  form.git_branch = mapped.git_branch;
  form.url = mapped.url;
  form.description = mapped.description;
  envForm.value = mapped.envForm;
}

watch(
  () => [visible.value, props.environment],
  ([open]) => {
    if (!open) {
      resetForm();
      return;
    }
    if (props.environment) {
      fillForm(props.environment);
    } else {
      resetForm();
    }
  }
);

async function handleSubmit() {
  const valid = await formRef.value?.validate().catch(() => false);
  if (!valid) return;

  emit("submit", {
    name: form.name.trim(),
    git_branch: form.git_branch.trim() || null,
    url: form.url.trim() || null,
    description: form.description.trim() || null,
    ...mapEnvironmentFormToApi(envForm.value),
  });
}

function handleClose() {
  visible.value = false;
}
</script>

<template>
  <el-dialog
    v-model="visible"
    :title="dialogTitle"
    width="560px"
    class="environment-form-dialog"
    :close-on-click-modal="!loading"
    :close-on-press-escape="!loading"
    @close="handleClose"
  >
    <el-scrollbar max-height="60vh">
      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="100px"
        @submit.prevent="handleSubmit"
      >
        <el-form-item label="环境名称" prop="name">
          <el-input v-model="form.name" placeholder="例如 开发环境" />
        </el-form-item>
        <el-form-item label="Git 分支" prop="git_branch">
          <el-input v-model="form.git_branch" placeholder="例如 develop" />
        </el-form-item>
        <el-form-item label="部署地址" prop="url">
          <el-input
            v-model="form.url"
            placeholder="例如 https://dev.example.com"
          />
        </el-form-item>
        <el-form-item label="环境说明" prop="description">
          <el-input
            v-model="form.description"
            type="textarea"
            :rows="2"
            placeholder="可选"
          />
        </el-form-item>
      </el-form>

      <EnvironmentForm v-model="envForm" />
    </el-scrollbar>

    <template #footer>
      <el-button :disabled="loading" @click="handleClose">取消</el-button>
      <el-button type="primary" :loading="loading" @click="handleSubmit">
        确定
      </el-button>
    </template>
  </el-dialog>
</template>

<style scoped>
.environment-form-dialog :deep(.el-dialog__body) {
  padding-top: 12px;
}
</style>

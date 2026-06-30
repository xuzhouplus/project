<script setup>
import { ref, reactive, watch, computed } from "vue";

const visible = defineModel({ type: Boolean, default: false });

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  project: {
    type: Object,
    default: null,
  },
});

const emit = defineEmits(["submit"]);

const formRef = ref();
const form = reactive({
  name: "",
  git_url: "",
  description: "",
});

const isEdit = computed(() => !!props.project);
const dialogTitle = computed(() => (isEdit.value ? "编辑项目" : "新建项目"));

const rules = {
  name: [{ required: true, message: "请输入项目名称", trigger: "blur" }],
  git_url: [{ required: true, message: "请输入 Git 地址", trigger: "blur" }],
};

function resetForm() {
  form.name = "";
  form.git_url = "";
  form.description = "";
  formRef.value?.resetFields();
}

function fillForm(project) {
  form.name = project.name ?? "";
  form.git_url = project.git ?? project.git_url ?? "";
  form.description = project.description ?? "";
}

watch(
  () => [visible.value, props.project],
  ([open]) => {
    if (!open) {
      resetForm();
      return;
    }
    if (props.project) {
      fillForm(props.project);
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
    git_url: form.git_url.trim(),
    description: form.description.trim() || null,
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
    width="480px"
    :close-on-click-modal="!loading"
    :close-on-press-escape="!loading"
    @close="handleClose"
  >
    <el-form
      ref="formRef"
      :model="form"
      :rules="rules"
      label-width="80px"
      @submit.prevent="handleSubmit"
    >
      <el-form-item label="项目名称" prop="name">
        <el-input v-model="form.name" placeholder="请输入项目名称" />
      </el-form-item>
      <el-form-item label="Git 地址" prop="git_url">
        <el-input
          v-model="form.git_url"
          placeholder="例如 https://github.com/org/repo"
        />
      </el-form-item>
      <el-form-item label="项目描述" prop="description">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="3"
          placeholder="可选"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button :disabled="loading" @click="handleClose">取消</el-button>
      <el-button type="primary" :loading="loading" @click="handleSubmit">
        确定
      </el-button>
    </template>
  </el-dialog>
</template>

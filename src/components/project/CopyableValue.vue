<script setup>
import { computed } from "vue";
import { ElMessage } from "element-plus";
import { copyToClipboard } from "../../utils/mysql.js";

const props = defineProps({
  value: {
    type: [String, Number],
    default: null,
  },
});

const hasValue = computed(() => {
  if (props.value === null || props.value === undefined) return false;
  if (typeof props.value === "string" && props.value.trim() === "") return false;
  return true;
});

const displayValue = computed(() =>
  hasValue.value ? String(props.value) : "—"
);

async function handleCopy() {
  if (!hasValue.value) return;

  try {
    await copyToClipboard(String(props.value));
    ElMessage.success("已复制");
  } catch {
    ElMessage.error("复制失败");
  }
}
</script>

<template>
  <span
    class="copyable-value"
    :class="{ 'is-copyable': hasValue }"
    @click="handleCopy"
  >
    {{ displayValue }}
  </span>
</template>

<style scoped>
.copyable-value {
  display: inline-block;
}

.copyable-value.is-copyable {
  color: var(--el-color-primary);
  cursor: pointer;
}
</style>

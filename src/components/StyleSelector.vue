<template>
  <div class="style-selector">
    <span class="label">优化风格</span>
    <div class="segmented-control">
      <button
        v-for="option in options"
        :key="option.value"
        class="segment"
        :class="{ active: modelValue === option.value }"
        @click="selectStyle(option.value)"
      >
        {{ option.label }}
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
// 优化风格类型（与后端 OptimizeStyle 对应）
export type OptimizeStyle = "general" | "concise" | "detailed";

defineProps<{
  modelValue: OptimizeStyle;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: OptimizeStyle): void;
}>();

const options = [
  { value: "general" as OptimizeStyle, label: "通用" },
  { value: "concise" as OptimizeStyle, label: "简洁" },
  { value: "detailed" as OptimizeStyle, label: "详细" },
];

function selectStyle(style: OptimizeStyle) {
  emit("update:modelValue", style);
}
</script>

<style scoped>
.style-selector {
  display: flex;
  align-items: center;
  gap: var(--space-md);
}

.label {
  font-size: var(--font-body);
  color: var(--text-secondary);
  white-space: nowrap;
}

/* 苹果分段控件（Segmented Control） */
.segmented-control {
  display: flex;
  background: var(--bg-input);
  border-radius: var(--radius-button);
  padding: 3px;
  gap: 2px;
}

.segment {
  padding: 6px 20px;
  border: none;
  border-radius: var(--radius-button);
  font-size: var(--font-caption);
  font-weight: 500;
  font-family: var(--font-family);
  color: var(--text-primary);
  background: transparent;
  cursor: pointer;
  transition: var(--transition-default);
}

.segment.active {
  background: var(--bg-card);
  color: var(--text-primary);
  box-shadow: var(--shadow-default);
}

.segment:not(.active):hover {
  color: var(--accent-blue);
}
</style>

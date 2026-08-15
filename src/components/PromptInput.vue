<template>
  <div class="prompt-input">
    <textarea
      v-model="input"
      class="apple-input prompt-textarea"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="handleInput"
    />
    <div class="char-count" v-if="showCharCount">
      {{ input.length }} 字符
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

defineProps<{
  placeholder?: string;
  disabled?: boolean;
  showCharCount?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const input = ref<string>("");

function handleInput() {
  emit("update:modelValue", input.value);
}
</script>

<style scoped>
.prompt-input {
  width: 100%;
}

.prompt-textarea {
  min-height: 120px;
  resize: vertical;
  line-height: 1.5;
}

.char-count {
  text-align: right;
  margin-top: var(--space-sm);
  font-size: var(--font-caption);
  color: var(--text-secondary);
}
</style>

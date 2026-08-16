<template>
  <div class="prompt-input">
    <textarea
      ref="textareaRef"
      v-model="input"
      class="apple-input prompt-textarea"
      :placeholder="placeholder"
      :disabled="disabled"
      @input="handleInput"
      @keydown="handleKeydown"
    />
    <div class="input-footer">
      <span class="key-hint">Enter 提交 · Ctrl+Enter 换行</span>
      <span v-if="showCharCount" class="char-count">{{ input.length }} 字符</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { nextTick, ref } from "vue";

defineProps<{
  placeholder?: string;
  disabled?: boolean;
  showCharCount?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
  (e: "submit"): void;
}>();

const input = ref<string>("");
const textareaRef = ref<HTMLTextAreaElement | null>(null);

function handleInput() {
  emit("update:modelValue", input.value);
}

// 键盘交互规范：
// - Enter：提交优化（禁止当作换行）
// - Ctrl+Enter / Shift+Enter：换行（手动插入，不触发优化）
// - isComposing 检查：中文输入法选词时按 Enter 是确认候选词，不干预
function handleKeydown(e: KeyboardEvent) {
  if (e.key !== "Enter") return;
  if (e.isComposing) return; // 输入法选词，不干预

  if (e.ctrlKey || e.shiftKey) {
    // Ctrl+Enter / Shift+Enter：手动插入换行
    e.preventDefault();
    insertNewline();
    return;
  }

  // 普通 Enter：提交
  e.preventDefault();
  emit("submit");
}

// 在光标位置手动插入换行符
function insertNewline() {
  const el = textareaRef.value;
  if (!el) return;

  const start = el.selectionStart;
  const end = el.selectionEnd;
  const newValue = input.value.slice(0, start) + "\n" + input.value.slice(end);

  input.value = newValue;
  emit("update:modelValue", newValue);

  // 光标移到换行符之后
  nextTick(() => {
    el.selectionStart = el.selectionEnd = start + 1;
  });
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

.input-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--space-sm);
}

.key-hint {
  font-size: var(--font-caption);
  color: var(--text-secondary);
}

.char-count {
  font-size: var(--font-caption);
  color: var(--text-secondary);
}
</style>

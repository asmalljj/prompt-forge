<template>
  <div class="result-display">
    <div class="result-header">
      <span class="result-title">优化结果</span>
      <button
        v-if="content"
        class="apple-btn secondary copy-btn"
        @click="copyContent"
      >
        {{ copied ? "已复制 ✓" : "复制" }}
      </button>
    </div>

    <div class="result-body">
      <!-- 加载状态 -->
      <div v-if="loading" class="loading-state">
        <div class="spinner"></div>
        <p>正在优化中...</p>
      </div>

      <!-- 空状态 -->
      <div v-else-if="!content && !error" class="empty-state">
        <p>输入提示词，点击「优化提示词」获取结果</p>
      </div>

      <!-- 错误状态 -->
      <div v-else-if="error" class="error-state">
        <p>{{ error }}</p>
      </div>

      <!-- 结果内容 -->
      <div v-else class="result-content">
        <p class="result-text">{{ content }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  content?: string;
  loading?: boolean;
  error?: string;
}>();

const copied = ref<boolean>(false);

// 内容变化时重置复制状态
watch(
  () => props.content,
  () => {
    copied.value = false;
  }
);

async function copyContent() {
  if (!props.content) return;

  try {
    await navigator.clipboard.writeText(props.content);
    copied.value = true;
    // 2 秒后恢复按钮状态
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch {
    copied.value = false;
  }
}
</script>

<style scoped>
.result-display {
  background: var(--bg-card);
  border-radius: var(--radius-card);
  padding: var(--space-lg);
  box-shadow: var(--shadow-default);
  min-height: 200px;
  display: flex;
  flex-direction: column;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--space-md);
}

.result-title {
  font-size: var(--font-h3);
  font-weight: 600;
}

.copy-btn {
  padding: 6px 16px;
  font-size: var(--font-caption);
}

.result-body {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-md);
  color: var(--text-secondary);
}

.spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--bg-input);
  border-top-color: var(--accent-blue);
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.empty-state {
  color: var(--text-secondary);
  text-align: center;
}

.error-state {
  color: var(--error-red);
  text-align: center;
}

.result-content {
  width: 100%;
  background: var(--bg-input);
  border-radius: var(--radius-input);
  padding: var(--space-md);
}

.result-text {
  white-space: pre-wrap;
  word-break: break-word;
  line-height: 1.6;
  font-size: var(--font-body);
}
</style>

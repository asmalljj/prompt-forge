<template>
  <div class="page">
    <div class="container">
      <!-- 页面标题 -->
      <header class="page-header">
        <h1>PromptForge</h1>
        <p class="subtitle">提示词优化工具 — 把大白话变成 AI 更懂的高质量提示词</p>
      </header>

      <!-- 输入卡片 -->
      <section class="apple-card input-card">
        <PromptInput
          v-model="promptInput"
          :disabled="loading"
          placeholder="输入你的大白话提示词，例如：帮我写一个介绍 Vue 的文案"
          show-char-count
        />
        <div class="input-actions">
          <StyleSelector v-model="style" />
          <button
            class="apple-btn primary optimize-btn"
            :disabled="loading || !promptInput.trim()"
            @click="optimize"
          >
            {{ loading ? "优化中..." : "优化提示词" }}
          </button>
        </div>
      </section>

      <!-- 结果展示 -->
      <section class="result-section">
        <ResultDisplay
          :content="result"
          :loading="loading"
          :error="error"
        />
      </section>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PromptInput from "./components/PromptInput.vue";
import StyleSelector, { type OptimizeStyle } from "./components/StyleSelector.vue";
import ResultDisplay from "./components/ResultDisplay.vue";

// 状态
const promptInput = ref<string>("");
const style = ref<OptimizeStyle>("general");
const result = ref<string>("");
const error = ref<string>("");
const loading = ref<boolean>(false);

// 优化提示词
async function optimize() {
  if (!promptInput.value.trim() || loading.value) return;

  loading.value = true;
  error.value = "";
  result.value = "";

  try {
    const response = await invoke<{
      success: boolean;
      data?: { optimizedPrompt: string };
      error?: string;
    }>("optimize_prompt", {
      request: {
        input: promptInput.value,
        style: style.value,
      },
    });

    if (response.success && response.data) {
      result.value = response.data.optimizedPrompt;
    } else {
      error.value = response.error || "优化失败，请重试";
    }
  } catch (e) {
    error.value = `调用失败: ${e}`;
  } finally {
    loading.value = false;
  }
}
</script>

<style scoped>
.page {
  min-height: 100vh;
  display: flex;
  justify-content: center;
  padding: var(--space-xxl) var(--space-lg);
}

.container {
  width: 100%;
  max-width: 680px;
  display: flex;
  flex-direction: column;
  gap: var(--space-xl);
}

/* 标题区 */
.page-header {
  text-align: center;
}

.page-header h1 {
  font-size: var(--font-h1);
  font-weight: 600;
  letter-spacing: -0.5px;
}

.subtitle {
  margin-top: var(--space-sm);
  font-size: var(--font-body);
  color: var(--text-secondary);
}

/* 输入卡片 */
.input-card {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.input-actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-md);
  flex-wrap: wrap;
}

.optimize-btn {
  min-width: 140px;
}

/* 结果区 */
.result-section {
  width: 100%;
}
</style>

<template>
  <div class="page">
    <div class="container">
      <!-- 顶部栏：标题 + 设置入口 -->
      <header class="top-bar">
        <h1>PromptForge</h1>
        <button class="settings-btn" title="设置" @click="openSettings">
          <span class="gear-icon">⚙︎</span>
        </button>
      </header>
      <p class="subtitle">提示词优化工具 — 把大白话变成 AI 更懂的高质量提示词</p>

      <!-- 未配置 API Key 引导条 -->
      <div v-if="!hasApiKey && !checkingKey" class="warning-banner">
        <span>⚠️ 尚未配置 API Key，请先完成设置</span>
        <button class="banner-btn" @click="openSettings">去设置</button>
      </div>

      <!-- 输入卡片 -->
      <section class="apple-card input-card">
        <PromptInput
          v-model="promptInput"
          :disabled="loading"
          placeholder="输入你的大白话提示词，例如：帮我写一个介绍 Vue 的文案"
          show-char-count
          @submit="optimize"
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

    <!-- 设置弹窗 -->
    <SettingsModal
      v-if="showSettings"
      @saved="onSettingsClosed"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import PromptInput from "./components/PromptInput.vue";
import StyleSelector, { type OptimizeStyle } from "./components/StyleSelector.vue";
import ResultDisplay from "./components/ResultDisplay.vue";
import SettingsModal from "./components/SettingsModal.vue";

// 状态
const promptInput = ref<string>("");
const style = ref<OptimizeStyle>("general");
const result = ref<string>("");
const error = ref<string>("");
const loading = ref<boolean>(false);
const showSettings = ref<boolean>(false);
const hasApiKey = ref<boolean>(false);
const checkingKey = ref<boolean>(true);

// 启动时检查是否已配置 API Key
onMounted(async () => {
  try {
    const response = await invoke<{ success: boolean; data?: { hasApiKey: boolean } }>(
      "get_settings"
    );
    hasApiKey.value = response.success && !!response.data?.hasApiKey;
  } catch {
    hasApiKey.value = false;
  } finally {
    checkingKey.value = false;
  }
});

// 打开设置
function openSettings() {
  showSettings.value = true;
}

// 设置关闭后刷新状态
async function onSettingsClosed() {
  showSettings.value = false;
  // 重新检查 API Key 状态
  try {
    const response = await invoke<{ success: boolean; data?: { hasApiKey: boolean } }>(
      "get_settings"
    );
    hasApiKey.value = response.success && !!response.data?.hasApiKey;
  } catch {
    hasApiKey.value = false;
  }
}

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

/* 顶部栏：标题居中，设置按钮固定右上角 */
.top-bar {
  position: relative;
  text-align: center;
  padding-top: var(--space-md);
}

.top-bar h1 {
  font-size: var(--font-h1);
  font-weight: 600;
  letter-spacing: -0.5px;
}

.settings-btn {
  position: absolute;
  top: var(--space-md);
  right: 0;
  border: none;
  background: var(--bg-input);
  width: 36px;
  height: 36px;
  border-radius: 50%;
  cursor: pointer;
  transition: var(--transition-default);
  display: flex;
  align-items: center;
  justify-content: center;
}

.settings-btn:hover {
  background: var(--bg-input-focus);
  transform: rotate(15deg);
}

.gear-icon {
  font-size: 20px;
  color: var(--text-primary);
}

.subtitle {
  margin-top: -var(--space-lg);
  font-size: var(--font-body);
  color: var(--text-secondary);
  text-align: center;
}

/* 未配置引导条 */
.warning-banner {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #FFF8E1;
  border-radius: var(--radius-card);
  padding: var(--space-md) var(--space-lg);
  font-size: var(--font-body);
  color: #8A6D00;
}

.banner-btn {
  border: none;
  background: var(--accent-blue);
  color: white;
  padding: 6px 16px;
  border-radius: var(--radius-button);
  font-size: var(--font-caption);
  cursor: pointer;
  transition: var(--transition-default);
}

.banner-btn:hover {
  background: var(--accent-blue-hover);
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

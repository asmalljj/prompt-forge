<template>
  <div class="settings-overlay" @click.self="close">
    <div class="settings-modal">
      <div class="modal-header">
        <h3>设置</h3>
        <button class="close-btn" @click="close">✕</button>
      </div>

      <div class="modal-body">
        <!-- 当前配置状态 -->
        <p v-if="alreadyConfigured" class="configured-status">
          ✓ 已配置 API Key，如更换请直接输入新的 Key
        </p>

        <label class="field-label">DeepSeek API Key</label>
        <input
          v-model="apiKey"
          type="password"
          class="apple-input"
          placeholder="sk-..."
          autocomplete="off"
        />
        <p class="field-hint">
          在 <a href="https://platform.deepseek.com/" target="_blank">platform.deepseek.com</a> 申请。
          Key 只保存在本机，不会上传到其他服务。
        </p>

        <p v-if="error" class="error-text">{{ error }}</p>
        <p v-if="success" class="success-text">✓ 保存成功</p>
      </div>

      <div class="modal-footer">
        <button class="apple-btn secondary" @click="close">取消</button>
        <button
          class="apple-btn primary"
          :disabled="saving || !apiKey.trim()"
          @click="save"
        >
          {{ saving ? "保存中..." : "保存" }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emit = defineEmits<{
  (e: "saved"): void;
}>();

const apiKey = ref<string>("");
const saving = ref<boolean>(false);
const error = ref<string>("");
const success = ref<boolean>(false);
const alreadyConfigured = ref<boolean>(false);

// 打开时检查是否已配置
(async () => {
  try {
    const response = await invoke<{ success: boolean; data?: { hasApiKey: boolean } }>(
      "get_settings"
    );
    alreadyConfigured.value = response.success && !!response.data?.hasApiKey;
  } catch {
    alreadyConfigured.value = false;
  }
})();

function close() {
  emit("saved");
}

async function save() {
  if (!apiKey.value.trim() || saving.value) return;

  saving.value = true;
  error.value = "";
  success.value = false;

  try {
    const result = await invoke<{ success: boolean; error?: string }>("save_settings", {
      request: { deepseekApiKey: apiKey.value.trim() },
    });

    if (result.success) {
      success.value = true;
      // 保存成功后 1 秒关闭
      setTimeout(() => {
        close();
      }, 1000);
    } else {
      error.value = result.error || "保存失败";
    }
  } catch (e) {
    error.value = `保存失败: ${e}`;
  } finally {
    saving.value = false;
  }
}
</script>

<style scoped>
.settings-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

.settings-modal {
  width: 420px;
  max-width: 90vw;
  background: var(--bg-card);
  border-radius: var(--radius-card);
  box-shadow: var(--shadow-hover);
  animation: modal-in 0.25s ease;
}

@keyframes modal-in {
  from {
    opacity: 0;
    transform: scale(0.96);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--space-lg);
  border-bottom: 1px solid var(--bg-input);
}

.modal-header h3 {
  font-size: var(--font-h3);
  font-weight: 600;
}

.close-btn {
  border: none;
  background: var(--bg-input);
  width: 28px;
  height: 28px;
  border-radius: 50%;
  cursor: pointer;
  font-size: var(--font-caption);
  color: var(--text-secondary);
  transition: var(--transition-default);
}

.close-btn:hover {
  background: var(--bg-input-focus);
}

.modal-body {
  padding: var(--space-lg);
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.field-label {
  font-size: var(--font-body);
  font-weight: 500;
}

.field-hint {
  font-size: var(--font-caption);
  color: var(--text-secondary);
  line-height: 1.5;
}

.field-hint a {
  color: var(--accent-blue);
  text-decoration: none;
}

.error-text {
  font-size: var(--font-caption);
  color: var(--error-red);
}

.configured-status {
  font-size: var(--font-caption);
  color: var(--success-green);
  background: rgba(52, 199, 89, 0.1);
  padding: 8px 12px;
  border-radius: var(--radius-input);
}

.success-text {
  font-size: var(--font-caption);
  color: var(--success-green);
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-sm);
  padding: var(--space-md) var(--space-lg);
  border-top: 1px solid var(--bg-input);
}
</style>

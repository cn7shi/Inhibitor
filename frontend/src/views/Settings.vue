<template>
  <div class="settings-page">
    <header class="page-header">
      <h2>System Settings</h2>
      <p class="subtitle">Live Configuration Dashboard (Hot-Reloading)</p>
    </header>

    <div class="settings-content" v-if="config">
      <!-- LLM Config Section -->
      <section class="config-section">
        <h3><span class="icon">🤖</span> LLM Provider</h3>
        
        <div class="form-group">
          <label for="model">Model Name</label>
          <input type="text" id="model" v-model="config.model" class="input-field" />
        </div>
        
        <div class="form-group">
          <label for="apiKey">Groq API Key</label>
          <input type="password" id="apiKey" v-model="config.groq_api_key" class="input-field" placeholder="gsk_..." />
        </div>
      </section>

      <!-- Pipeline Config Section -->
      <section class="config-section">
        <h3><span class="icon">⚡</span> Pipeline Strategies</h3>
        
        <div class="form-group">
          <label for="maxRetries">Circuit Breaker Max Retries</label>
          <input type="number" id="maxRetries" v-model.number="config.max_retries" class="input-field" min="0" max="10" />
        </div>
      </section>

      <!-- Notification Config Section -->
      <section class="config-section">
        <h3><span class="icon">🔔</span> Notification Webhooks</h3>
        
        <div class="form-group">
          <label for="feishu">Feishu Webhook URL</label>
          <input type="text" id="feishu" v-model="config.feishu_webhook" class="input-field" placeholder="Leave empty to disable" />
        </div>

        <div class="form-group">
          <label for="dingtalk">DingTalk Webhook URL</label>
          <input type="text" id="dingtalk" v-model="config.dingtalk_webhook" class="input-field" placeholder="Leave empty to disable" />
        </div>

        <div class="form-group">
          <label for="discord">Discord Webhook URL</label>
          <input type="text" id="discord" v-model="config.discord_webhook" class="input-field" placeholder="Leave empty to disable" />
        </div>

        <div class="form-group">
          <label for="keyword">Custom Keyword (for bypassing security)</label>
          <input type="text" id="keyword" v-model="config.notify_keyword" class="input-field" placeholder="e.g. 任务" />
        </div>
      </section>

      <!-- Action Footer -->
      <div class="action-footer">
        <button class="btn btn-primary" @click="saveConfig" :disabled="isSaving">
          {{ isSaving ? 'Saving...' : 'Apply & Hot Reload' }}
        </button>
        <span class="save-status" :class="{ 'show': showSuccess }">✅ Successfully applied instantly!</span>
      </div>
    </div>
    
    <div v-else class="loading-state">
      <div class="spinner"></div>
      <p>Loading configuration...</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';

const API_BASE = 'http://localhost:3000';
const config = ref(null);
const isSaving = ref(false);
const showSuccess = ref(false);

const loadConfig = async () => {
  try {
    const res = await fetch(`${API_BASE}/api/config`);
    if (res.ok) {
      config.value = await res.json();
    } else {
      console.error("Failed to load config", await res.text());
    }
  } catch (e) {
    console.error("Network error while loading config", e);
  }
};

const saveConfig = async () => {
  if (!config.value) return;
  isSaving.value = true;
  showSuccess.value = false;
  
  try {
    const res = await fetch(`${API_BASE}/api/config`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(config.value)
    });
    
    if (res.ok) {
      showSuccess.value = true;
      setTimeout(() => {
        showSuccess.value = false;
      }, 3000);
    } else {
      alert("Failed to save configuration!");
      console.error(await res.text());
    }
  } catch (e) {
    console.error("Network error while saving config", e);
    alert("Network error while saving");
  } finally {
    isSaving.value = false;
  }
};

onMounted(() => {
  loadConfig();
});
</script>

<style scoped>
.settings-page {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.page-header {
  margin-bottom: 2rem;
}

.page-header h2 {
  margin: 0;
  font-size: 1.8rem;
  font-weight: 600;
  color: #fff;
}

.subtitle {
  color: var(--text-secondary);
  margin-top: 0.5rem;
}

.settings-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  max-width: 800px;
  padding-bottom: 3rem;
}

.config-section {
  background: var(--panel-bg);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 12px;
  padding: 1.5rem 2rem;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
}

.config-section h3 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  font-size: 1.2rem;
  color: #fff;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding-bottom: 0.8rem;
}

.form-group {
  margin-bottom: 1.2rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  font-size: 0.9rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.input-field {
  background: rgba(15, 23, 42, 0.6);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  padding: 0.75rem 1rem;
  color: var(--text-primary);
  font-family: 'JetBrains Mono', monospace;
  font-size: 0.9rem;
  transition: all 0.2s;
}

.input-field:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 2px var(--accent-glow);
}

.action-footer {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-top: 1rem;
}

.btn {
  padding: 0.8rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
}

.btn-primary {
  background: var(--accent);
  color: #fff;
  box-shadow: 0 4px 14px 0 var(--accent-glow);
}

.btn-primary:hover:not(:disabled) {
  background: #2563eb;
  transform: translateY(-1px);
}

.btn-primary:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.save-status {
  color: #4ade80;
  font-weight: 500;
  opacity: 0;
  transform: translateX(-10px);
  transition: all 0.3s;
}

.save-status.show {
  opacity: 1;
  transform: translateX(0);
}

.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--text-secondary);
}

.spinner {
  width: 30px;
  height: 30px;
  border: 3px solid rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  border-top-color: var(--accent);
  animation: spin 1s ease-in-out infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>

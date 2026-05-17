<template>
  <div class="monitor-container">
    <div class="header">
      <div class="title-area">
        <h2>Live System Logs</h2>
        <p class="subtitle">Real-time execution trace from Inhibitor Runtime</p>
      </div>
      <div class="actions">
        <label class="auto-scroll-toggle">
          <input type="checkbox" v-model="autoScroll">
          <span>Auto-Scroll</span>
        </label>
        <div class="status">
          <span class="indicator" :class="{ connected: isConnected }"></span>
          {{ isConnected ? 'Connected to Stream' : 'Connecting...' }}
        </div>
        <button @click="clearLogs" class="btn-clear">Clear Logs</button>
      </div>
    </div>
    
    <div class="log-container">
      <div class="terminal" ref="terminalRef" @scroll="handleScroll">
        <div 
          v-for="(log, index) in logs" 
          :key="index" 
          class="log-entry" 
          :class="[getLogLevel(log), { 'highlight-response': log.includes('上游模型回复内容') || log.includes('工具调用最终回复') }]"
        >
          <span class="timestamp">{{ new Date().toLocaleTimeString() }}</span>
          <span class="content">{{ log }}</span>
        </div>
        <div v-if="logs.length === 0" class="empty-state">
          Waiting for system activity...
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, nextTick } from 'vue';

const logs = ref([]);
const isConnected = ref(false);
const autoScroll = ref(true);
const terminalRef = ref(null);
let eventSource = null;

const handleScroll = (e) => {
  const { scrollTop, scrollHeight, clientHeight } = e.target;
  // 判断距离底部是否小于 20px
  if (scrollTop + clientHeight < scrollHeight - 20) {
    autoScroll.value = false;
  } else {
    autoScroll.value = true;
  }
};

const connectSSE = () => {
  if (eventSource) {
    eventSource.close();
  }
  
  eventSource = new EventSource('http://127.0.0.1:3000/api/logs');
  
  eventSource.onopen = () => {
    isConnected.value = true;
  };
  
  eventSource.onmessage = (event) => {
    logs.value.push(event.data);
    scrollToBottom();
  };
  
  eventSource.onerror = () => {
    isConnected.value = false;
    eventSource.close();
    // Try to reconnect after 3 seconds
    setTimeout(connectSSE, 3000);
  };
};

const getLogLevel = (logMsg) => {
  if (logMsg.includes('ERROR')) return 'level-error';
  if (logMsg.includes('WARN')) return 'level-warn';
  if (logMsg.includes('INFO')) return 'level-info';
  if (logMsg.includes('DEBUG')) return 'level-debug';
  return 'level-info';
};

const scrollToBottom = () => {
  nextTick(() => {
    if (terminalRef.value && autoScroll.value) {
      terminalRef.value.scrollTop = terminalRef.value.scrollHeight;
    }
  });
};

const clearLogs = () => {
  logs.value = [];
};

onMounted(() => {
  connectSSE();
});

onUnmounted(() => {
  if (eventSource) {
    eventSource.close();
  }
});
</script>

<style scoped>
.monitor-container {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  background: var(--panel-bg);
  padding: 1.5rem 2rem;
  border-radius: 16px;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 4px 30px rgba(0, 0, 0, 0.1);
}

.title-area h2 {
  margin: 0;
  font-size: 1.4rem;
  font-weight: 600;
  color: var(--text-primary);
}

.subtitle {
  margin: 0.25rem 0 0 0;
  font-size: 0.9rem;
  color: var(--text-secondary);
}

.actions {
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.auto-scroll-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
  cursor: pointer;
  user-select: none;
}

.auto-scroll-toggle input {
  accent-color: var(--accent);
  cursor: pointer;
}

.auto-scroll-toggle:hover {
  color: var(--text-primary);
}

.status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: var(--text-secondary);
  background: rgba(0, 0, 0, 0.2);
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
}

.indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background-color: var(--log-error);
  transition: background-color 0.3s ease;
}

.indicator.connected {
  background-color: #22c55e;
  box-shadow: 0 0 8px #22c55e;
}

.log-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--panel-bg);
  border-radius: 16px;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

.btn-clear {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  color: var(--text-secondary);
  padding: 0.5rem 1.2rem;
  border-radius: 8px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-clear:hover {
  background: rgba(255, 255, 255, 0.1);
  color: var(--text-primary);
  border-color: rgba(255, 255, 255, 0.2);
}

.terminal {
  flex: 1;
  padding: 1.5rem;
  overflow-y: auto;
  font-family: 'JetBrains Mono', 'Fira Code', 'Courier New', monospace;
  font-size: 0.9rem;
  line-height: 1.6;
  scrollbar-width: thin;
  scrollbar-color: rgba(255, 255, 255, 0.2) transparent;
}

.terminal::-webkit-scrollbar {
  width: 8px;
}

.terminal::-webkit-scrollbar-track {
  background: transparent;
}

.terminal::-webkit-scrollbar-thumb {
  background-color: rgba(255, 255, 255, 0.2);
  border-radius: 20px;
}

.log-entry {
  margin-bottom: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: background-color 0.2s;
  display: flex;
  gap: 1.5rem;
}

.log-entry:hover {
  background-color: rgba(255, 255, 255, 0.03);
}

.timestamp {
  color: #64748b;
  flex-shrink: 0;
  user-select: none;
}

.content {
  word-break: break-all;
  white-space: pre-wrap;
}

.level-info .content { color: var(--log-info); }
.level-warn .content { color: var(--log-warn); }
.level-error .content { color: var(--log-error); }
.level-debug .content { color: var(--log-debug); }

.empty-state {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-secondary);
  font-style: italic;
  opacity: 0.5;
}

/* 大模型回复高亮特效 - 极简极客风 */
.highlight-response {
  background: rgba(0, 0, 0, 0.25) !important;
  border: 1px solid rgba(59, 130, 246, 0.15);
  border-left: 3px solid var(--accent);
  padding: 1rem 1.2rem !important;
  margin: 0.75rem 0 !important;
  border-radius: 6px;
}

.highlight-response .content {
  color: #e2e8f0 !important;
  font-weight: 400;
  font-size: 0.95rem;
  line-height: 1.6;
}

.highlight-response .timestamp {
  color: var(--accent);
  opacity: 0.7;
}
</style>

<template>
  <div class="layout">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="logo">
        <div class="pulse"></div>
        <h1>Inhibitor</h1>
      </div>
      
      <nav class="nav-menu">
        <router-link to="/" class="nav-item" active-class="active">
          <span class="icon">🏠</span>
          <span class="label">Home</span>
        </router-link>
        
        <router-link to="/monitor" class="nav-item" active-class="active">
          <span class="icon">📊</span>
          <span class="label">Monitor</span>
        </router-link>
        
        <router-link to="/settings" class="nav-item" active-class="active">
          <span class="icon">⚙️</span>
          <span class="label">Settings</span>
        </router-link>
      </nav>
      
      <div class="sidebar-footer">
        <div class="version">v0.1.0-alpha</div>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="main-content">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
  </div>
</template>

<style>
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&family=JetBrains+Mono:wght@400;500&display=swap');

:root {
  --bg-color: #0f172a;
  --sidebar-bg: rgba(15, 23, 42, 0.8);
  --panel-bg: rgba(30, 41, 59, 0.7);
  --text-primary: #f8fafc;
  --text-secondary: #94a3b8;
  --accent: #3b82f6;
  --accent-glow: rgba(59, 130, 246, 0.5);
  
  /* Log colors */
  --log-info: #38bdf8;
  --log-warn: #fbbf24;
  --log-error: #f87171;
  --log-debug: #a78bfa;
}

body {
  margin: 0;
  font-family: 'Inter', sans-serif;
  background-color: var(--bg-color);
  color: var(--text-primary);
  background-image: 
    radial-gradient(at 0% 0%, rgba(15, 23, 42, 1) 0, transparent 50%), 
    radial-gradient(at 50% 0%, rgba(30, 58, 138, 0.3) 0, transparent 50%), 
    radial-gradient(at 100% 0%, rgba(15, 23, 42, 1) 0, transparent 50%);
  background-attachment: fixed;
  min-height: 100vh;
  overflow: hidden; /* Prevent body scrolling */
}

#app {
  height: 100vh;
}

.layout {
  display: flex;
  height: 100vh;
  width: 100vw;
}

/* Sidebar Styles */
.sidebar {
  width: 260px;
  background: var(--sidebar-bg);
  backdrop-filter: blur(20px);
  border-right: 1px solid rgba(255, 255, 255, 0.05);
  display: flex;
  flex-direction: column;
  padding: 2rem 1.5rem;
  box-shadow: 5px 0 25px rgba(0, 0, 0, 0.2);
  z-index: 10;
}

.logo {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 3rem;
  padding-left: 0.5rem;
}

.pulse {
  width: 12px;
  height: 12px;
  background-color: var(--accent);
  border-radius: 50%;
  box-shadow: 0 0 0 0 var(--accent-glow);
  animation: pulsing 2s infinite;
}

@keyframes pulsing {
  0% { transform: scale(0.95); box-shadow: 0 0 0 0 var(--accent-glow); }
  70% { transform: scale(1); box-shadow: 0 0 0 10px rgba(59, 130, 246, 0); }
  100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(59, 130, 246, 0); }
}

.logo h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  background: linear-gradient(90deg, #fff, #94a3b8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-menu {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.nav-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.8rem 1rem;
  border-radius: 10px;
  color: var(--text-secondary);
  text-decoration: none;
  font-weight: 500;
  transition: all 0.2s ease;
}

.nav-item:hover {
  background: rgba(255, 255, 255, 0.05);
  color: var(--text-primary);
}

.nav-item.active {
  background: rgba(59, 130, 246, 0.15);
  color: var(--accent);
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.nav-item .icon {
  font-size: 1.2rem;
}

.sidebar-footer {
  margin-top: auto;
  padding-top: 1rem;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
  text-align: center;
}

.version {
  font-size: 0.8rem;
  color: var(--text-secondary);
  opacity: 0.5;
}

/* Main Content Styles */
.main-content {
  flex: 1;
  padding: 2rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

/* Page Transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
</style>

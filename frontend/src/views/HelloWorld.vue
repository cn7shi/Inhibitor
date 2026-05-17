<template>
  <div class="hello-container">
    <div class="card">
      <h2>👋 Hello World</h2>
      <p>Welcome to the Inhibitor Console.</p>
      <p class="subtitle">This is a placeholder page for future expansion. You can add more tools and configuration panels here.</p>
      
      <div class="features">
        <div class="feature-item">
          <div class="feature-icon">✨</div>
          <div class="feature-text">Scalable Architecture</div>
        </div>
        <div class="feature-item">
          <div class="feature-icon">🚀</div>
          <div class="feature-text">High Performance</div>
        </div>
        <div class="feature-item">
          <div class="feature-icon">🔒</div>
          <div class="feature-text">Secure by Design</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.hello-container {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  padding: 2rem;
}

.card {
  background: var(--panel-bg);
  padding: 3rem;
  border-radius: 16px;
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.1);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  max-width: 600px;
  text-align: center;
}

h2 {
  font-size: 2.5rem;
  margin-top: 0;
  margin-bottom: 1rem;
  background: linear-gradient(135deg, #fff, #94a3b8);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

p {
  color: var(--text-primary);
  font-size: 1.1rem;
  line-height: 1.6;
}

.subtitle {
  color: var(--text-secondary);
  font-size: 0.95rem;
  margin-bottom: 3rem;
}

.features {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.5rem;
  margin-top: 2rem;
}

.feature-item {
  background: rgba(255, 255, 255, 0.03);
  padding: 1.5rem 1rem;
  border-radius: 12px;
  border: 1px solid rgba(255, 255, 255, 0.05);
  transition: transform 0.3s ease, background 0.3s ease;
}

.feature-item:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.05);
}

.feature-icon {
  font-size: 2rem;
  margin-bottom: 0.5rem;
}

.feature-text {
  color: var(--text-primary);
  font-size: 0.9rem;
  font-weight: 500;
}
</style>

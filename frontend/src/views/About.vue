<template>
  <div class="about">
    <div class="about-card">
      <div class="about-header">
        <div class="logo-section">
          <div class="logo">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"/>
            </svg>
          </div>
          <h1 class="app-title">管理系统</h1>
          <p class="app-subtitle">现代化全栈管理平台</p>
        </div>
      </div>
      
      <div class="about-content">
        <div class="info-section">
          <h2 class="section-title">关于我们</h2>
          <p class="section-text">
            这是一个基于现代化技术栈构建的全栈管理系统，提供完整的权限管理、用户管理、角色管理等核心功能。
          </p>
        </div>
        
        <div class="features-grid">
          <div class="feature-item">
            <div class="feature-icon primary">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4zm0 10.99h7c-.53 4.12-3.28 7.79-7 8.94V12H5V6.3l7-3.11v8.8z"/>
              </svg>
            </div>
            <h3 class="feature-title">安全可靠</h3>
            <p class="feature-text">完善的权限控制系统，保障数据安全</p>
          </div>
          
          <div class="feature-item">
            <div class="feature-icon success">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M9 16.17L4.83 12l-1.42 1.41L9 19 21 7l-1.41-1.41z"/>
              </svg>
            </div>
            <h3 class="feature-title">易于使用</h3>
            <p class="feature-text">简洁直观的用户界面，操作简单便捷</p>
          </div>
          
          <div class="feature-item">
            <div class="feature-icon warning">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14H7v-2h7v2zm3-4H7v-2h10v2zm0-4H7V7h10v2z"/>
              </svg>
            </div>
            <h3 class="feature-title">功能完善</h3>
            <p class="feature-text">丰富的功能模块，满足各类管理需求</p>
          </div>
          
          <div class="feature-item">
            <div class="feature-icon danger">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M10 4h4v4h-4V4zM3 16h14v2H3zm9-9h4v4h-4V7zm-6 7h4v4H6v-4zm3-9h4v4H9V4z"/>
              </svg>
            </div>
            <h3 class="feature-title">快速响应</h3>
            <p class="feature-text">高性能架构，提供流畅的用户体验</p>
          </div>
        </div>
        
        <div class="tech-stack">
          <h2 class="section-title">技术栈</h2>
          <div class="tech-list">
            <div class="tech-item">
              <span class="tech-label">前端框架</span>
              <span class="tech-value">Vue 3</span>
            </div>
            <div class="tech-item">
              <span class="tech-label">UI 组件库</span>
              <span class="tech-value">Element Plus</span>
            </div>
            <div class="tech-item">
              <span class="tech-label">构建工具</span>
              <span class="tech-value">Vite</span>
            </div>
            <div class="tech-item">
              <span class="tech-label">状态管理</span>
              <span class="tech-value">Pinia</span>
            </div>
            <div class="tech-item">
              <span class="tech-label">路由管理</span>
              <span class="tech-value">Vue Router</span>
            </div>
            <div class="tech-item">
              <span class="tech-label">类型系统</span>
              <span class="tech-value">TypeScript</span>
            </div>
          </div>
        </div>
      </div>
      
      <div class="about-footer">
        <p class="footer-text">© 2024 管理系统. 保留所有权利.</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// About page component
</script>

<style scoped>
.about {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

.about-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  overflow: hidden;
  transition: all var(--transition);
}

.about-card:hover {
  box-shadow: var(--shadow);
}

.about-header {
  background: linear-gradient(135deg, var(--primary), #409eff);
  padding: var(--spacing-xl) var(--spacing-lg);
  text-align: center;
}

.logo-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-md);
}

.logo {
  width: 80px;
  height: 80px;
  background: rgba(255, 255, 255, 0.2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.logo svg {
  width: 48px;
  height: 48px;
}

.app-title {
  font-size: var(--font-size-xxl);
  font-weight: 700;
  color: white;
  margin: 0;
}

.app-subtitle {
  font-size: var(--font-size);
  color: rgba(255, 255, 255, 0.9);
  margin: 0;
}

.about-content {
  padding: var(--spacing-xl) var(--spacing-lg);
}

.info-section {
  margin-bottom: var(--spacing-xl);
  text-align: center;
}

.section-title {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-md) 0;
}

.section-text {
  font-size: var(--font-size);
  color: var(--text-secondary);
  line-height: 1.6;
  margin: 0;
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-xl);
}

.feature-item {
  text-align: center;
  padding: var(--spacing-lg);
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.feature-item:hover {
  transform: translateY(-4px);
  box-shadow: var(--shadow);
}

.feature-icon {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto var(--spacing-md) auto;
  color: white;
}

.feature-icon.primary {
  background: linear-gradient(135deg, var(--primary), #409eff);
}

.feature-icon.success {
  background: linear-gradient(135deg, var(--success), #67c23a);
}

.feature-icon.warning {
  background: linear-gradient(135deg, var(--warning), #e6a23c);
}

.feature-icon.danger {
  background: linear-gradient(135deg, var(--danger), #f56c6c);
}

.feature-icon svg {
  width: 32px;
  height: 32px;
}

.feature-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 var(--spacing-sm) 0;
}

.feature-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
  line-height: 1.5;
}

.tech-stack {
  padding: var(--spacing-lg);
  background: var(--bg-secondary);
  border-radius: var(--radius-md);
}

.tech-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: var(--spacing-md);
}

.tech-item {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-xs);
  padding: var(--spacing-md);
  background: var(--bg-primary);
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.tech-item:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-sm);
}

.tech-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
}

.tech-value {
  font-size: var(--font-size);
  color: var(--text-primary);
  font-weight: 600;
}

.about-footer {
  padding: var(--spacing-lg);
  text-align: center;
  border-top: 1px solid var(--border);
}

.footer-text {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  margin: 0;
}

@media screen and (max-width: 768px) {
  .about {
    padding: var(--spacing-md);
  }
  
  .about-header {
    padding: var(--spacing-lg) var(--spacing-md);
  }
  
  .logo {
    width: 64px;
    height: 64px;
  }
  
  .logo svg {
    width: 36px;
    height: 36px;
  }
  
  .app-title {
    font-size: var(--font-size-xl);
  }
  
  .about-content {
    padding: var(--spacing-lg) var(--spacing-md);
  }
  
  .features-grid {
    grid-template-columns: 1fr;
    gap: var(--spacing-md);
  }
  
  .tech-list {
    grid-template-columns: 1fr;
  }
}
</style>
<template>
  <div class="home">
    <div class="welcome-card">
      <h1 class="welcome-title">欢迎使用管理系统</h1>
      <p class="welcome-description">这是一个现代化的全栈管理系统，提供完整的系统管理功能</p>
    </div>
    
    <div class="counter-card">
      <div class="counter-header">
        <h2 class="counter-title">计数器示例</h2>
      </div>
      <div class="counter-content">
        <div class="counter-display">
          <span class="counter-label">计数:</span>
          <span class="counter-value">{{ counter.count }}</span>
        </div>
        <div class="counter-display">
          <span class="counter-label">双倍计数:</span>
          <span class="counter-value double">{{ counter.doubleCount }}</span>
        </div>
        <div class="counter-actions">
          <el-button type="primary" @click="counter.increment" class="action-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13h-6v6h-2v-6H5v-2h6V5h2v6h6v2z"/>
              </svg>
            </el-icon>
            增加
          </el-button>
          <el-button type="danger" @click="counter.decrement" class="action-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M19 13H5v-2h14v2z"/>
              </svg>
            </el-icon>
            减少
          </el-button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useCounterStore } from '../stores/counter'

const counter = useCounterStore()
</script>

<style scoped>
.home {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

.welcome-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
  text-align: center;
}

.welcome-card:hover {
  box-shadow: var(--shadow);
}

.welcome-title {
  font-size: var(--font-size-xxl);
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: var(--spacing-md);
}

.welcome-description {
  font-size: var(--font-size);
  color: var(--text-secondary);
  margin: 0;
}

.counter-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  transition: all var(--transition);
}

.counter-card:hover {
  box-shadow: var(--shadow);
}

.counter-header {
  margin-bottom: var(--spacing-lg);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.counter-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.counter-content {
  display: flex;
  flex-direction: column;
  gap: var(--spacing-md);
}

.counter-display {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--spacing-md);
  background-color: var(--bg-secondary);
  border-radius: var(--radius);
  transition: all var(--transition-fast);
}

.counter-display:hover {
  background-color: var(--bg-tertiary);
}

.counter-label {
  font-size: var(--font-size);
  color: var(--text-secondary);
  font-weight: 500;
}

.counter-value {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--primary);
}

.counter-value.double {
  color: var(--success);
}

.counter-actions {
  display: flex;
  gap: var(--spacing-md);
  justify-content: center;
  margin-top: var(--spacing-md);
}

.action-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
  padding: var(--spacing-sm) var(--spacing-lg);
}

.action-button:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow);
}

.button-icon {
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .home {
    padding: var(--spacing-md);
  }
  
  .welcome-card, .counter-card {
    padding: var(--spacing-md);
  }
  
  .welcome-title {
    font-size: var(--font-size-xl);
  }
  
  .counter-display {
    flex-direction: column;
    gap: var(--spacing-xs);
    text-align: center;
  }
  
  .counter-actions {
    flex-direction: column;
    align-items: stretch;
  }
  
  .action-button {
    justify-content: center;
  }
}
</style>
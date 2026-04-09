<template>
  <div class="server-info-container">
    <div class="page-header">
      <h1 class="page-title">服务器信息</h1>
      <el-button type="primary" @click="refreshServerInfo" class="action-button">
        <el-icon class="button-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
          </svg>
        </el-icon>
        刷新信息
      </el-button>
    </div>
    
    <div class="info-cards">
      <div class="info-card">
        <div class="card-header">
          <div class="card-icon system">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2H0v2h24v-2h-4zM4 6h16v10H4V6z"/>
            </svg>
          </div>
          <span class="card-title">系统信息</span>
        </div>
        <div class="card-content">
          <div class="info-item">
            <span class="info-label">服务器名称</span>
            <span class="info-value">{{ serverInfo.serverName }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">系统版本</span>
            <span class="info-value">{{ serverInfo.osVersion }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">运行时间</span>
            <span class="info-value">{{ serverInfo.runTime }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">JVM版本</span>
            <span class="info-value">{{ serverInfo.jvmVersion }}</span>
          </div>
        </div>
      </div>
      
      <div class="info-card">
        <div class="card-header">
          <div class="card-icon cpu">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M9 3v2H6v2H4V7H2v10h2v-2h2v2h3v2h6v-2h3v2h2v-2h2V7h-2v2h-2V7h-3V3H9zm2 2h2v2h-2V5zm-2 4h2v2H9V9zm0 4h2v2H9v-2zm6-4h2v2h-2V9zm0 4h2v2h-2v-2zm2-8h2v2h-2V5zM5 5h2v2H5V5zm0 4h2v2H5V9zm0 4h2v2H5v-2z"/>
            </svg>
          </div>
          <span class="card-title">CPU信息</span>
        </div>
        <div class="card-content">
          <div class="info-item">
            <span class="info-label">CPU核心数</span>
            <span class="info-value">{{ serverInfo.cpuCore }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">CPU使用率</span>
            <span class="info-value highlight">{{ serverInfo.cpuUsage }}%</span>
          </div>
          <div class="info-item">
            <span class="info-label">系统负载</span>
            <span class="info-value">{{ serverInfo.systemLoad }}</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${serverInfo.cpuUsage}%` }"></div>
          </div>
        </div>
      </div>
      
      <div class="info-card">
        <div class="card-header">
          <div class="card-icon memory">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M15 9H9v6h6V9zm-2 4h-2v-2h2v2zm6-4h-2v2h2V9zm0 4h-2v2h2v-2zM7 9H5v2h2V9zm0 4H5v2h2v-2zm14-6v12c0 1.1-.9 2-2 2H5c-1.1 0-2-.9-2-2V7c0-1.1.9-2 2-2h14c1.1 0 2 .9 2 2zm-2 0H5v12h14V7z"/>
            </svg>
          </div>
          <span class="card-title">内存信息</span>
        </div>
        <div class="card-content">
          <div class="info-item">
            <span class="info-label">总内存</span>
            <span class="info-value">{{ serverInfo.totalMemory }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">已用内存</span>
            <span class="info-value">{{ serverInfo.usedMemory }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">内存使用率</span>
            <span class="info-value highlight">{{ serverInfo.memoryUsage }}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${serverInfo.memoryUsage}%` }"></div>
          </div>
        </div>
      </div>
      
      <div class="info-card">
        <div class="card-header">
          <div class="card-icon disk">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M20 18c1.1 0 1.99-.9 1.99-2L22 6c0-1.1-.9-2-2-2H4c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h16zM4 6h16v2H4V6zm0 6h16v2H4v-2zm0 4h16v2H4v-2z"/>
            </svg>
          </div>
          <span class="card-title">磁盘信息</span>
        </div>
        <div class="card-content">
          <div class="info-item">
            <span class="info-label">总磁盘空间</span>
            <span class="info-value">{{ serverInfo.totalDisk }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">已用磁盘空间</span>
            <span class="info-value">{{ serverInfo.usedDisk }}</span>
          </div>
          <div class="info-item">
            <span class="info-label">磁盘使用率</span>
            <span class="info-value highlight">{{ serverInfo.diskUsage }}%</span>
          </div>
          <div class="progress-bar">
            <div class="progress-fill" :style="{ width: `${serverInfo.diskUsage}%` }"></div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="network-card">
      <div class="card-header">
        <div class="card-icon network">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 17.93c-3.95-.49-7-3.85-7-7.93 0-.62.08-1.21.21-1.79L9 15v1c0 1.1.9 2 2 2v1.93zm6.9-2.54c-.26-.81-1-1.39-1.9-1.39h-1v-3c0-.55-.45-1-1-1H8v-2h2c.55 0 1-.45 1-1V7h2c1.1 0 2-.9 2-2v-.41c2.93 1.19 5 4.06 5 7.41 0 2.08-.8 3.97-2.1 5.39z"/>
          </svg>
        </div>
        <span class="card-title">网络信息</span>
      </div>
      <div class="card-content">
        <el-table :data="serverInfo.networkInterfaces" style="width: 100%" class="data-table">
          <el-table-column prop="name" label="网络接口" width="150" />
          <el-table-column prop="ip" label="IP地址" width="150" />
          <el-table-column prop="mac" label="MAC地址" width="180" />
          <el-table-column prop="mtu" label="MTU" width="100" />
        </el-table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { request } from '../api/request'

interface NetworkInterface {
  name: string
  ip: string
  mac: string
  mtu: number
}

interface ServerInfo {
  serverName: string
  osVersion: string
  runTime: string
  jvmVersion: string
  cpuCore: number
  cpuUsage: number
  systemLoad: number
  totalMemory: string
  usedMemory: string
  memoryUsage: number
  totalDisk: string
  usedDisk: string
  diskUsage: number
  networkInterfaces: NetworkInterface[]
}

const serverInfo = ref<ServerInfo>({
  serverName: '',
  osVersion: '',
  runTime: '',
  jvmVersion: '',
  cpuCore: 0,
  cpuUsage: 0,
  systemLoad: 0,
  totalMemory: '',
  usedMemory: '',
  memoryUsage: 0,
  totalDisk: '',
  usedDisk: '',
  diskUsage: 0,
  networkInterfaces: []
})

const refreshServerInfo = async () => {
  await fetchServerInfo()
}

const fetchServerInfo = async () => {
  try {
    const response = await request.get('/sys/serverinfo/server_update')
    serverInfo.value = response.data
  } catch (error) {
    console.error('获取服务器信息失败:', error)
  }
}

onMounted(() => {
  fetchServerInfo()
})
</script>

<style scoped>
.server-info-container {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-lg);
}

.page-title {
  font-size: var(--font-size-xxl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.action-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
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

.info-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
}

.info-card,
.network-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition);
  overflow: hidden;
}

.info-card:hover,
.network-card:hover {
  box-shadow: var(--shadow);
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
  padding: var(--spacing-lg);
  padding-bottom: var(--spacing-md);
  border-bottom: 1px solid var(--border);
}

.card-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
}

.card-icon.system {
  background: linear-gradient(135deg, var(--primary), #409eff);
}

.card-icon.cpu {
  background: linear-gradient(135deg, var(--success), #67c23a);
}

.card-icon.memory {
  background: linear-gradient(135deg, var(--warning), #e6a23c);
}

.card-icon.disk {
  background: linear-gradient(135deg, var(--danger), #f56c6c);
}

.card-icon.network {
  background: linear-gradient(135deg, var(--info), #909399);
}

.card-icon svg {
  width: 24px;
  height: 24px;
}

.card-title {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--text-primary);
}

.card-content {
  padding: var(--spacing-lg);
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--spacing-md);
  padding: var(--spacing-sm) 0;
}

.info-item:last-of-type {
  margin-bottom: var(--spacing-sm);
}

.info-label {
  font-size: var(--font-size-sm);
  color: var(--text-secondary);
  font-weight: 500;
}

.info-value {
  font-size: var(--font-size);
  color: var(--text-primary);
  font-weight: 600;
}

.info-value.highlight {
  color: var(--primary);
  font-size: var(--font-size-lg);
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: var(--bg-secondary);
  border-radius: 4px;
  overflow: hidden;
  margin-top: var(--spacing-sm);
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, var(--primary), #409eff);
  border-radius: 4px;
  transition: width var(--transition);
}

.data-table {
  border-radius: var(--radius);
  overflow: hidden;
}

@media screen and (max-width: 768px) {
  .server-info-container {
    padding: var(--spacing-md);
  }
  
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: var(--spacing-md);
  }
  
  .action-button {
    width: 100%;
    justify-content: center;
  }
  
  .info-cards {
    grid-template-columns: 1fr;
    gap: var(--spacing-md);
  }
  
  .info-card,
  .network-card {
    padding: 0;
  }
  
  .card-header {
    padding: var(--spacing-md);
  }
  
  .card-content {
    padding: var(--spacing-md);
  }
}
</style>
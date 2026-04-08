<template>
  <div class="server-info-container">
    <h2>服务器信息</h2>
    <el-button type="primary" @click="refreshServerInfo" style="margin-bottom: 20px">刷新信息</el-button>
    
    <div class="info-cards">
      <el-card class="info-card">
        <template #header>
          <div class="card-header">
            <span>系统信息</span>
          </div>
        </template>
        <div class="info-item">
          <span class="info-label">服务器名称:</span>
          <span class="info-value">{{ serverInfo.serverName }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">系统版本:</span>
          <span class="info-value">{{ serverInfo.osVersion }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">运行时间:</span>
          <span class="info-value">{{ serverInfo.runTime }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">JVM版本:</span>
          <span class="info-value">{{ serverInfo.jvmVersion }}</span>
        </div>
      </el-card>
      
      <el-card class="info-card">
        <template #header>
          <div class="card-header">
            <span>CPU信息</span>
          </div>
        </template>
        <div class="info-item">
          <span class="info-label">CPU核心数:</span>
          <span class="info-value">{{ serverInfo.cpuCore }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">CPU使用率:</span>
          <span class="info-value">{{ serverInfo.cpuUsage }}%</span>
        </div>
        <div class="info-item">
          <span class="info-label">系统负载:</span>
          <span class="info-value">{{ serverInfo.systemLoad }}</span>
        </div>
      </el-card>
      
      <el-card class="info-card">
        <template #header>
          <div class="card-header">
            <span>内存信息</span>
          </div>
        </template>
        <div class="info-item">
          <span class="info-label">总内存:</span>
          <span class="info-value">{{ serverInfo.totalMemory }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">已用内存:</span>
          <span class="info-value">{{ serverInfo.usedMemory }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">内存使用率:</span>
          <span class="info-value">{{ serverInfo.memoryUsage }}%</span>
        </div>
      </el-card>
      
      <el-card class="info-card">
        <template #header>
          <div class="card-header">
            <span>磁盘信息</span>
          </div>
        </template>
        <div class="info-item">
          <span class="info-label">总磁盘空间:</span>
          <span class="info-value">{{ serverInfo.totalDisk }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">已用磁盘空间:</span>
          <span class="info-value">{{ serverInfo.usedDisk }}</span>
        </div>
        <div class="info-item">
          <span class="info-label">磁盘使用率:</span>
          <span class="info-value">{{ serverInfo.diskUsage }}%</span>
        </div>
      </el-card>
    </div>
    
    <el-card class="info-card" style="margin-top: 20px">
      <template #header>
        <div class="card-header">
          <span>网络信息</span>
        </div>
      </template>
      <el-table :data="serverInfo.networkInterfaces" style="width: 100%">
        <el-table-column prop="name" label="网络接口" width="150" />
        <el-table-column prop="ip" label="IP地址" width="150" />
        <el-table-column prop="mac" label="MAC地址" width="180" />
        <el-table-column prop="mtu" label="MTU" width="100" />
      </el-table>
    </el-card>
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
  padding: 20px;
  background-color: #f5f7fa;
  min-height: 100vh;
}

.info-cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
  margin-bottom: 20px;
}

.info-card {
  background-color: #fff;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}

.card-header {
  font-weight: bold;
  font-size: 16px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
  padding: 5px 0;
  border-bottom: 1px solid #f0f0f0;
}

.info-label {
  color: #606266;
}

.info-value {
  font-weight: 500;
  color: #303133;
}
</style>
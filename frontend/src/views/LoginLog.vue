<template>
  <div class="login-log-container">
    <div class="page-header">
      <h1 class="page-title">登录日志</h1>
    </div>
    
    <div class="search-card">
      <div class="search-form">
        <el-input
          v-model="searchForm.username"
          placeholder="用户名"
          class="search-input"
          clearable
        />
        <el-date-picker
          v-model="searchForm.dateRange"
          type="daterange"
          range-separator="至"
          start-placeholder="开始日期"
          end-placeholder="结束日期"
          class="search-date"
        />
        <div class="search-buttons">
          <el-button type="primary" @click="handleSearch" class="search-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
              </svg>
            </el-icon>
            查询
          </el-button>
          <el-button @click="resetForm" class="search-button">
            <el-icon class="button-icon">
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                <path d="M17.65 6.35A7.958 7.958 0 0 0 12 4c-4.42 0-7.99 3.58-7.99 8s3.57 8 7.99 8c3.73 0 6.84-2.55 7.73-6h-2.08A5.99 5.99 0 0 1 12 18c-3.31 0-6-2.69-6-6s2.69-6 6-6c1.66 0 3.14.69 4.22 1.78L13 11h7V4l-2.35 2.35z"/>
              </svg>
            </el-icon>
            重置
          </el-button>
        </div>
      </div>
    </div>
    
    <div class="table-card">
      <el-table :data="loginLogs" style="width: 100%" class="data-table">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="username" label="用户名" width="120" />
        <el-table-column prop="ipaddr" label="IP地址" width="150" />
        <el-table-column prop="loginLocation" label="登录地点" width="150" />
        <el-table-column prop="browser" label="浏览器" width="150" />
        <el-table-column prop="os" label="操作系统" width="120" />
        <el-table-column prop="status" label="状态" width="100">
          <template #default="scope">
            <el-tag :type="scope.row.status === '成功' ? 'success' : 'danger'" effect="light">
              {{ scope.row.status }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="msg" label="登录信息" />
        <el-table-column prop="loginTime" label="登录时间" width="180" />
      </el-table>
      
      <div class="pagination">
        <el-pagination
          v-model:current-page="pagination.currentPage"
          v-model:page-size="pagination.pageSize"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          :total="pagination.total"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { request } from '../api/request'

interface LoginLog {
  id: number
  username: string
  ipaddr: string
  loginLocation: string
  browser: string
  os: string
  status: string
  msg: string
  loginTime: string
}

interface SearchForm {
  username: string
  dateRange: [Date, Date] | null
}

interface Pagination {
  currentPage: number
  pageSize: number
  total: number
}

const loginLogs = ref<LoginLog[]>([])
const searchForm = ref<SearchForm>({
  username: '',
  dateRange: null
})
const pagination = ref<Pagination>({
  currentPage: 1,
  pageSize: 10,
  total: 0
})

const handleSearch = async () => {
  pagination.value.currentPage = 1
  await fetchLoginLogs()
}

const resetForm = () => {
  searchForm.value = {
    username: '',
    dateRange: null
  }
  handleSearch()
}

const handleSizeChange = async (size: number) => {
  pagination.value.pageSize = size
  await fetchLoginLogs()
}

const handleCurrentChange = async (current: number) => {
  pagination.value.currentPage = current
  await fetchLoginLogs()
}

const fetchLoginLogs = async () => {
  try {
    const params = {
      page: pagination.value.currentPage,
      limit: pagination.value.pageSize,
      username: searchForm.value.username,
      startTime: searchForm.value.dateRange ? searchForm.value.dateRange[0].toISOString() : undefined,
      endTime: searchForm.value.dateRange ? searchForm.value.dateRange[1].toISOString() : undefined
    }
    const response = await request.get('/sys/logininfo/list', { params })
    loginLogs.value = response.data
    pagination.value.total = response.total
  } catch (error) {
    console.error('获取登录日志失败:', error)
  }
}

onMounted(() => {
  fetchLoginLogs()
})
</script>

<style scoped>
.login-log-container {
  padding: var(--spacing-lg);
  background-color: var(--bg-secondary);
  min-height: 100vh;
  transition: all var(--transition);
}

.page-header {
  margin-bottom: var(--spacing-lg);
}

.page-title {
  font-size: var(--font-size-xxl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
}

.search-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.search-card:hover {
  box-shadow: var(--shadow);
}

.search-form {
  display: flex;
  flex-wrap: wrap;
  gap: var(--spacing-md);
  align-items: center;
}

.search-input {
  min-width: 200px;
}

.search-date {
  min-width: 300px;
}

.search-buttons {
  display: flex;
  gap: var(--spacing-md);
  margin-left: auto;
}

.search-button {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  border-radius: var(--radius-md);
}

.search-button:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow);
}

.button-icon {
  font-size: 16px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.table-card {
  background: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  padding: var(--spacing-lg);
  transition: all var(--transition);
}

.table-card:hover {
  box-shadow: var(--shadow);
}

.data-table {
  border-radius: var(--radius);
  overflow: hidden;
}

.pagination {
  margin-top: var(--spacing-lg);
  display: flex;
  justify-content: flex-end;
}

@media screen and (max-width: 768px) {
  .login-log-container {
    padding: var(--spacing-md);
  }
  
  .search-card,
  .table-card {
    padding: var(--spacing-md);
  }
  
  .search-form {
    flex-direction: column;
  }
  
  .search-input,
  .search-date {
    width: 100%;
    min-width: auto;
  }
  
  .search-buttons {
    width: 100%;
    margin-left: 0;
  }
  
  .search-buttons .el-button {
    flex: 1;
  }
  
  .pagination {
    justify-content: center;
  }
}
</style>
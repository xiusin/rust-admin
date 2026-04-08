<template>
  <div class="login-log-container">
    <h2>登录日志</h2>
    <div class="search-bar">
      <el-input
        v-model="searchForm.username"
        placeholder="用户名"
        style="width: 200px; margin-right: 10px"
      />
      <el-date-picker
        v-model="searchForm.dateRange"
        type="daterange"
        range-separator="至"
        start-placeholder="开始日期"
        end-placeholder="结束日期"
        style="width: 300px; margin-right: 10px"
      />
      <el-button type="primary" @click="handleSearch">查询</el-button>
      <el-button @click="resetForm">重置</el-button>
    </div>
    <el-table :data="loginLogs" style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="username" label="用户名" width="120" />
      <el-table-column prop="ipaddr" label="IP地址" width="150" />
      <el-table-column prop="loginLocation" label="登录地点" width="150" />
      <el-table-column prop="browser" label="浏览器" width="150" />
      <el-table-column prop="os" label="操作系统" width="120" />
      <el-table-column prop="status" label="状态" width="80">
        <template #default="scope">
          <el-tag :type="scope.row.status === '成功' ? 'success' : 'danger'">
            {{ scope.row.status }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column prop="msg" label="登录信息" />
      <el-table-column prop="loginTime" label="登录时间" width="180" />
    </el-table>
    <div class="pagination" style="margin-top: 20px">
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
  padding: 20px;
  background-color: #f5f7fa;
  min-height: 100vh;
}

.search-bar {
  margin-bottom: 20px;
  padding: 20px;
  background-color: #fff;
  border-radius: 8px;
  box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.1);
}
</style>
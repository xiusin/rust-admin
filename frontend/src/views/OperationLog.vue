<template>
  <div class="operation-log-container">
    <h2>操作日志</h2>
    <div class="search-bar">
      <el-input
        v-model="searchForm.username"
        placeholder="用户名"
        style="width: 200px; margin-right: 10px"
      />
      <el-input
        v-model="searchForm.operation"
        placeholder="操作内容"
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
    <el-table :data="operationLogs" style="width: 100%">
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="username" label="用户名" width="120" />
      <el-table-column prop="operation" label="操作内容" />
      <el-table-column prop="method" label="请求方法" width="100" />
      <el-table-column prop="path" label="请求路径" width="200" />
      <el-table-column prop="ipaddr" label="IP地址" width="150" />
      <el-table-column prop="browser" label="浏览器" width="150" />
      <el-table-column prop="operTime" label="操作时间" width="180" />
      <el-table-column prop="costTime" label="耗时(ms)" width="100" />
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

interface OperationLog {
  id: number
  username: string
  operation: string
  method: string
  path: string
  ipaddr: string
  browser: string
  operTime: string
  costTime: number
}

interface SearchForm {
  username: string
  operation: string
  dateRange: [Date, Date] | null
}

interface Pagination {
  currentPage: number
  pageSize: number
  total: number
}

const operationLogs = ref<OperationLog[]>([])
const searchForm = ref<SearchForm>({
  username: '',
  operation: '',
  dateRange: null
})
const pagination = ref<Pagination>({
  currentPage: 1,
  pageSize: 10,
  total: 0
})

const handleSearch = async () => {
  pagination.value.currentPage = 1
  await fetchOperationLogs()
}

const resetForm = () => {
  searchForm.value = {
    username: '',
    operation: '',
    dateRange: null
  }
  handleSearch()
}

const handleSizeChange = async (size: number) => {
  pagination.value.pageSize = size
  await fetchOperationLogs()
}

const handleCurrentChange = async (current: number) => {
  pagination.value.currentPage = current
  await fetchOperationLogs()
}

const fetchOperationLogs = async () => {
  try {
    const params = {
      page: pagination.value.currentPage,
      limit: pagination.value.pageSize,
      username: searchForm.value.username,
      operation: searchForm.value.operation,
      startTime: searchForm.value.dateRange ? searchForm.value.dateRange[0].toISOString() : undefined,
      endTime: searchForm.value.dateRange ? searchForm.value.dateRange[1].toISOString() : undefined
    }
    const response = await request.get('/sys/operationlog/list', { params })
    operationLogs.value = response.data
    pagination.value.total = response.total
  } catch (error) {
    console.error('获取操作日志失败:', error)
  }
}

onMounted(() => {
  fetchOperationLogs()
})
</script>

<style scoped>
.operation-log-container {
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
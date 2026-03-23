<template>
  <div class="login-logs-container">
    <a-card>
      <template #title>登录日志</template>
      <a-table :data="logs" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="手机号" data-index="phone" />
          <a-table-column title="登录方式" data-index="login_type">
            <template #cell="{ record }">
              {{ record.login_type === 'phone' ? '手机号' : '微信' }}
            </template>
          </a-table-column>
          <a-table-column title="是否成功" data-index="success">
            <template #cell="{ record }">
              <a-tag :color="record.success ? 'green' : 'red'">
                {{ record.success ? '成功' : '失败' }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="IP地址" data-index="ip_address" />
          <a-table-column title="设备类型" data-index="device_type" />
          <a-table-column title="登录时间" data-index="login_at" />
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { consumerApi, LoginLog } from '@/api/modules/consumer';

const logs = ref<LoginLog[]>([]);
const loading = ref(false);
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});

const loadLogs = async () => {
  loading.value = true;
  try {
    const stored = localStorage.getItem('consumer_info');
    const consumer_id = stored ? JSON.parse(stored).id : 0;
    const res = await consumerApi.loginLogs({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
      consumer_id,
    });
    logs.value = res.data?.list || [];
    pagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadLogs();
};

onMounted(() => {
  loadLogs();
});
</script>

<style scoped>
.login-logs-container {
  padding: 20px;
}
</style>
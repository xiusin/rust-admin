<template>
  <div class="finance-container">
    <a-row :gutter="20">
      <a-col :span="6">
        <a-card>
          <a-statistic title="账户余额" :value="account.balance" suffix="元" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="冻结金额" :value="account.frozen_balance" suffix="元" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="可用余额" :value="account.available_balance" suffix="元" />
        </a-card>
      </a-col>
    </a-row>

    <a-card style="margin-top: 20px">
      <template #title>交易流水</template>
      <a-table :data="transactions" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="流水号" data-index="transaction_no" />
          <a-table-column title="类型" data-index="transaction_type">
            <template #cell="{ record }">
              <a-tag :color="getTypeColor(record.transaction_type)">
                {{ getTypeName(record.transaction_type) }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="金额" data-index="amount">
            <template #cell="{ record }">
              <span :class="record.transaction_type === 'recharge' || record.transaction_type === 'refund' ? 'text-success' : 'text-danger'">
                {{ record.transaction_type === 'recharge' || record.transaction_type === 'refund' ? '+' : '-' }}{{ record.amount }}
              </span>
            </template>
          </a-table-column>
          <a-table-column title="余额" data-index="balance_after" />
          <a-table-column title="时间" data-index="created_at" />
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { financeApi, AccountInfo, Transaction } from '@/api/modules/consumer/finance';

const account = ref<AccountInfo>({
  id: 0,
  tenant_id: 0,
  consumer_id: 0,
  balance: '0',
  frozen_balance: '0',
  available_balance: '0',
});

const transactions = ref<Transaction[]>([]);
const loading = ref(false);
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});

const getTypeName = (type: string) => {
  const map: Record<string, string> = {
    recharge: '充值',
    consume: '消费',
    withdraw: '提现',
    refund: '退款',
  };
  return map[type] || type;
};

const getTypeColor = (type: string) => {
  const map: Record<string, string> = {
    recharge: 'green',
    consume: 'orange',
    withdraw: 'red',
    refund: 'cyan',
  };
  return map[type] || 'gray';
};

const loadData = async () => {
  loading.value = true;
  try {
    const stored = localStorage.getItem('consumer_info');
    const consumer_id = stored ? JSON.parse(stored).id : 0;

    account.value = await financeApi.getAccount(consumer_id);
    const res = await financeApi.transactions({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
    });
    transactions.value = res.data?.list || [];
    pagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.finance-container {
  padding: 20px;
}
.text-success {
  color: green;
}
.text-danger {
  color: red;
}
</style>
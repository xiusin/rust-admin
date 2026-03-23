<template>
  <div class="page-container">
    <a-row :gutter="16">
      <a-col :span="6">
        <a-card>
          <a-statistic title="今日支付金额" :value="statistics.todayAmount" prefix="¥" :precision="2" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="今日支付笔数" :value="statistics.todayCount" suffix="笔" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="今日退款金额" :value="statistics.todayRefund" prefix="¥" :precision="2" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="待处理退款" :value="statistics.pendingRefund" suffix="笔" />
        </a-card>
      </a-col>
    </a-row>

    <a-card style="margin-top: 16px">
      <template #title>支付趋势</template>
      <div id="chart" style="height: 300px"></div>
    </a-card>

    <a-row :gutter="16" style="margin-top: 16px">
      <a-col :span="12">
        <a-card>
          <template #title>支付方式分布</template>
          <a-table :data="methodData" :pagination="false">
            <template #columns>
              <a-table-column title="支付方式" data-index="name" />
              <a-table-column title="支付笔数" data-index="count" />
              <a-table-column title="支付金额" data-index="amount" />
              <a-table-column title="占比" :width="200">
                <template #cell="{ record }">
                  <a-progress :percent="record.percent" size="small" />
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-col>
      <a-col :span="12">
        <a-card>
          <template #title>订单状态分布</template>
          <a-table :data="statusData" :pagination="false">
            <template #columns>
              <a-table-column title="状态" data-index="name" />
              <a-table-column title="订单数" data-index="count" />
              <a-table-column title="金额" data-index="amount" />
              <a-table-column title="占比" :width="200">
                <template #cell="{ record }">
                  <a-progress :percent="record.percent" size="small" :status="record.status" />
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

const statistics = reactive({
  todayAmount: 15890.50,
  todayCount: 128,
  todayRefund: 1250.00,
  pendingRefund: 5,
});

const methodData = ref([
  { name: '微信支付', count: 85, amount: '¥10,580.00', percent: 66 },
  { name: '支付宝', count: 43, amount: '¥5,310.50', percent: 34 },
]);

const statusData = ref([
  { name: '已支付', count: 120, amount: '¥14,640.50', percent: 94, status: 'success' },
  { name: '待支付', count: 5, amount: '¥750.00', percent: 4, status: 'warning' },
  { name: '已退款', count: 3, amount: '¥500.00', percent: 2, status: 'danger' },
]);

onMounted(() => {
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

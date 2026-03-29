<template>
  <div class="page-container">
    <a-row :gutter="16">
      <a-col :span="6">
        <a-card>
          <a-statistic title="今日订单数" :value="statistics.todayOrders" suffix="笔" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="今日销售额" :value="statistics.todaySales" prefix="¥" :precision="2" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="待发货订单" :value="statistics.pendingShipment" suffix="笔" :value-style="{ color: '#ff7d00' }" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="待处理退款" :value="statistics.pendingRefund" suffix="笔" :value-style="{ color: '#f53f3f' }" />
        </a-card>
      </a-col>
    </a-row>

    <a-card style="margin-top: 16px">
      <template #title>订单趋势</template>
      <div id="chart" style="height: 300px"></div>
    </a-card>

    <a-row :gutter="16" style="margin-top: 16px">
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
                  <a-progress :percent="record.percent" size="small" :status="record.progressStatus" />
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-col>
      <a-col :span="12">
        <a-card>
          <template #title>热销商品TOP10</template>
          <a-table :data="topProducts" :pagination="false">
            <template #columns>
              <a-table-column title="排名" :width="60">
                <template #cell="{ rowIndex }">
                  <a-tag v-if="rowIndex < 3" :color="['gold', 'silver', '#cd7f32'][rowIndex]">{{ rowIndex + 1 }}</a-tag>
                  <span v-else>{{ rowIndex + 1 }}</span>
                </template>
              </a-table-column>
              <a-table-column title="商品名称" data-index="name" />
              <a-table-column title="销量" data-index="sales" :width="80" />
              <a-table-column title="销售额" data-index="amount" :width="100" />
            </template>
          </a-table>
        </a-card>
      </a-col>
    </a-row>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";

const statistics = reactive({
  todayOrders: 128,
  todaySales: 15890.5,
  pendingShipment: 15,
  pendingRefund: 5
});

const statusData = ref([
  { name: "待支付", count: 23, amount: "¥3,450.00", percent: 18, progressStatus: "warning" },
  { name: "已支付", count: 15, amount: "¥2,890.00", percent: 12, progressStatus: "normal" },
  { name: "已发货", count: 45, amount: "¥6,780.00", percent: 35, progressStatus: "normal" },
  { name: "已完成", count: 40, amount: "¥5,230.00", percent: 31, progressStatus: "success" },
  { name: "已取消", count: 5, amount: "¥540.50", percent: 4, progressStatus: "danger" }
]);

const topProducts = ref([
  { name: "iPhone 15 Pro 手机壳", sales: 156, amount: "¥15,444.00" },
  { name: "MacBook Pro 保护套", sales: 89, amount: "¥26,611.00" },
  { name: "AirPods Pro 保护套", sales: 78, amount: "¥3,822.00" },
  { name: "Apple Watch 表带", sales: 65, amount: "¥6,500.00" },
  { name: "iPad 支架", sales: 52, amount: "¥5,200.00" }
]);

onMounted(() => {});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

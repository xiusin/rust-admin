<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-card>
        <template #title>
          <a-space>
            <span>销售统计</span>
            <a-select
              v-if="pluginOptions.length > 0"
              v-model="selectedPluginId"
              placeholder="选择插件"
              style="width: 200px"
              allow-clear
            >
              <a-option :value="0">全部插件</a-option>
              <a-option v-for="plugin in pluginOptions" :key="plugin.id" :value="plugin.id">{{ plugin.name }}</a-option>
            </a-select>
          </a-space>
        </template>
        <template #extra>
          <a-space>
            <a-radio-group v-model="dateRangeType" type="button" size="small">
              <a-radio value="today">今日</a-radio>
              <a-radio value="week">本周</a-radio>
              <a-radio value="month">本月</a-radio>
              <a-radio value="year">本年</a-radio>
              <a-radio value="custom">自定义</a-radio>
            </a-radio-group>
            <a-button v-if="dateRangeType === 'custom'" type="outline" size="small" @click="onSelectDateRange">
              <template #icon><icon-calendar /></template>
              {{ dateRangeText }}
            </a-button>
            <a-button type="outline" size="small" @click="onExport">
              <template #icon><icon-download /></template>
              导出
            </a-button>
          </a-space>
        </template>

        <a-row :gutter="16" style="margin-bottom: 24px">
          <a-col :xs="24" :sm="12" :md="6">
            <a-statistic title="总收入" :value="statsData.totalRevenue" :precision="2" prefix="¥" />
          </a-col>
          <a-col :xs="24" :sm="12" :md="6">
            <a-statistic title="订单数" :value="statsData.totalOrders" suffix="笔" />
          </a-col>
          <a-col :xs="24" :sm="12" :md="6">
            <a-statistic title="下载量" :value="statsData.totalDownloads" suffix="次" />
          </a-col>
          <a-col :xs="24" :sm="12" :md="6">
            <a-statistic title="转化率" :value="statsData.conversionRate" suffix="%" :precision="2" />
          </a-col>
        </a-row>

        <a-row :gutter="16">
          <a-col :xs="24" :lg="12">
            <div class="chart-container">
              <div class="chart-title">收入趋势</div>
              <div ref="revenueChartRef" style="height: 300px"></div>
            </div>
          </a-col>
          <a-col :xs="24" :lg="12">
            <div class="chart-container">
              <div class="chart-title">订单趋势</div>
              <div ref="orderChartRef" style="height: 300px"></div>
            </div>
          </a-col>
        </a-row>

        <a-row :gutter="16" style="margin-top: 24px">
          <a-col :xs="24" :lg="12">
            <div class="chart-container">
              <div class="chart-title">下载量趋势</div>
              <div ref="downloadChartRef" style="height: 300px"></div>
            </div>
          </a-col>
          <a-col :xs="24" :lg="12">
            <a-card title="插件销售排行" :bordered="false">
              <a-table :data="pluginRanking" :pagination="false" :bordered="false">
                <template #columns>
                  <a-table-column title="排名" :width="60">
                    <template #cell="{ rowIndex }">
                      <a-tag v-if="rowIndex < 3" :color="['gold', 'silver', '#cd7f32'][rowIndex]">{{ rowIndex + 1 }}</a-tag>
                      <span v-else>{{ rowIndex + 1 }}</span>
                    </template>
                  </a-table-column>
                  <a-table-column title="插件名称" dataIndex="name" />
                  <a-table-column title="销售额" :width="100">
                    <template #cell="{ record }">
                      <span class="price">¥{{ record.revenue }}</span>
                    </template>
                  </a-table-column>
                  <a-table-column title="订单数" dataIndex="orders" :width="80" />
                </template>
              </a-table>
            </a-card>
          </a-col>
        </a-row>
      </a-card>

      <a-card title="订单明细" style="margin-top: 16px">
        <a-table
          row-key="id"
          :loading="loading"
          :columns="columns"
          :data="orderList"
          :pagination="pagination"
          @page-change="handlePageChange"
          @page-size-change="handlePageSizeChange"
        >
          <template #pluginName="{ record }">
            <div class="plugin-cell">
              <img v-if="record.pluginCover" :src="record.pluginCover" class="plugin-cover" />
              <span>{{ record.pluginName }}</span>
            </div>
          </template>
          <template #amount="{ record }">
            <span class="price">¥{{ record.amount }}</span>
          </template>
          <template #status="{ record }">
            <a-tag :color="getOrderStatusColor(record.status)" size="small">{{ record.statusName }}</a-tag>
          </template>
          <template #paymentMethod="{ record }">
            {{ record.paymentMethodName }}
          </template>
        </a-table>
      </a-card>

      <a-modal
        v-model:visible="dateRangeModalVisible"
        title="选择日期范围"
        :width="400"
        @ok="onConfirmDateRange"
        @cancel="dateRangeModalVisible = false"
      >
        <a-form layout="vertical">
          <a-form-item label="开始日期">
            <a-date-picker v-model="customDateRange.start" style="width: 100%" />
          </a-form-item>
          <a-form-item label="结束日期">
            <a-date-picker v-model="customDateRange.end" style="width: 100%" />
          </a-form-item>
        </a-form>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from "vue";
import { useRoute } from "vue-router";
import { default as VChart } from "@visactor/vchart";
import { developer } from "@/api/modules/plugin-market/market";
import { order } from "@/api/modules/plugin-market/order";
import { Message } from "@arco-design/web-vue";

interface OrderItem {
  id: number;
  orderNo: string;
  pluginId: number;
  pluginName: string;
  pluginCover?: string;
  planName: string;
  amount: number;
  status: number;
  statusName: string;
  paymentMethod: number;
  paymentMethodName: string;
  createdAt: string;
}

interface PluginOption {
  id: number;
  name: string;
}

const route = useRoute();
const loading = ref(false);
const revenueChartRef = ref();
const orderChartRef = ref();
const downloadChartRef = ref();
const dateRangeModalVisible = ref(false);
const selectedPluginId = ref(0);
const dateRangeType = ref("month");
const dateRangeText = ref("请选择日期");

const customDateRange = reactive({
  start: "",
  end: ""
});

const pluginOptions = ref<PluginOption[]>([
  { id: 1, name: "智能优惠券" },
  { id: 2, name: "限时秒杀" },
  { id: 3, name: "AI智能客服" }
]);

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { title: "订单号", dataIndex: "orderNo", width: 180 },
  { title: "插件", slotName: "pluginName", width: 200 },
  { title: "方案", dataIndex: "planName", width: 120 },
  { title: "金额", slotName: "amount", width: 100 },
  { title: "状态", slotName: "status", width: 90 },
  { title: "支付方式", slotName: "paymentMethod", width: 100 },
  { title: "下单时间", dataIndex: "createdAt", width: 180 }
];

const statsData = reactive({
  totalRevenue: 0,
  totalOrders: 0,
  totalDownloads: 0,
  conversionRate: 0
});

const pluginRanking = ref([]);

const orderList = ref<OrderItem[]>([]);

const chartData = ref({
  revenue: [],
  orders: [],
  downloads: []
});

const getChartData = async () => {
  try {
    const res = await developer.chart({
      type: dateRangeType.value,
      pluginId: selectedPluginId.value || undefined,
      startDate: customDateRange.start,
      endDate: customDateRange.end
    });
    if (res.data) {
      chartData.value = res.data;
    }
  } catch (error) {
    console.error(error);
  }
  return chartData.value;
};

const fetchRanking = async () => {
  try {
    const res = await developer.ranking({
      type: dateRangeType.value,
      startDate: customDateRange.start,
      endDate: customDateRange.end
    });
    pluginRanking.value = res.data || [];
  } catch (error) {
    console.error(error);
  }
};

const getOrderStatusColor = (status: number) => {
  const colors: Record<number, string> = { 0: "orange", 1: "green", 2: "gray", 3: "red" };
  return colors[status] || "default";
};

const initRevenueChart = async () => {
  if (!revenueChartRef.value) return;
  const data = chartData.value;
  const xField = dateRangeType.value === "year" ? "month" : "day";
  const spec = {
    type: "line",
    data: [{ id: "revenue", values: data.revenue }],
    xField,
    yField: "value",
    smooth: true,
    label: { visible: true, position: "top" },
    area: { style: { fill: "gradient", fillColor: [{ start: "#165dff", end: "#ffffff" }] } },
    tooltip: { mark: { content: [{ key: (datum: any) => datum[xField], value: (datum: any) => `¥${datum.value}` }] } }
  };
  const vchart = new VChart(spec as any, { dom: revenueChartRef.value });
  vchart.renderSync();
};

const initOrderChart = async () => {
  if (!orderChartRef.value) return;
  const data = chartData.value;
  const xField = dateRangeType.value === "year" ? "month" : "day";
  const spec = {
    type: "bar",
    data: [{ id: "orders", values: data.orders }],
    xField,
    yField: "value",
    barWidth: 20,
    tooltip: { mark: { content: [{ key: (datum: any) => datum[xField], value: (datum: any) => `${datum.value} 笔` }] } }
  };
  const vchart = new VChart(spec as any, { dom: orderChartRef.value });
  vchart.renderSync();
};

const initDownloadChart = async () => {
  if (!downloadChartRef.value) return;
  const data = chartData.value;
  const xField = dateRangeType.value === "year" ? "month" : "day";
  const spec = {
    type: "area",
    data: [{ id: "downloads", values: data.downloads }],
    xField,
    yField: "value",
    smooth: true,
    area: { style: { fill: "gradient", fillColor: [{ start: "#00b42a", end: "#ffffff" }] } },
    tooltip: { mark: { content: [{ key: (datum: any) => datum[xField], value: (datum: any) => `${datum.value} 次` }] } }
  };
  const vchart = new VChart(spec as any, { dom: downloadChartRef.value });
  vchart.renderSync();
};

const initCharts = async () => {
  await getChartData();
  if (revenueChartRef.value) revenueChartRef.value.innerHTML = '';
  if (orderChartRef.value) orderChartRef.value.innerHTML = '';
  if (downloadChartRef.value) downloadChartRef.value.innerHTML = '';
  initRevenueChart();
  initOrderChart();
  initDownloadChart();
  fetchRanking();
  fetchOrderList();
};

const fetchStats = async () => {
  try {
    const res = await developer.stats();
    if (res.data) {
      statsData.totalRevenue = res.data.totalRevenue || 0;
      statsData.totalOrders = res.data.totalOrders || 0;
      statsData.totalDownloads = res.data.totalDownloads || 0;
      statsData.conversionRate = res.data.conversionRate || 0;
    }
  } catch (error) {
    console.error(error);
  }
};

const fetchOrderList = async () => {
  loading.value = true;
  try {
    const res = await order.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      pluginId: selectedPluginId.value || undefined
    });
    orderList.value = res.data?.list || [];
    pagination.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  fetchOrderList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  fetchOrderList();
};

const onSelectDateRange = () => {
  dateRangeModalVisible.value = true;
};

const onConfirmDateRange = () => {
  if (customDateRange.start && customDateRange.end) {
    dateRangeText.value = `${customDateRange.start} - ${customDateRange.end}`;
    initCharts();
  }
  dateRangeModalVisible.value = false;
};

const onExport = () => {
  Message.success("导出成功");
};

watch(dateRangeType, () => {
  initCharts();
});

watch(selectedPluginId, () => {
  initCharts();
});

onMounted(() => {
  const pluginId = route.params.pluginId;
  if (pluginId) {
    selectedPluginId.value = Number(pluginId);
  }
  initCharts();
  fetchStats();
  fetchOrderList();
});
</script>

<style scoped lang="scss">
.chart-container {
  padding: 16px;
  background: var(--color-fill-1);
  border-radius: 8px;

  .chart-title {
    font-size: 14px;
    font-weight: 500;
    margin-bottom: 16px;
    color: $color-text-1;
  }
}

.plugin-cell {
  display: flex;
  align-items: center;
  gap: 8px;

  .plugin-cover {
    width: 32px;
    height: 32px;
    border-radius: 4px;
    object-fit: cover;
  }
}

.price {
  color: #f53f3f;
  font-weight: 600;
}
</style>

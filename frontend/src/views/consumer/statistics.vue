<template>
  <div class="statistics-container">
    <a-card>
      <template #title>消费统计</template>
      <template #extra>
        <a-space>
          <a-input-number v-model="searchForm.consumer_id" placeholder="用户ID" style="width: 150px" />
          <a-select v-model="searchForm.year" placeholder="年份" style="width: 120px">
            <a-option v-for="year in yearOptions" :key="year" :value="year.toString()">{{ year }}年</a-option>
          </a-select>
          <a-button type="primary" @click="loadStatistics">
            <template #icon><icon-search /></template>
            查询
          </a-button>
        </a-space>
      </template>

      <a-row :gutter="20" style="margin-bottom: 20px">
        <a-col :span="6">
          <a-card>
            <a-statistic title="订单总数" :value="statistics.total_orders" suffix="笔" />
          </a-card>
        </a-col>
        <a-col :span="6">
          <a-card>
            <a-statistic title="消费总额" :value="statistics.total_amount" prefix="¥" />
          </a-card>
        </a-col>
        <a-col :span="6">
          <a-card>
            <a-statistic title="退款金额" :value="statistics.total_refund" prefix="¥" />
          </a-card>
        </a-col>
        <a-col :span="6">
          <a-card>
            <a-statistic title="实际消费" :value="statistics.total_expense" prefix="¥" />
          </a-card>
        </a-col>
      </a-row>

      <a-card title="消费趋势">
        <div ref="chartRef" style="height: 300px"></div>
      </a-card>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from "vue";
import { Message } from "@arco-design/web-vue";
import { userExtensionApi, ConsumerStatisticsModel } from "@/api/modules/consumer/userExtension";
import * as echarts from "echarts";

const chartRef = ref<HTMLElement>();
let chart: echarts.ECharts | null = null;

const searchForm = reactive({
  consumer_id: 0,
  year: new Date().getFullYear().toString()
});

const statistics = ref<ConsumerStatisticsModel>({
  id: 0,
  consumer_id: 0,
  year: new Date().getFullYear(),
  total_orders: 0,
  total_amount: "0",
  total_refund: "0",
  total_expense: "0",
  avg_order_amount: "0"
});

const trendData = ref<{ month: string; amount: string }[]>([]);

const yearOptions = computed(() => {
  const currentYear = new Date().getFullYear();
  return Array.from({ length: 5 }, (_, i) => currentYear - i);
});

const loadStatistics = async () => {
  if (!searchForm.consumer_id) {
    Message.warning("请输入用户ID");
    return;
  }
  try {
    const res = await userExtensionApi.getYearStatistics({
      consumer_id: searchForm.consumer_id,
      year: searchForm.year
    });
    statistics.value = res.data || statistics.value;

    const trendRes = await userExtensionApi.getConsumeTrend({
      consumer_id: searchForm.consumer_id,
      months: 12
    });
    trendData.value = trendRes.data || [];
    updateChart();
  } catch (error) {
    console.error(error);
  }
};

const updateChart = () => {
  if (!chartRef.value) return;

  if (!chart) {
    chart = echarts.init(chartRef.value);
  }

  const option = {
    tooltip: {
      trigger: "axis"
    },
    xAxis: {
      type: "category",
      data: trendData.value.map(item => item.month)
    },
    yAxis: {
      type: "value",
      axisLabel: {
        formatter: "¥{value}"
      }
    },
    series: [
      {
        name: "消费金额",
        type: "line",
        smooth: true,
        data: trendData.value.map(item => parseFloat(item.amount) || 0),
        areaStyle: {
          color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
            { offset: 0, color: "rgba(24, 144, 255, 0.3)" },
            { offset: 1, color: "rgba(24, 144, 255, 0.05)" }
          ])
        },
        lineStyle: {
          color: "#1890ff"
        },
        itemStyle: {
          color: "#1890ff"
        }
      }
    ]
  };

  chart.setOption(option);
};

onMounted(() => {
  window.addEventListener("resize", () => {
    chart?.resize();
  });
});
</script>

<style scoped>
.statistics-container {
  padding: 20px;
}
</style>

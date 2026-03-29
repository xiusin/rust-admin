<template>
  <div class="snow-page">
    <div class="dashboard-page">
      <a-row :gutter="16">
        <a-col :xs="24" :sm="12" :md="6">
          <a-card hoverable class="stat-card">
            <div class="stat-icon revenue">
              <icon-money />
            </div>
            <a-statistic
              title="总收入"
              :value="statsData.totalRevenue"
              :precision="2"
              prefix="¥"
              :value-from="0"
              :start="true"
              animation
            />
            <div class="stat-trend up">
              <icon-caret-up />
              <span>{{ statsData.revenueGrowth }}%</span>
              <span class="trend-text">较上月</span>
            </div>
          </a-card>
        </a-col>
        <a-col :xs="24" :sm="12" :md="6">
          <a-card hoverable class="stat-card">
            <div class="stat-icon orders">
              <icon-shopping-cart />
            </div>
            <a-statistic title="订单数" :value="statsData.totalOrders" suffix="笔" :value-from="0" :start="true" animation />
            <div class="stat-trend up">
              <icon-caret-up />
              <span>{{ statsData.ordersGrowth }}%</span>
              <span class="trend-text">较上月</span>
            </div>
          </a-card>
        </a-col>
        <a-col :xs="24" :sm="12" :md="6">
          <a-card hoverable class="stat-card">
            <div class="stat-icon downloads">
              <icon-download />
            </div>
            <a-statistic title="下载量" :value="statsData.totalDownloads" suffix="次" :value-from="0" :start="true" animation />
            <div class="stat-trend down">
              <icon-caret-down />
              <span>{{ statsData.downloadsGrowth }}%</span>
              <span class="trend-text">较上月</span>
            </div>
          </a-card>
        </a-col>
        <a-col :xs="24" :sm="12" :md="6">
          <a-card hoverable class="stat-card">
            <div class="stat-icon plugins">
              <icon-code />
            </div>
            <a-statistic title="插件数" :value="statsData.totalPlugins" suffix="个" :value-from="0" :start="true" animation />
            <div class="stat-trend neutral">
              <icon-minus />
              <span>0%</span>
              <span class="trend-text">较上月</span>
            </div>
          </a-card>
        </a-col>
      </a-row>

      <a-row :gutter="16" style="margin-top: 16px">
        <a-col :xs="24" :lg="16">
          <a-card>
            <template #title>
              <span>收入趋势</span>
              <a-radio-group v-model="revenueChartType" type="button" size="small" style="margin-left: 16px">
                <a-radio value="week">本周</a-radio>
                <a-radio value="month">本月</a-radio>
                <a-radio value="year">本年</a-radio>
              </a-radio-group>
            </template>
            <div ref="revenueChartRef" style="height: 300px"></div>
          </a-card>
        </a-col>
        <a-col :xs="24" :lg="8">
          <a-card>
            <template #title>收入构成</template>
            <div ref="pieChartRef" style="height: 300px"></div>
          </a-card>
        </a-col>
      </a-row>

      <a-row :gutter="16" style="margin-top: 16px">
        <a-col :xs="24" :lg="12">
          <a-card>
            <template #title>近期订单</template>
            <template #extra>
              <a-link @click="goToOrders">查看全部</a-link>
            </template>
            <a-table :data="recentOrders" :pagination="false" :bordered="false">
              <template #columns>
                <a-table-column title="订单号" data-index="orderNo" :width="180" />
                <a-table-column title="插件" data-index="pluginName" />
                <a-table-column title="金额" :width="100">
                  <template #cell="{ record }">
                    <span class="price-text">¥{{ record.amount }}</span>
                  </template>
                </a-table-column>
                <a-table-column title="状态" :width="80">
                  <template #cell="{ record }">
                    <a-tag :color="getOrderStatusColor(record.status)" size="small">{{ record.statusName }}</a-tag>
                  </template>
                </a-table-column>
                <a-table-column title="时间" data-index="createdAt" :width="160" />
              </template>
            </a-table>
          </a-card>
        </a-col>
        <a-col :xs="24" :lg="12">
          <a-card>
            <template #title>热门插件</template>
            <template #extra>
              <a-link @click="goToPlugins">管理插件</a-link>
            </template>
            <div class="top-plugin-list">
              <div v-for="(plugin, index) in topPlugins" :key="plugin.id" class="top-plugin-item">
                <div class="plugin-rank" :class="{ gold: index === 0, silver: index === 1, bronze: index === 2 }">
                  {{ index + 1 }}
                </div>
                <div class="plugin-info">
                  <div class="plugin-name">{{ plugin.name }}</div>
                  <div class="plugin-stats">
                    <span><icon-download /> {{ plugin.downloadCount }}</span>
                    <span><icon-star-fill /> {{ plugin.rating }}</span>
                    <span class="price">¥{{ plugin.revenue }}</span>
                  </div>
                </div>
                <a-tag v-if="plugin.status === 1" color="green" size="small">上架</a-tag>
                <a-tag v-else color="orange" size="small">待审</a-tag>
              </div>
            </div>
          </a-card>
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watch } from "vue";
import { useRouter } from "vue-router";
import { default as VChart } from "@visactor/vchart";
import { developer } from "@/api/modules/plugin-market/market";

const router = useRouter();
const loading = ref(false);
const revenueChartType = ref("month");
const revenueChartRef = ref();
const pieChartRef = ref();

const statsData = reactive({
  totalRevenue: 0,
  revenueGrowth: 0,
  totalOrders: 0,
  ordersGrowth: 0,
  totalDownloads: 0,
  downloadsGrowth: 0,
  totalPlugins: 0
});

const recentOrders = ref([
  {
    id: 1,
    orderNo: "PLM202603201030000001",
    pluginName: "智能优惠券",
    amount: 299,
    status: 1,
    statusName: "已支付",
    createdAt: "2024-03-20 10:30"
  },
  {
    id: 2,
    orderNo: "PLM202603191520000002",
    pluginName: "限时秒杀",
    amount: 199,
    status: 1,
    statusName: "已支付",
    createdAt: "2024-03-19 15:20"
  },
  {
    id: 3,
    orderNo: "PLM202603181030000003",
    pluginName: "数据统计分析",
    amount: 399,
    status: 1,
    statusName: "已支付",
    createdAt: "2024-03-18 10:30"
  },
  {
    id: 4,
    orderNo: "PLM202603171520000004",
    pluginName: "AI智能客服",
    amount: 799,
    status: 0,
    statusName: "待支付",
    createdAt: "2024-03-17 15:20"
  }
]);

const topPlugins = ref([
  { id: 1, name: "智能优惠券", downloadCount: 2560, rating: 4.8, revenue: 125600, status: 1 },
  { id: 2, name: "限时秒杀", downloadCount: 1890, rating: 4.9, revenue: 89600, status: 1 },
  { id: 3, name: "AI智能客服", downloadCount: 2100, rating: 4.9, revenue: 78600, status: 1 },
  { id: 4, name: "短信通知", downloadCount: 1230, rating: 4.6, revenue: 45600, status: 2 },
  { id: 5, name: "数据统计分析", downloadCount: 980, rating: 4.7, revenue: 32100, status: 1 }
]);

const revenueChartData = {
  week: [
    { day: "周一", revenue: 1200 },
    { day: "周二", revenue: 1500 },
    { day: "周三", revenue: 1800 },
    { day: "周四", revenue: 2100 },
    { day: "周五", revenue: 2400 },
    { day: "周六", revenue: 2800 },
    { day: "周日", revenue: 3200 }
  ],
  month: [
    { day: "1日", revenue: 1200 },
    { day: "5日", revenue: 1500 },
    { day: "10日", revenue: 1800 },
    { day: "15日", revenue: 2100 },
    { day: "20日", revenue: 2400 },
    { day: "25日", revenue: 2800 },
    { day: "30日", revenue: 3200 }
  ],
  year: [
    { month: "1月", revenue: 12000 },
    { month: "2月", revenue: 15000 },
    { month: "3月", revenue: 18000 },
    { month: "4月", revenue: 21000 },
    { month: "5月", revenue: 24000 },
    { month: "6月", revenue: 28000 },
    { month: "7月", revenue: 32000 },
    { month: "8月", revenue: 35000 },
    { month: "9月", revenue: 38000 },
    { month: "10月", revenue: 42000 },
    { month: "11月", revenue: 45000 },
    { month: "12月", revenue: 48000 }
  ]
};

const pieChartData = [
  { type: "智能优惠券", value: 45 },
  { type: "限时秒杀", value: 25 },
  { type: "AI智能客服", value: 18 },
  { type: "短信通知", value: 8 },
  { type: "其他", value: 4 }
];

const getOrderStatusColor = (status: number) => {
  const colors: Record<number, string> = { 0: "orange", 1: "green", 2: "gray", 3: "red" };
  return colors[status] || "default";
};

const initRevenueChart = () => {
  if (!revenueChartRef.value) return;
  const data =
    revenueChartType.value === "year"
      ? revenueChartData.year
      : revenueChartType.value === "month"
        ? revenueChartData.month
        : revenueChartData.week;
  const xField = revenueChartType.value === "year" ? "month" : "day";
  const spec = {
    type: "bar",
    data: [{ id: "revenue", values: data }],
    xField,
    yField: "revenue",
    barWidth: 20,
    label: { visible: true, position: "top" },
    tooltip: { mark: { content: [{ key: (datum: any) => datum[xField], value: (datum: any) => `¥${datum.revenue}` }] } }
  };
  const vchart = new VChart(spec as any, { dom: revenueChartRef.value });
  vchart.renderSync();
};

const initPieChart = () => {
  if (!pieChartRef.value) return;
  const spec = {
    type: "pie",
    data: [{ id: "pie", values: pieChartData }],
    outerRadius: 0.7,
    innerRadius: 0.5,
    valueField: "value",
    categoryField: "type",
    label: { visible: true },
    tooltip: { mark: { content: [{ key: (datum: any) => datum["type"], value: (datum: any) => `${datum.value}%` }] } }
  };
  const vchart = new VChart(spec as any, { dom: pieChartRef.value });
  vchart.renderSync();
};

const fetchStats = async () => {
  loading.value = true;
  try {
    const res = await developer.stats();
    Object.assign(statsData, res.data || {});
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const goToOrders = () => {
  router.push({ name: "plugin-order-list" });
};

const goToPlugins = () => {
  router.push({ name: "plugin-list" });
};

watch(revenueChartType, () => {
  initRevenueChart();
});

onMounted(() => {
  initRevenueChart();
  initPieChart();
});
</script>

<style lang="scss" scoped>
.dashboard-page {
  padding: $padding;
  background: $color-bg-1;
}

.stat-card {
  position: relative;
  overflow: hidden;

  .stat-icon {
    position: absolute;
    top: 16px;
    right: 16px;
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 8px;
    font-size: 24px;
    color: #fff;

    &.revenue {
      background: linear-gradient(135deg, #165dff, #4080ff);
    }
    &.orders {
      background: linear-gradient(135deg, #00b42a, #23c343);
    }
    &.downloads {
      background: linear-gradient(135deg, #ff7d00, #ff9d00);
    }
    &.plugins {
      background: linear-gradient(135deg, #722ed1, #9254de);
    }
  }

  .stat-trend {
    display: flex;
    align-items: center;
    margin-top: 8px;
    font-size: 12px;

    &.up {
      color: #00b42a;
    }
    &.down {
      color: #f53f3f;
    }
    &.neutral {
      color: #86909c;
    }

    .trend-text {
      margin-left: 4px;
      color: $color-text-3;
    }
  }
}

.price-text {
  color: #f53f3f;
  font-weight: 600;
}

.top-plugin-list {
  .top-plugin-item {
    display: flex;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid var(--color-fill-2);

    &:last-child {
      border-bottom: none;
    }
  }

  .plugin-rank {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 4px;
    font-weight: 600;
    font-size: 12px;
    margin-right: 12px;
    background: #e5e6eb;
    color: $color-text-3;

    &.gold {
      background: linear-gradient(135deg, #ffd700, #ffb700);
      color: #fff;
    }
    &.silver {
      background: linear-gradient(135deg, #c0c0c0, #a8a8a8);
      color: #fff;
    }
    &.bronze {
      background: linear-gradient(135deg, #cd7f32, #b87333);
      color: #fff;
    }
  }

  .plugin-info {
    flex: 1;

    .plugin-name {
      font-weight: 500;
      margin-bottom: 4px;
    }

    .plugin-stats {
      display: flex;
      gap: 12px;
      font-size: 12px;
      color: $color-text-3;

      span {
        display: flex;
        align-items: center;
        gap: 2px;
      }

      .price {
        color: #f53f3f;
        font-weight: 500;
      }
    }
  }
}
</style>

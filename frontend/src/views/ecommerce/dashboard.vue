<template>
  <div class="ecommerce-dashboard">
    <!-- 概览区域 -->
    <div class="overview-section">
      <div class="overview-header">
        <h1 class="overview-title">电商管理</h1>
        <div class="overview-actions">
          <a-button type="primary" @click="addPlatform">
            <template #icon><icon-plus /></template>
            新增平台
          </a-button>
          <a-button @click="syncData">
            <template #icon><icon-refresh /></template>
            同步数据
          </a-button>
        </div>
      </div>
      
      <!-- 核心指标 -->
      <div class="metrics-grid">
        <div 
          v-for="metric in metrics" 
          :key="metric.id" 
          class="metric-card"
          :class="metric.color"
        >
          <div class="metric-icon">
            <s-svg-icon :name="metric.icon" :size="28" />
          </div>
          <div class="metric-content">
            <div class="metric-value">{{ metric.value }}</div>
            <div class="metric-label">{{ metric.label }}</div>
            <div class="metric-change" :class="metric.changeType">
              <span v-if="metric.changeType === 'positive'">↑</span>
              <span v-else>↓</span>
              {{ metric.change }}%
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 数据概览 -->
    <div class="data-overview">
      <!-- 平台管理 -->
      <div class="panel">
        <div class="panel-header">
          <h2 class="panel-title">平台管理</h2>
          <a-button type="text" @click="viewAllPlatforms">
            查看全部
          </a-button>
        </div>
        <div class="panel-content">
          <a-table 
            :columns="platformColumns" 
            :data="platforms" 
            :loading="loading"
            :pagination="false"
            size="small"
          >
            <template #bodyCell="{ record, column }">
              <template v-if="column.key === 'status'">
                <a-tag :color="record.status === 1 ? 'success' : 'default'" size="small">
                  {{ record.status === 1 ? '启用' : '禁用' }}
                </a-tag>
              </template>
              <template v-else-if="column.key === 'actions'">
                <a-space size="small">
                  <a-button type="text" size="small" @click="editPlatform(record)">
                    编辑
                  </a-button>
                  <a-button type="text" size="small" @click="testConnection(record.id)">
                    测试
                  </a-button>
                </a-space>
              </template>
              <template v-else>
                {{ record[column.key] }}
              </template>
            </template>
          </a-table>
        </div>
      </div>

      <!-- 最近订单 -->
      <div class="panel">
        <div class="panel-header">
          <h2 class="panel-title">最近订单</h2>
          <a-button type="text" @click="viewAllOrders">
            查看全部
          </a-button>
        </div>
        <div class="panel-content">
          <a-table 
            :columns="orderColumns" 
            :data="recentOrders" 
            :loading="loading"
            :pagination="false"
            size="small"
          >
            <template #bodyCell="{ record, column }">
              <template v-if="column.key === 'status'">
                <a-tag :color="getStatusColor(record.status)" size="small">
                  {{ record.status }}
                </a-tag>
              </template>
              <template v-else-if="column.key === 'amount'">
                <span class="order-amount">¥{{ record.amount.toFixed(2) }}</span>
              </template>
              <template v-else>
                {{ record[column.key] }}
              </template>
            </template>
          </a-table>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const loading = ref(false)

// 核心指标数据
const metrics = ref([
  {
    id: 1,
    value: '¥128,500',
    label: '总销售额',
    change: '+15.2',
    changeType: 'positive',
    icon: 'data-analysis',
    color: 'primary'
  },
  {
    id: 2,
    value: '1,248',
    label: '订单数',
    change: '+8.7',
    changeType: 'positive',
    icon: 'data-queries',
    color: 'success'
  },
  {
    id: 3,
    value: '89.3%',
    label: '订单完成率',
    change: '+2.1',
    changeType: 'positive',
    icon: 'financial-statement',
    color: 'warning'
  },
  {
    id: 4,
    value: '5',
    label: '已连接平台',
    change: '+2',
    changeType: 'positive',
    icon: 'earth',
    color: 'default'
  }
])

// 平台数据
const platforms = ref([
  {
    id: 1,
    platformType: 'taobao',
    name: '淘宝店铺',
    status: 1,
    createdAt: '2026-04-01'
  },
  {
    id: 2,
    platformType: 'pdd',
    name: '拼多多店铺',
    status: 1,
    createdAt: '2026-04-02'
  },
  {
    id: 3,
    platformType: 'douyin',
    name: '抖音小店',
    status: 0,
    createdAt: '2026-04-03'
  }
])

// 平台表格列
const platformColumns = [
  { title: '平台', dataIndex: 'name', key: 'name', width: 120 },
  { title: '类型', dataIndex: 'platformType', key: 'platformType', width: 100 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 80 },
  { title: '创建时间', dataIndex: 'createdAt', key: 'createdAt', width: 120 },
  { title: '操作', key: 'actions', width: 100 }
]

// 最近订单数据
const recentOrders = ref([
  {
    id: 'TB12345',
    platform: '淘宝',
    amount: 299.99,
    status: '已完成',
    createdAt: '2026-04-17 10:30'
  },
  {
    id: 'PD67890',
    platform: '拼多多',
    amount: 159.90,
    status: '待发货',
    createdAt: '2026-04-17 09:15'
  },
  {
    id: 'DY54321',
    platform: '抖音',
    amount: 89.99,
    status: '已付款',
    createdAt: '2026-04-17 08:45'
  }
])

// 订单表格列
const orderColumns = [
  { title: '订单号', dataIndex: 'id', key: 'id', width: 120 },
  { title: '平台', dataIndex: 'platform', key: 'platform', width: 80 },
  { title: '金额', dataIndex: 'amount', key: 'amount', width: 100 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 80 },
  { title: '创建时间', dataIndex: 'createdAt', key: 'createdAt', width: 140 }
]

// 获取状态颜色
const getStatusColor = (status: string) => {
  const colorMap: Record<string, string> = {
    '已完成': 'success',
    '待发货': 'warning',
    '已付款': 'blue',
    '已取消': 'default'
  }
  return colorMap[status] || 'default'
}

// 新增平台
const addPlatform = () => {
  router.push('/ecommerce/platforms/create')
}

// 编辑平台
const editPlatform = (record: any) => {
  router.push(`/ecommerce/platforms/edit/${record.id}`)
}

// 测试连接
const testConnection = (id: number) => {
  // 实现测试连接逻辑
  console.log('测试连接:', id)
}

// 同步数据
const syncData = () => {
  // 实现同步数据逻辑
  console.log('同步数据')
}

// 查看全部平台
const viewAllPlatforms = () => {
  router.push('/ecommerce/platforms/list')
}

// 查看全部订单
const viewAllOrders = () => {
  router.push('/ecommerce/orders')
}

// 生命周期
onMounted(() => {
  // 初始化数据
  console.log('电商仪表板加载完成')
})
</script>

<style lang="scss" scoped>
.ecommerce-dashboard {
  background: $color-bg-1;
  min-height: 100vh;
  padding: 24px;
}

.overview-section {
  margin-bottom: 32px;

  .overview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;

    .overview-title {
      font-size: 28px;
      font-weight: 600;
      color: $color-text-1;
      margin: 0;
    }

    .overview-actions {
      display: flex;
      gap: 12px;
    }
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 16px;

    .metric-card {
      background: white;
      border-radius: 12px;
      padding: 24px;
      display: flex;
      align-items: center;
      gap: 20px;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      border: 1px solid transparent;

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.08);
      }

      .metric-icon {
        width: 56px;
        height: 56px;
        border-radius: 12px;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;

        & + .metric-content {
          flex: 1;
        }
      }

      .metric-content {
        .metric-value {
          font-size: 24px;
          font-weight: 600;
          color: $color-text-1;
          margin-bottom: 4px;
        }

        .metric-label {
          font-size: 14px;
          color: $color-text-3;
          margin-bottom: 8px;
        }

        .metric-change {
          font-size: 12px;
          font-weight: 500;
          display: flex;
          align-items: center;
          gap: 4px;

          &.positive {
            color: #52c41a;
          }

          &.negative {
            color: #ff4d4f;
          }
        }
      }

      &.primary {
        .metric-icon {
          background: #e6f7ff;
          color: #1890ff;
        }
      }

      &.success {
        .metric-icon {
          background: #f6ffed;
          color: #52c41a;
        }
      }

      &.warning {
        .metric-icon {
          background: #fff7e6;
          color: #fa8c16;
        }
      }

      &.default {
        .metric-icon {
          background: #f9f0ff;
          color: #722ed1;
        }
      }
    }
  }
}

.data-overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(500px, 1fr));
  gap: 24px;

  .panel {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
    overflow: hidden;

    .panel-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 20px 24px;
      border-bottom: 1px solid $color-border;

      .panel-title {
        font-size: 18px;
        font-weight: 600;
        color: $color-text-1;
        margin: 0;
      }
    }

    .panel-content {
      padding: 0;

      :deep(.arco-table-container) {
        border: none;
        border-radius: 0;
      }

      :deep(.arco-table-th) {
        font-weight: 500;
        color: $color-text-2;
        background: $color-bg-1;
      }

      :deep(.arco-table-tr) {
        &:hover {
          background: $color-bg-1;
        }
      }

      .order-amount {
        font-weight: 500;
        color: $color-text-1;
      }
    }
  }
}

@media (max-width: 768px) {
  .ecommerce-dashboard {
    padding: 16px;
  }

  .overview-section {
    .overview-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;

      .overview-actions {
        width: 100%;
        justify-content: space-between;
      }
    }

    .metrics-grid {
      grid-template-columns: 1fr;
    }
  }

  .data-overview {
    grid-template-columns: 1fr;

    .panel {
      .panel-header {
        padding: 16px 20px;

        .panel-title {
          font-size: 16px;
        }
      }
    }
  }
}
</style>

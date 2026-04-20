<template>
  <div class="platforms-list">
    <div class="page-header">
      <h1 class="page-title">平台管理</h1>
      <a-button type="primary" @click="createPlatform">
        <template #icon><icon-plus /></template>
        新增平台
      </a-button>
    </div>
    
    <div class="panel">
      <div class="panel-header">
        <div class="search-bar">
          <a-input-search
            placeholder="搜索平台名称"
            v-model:value="searchKeyword"
            @search="handleSearch"
            style="width: 300px"
          >
            <template #addonAfter>
              <a-select v-model:value="platformType" placeholder="平台类型">
                <a-option value="">全部</a-option>
                <a-option value="taobao">淘宝</a-option>
                <a-option value="pdd">拼多多</a-option>
                <a-option value="douyin">抖音</a-option>
                <a-option value="xianyu">闲鱼</a-option>
                <a-option value="amazon">亚马逊</a-option>
                <a-option value="wechat">微信</a-option>
              </a-select>
            </template>
          </a-input-search>
        </div>
      </div>
      
      <div class="panel-content">
        <a-table
          :columns="columns"
          :data="platforms"
          :loading="loading"
          :pagination="pagination"
          @change="handleTableChange"
          size="default"
        >
          <template #bodyCell="{ record, column }">
            <template v-if="column.key === 'status'">
              <a-tag :color="record.status === 1 ? 'success' : 'default'">
                {{ record.status === 1 ? '启用' : '禁用' }}
              </a-tag>
            </template>
            <template v-else-if="column.key === 'actions'">
              <a-space size="small">
                <a-button type="text" @click="editPlatform(record)">
                  编辑
                </a-button>
                <a-button type="text" @click="testConnection(record.id)">
                  测试
                </a-button>
                <a-button 
                  type="text" 
                  :color="record.status === 1 ? 'default' : 'success'"
                  @click="toggleStatus(record)"
                >
                  {{ record.status === 1 ? '禁用' : '启用' }}
                </a-button>
                <a-button type="text" danger @click="deletePlatform(record.id)">
                  删除
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const loading = ref(false)
const searchKeyword = ref('')
const platformType = ref('')

// 分页配置
const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0
})

// 平台数据
const platforms = ref([
  {
    id: 1,
    platformType: 'taobao',
    name: '淘宝店铺',
    status: 1,
    createdAt: '2026-04-01',
    updatedAt: '2026-04-10'
  },
  {
    id: 2,
    platformType: 'pdd',
    name: '拼多多店铺',
    status: 1,
    createdAt: '2026-04-02',
    updatedAt: '2026-04-10'
  },
  {
    id: 3,
    platformType: 'douyin',
    name: '抖音小店',
    status: 0,
    createdAt: '2026-04-03',
    updatedAt: '2026-04-05'
  },
  {
    id: 4,
    platformType: 'xianyu',
    name: '闲鱼店铺',
    status: 1,
    createdAt: '2026-04-04',
    updatedAt: '2026-04-08'
  },
  {
    id: 5,
    platformType: 'amazon',
    name: '亚马逊店铺',
    status: 1,
    createdAt: '2026-04-05',
    updatedAt: '2026-04-09'
  }
])

// 表格列配置
const columns = [
  { title: '平台名称', dataIndex: 'name', key: 'name', width: 180 },
  { title: '平台类型', dataIndex: 'platformType', key: 'platformType', width: 120 },
  { title: '状态', dataIndex: 'status', key: 'status', width: 80 },
  { title: '创建时间', dataIndex: 'createdAt', key: 'createdAt', width: 140 },
  { title: '更新时间', dataIndex: 'updatedAt', key: 'updatedAt', width: 140 },
  { title: '操作', key: 'actions', width: 200, fixed: 'right' }
]

// 搜索
const handleSearch = () => {
  // 实现搜索逻辑
  console.log('搜索:', searchKeyword.value, platformType.value)
}

// 表格变化
const handleTableChange = (pagination: any) => {
  // 实现分页逻辑
  console.log('分页:', pagination)
}

// 新增平台
const createPlatform = () => {
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

// 切换状态
const toggleStatus = (record: any) => {
  // 实现状态切换逻辑
  console.log('切换状态:', record.id, record.status)
}

// 删除平台
const deletePlatform = (id: number) => {
  // 实现删除平台逻辑
  console.log('删除平台:', id)
}

// 生命周期
onMounted(() => {
  // 初始化数据
  console.log('平台列表加载完成')
})
</script>

<style lang="scss" scoped>
.platforms-list {
  padding: 24px;
  background: $color-bg-1;
  min-height: 100vh;

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;

    .page-title {
      font-size: 28px;
      font-weight: 600;
      color: $color-text-1;
      margin: 0;
    }
  }

  .panel {
    background: white;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
    overflow: hidden;

    .panel-header {
      padding: 20px 24px;
      border-bottom: 1px solid $color-border;

      .search-bar {
        display: flex;
        align-items: center;
        gap: 16px;
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

      :deep(.arco-table-td:last-child) {
        text-align: right;
      }
    }
  }
}

@media (max-width: 768px) {
  .platforms-list {
    padding: 16px;

    .page-header {
      flex-direction: column;
      align-items: flex-start;
      gap: 16px;
    }

    .panel {
      .panel-header {
        padding: 16px 20px;

        .search-bar {
          flex-direction: column;
          align-items: stretch;

          :deep(.arco-input-wrapper) {
            width: 100% !important;
          }
        }
      }
    }
  }
}
</style>

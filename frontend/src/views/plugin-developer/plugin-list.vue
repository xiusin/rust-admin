<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="8" :xxl="6">
            <a-form-item field="name" label="插件名称">
              <a-input v-model="searchForm.name" placeholder="请输入插件名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option :value="0">待审核</a-option>
                <a-option :value="1">已上架</a-option>
                <a-option :value="2">已下架</a-option>
                <a-option :value="3">审核拒绝</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" size="small" @click="search">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button size="small" @click="reset">
                <template #icon><icon-refresh /></template>
                <template #default>重置</template>
              </a-button>
            </a-space>
          </a-col>
        </a-row>
      </a-form>

      <a-divider :margin="0" />

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              <template #default>新增插件</template>
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1400 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #coverImage="{ record }">
          <a-image-preview>
            <div class="plugin-image" v-if="record.coverImage">
              <img :src="record.coverImage" :alt="record.name" />
            </div>
            <div class="plugin-image empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </template>
        <template #name="{ record }">
          <div class="plugin-name-cell">
            <div class="plugin-name">{{ record.name }}</div>
            <div class="plugin-version">v{{ record.version }}</div>
          </div>
        </template>
        <template #price="{ record }">
          <div class="price-cell">
            <template v-if="record.priceType === 0">
              <a-tag color="green" size="small">免费</a-tag>
            </template>
            <template v-else-if="record.priceType === 1">
              <span class="price">¥{{ record.price }}</span>
              <span class="price-label">一次性</span>
            </template>
            <template v-else>
              <span class="price">¥{{ record.price }}</span>
              <span class="price-label">/月</span>
            </template>
          </div>
        </template>
        <template #verifyLevel="{ record }">
          <a-tag :color="getVerifyLevelColor(record.verifyLevel)" size="small">
            {{ record.verifyLevelName }}
          </a-tag>
        </template>
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)" size="small">{{ record.statusName }}</a-tag>
        </template>
        <template #tags="{ record }">
          <div class="tags-cell">
            <a-tag v-for="tag in record.tags" :key="tag" size="small">{{ tag }}</a-tag>
          </div>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onVersion(record)">版本</a-button>
            <a-button type="text" size="mini" @click="onSales(record)">销售</a-button>
            <a-button v-if="record.status === 1" type="text" size="mini" @click="onShelves(record, 2)">下架</a-button>
            <a-button v-if="record.status === 2" type="text" size="mini" @click="onShelves(record, 1)">上架</a-button>
            <a-button type="text" size="mini" status="danger" @click="onDelete(record)">删除</a-button>
          </a-space>
        </template>
      </a-table>

      <a-modal v-model:visible="deleteModalVisible" title="删除确认" :width="400" @ok="onConfirmDelete" @cancel="deleteModalVisible = false">
        <div style="text-align: center; padding: 20px 0">
          <a-result status="warning" title="确定要删除此插件吗？" subtitle="删除后不可恢复，相关订单数据将保留" />
        </div>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { developer } from '@/api/modules/plugin-market/market';
import { Message } from '@arco-design/web-vue';

interface PluginItem {
  id: number;
  code: string;
  name: string;
  categoryId: number;
  categoryName: string;
  summary: string;
  coverImage?: string;
  version: string;
  priceType: number;
  priceTypeName: string;
  price: number;
  verifyLevel: number;
  verifyLevelName: string;
  status: number;
  statusName: string;
  tags: string[];
  downloadCount: number;
  rating: number;
  createdAt: string;
}

const router = useRouter();
const loading = ref(false);
const tableData = ref<PluginItem[]>([]);
const searchFormRef = ref();
const deleteModalVisible = ref(false);
const currentRecord = ref<PluginItem | null>(null);

const searchForm = reactive({
  name: '',
  status: null as number | null,
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { title: '插件封面', dataIndex: 'coverImage', slotName: 'coverImage', width: 100 },
  { title: '插件名称', slotName: 'name', width: 200 },
  { title: '价格', slotName: 'price', width: 120 },
  { title: '验证级别', slotName: 'verifyLevel', width: 100 },
  { title: '状态', slotName: 'status', width: 90 },
  { title: '标签', slotName: 'tags', width: 180 },
  { title: '下载量', dataIndex: 'downloadCount', width: 90 },
  { title: '评分', dataIndex: 'rating', width: 80 },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 260, fixed: 'right' },
];

const getVerifyLevelColor = (level: number) => {
  const colors: Record<number, string> = { 0: 'gray', 1: 'blue', 2: 'purple', 3: 'gold' };
  return colors[level] || 'default';
};

const getStatusColor = (status: number) => {
  const colors: Record<number, string> = { 0: 'orange', 1: 'green', 2: 'gray', 3: 'red' };
  return colors[status] || 'default';
};

const getList = async () => {
  loading.value = true;
  try {
    const params = {
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status ?? undefined,
    };
    const res = await developer.list(params);
    tableData.value = res.data?.list || [];
    pagination.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.current = 1;
  getList();
};

const refresh = () => {
  getList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  getList();
};

const onAdd = () => {
  router.push({ name: 'plugin-edit', params: { id: 'new' } });
};

const onEdit = (record: PluginItem) => {
  router.push({ name: 'plugin-edit', params: { id: record.id } });
};

const onVersion = (record: PluginItem) => {
  router.push({ name: 'plugin-version', params: { pluginId: record.id } });
};

const onSales = (record: PluginItem) => {
  router.push({ name: 'plugin-sales', params: { pluginId: record.id } });
};

const onShelves = async (record: PluginItem, status: number) => {
  try {
    await developer.edit({ id: record.id, status });
    Message.success(status === 1 ? '上架成功' : '下架成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDelete = (record: PluginItem) => {
  currentRecord.value = record;
  deleteModalVisible.value = true;
};

const onConfirmDelete = async () => {
  if (!currentRecord.value) return;
  try {
    await developer.delete(currentRecord.value.id);
    Message.success('删除成功');
    deleteModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.search-btn {
  margin-bottom: 20px;
}

.action-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background-color: var(--color-fill-2);
  }
}

.plugin-image {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  &.empty {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f2f3f5;
    color: #86909c;
    font-size: 20px;
  }
}

.plugin-name-cell {
  .plugin-name {
    font-weight: 500;
  }

  .plugin-version {
    font-size: 12px;
    color: $color-text-3;
  }
}

.price-cell {
  display: flex;
  flex-direction: column;

  .price {
    color: #f53f3f;
    font-weight: 600;
  }

  .price-label {
    font-size: 12px;
    color: $color-text-3;
  }
}

.tags-cell {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
</style>

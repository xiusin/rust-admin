<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="productName" label="商品名称">
              <a-input v-model="searchForm.productName" placeholder="请输入商品名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="stockStatus" label="库存状态">
              <a-select v-model="searchForm.stockStatus" placeholder="请选择状态" allow-clear>
                <a-option value="normal">正常</a-option>
                <a-option value="low">库存不足</a-option>
                <a-option value="out">缺货</a-option>
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
            <a-button type="primary" size="small" @click="onBatchAdjust">
              <template #icon><icon-edit /></template>
              批量调整
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
        row-key="skuId"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1200 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #productImage="{ record }">
          <a-image-preview>
            <div class="product-image" v-if="record.productImage">
              <img :src="record.productImage" :alt="record.productName" />
            </div>
            <div class="product-image empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </template>
        <template #stock="{ record }">
          <span :class="['stock-value', getStockClass(record)]">{{ record.stock }}</span>
        </template>
        <template #isAlert="{ record }">
          <a-tag :color="record.isAlert ? 'red' : 'green'" bordered size="small">
            {{ record.isAlert ? '预警' : '正常' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onAdjust(record)">调整</a-button>
            <a-button type="text" size="mini" @click="onViewLog(record)">记录</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="adjustModalVisible" title="库存调整" :width="500" @ok="onSubmitAdjust" @cancel="adjustModalVisible = false">
      <a-form :model="adjustForm" auto-label-width layout="vertical">
        <a-form-item label="商品信息">
          <a-input :value="adjustForm.productName" disabled />
        </a-form-item>
        <a-form-item label="SKU信息" v-if="adjustForm.skuCode">
          <a-input :value="adjustForm.skuCode" disabled />
        </a-form-item>
        <a-form-item label="当前库存">
          <a-input-number v-model="adjustForm.currentStock" disabled :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item field="changeType" label="调整类型">
          <a-radio-group v-model="adjustForm.changeType">
            <a-radio :value="0">增加</a-radio>
            <a-radio :value="1">减少</a-radio>
            <a-radio :value="2">设为</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item field="changeNum" label="调整数量">
          <a-input-number v-model="adjustForm.changeNum" :min="0" placeholder="请输入调整数量" style="width: 100%" />
        </a-form-item>
        <a-form-item field="remark" label="备注">
          <a-textarea v-model="adjustForm.remark" placeholder="请输入备注" :rows="3" />
        </a-form-item>
        <a-form-item label="调整后库存">
          <div class="result-stock">
            <span class="result-value">{{ calculateAfterStock() }}</span>
          </div>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="logModalVisible" title="库存变动记录" :width="900" :footer="null">
      <a-table
        :loading="logLoading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: 400 }"
        :columns="logColumns"
        :data="logTableData"
        :pagination="logPagination"
        @page-change="handleLogPageChange"
        @page-size-change="handleLogPageSizeChange"
      >
        <template #changeType="{ record }">
          <a-tag :color="getChangeTypeColor(record.changeType)" bordered size="small">{{ record.changeTypeName }}</a-tag>
        </template>
        <template #changeNum="{ record }">
          <span :class="['change-num', record.changeNum > 0 ? 'positive' : 'negative']">
            {{ record.changeNum > 0 ? '+' : '' }}{{ record.changeNum }}
          </span>
        </template>
      </a-table>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { stockApi, StockListItem, StockLogItem } from '@/api/modules/product/stock';

const loading = ref(false);
const logLoading = ref(false);
const tableData = ref<StockListItem[]>([]);
const logTableData = ref<StockLogItem[]>([]);
const adjustModalVisible = ref(false);
const logModalVisible = ref(false);
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const currentSkuId = ref<number>(0);

const searchForm = reactive({
  productName: '',
  stockStatus: '',
});

const adjustForm = reactive({
  productId: 0,
  productName: '',
  skuId: undefined as number | undefined,
  skuCode: '',
  currentStock: 0,
  changeType: 0,
  changeNum: 0,
  remark: '',
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const logPagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60, fixed: 'left' },
  { title: '商品图片', dataIndex: 'productImage', slotName: 'productImage', width: 100 },
  { title: '商品名称', dataIndex: 'productName', width: 200 },
  { title: 'SKU编码', dataIndex: 'skuCode', width: 130 },
  { title: '规格', dataIndex: 'specText', width: 150 },
  { title: '库存', dataIndex: 'stock', slotName: 'stock', width: 100, sortable: true },
  { title: '销量', dataIndex: 'sales', width: 80, sortable: true },
  { title: '预警状态', dataIndex: 'isAlert', slotName: 'isAlert', width: 100 },
  { title: '操作', slotName: 'optional', width: 120, fixed: 'right' },
];

const logColumns = [
  { title: '商品', dataIndex: 'productName', width: 150 },
  { title: 'SKU', dataIndex: 'skuCode', width: 120 },
  { title: '变动类型', slotName: 'changeType', width: 100 },
  { title: '变动数量', slotName: 'changeNum', width: 100 },
  { title: '变动前', dataIndex: 'beforeStock', width: 80 },
  { title: '变动后', dataIndex: 'afterStock', width: 80 },
  { title: '订单号', dataIndex: 'orderNo', width: 150, ellipsis: true },
  { title: '操作人', dataIndex: 'operatorName', width: 100 },
  { title: '备注', dataIndex: 'remark', width: 150, ellipsis: true },
  { title: '时间', dataIndex: 'createdAt', width: 180 },
];

const getStockClass = (record: StockListItem) => {
  if (record.stock === 0) return 'out-of-stock';
  if (record.isAlert) return 'low-stock';
  return '';
};

const getChangeTypeColor = (type: number) => {
  const colors: Record<number, string> = { 0: 'green', 1: 'red', 2: 'blue', 3: 'orange', 4: 'red' };
  return colors[type] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await stockApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      productName: searchForm.productName || undefined,
      stockStatus: searchForm.stockStatus || undefined,
    });
    tableData.value = data.list || [];
    pagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const getLogList = async () => {
  logLoading.value = true;
  try {
    const data = await stockApi.logList({
      pageNum: logPagination.current,
      pageSize: logPagination.pageSize,
      skuId: currentSkuId.value || undefined,
    });
    logTableData.value = data.list || [];
    logPagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    logLoading.value = false;
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

const handleLogPageChange = (page: number) => {
  logPagination.current = page;
  getLogList();
};

const handleLogPageSizeChange = (pageSize: number) => {
  logPagination.pageSize = pageSize;
  getLogList();
};

const calculateAfterStock = () => {
  if (adjustForm.changeType === 0) return adjustForm.currentStock + adjustForm.changeNum;
  if (adjustForm.changeType === 1) return adjustForm.currentStock - adjustForm.changeNum;
  return adjustForm.changeNum;
};

const onBatchAdjust = () => {
  arcoMessage('warning', '请选择要调整的商品');
};

const onAdjust = (record: StockListItem) => {
  adjustForm.productId = record.productId;
  adjustForm.productName = record.productName;
  adjustForm.skuId = record.skuId;
  adjustForm.skuCode = record.skuCode || '';
  adjustForm.currentStock = record.stock;
  adjustForm.changeType = 0;
  adjustForm.changeNum = 0;
  adjustForm.remark = '';
  adjustModalVisible.value = true;
};

const onViewLog = (record: StockListItem) => {
  currentSkuId.value = record.skuId;
  logPagination.current = 1;
  getLogList();
  logModalVisible.value = true;
};

const onSubmitAdjust = async () => {
  if (adjustForm.changeNum <= 0) {
    arcoMessage('warning', '请输入调整数量');
    return;
  }
  try {
    await stockApi.adjust({
      productId: adjustForm.productId,
      skuId: adjustForm.skuId,
      changeType: adjustForm.changeType,
      changeNum: adjustForm.changeNum,
      remark: adjustForm.remark,
    });
    arcoMessage('success', '库存调整成功');
    adjustModalVisible.value = false;
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

.product-image {
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

.stock-value {
  font-weight: 600;

  &.out-of-stock {
    color: #f53f3f;
  }

  &.low-stock {
    color: #ff7d00;
  }
}

.change-num {
  font-weight: 600;

  &.positive {
    color: #52c41a;
  }

  &.negative {
    color: #f53f3f;
  }
}

.result-stock {
  padding: 12px 16px;
  background: #f2f3f5;
  border-radius: 4px;
  text-align: center;
}

.result-value {
  font-size: 24px;
  font-weight: 600;
  color: #165dff;
}
</style>

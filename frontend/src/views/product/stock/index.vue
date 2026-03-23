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
            <a-form-item field="categoryId" label="商品分类">
              <a-tree-select
                v-model="searchForm.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择分类"
                allow-clear
              />
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
        <a-col :span="24">
          <a-space size="medium">
            <a-statistic title="总库存" :value="statistics.totalStock" />
            <a-statistic title="库存预警" :value="statistics.alertCount" status="warning" />
            <a-statistic title="缺货商品" :value="statistics.outOfStockCount" status="error" />
            <a-statistic title="库存不足" :value="statistics.lowStockCount" status="warning" />
          </a-space>
        </a-col>
      </a-row>

      <a-tabs v-model:active-key="activeTab">
        <a-tab-pane key="stock" title="库存列表">
          <a-row :gutter="16" style="margin-bottom: 16px">
            <a-col :span="12">
              <a-button type="primary" size="small" @click="onAdjust">
                <template #icon><icon-edit /></template>
                库存调整
              </a-button>
            </a-col>
            <a-col :span="12" style="text-align: right">
              <a-tooltip content="刷新">
                <div class="action-icon" @click="getList"><icon-refresh size="18" /></div>
              </a-tooltip>
            </a-col>
          </a-row>

          <a-table
            ref="tableRef"
            row-key="productId"
            :loading="loading"
            :bordered="{ cell: true }"
            :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
            :columns="columns"
            :data="tableData"
            :pagination="pagination"
            @page-change="handlePageChange"
            @page-size-change="handlePageSizeChange"
          >
            <template #productImage="{ record }">
              <a-avatar v-if="record.productImage" :size="40" :image-url="record.productImage" />
              <a-avatar v-else :size="40"><icon-image /></a-avatar>
            </template>
            <template #stockStatus="{ record }">
              <a-tag :color="getStockStatusColor(record)">{{ getStockStatusText(record) }}</a-tag>
            </template>
            <template #optional="{ record }">
              <a-space>
                <a-button type="text" size="mini" @click="onAdjustOne(record)">调整</a-button>
                <a-button type="text" size="mini" @click="onViewLog(record)">记录</a-button>
                <a-button type="text" size="mini" @click="onConfigAlert(record)">预警</a-button>
              </a-space>
            </template>
          </a-table>
        </a-tab-pane>

        <a-tab-pane key="alert" title="库存预警">
          <a-table
            :loading="alertLoading"
            :bordered="{ cell: true }"
            :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
            :columns="alertColumns"
            :data="alertTableData"
            :pagination="alertPagination"
            @page-change="handleAlertPageChange"
            @page-size-change="handleAlertPageSizeChange"
          >
            <template #productImage="{ record }">
              <a-avatar v-if="record.productImage" :size="40" :image-url="record.productImage" />
              <a-avatar v-else :size="40"><icon-image /></a-avatar>
            </template>
            <template #isAlert="{ record }">
              <a-tag :color="record.isAlert === 1 ? 'red' : 'green'">{{ record.isAlert === 1 ? '已预警' : '正常' }}</a-tag>
            </template>
            <template #optional="{ record }">
              <a-space>
                <a-button type="text" size="mini" @click="onConfigAlert(record)">设置</a-button>
                <a-button type="text" size="mini" @click="onViewLog(record)">记录</a-button>
              </a-space>
            </template>
          </a-table>
        </a-tab-pane>
      </a-tabs>
    </div>

    <a-modal v-model:visible="adjustModalVisible" title="库存调整" :width="600" @ok="onSubmitAdjust" @cancel="adjustModalVisible = false">
      <a-form :model="adjustForm" auto-label-width>
        <a-form-item label="商品">
          <a-input :value="adjustForm.productName" disabled />
        </a-form-item>
        <a-form-item label="SKU" v-if="adjustForm.skuCode">
          <a-input :value="adjustForm.skuCode + ' ' + adjustForm.specText" disabled />
        </a-form-item>
        <a-form-item label="当前库存">
          <a-input-number v-model="adjustForm.currentStock" disabled :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item field="changeType" label="调整类型" :rules="[{ required: true, message: '请选择调整类型' }]">
          <a-select v-model="adjustForm.changeType" placeholder="请选择调整类型">
            <a-option :value="0">增加</a-option>
            <a-option :value="1">减少</a-option>
            <a-option :value="2">设为</a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="changeNum" label="调整数量" :rules="[{ required: true, message: '请输入调整数量' }]">
          <a-input-number v-model="adjustForm.changeNum" :min="0" placeholder="请输入调整数量" style="width: 100%" />
        </a-form-item>
        <a-form-item field="remark" label="备注">
          <a-textarea v-model="adjustForm.remark" placeholder="请输入备注" :rows="3" />
        </a-form-item>
        <a-form-item label="调整后库存">
          <a-input-number :value="calculateAfterStock()" disabled :min="0" style="width: 100%" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="logModalVisible" title="库存记录" :width="900" :footer="false">
      <a-form :model="logSearchForm" auto-label-width style="margin-bottom: 16px">
        <a-row :gutter="16">
          <a-col :span="8">
            <a-form-item field="changeType" label="变动类型">
              <a-select v-model="logSearchForm.changeType" placeholder="请选择类型" allow-clear>
                <a-option :value="0">增加</a-option>
                <a-option :value="1">减少</a-option>
                <a-option :value="2">预扣</a-option>
                <a-option :value="3">释放</a-option>
                <a-option :value="4">扣减</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-button type="primary" size="small" @click="getLogList">查询</a-button>
          </a-col>
        </a-row>
      </a-form>
      <a-table
        :data="logTableData"
        :pagination="logPagination"
        @page-change="handleLogPageChange"
        @page-size-change="handleLogPageSizeChange"
      >
        <template #columns>
          <a-table-column title="商品" data-index="productName" :width="150" />
          <a-table-column title="SKU" data-index="skuCode" :width="120" />
          <a-table-column title="变动类型">
            <template #cell="{ record }">
              <a-tag :color="getChangeTypeColor(record.changeType)">{{ record.changeTypeName }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="变动数量" :width="100">
            <template #cell="{ record }">
              <span :style="{ color: record.changeNum > 0 ? 'green' : 'red' }">
                {{ record.changeNum > 0 ? '+' : '' }}{{ record.changeNum }}
              </span>
            </template>
          </a-table-column>
          <a-table-column title="变动前" data-index="beforeStock" :width="80" />
          <a-table-column title="变动后" data-index="afterStock" :width="80" />
          <a-table-column title="订单号" data-index="orderNo" :width="150" />
          <a-table-column title="操作人" data-index="operatorName" :width="100" />
          <a-table-column title="备注" data-index="remark" :width="150" ellipsis />
          <a-table-column title="时间" data-index="createdAt" :width="180" />
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="alertModalVisible" title="预警设置" :width="500" @ok="onSubmitAlert" @cancel="alertModalVisible = false">
      <a-form :model="alertForm" auto-label-width>
        <a-form-item label="商品">
          <a-input :value="alertForm.productName" disabled />
        </a-form-item>
        <a-form-item label="SKU" v-if="alertForm.skuCode">
          <a-input :value="alertForm.skuCode" disabled />
        </a-form-item>
        <a-form-item label="当前库存">
          <a-input-number v-model="alertForm.currentStock" disabled :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item field="alertStock" label="预警库存" :rules="[{ required: true, message: '请输入预警库存' }]">
          <a-input-number v-model="alertForm.alertStock" :min="0" placeholder="库存低于此值时预警" style="width: 100%" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { stockApi, StockListItem, StockLogItem, StockAlertItem, StockStatistics } from '@/api/modules/product/stock';
import { categoryApi } from '@/api/modules/product/category';

const loading = ref(false);
const alertLoading = ref(false);
const tableData = ref<StockListItem[]>([]);
const alertTableData = ref<StockAlertItem[]>([]);
const logTableData = ref<StockLogItem[]>([]);
const categoryTree = ref<any[]>([]);

const adjustModalVisible = ref(false);
const logModalVisible = ref(false);
const alertModalVisible = ref(false);

const activeTab = ref('stock');

const statistics = reactive<StockStatistics>({
  totalStock: 0,
  alertCount: 0,
  outOfStockCount: 0,
  lowStockCount: 0,
});

const searchForm = reactive({
  productName: '',
  categoryId: undefined as number | undefined,
  stockStatus: '',
});

const adjustForm = reactive({
  productId: 0,
  productName: '',
  skuId: undefined as number | undefined,
  skuCode: '',
  specText: '',
  currentStock: 0,
  changeType: undefined as number | undefined,
  changeNum: 0,
  remark: '',
});

const logSearchForm = reactive({
  changeType: undefined as number | undefined,
});

const alertForm = reactive({
  productId: 0,
  productName: '',
  skuId: undefined as number | undefined,
  skuCode: '',
  currentStock: 0,
  alertStock: 0,
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const alertPagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const logPagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  {
    title: '商品图片',
    dataIndex: 'productImage',
    slotName: 'productImage',
    width: 80,
  },
  {
    title: '商品名称',
    dataIndex: 'productName',
    width: 200,
  },
  {
    title: '分类',
    dataIndex: 'categoryName',
    width: 120,
  },
  {
    title: 'SKU编码',
    dataIndex: 'skuCode',
    width: 120,
  },
  {
    title: '规格',
    dataIndex: 'specText',
    width: 150,
    ellipsis: true,
  },
  {
    title: '库存',
    dataIndex: 'stock',
    width: 80,
  },
  {
    title: '销量',
    dataIndex: 'sales',
    width: 80,
  },
  {
    title: '状态',
    slotName: 'stockStatus',
    width: 100,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 200,
    fixed: 'right',
  },
];

const alertColumns = [
  {
    title: '商品图片',
    dataIndex: 'productImage',
    slotName: 'productImage',
    width: 80,
  },
  {
    title: '商品名称',
    dataIndex: 'productName',
    width: 200,
  },
  {
    title: 'SKU编码',
    dataIndex: 'skuCode',
    width: 120,
  },
  {
    title: '当前库存',
    dataIndex: 'stock',
    width: 100,
  },
  {
    title: '预警值',
    dataIndex: 'alertStock',
    width: 100,
  },
  {
    title: '状态',
    slotName: 'isAlert',
    width: 100,
  },
  {
    title: '最后预警',
    dataIndex: 'lastAlertAt',
    width: 180,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 150,
    fixed: 'right',
  },
];

const getStockStatusColor = (record: StockListItem) => {
  if (record.stock === 0) return 'red';
  if (record.isAlert) return 'orange';
  return 'green';
};

const getStockStatusText = (record: StockListItem) => {
  if (record.stock === 0) return '缺货';
  if (record.isAlert) return '库存不足';
  return '正常';
};

const getChangeTypeColor = (type: number) => {
  const colors: Record<number, string> = {
    0: 'green',
    1: 'red',
    2: 'blue',
    3: 'orange',
    4: 'red',
  };
  return colors[type] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const res = await stockApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      productName: searchForm.productName || undefined,
      categoryId: searchForm.categoryId,
      stockStatus: searchForm.stockStatus || undefined,
    });
    if (res.code === 200) {
      tableData.value = res.data?.list || [];
      pagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const getAlertList = async () => {
  alertLoading.value = true;
  try {
    const res = await stockApi.alertList({
      pageNum: alertPagination.current,
      pageSize: alertPagination.pageSize,
    });
    if (res.code === 200) {
      alertTableData.value = res.data?.list || [];
      alertPagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  } finally {
    alertLoading.value = false;
  }
};

const getLogList = async () => {
  try {
    const res = await stockApi.logList({
      pageNum: logPagination.current,
      pageSize: logPagination.pageSize,
      changeType: logSearchForm.changeType,
    });
    if (res.code === 200) {
      logTableData.value = res.data?.list || [];
      logPagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  }
};

const getStatistics = async () => {
  try {
    const res = await stockApi.statistics();
    if (res.code === 200 && res.data) {
      statistics.totalStock = res.data.totalStock;
      statistics.alertCount = res.data.alertCount;
      statistics.outOfStockCount = res.data.outOfStockCount;
      statistics.lowStockCount = res.data.lowStockCount;
    }
  } catch (error) {
    console.error(error);
  }
};

const getCategoryTree = async () => {
  try {
    const res = await categoryApi.tree();
    if (res.code === 200) {
      categoryTree.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchForm.productName = '';
  searchForm.categoryId = undefined;
  searchForm.stockStatus = '';
  pagination.current = 1;
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

const handleAlertPageChange = (page: number) => {
  alertPagination.current = page;
  getAlertList();
};

const handleAlertPageSizeChange = (pageSize: number) => {
  alertPagination.pageSize = pageSize;
  getAlertList();
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
  if (adjustForm.changeType === 0) {
    return adjustForm.currentStock + adjustForm.changeNum;
  } else if (adjustForm.changeType === 1) {
    return adjustForm.currentStock - adjustForm.changeNum;
  } else {
    return adjustForm.changeNum;
  }
};

const onAdjust = () => {
  Message.info('请选择要调整的商品');
};

const onAdjustOne = (record: StockListItem) => {
  adjustForm.productId = record.productId;
  adjustForm.productName = record.productName;
  adjustForm.skuId = record.skuId;
  adjustForm.skuCode = record.skuCode || '';
  adjustForm.specText = record.specText || '';
  adjustForm.currentStock = record.stock;
  adjustForm.changeType = undefined;
  adjustForm.changeNum = 0;
  adjustForm.remark = '';
  adjustModalVisible.value = true;
};

const onViewLog = (record: StockListItem | StockAlertItem) => {
  logSearchForm.changeType = undefined;
  logPagination.current = 1;
  getLogList();
  logModalVisible.value = true;
};

const onConfigAlert = (record: StockListItem | StockAlertItem) => {
  alertForm.productId = record.productId;
  alertForm.productName = record.productName;
  alertForm.skuId = record.skuId;
  alertForm.skuCode = record.skuCode || '';
  alertForm.currentStock = record.stock;
  alertForm.alertStock = (record as any).alertStock || 0;
  alertModalVisible.value = true;
};

const onSubmitAdjust = async () => {
  if (!adjustForm.changeType) {
    Message.warning('请选择调整类型');
    return;
  }
  if (adjustForm.changeNum <= 0) {
    Message.warning('请输入调整数量');
    return;
  }

  try {
    const res = await stockApi.adjust({
      productId: adjustForm.productId,
      skuId: adjustForm.skuId,
      changeType: adjustForm.changeType,
      changeNum: adjustForm.changeNum,
      remark: adjustForm.remark,
    });
    if (res.code === 200) {
      Message.success('库存调整成功');
      adjustModalVisible.value = false;
      getList();
      getStatistics();
    } else {
      Message.error(res.message || '调整失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onSubmitAlert = async () => {
  try {
    const res = await stockApi.alertConfig({
      productId: alertForm.productId,
      skuId: alertForm.skuId,
      alertStock: alertForm.alertStock,
    });
    if (res.code === 200) {
      Message.success('预警设置成功');
      alertModalVisible.value = false;
      getAlertList();
      getStatistics();
    } else {
      Message.error(res.message || '设置失败');
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  getList();
  getStatistics();
  getCategoryTree();
  getAlertList();
});
</script>

<style scoped lang="less">
.snow-page {
  padding: 16px;
  height: 100%;
  box-sizing: border-box;
}

.snow-inner {
  background: var(--color-bg-2);
  padding: 16px;
  border-radius: 4px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.search-btn {
  display: flex;
  align-items: center;
}

.action-icon {
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  &:hover {
    background: var(--color-fill-2);
  }
}

:deep(.arco-statistic) {
  margin-right: 32px;
}
</style>

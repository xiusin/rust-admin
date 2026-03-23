<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="code" label="核销码">
              <a-input v-model="searchForm.code" placeholder="请输入核销码" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="orderNo" label="订单号">
              <a-input v-model="searchForm.orderNo" placeholder="请输入订单号" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="核销状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option :value="0">待使用</a-option>
                <a-option :value="1">已使用</a-option>
                <a-option :value="2">已过期</a-option>
                <a-option :value="3">已退款</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="storeId" label="核销门店">
              <a-select v-model="searchForm.storeId" placeholder="请选择门店" allow-clear>
                <a-option v-for="item in storeList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
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
            <a-statistic title="核销码总数" :value="statistics.totalCodes" />
            <a-statistic title="待使用" :value="statistics.pendingCount" status="warning" />
            <a-statistic title="已使用" :value="statistics.verifiedCount" status="success" />
            <a-statistic title="已过期" :value="statistics.expiredCount" status="error" />
            <a-statistic title="已退款" :value="statistics.refundedCount" status="normal" />
            <a-statistic title="今日核销" :value="statistics.todayVerifiedCount" status="success" />
          </a-space>
        </a-col>
      </a-row>

      <a-tabs v-model:active-key="activeTab">
        <a-tab-pane key="list" title="核销码列表">
          <a-row :gutter="16" style="margin-bottom: 16px">
            <a-col :span="12">
              <a-button type="primary" size="small" @click="openVerifyModal">
                <template #icon><icon-check-circle /></template>
                核销
              </a-button>
              <a-button type="primary" size="small" @click="openQueryModal">
                <template #icon><icon-search /></template>
                查询
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
            <template #productImage="{ record }">
              <a-avatar v-if="record.productImage" :size="40" :image-url="record.productImage" />
              <a-avatar v-else :size="40"><icon-image /></a-avatar>
            </template>
            <template #status="{ record }">
              <a-tag :color="getStatusColor(record.status)">{{ record.statusName }}</a-tag>
            </template>
            <template #optional="{ record }">
              <a-space>
                <a-button type="text" size="mini" @click="onViewDetail(record)">详情</a-button>
                <a-button type="text" size="mini" @click="onViewLogs(record)">记录</a-button>
                <a-button type="text" size="mini" @click="onVerify(record)" v-if="record.status === 0">核销</a-button>
              </a-space>
            </template>
          </a-table>
        </a-tab-pane>

        <a-tab-pane key="logs" title="核销记录">
          <a-table
            :loading="logLoading"
            :bordered="{ cell: true }"
            :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
            :columns="logColumns"
            :data="logTableData"
            :pagination="logPagination"
            @page-change="handleLogPageChange"
            @page-size-change="handleLogPageSizeChange"
          >
            <template #optional="{ record }">
              <a-button type="text" size="mini" @click="onViewLogs(record)">详情</a-button>
            </template>
          </a-table>
        </a-tab-pane>
      </a-tabs>
    </div>

    <a-modal v-model:visible="verifyModalVisible" title="核销" :width="500" @ok="onSubmitVerify" @cancel="verifyModalVisible = false">
      <a-form :model="verifyForm" auto-label-width>
        <a-form-item field="code" label="核销码" :rules="[{ required: true, message: '请输入核销码' }]">
          <a-input v-model="verifyForm.code" placeholder="请输入或扫描核销码" />
        </a-form-item>
        <a-form-item field="storeId" label="核销门店">
          <a-select v-model="verifyForm.storeId" placeholder="请选择核销门店">
            <a-option v-for="item in storeList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="remark" label="备注">
          <a-textarea v-model="verifyForm.remark" placeholder="请输入备注" :rows="2" />
        </a-form-item>
        <a-alert v-if="verifyResult" :type="verifyResult.success ? 'success' : 'error'" style="margin-top: 16px">
          <template #title>{{ verifyResult.message }}</template>
          <div v-if="verifyResult.success">
            <div>商品: {{ verifyResult.productName }}</div>
            <div>规格: {{ verifyResult.specText || '-' }}</div>
            <div>订单号: {{ verifyResult.orderNo }}</div>
            <div>剩余次数: {{ verifyResult.remainCount }}</div>
          </div>
        </a-alert>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="queryModalVisible" title="查询核销码" :width="500" :footer="false">
      <a-form :model="queryForm" auto-label-width>
        <a-form-item field="code" label="核销码" :rules="[{ required: true, message: '请输入核销码' }]">
          <a-input v-model="queryForm.code" placeholder="请输入核销码" />
        </a-form-item>
        <a-button type="primary" @click="onQuery" :loading="queryLoading">查询</a-button>
      </a-form>
      <a-alert v-if="queryResult" :type="queryResult.isValid ? 'success' : 'warning'" style="margin-top: 16px">
        <template #title>{{ queryResult.message }}</template>
        <div v-if="queryResult.isValid">
          <a-descriptions :data="queryDescData" :column="2" bordered size="small" />
        </div>
      </a-alert>
    </a-modal>

    <a-modal v-model:visible="detailModalVisible" title="核销码详情" :width="700" :footer="false">
      <a-descriptions v-if="currentRecord" :column="2" bordered>
        <a-descriptions-item label="核销码">{{ currentRecord.code }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="getStatusColor(currentRecord.status)">{{ currentRecord.statusName }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="商品名称">{{ currentRecord.productName }}</a-descriptions-item>
        <a-descriptions-item label="商品图片">
          <a-avatar v-if="currentRecord.productImage" :size="40" :image-url="currentRecord.productImage" />
        </a-descriptions-item>
        <a-descriptions-item label="SKU编码">{{ currentRecord.skuCode || '-' }}</a-descriptions-item>
        <a-descriptions-item label="规格">{{ currentRecord.specText || '-' }}</a-descriptions-item>
        <a-descriptions-item label="订单号">{{ currentRecord.orderNo }}</a-descriptions-item>
        <a-descriptions-item label="总次数">{{ currentRecord.totalCount }}</a-descriptions-item>
        <a-descriptions-item label="已用次数">{{ currentRecord.usedCount }}</a-descriptions-item>
        <a-descriptions-item label="剩余次数">{{ currentRecord.remainCount }}</a-descriptions-item>
        <a-descriptions-item label="核销门店">{{ currentRecord.storeName || '-' }}</a-descriptions-item>
        <a-descriptions-item label="核销时间">{{ currentRecord.verifiedAt || '-' }}</a-descriptions-item>
        <a-descriptions-item label="过期时间">{{ currentRecord.expireAt || '-' }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.createdAt || '-' }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>

    <a-modal v-model:visible="logModalVisible" title="核销记录" :width="900" :footer="false">
      <a-table :data="detailLogTableData" :pagination="false">
        <template #columns>
          <a-table-column title="核销码" data-index="code" :width="120" />
          <a-table-column title="商品" data-index="productName" :width="150" />
          <a-table-column title="核销门店" data-index="storeName" :width="120" />
          <a-table-column title="核销人" data-index="verifiedByName" :width="100" />
          <a-table-column title="备注" data-index="remark" :width="150" />
          <a-table-column title="核销时间" data-index="createdAt" :width="180" />
        </template>
      </a-table>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import { Message } from '@arco-design/web-vue';
import { verificationApi, VerificationCodeListItem, VerificationLogItem, VerificationStatistics, VerificationResult, VerificationQueryResult } from '@/api/modules/product/verification';
import { storeApi } from '@/api/modules/product/store';

const loading = ref(false);
const logLoading = ref(false);
const tableData = ref<VerificationCodeListItem[]>([]);
const logTableData = ref<VerificationLogItem[]>([]);
const detailLogTableData = ref<VerificationLogItem[]>([]);
const storeList = ref<any[]>([]);
const currentRecord = ref<VerificationCodeListItem | null>(null);

const verifyModalVisible = ref(false);
const queryModalVisible = ref(false);
const detailModalVisible = ref(false);
const logModalVisible = ref(false);
const queryLoading = ref(false);

const activeTab = ref('list');

const verifyResult = ref<VerificationResult | null>(null);
const queryResult = ref<VerificationQueryResult | null>(null);

const statistics = reactive<VerificationStatistics>({
  totalCodes: 0,
  pendingCount: 0,
  verifiedCount: 0,
  expiredCount: 0,
  refundedCount: 0,
  todayVerifiedCount: 0,
});

const searchForm = reactive({
  code: '',
  orderNo: '',
  status: undefined as number | undefined,
  storeId: undefined as number | undefined,
});

const verifyForm = reactive({
  code: '',
  storeId: undefined as number | undefined,
  remark: '',
});

const queryForm = reactive({
  code: '',
});

const pagination = reactive({
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
    title: '核销码',
    dataIndex: 'code',
    width: 150,
  },
  {
    title: '商品图片',
    dataIndex: 'productImage',
    slotName: 'productImage',
    width: 80,
  },
  {
    title: '商品名称',
    dataIndex: 'productName',
    width: 150,
  },
  {
    title: '规格',
    dataIndex: 'specText',
    width: 120,
  },
  {
    title: '订单号',
    dataIndex: 'orderNo',
    width: 150,
  },
  {
    title: '总次数',
    dataIndex: 'totalCount',
    width: 80,
  },
  {
    title: '已用',
    dataIndex: 'usedCount',
    width: 80,
  },
  {
    title: '剩余',
    dataIndex: 'remainCount',
    width: 80,
  },
  {
    title: '状态',
    dataIndex: 'status',
    slotName: 'status',
    width: 100,
  },
  {
    title: '核销门店',
    dataIndex: 'storeName',
    width: 120,
  },
  {
    title: '过期时间',
    dataIndex: 'expireAt',
    width: 180,
  },
  {
    title: '创建时间',
    dataIndex: 'createdAt',
    width: 180,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 150,
    fixed: 'right',
  },
];

const logColumns = [
  {
    title: '核销码',
    dataIndex: 'code',
    width: 150,
  },
  {
    title: '商品',
    dataIndex: 'productName',
    width: 150,
  },
  {
    title: '订单号',
    dataIndex: 'orderNo',
    width: 150,
  },
  {
    title: '核销门店',
    dataIndex: 'storeName',
    width: 120,
  },
  {
    title: '核销人',
    dataIndex: 'verifiedByName',
    width: 100,
  },
  {
    title: '备注',
    dataIndex: 'remark',
    width: 150,
  },
  {
    title: '核销时间',
    dataIndex: 'createdAt',
    width: 180,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 100,
    fixed: 'right',
  },
];

const queryDescData = computed(() => {
  if (!queryResult.value) return [];
  return [
    { label: '核销码', value: queryResult.value.code },
    { label: '商品', value: queryResult.value.productName },
    { label: '规格', value: queryResult.value.specText || '-' },
    { label: '订单号', value: queryResult.value.orderNo },
    { label: '总次数', value: queryResult.value.totalCount },
    { label: '已用次数', value: queryResult.value.usedCount },
    { label: '剩余次数', value: queryResult.value.remainCount },
    { label: '状态', value: queryResult.value.statusName },
    { label: '过期时间', value: queryResult.value.expireAt || '-' },
  ];
});

const getStatusColor = (status: number) => {
  const colors: Record<number, string> = {
    0: 'blue',
    1: 'green',
    2: 'red',
    3: 'gray',
  };
  return colors[status] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const res = await verificationApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      code: searchForm.code || undefined,
      orderNo: searchForm.orderNo || undefined,
      status: searchForm.status,
      storeId: searchForm.storeId,
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

const getLogList = async () => {
  logLoading.value = true;
  try {
    const res = await verificationApi.logList({
      pageNum: logPagination.current,
      pageSize: logPagination.pageSize,
    });
    if (res.code === 200) {
      logTableData.value = res.data?.list || [];
      logPagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  } finally {
    logLoading.value = false;
  }
};

const getStatistics = async () => {
  try {
    const res = await verificationApi.statistics();
    if (res.code === 200 && res.data) {
      statistics.totalCodes = res.data.totalCodes;
      statistics.pendingCount = res.data.pendingCount;
      statistics.verifiedCount = res.data.verifiedCount;
      statistics.expiredCount = res.data.expiredCount;
      statistics.refundedCount = res.data.refundedCount;
      statistics.todayVerifiedCount = res.data.todayVerifiedCount;
    }
  } catch (error) {
    console.error(error);
  }
};

const getStoreList = async () => {
  try {
    const res = await storeApi.list({ pageSize: 100 });
    if (res.code === 200) {
      storeList.value = res.data?.list || [];
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
  searchForm.code = '';
  searchForm.orderNo = '';
  searchForm.status = undefined;
  searchForm.storeId = undefined;
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

const handleLogPageChange = (page: number) => {
  logPagination.current = page;
  getLogList();
};

const handleLogPageSizeChange = (pageSize: number) => {
  logPagination.pageSize = pageSize;
  getLogList();
};

const openVerifyModal = () => {
  verifyForm.code = '';
  verifyForm.storeId = undefined;
  verifyForm.remark = '';
  verifyResult.value = null;
  verifyModalVisible.value = true;
};

const openQueryModal = () => {
  queryForm.code = '';
  queryResult.value = null;
  queryModalVisible.value = true;
};

const onVerify = (record: VerificationCodeListItem) => {
  verifyForm.code = record.code;
  verifyForm.storeId = undefined;
  verifyForm.remark = '';
  verifyResult.value = null;
  verifyModalVisible.value = true;
};

const onSubmitVerify = async () => {
  if (!verifyForm.code) {
    Message.warning('请输入核销码');
    return;
  }

  try {
    const res = await verificationApi.verify({
      code: verifyForm.code,
      storeId: verifyForm.storeId,
      remark: verifyForm.remark,
    });
    if (res.code === 200 && res.data) {
      verifyResult.value = res.data;
      if (res.data.success) {
        Message.success('核销成功');
        getList();
        getStatistics();
      } else {
        Message.error(res.data.message);
      }
    }
  } catch (error) {
    console.error(error);
  }
};

const onQuery = async () => {
  if (!queryForm.code) {
    Message.warning('请输入核销码');
    return;
  }

  queryLoading.value = true;
  try {
    const res = await verificationApi.query(queryForm.code);
    if (res.code === 200 && res.data) {
      queryResult.value = res.data;
    }
  } catch (error) {
    console.error(error);
  } finally {
    queryLoading.value = false;
  }
};

const onViewDetail = (record: VerificationCodeListItem) => {
  currentRecord.value = record;
  detailModalVisible.value = true;
};

const onViewLogs = async (record: VerificationCodeListItem) => {
  try {
    const res = await verificationApi.logList({
      pageSize: 100,
      code: record.code,
    });
    if (res.code === 200) {
      detailLogTableData.value = res.data?.list || [];
      logModalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  getList();
  getLogList();
  getStatistics();
  getStoreList();
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
  margin-right: 24px;
}
</style>

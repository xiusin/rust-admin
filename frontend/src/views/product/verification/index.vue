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
            <a-button type="primary" size="small" @click="openVerifyModal">
              <template #icon><icon-check-circle /></template>
              核销
            </a-button>
            <a-button size="small" @click="openQueryModal">
              <template #icon><icon-search /></template>
              查询
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
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)" bordered size="small">{{ record.statusName }}</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onViewDetail(record)">详情</a-button>
            <a-button v-if="record.status === 0" type="text" size="mini" @click="onVerify(record)">核销</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="verifyModalVisible" title="核销" :width="500" @ok="onSubmitVerify" @cancel="verifyModalVisible = false">
      <a-form :model="verifyForm" auto-label-width layout="vertical">
        <a-form-item field="code" label="核销码" :rules="[{ required: true, message: '请输入核销码' }]">
          <a-input v-model="verifyForm.code" placeholder="请输入或扫描核销码" allow-clear />
        </a-form-item>
        <a-form-item field="storeId" label="核销门店">
          <a-select v-model="verifyForm.storeId" placeholder="请选择核销门店" style="width: 100%">
            <a-option v-for="item in storeList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="remark" label="备注">
          <a-textarea v-model="verifyForm.remark" placeholder="请输入备注" :rows="2" />
        </a-form-item>
        <a-alert v-if="verifyResult" :type="verifyResult.success ? 'success' : 'error'" style="margin-top: 16px">
          <template #title>{{ verifyResult.message }}</template>
        </a-alert>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="queryModalVisible" title="查询核销码" :width="500" :footer="null">
      <a-form :model="queryForm" auto-label-width layout="vertical">
        <a-form-item field="code" label="核销码" :rules="[{ required: true, message: '请输入核销码' }]">
          <a-input v-model="queryForm.code" placeholder="请输入核销码" allow-clear />
        </a-form-item>
        <a-button type="primary" @click="onQuery" :loading="queryLoading" style="width: 100%">查询</a-button>
      </a-form>
      <a-alert v-if="queryResult" :type="queryResult.isValid ? 'success' : 'warning'" style="margin-top: 16px">
        <template #title>{{ queryResult.message }}</template>
      </a-alert>
    </a-modal>

    <a-modal v-model:visible="detailModalVisible" title="核销码详情" :width="700" :footer="null">
      <a-descriptions v-if="currentRecord" :column="2" bordered size="large">
        <a-descriptions-item label="核销码">
          <span class="code-text">{{ currentRecord.code }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="getStatusColor(currentRecord.status)" bordered size="small">{{ currentRecord.statusName }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="商品名称" :span="2">{{ currentRecord.productName }}</a-descriptions-item>
        <a-descriptions-item label="商品图片">
          <a-image-preview>
            <div class="product-image-large" v-if="currentRecord.productImage">
              <img :src="currentRecord.productImage" :alt="currentRecord.productName" />
            </div>
            <div class="product-image-large empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </a-descriptions-item>
        <a-descriptions-item label="SKU编码">{{ currentRecord.skuCode || '-' }}</a-descriptions-item>
        <a-descriptions-item label="订单号">{{ currentRecord.orderNo }}</a-descriptions-item>
        <a-descriptions-item label="核销门店">{{ currentRecord.storeName || '-' }}</a-descriptions-item>
        <a-descriptions-item label="总/已用/剩余">{{ currentRecord.totalCount }} / {{ currentRecord.usedCount }} / {{ currentRecord.remainCount }}</a-descriptions-item>
        <a-descriptions-item label="过期时间">{{ currentRecord.expireAt || '永久有效' }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.createdAt || '-' }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { verificationApi, VerificationCodeListItem, VerificationResult, VerificationQueryResult } from '@/api/modules/product/verification';
import { storeApi } from '@/api/modules/product/store';

const loading = ref(false);
const tableData = ref<VerificationCodeListItem[]>([]);
const storeList = ref<any[]>([]);
const currentRecord = ref<VerificationCodeListItem | null>(null);

const verifyModalVisible = ref(false);
const queryModalVisible = ref(false);
const detailModalVisible = ref(false);
const queryLoading = ref(false);

const verifyResult = ref<VerificationResult | null>(null);
const queryResult = ref<VerificationQueryResult | null>(null);

const searchForm = reactive({
  code: '',
  status: undefined as number | undefined,
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
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { title: '核销码', dataIndex: 'code', width: 150, fixed: 'left' },
  { title: '商品图片', dataIndex: 'productImage', slotName: 'productImage', width: 100 },
  { title: '商品名称', dataIndex: 'productName', width: 180, ellipsis: true },
  { title: '订单号', dataIndex: 'orderNo', width: 150 },
  { title: '状态', slotName: 'status', width: 100 },
  { title: '剩余次数', dataIndex: 'remainCount', width: 100 },
  { title: '核销门店', dataIndex: 'storeName', width: 120 },
  { title: '过期时间', dataIndex: 'expireAt', width: 180 },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 120, fixed: 'right' },
];

const getStatusColor = (status: number) => {
  const colors: Record<number, string> = { 0: 'blue', 1: 'green', 2: 'red', 3: 'gray' };
  return colors[status] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await verificationApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      code: searchForm.code || undefined,
      status: searchForm.status,
    });
    tableData.value = data.list || [];
    pagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const getStoreList = async () => {
  try {
    const data = await storeApi.list({ pageSize: 100 });
    storeList.value = data.list || [];
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
  searchForm.status = undefined;
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
    arcoMessage('warning', '请输入核销码');
    return;
  }
  try {
    const result = await verificationApi.verify({
      code: verifyForm.code,
      storeId: verifyForm.storeId,
      remark: verifyForm.remark,
    });
    verifyResult.value = result;
    if (result.success) {
      arcoMessage('success', '核销成功');
      getList();
    } else {
      arcoMessage('error', result.message);
    }
  } catch (error) {
    console.error(error);
  }
};

const onQuery = async () => {
  if (!queryForm.code) {
    arcoMessage('warning', '请输入核销码');
    return;
  }
  queryLoading.value = true;
  try {
    queryResult.value = await verificationApi.query(queryForm.code);
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

onMounted(() => {
  getList();
  getStoreList();
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

.product-image-large {
  width: 80px;
  height: 80px;
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
    font-size: 32px;
  }
}

.code-text {
  font-family: monospace;
  font-weight: 600;
  color: #165dff;
}
</style>

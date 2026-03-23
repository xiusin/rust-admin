<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="refund_no" label="退款单号">
              <a-input v-model="formData.form.refund_no" placeholder="请输入退款单号" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="退款状态">
              <a-select v-model="formData.form.status" placeholder="请选择状态" allow-clear>
                <a-option value="pending">待处理</a-option>
                <a-option value="processing">处理中</a-option>
                <a-option value="success">退款成功</a-option>
                <a-option value="failed">退款失败</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" @click="loadData">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button @click="onReset">
                <template #icon><icon-refresh /></template>
                <template #default>重置</template>
              </a-button>
            </a-space>
          </a-col>
        </a-row>
      </a-form>
      <a-divider :margin="0" />
      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="24" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="loadData"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>
      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #amount="{ record }">
          <span class="amount">¥{{ parseFloat(record.refund_amount || 0).toFixed(2) }}</span>
        </template>
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)">
            {{ getStatusName(record.status) }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="detailVisible" title="退款详情" :footer="false" :width="600">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="退款单号">{{ currentRecord.refund_no }}</a-descriptions-item>
        <a-descriptions-item label="第三方退款号">{{ currentRecord.transaction_id || '-' }}</a-descriptions-item>
        <a-descriptions-item label="售后单ID">{{ currentRecord.after_sale_id }}</a-descriptions-item>
        <a-descriptions-item label="退款渠道">{{ currentRecord.refund_channel || '-' }}</a-descriptions-item>
        <a-descriptions-item label="退款金额">
          <span class="amount">¥{{ parseFloat(currentRecord.refund_amount || 0).toFixed(2) }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="退款状态">
          <a-tag :color="getStatusColor(currentRecord.status)">
            {{ getStatusName(currentRecord.status) }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="退款时间">{{ currentRecord.refund_at || '-' }}</a-descriptions-item>
        <a-descriptions-item label="失败原因">{{ currentRecord.fail_reason || '-' }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

interface RefundRecord {
  id: number;
  refund_no: string;
  transaction_id: string;
  after_sale_id: number;
  refund_channel: string;
  refund_amount: string;
  status: string;
  refund_at: string;
  fail_reason: string;
  retry_count: number;
}

const formData = reactive({
  form: {
    refund_no: '',
    status: null as string | null,
  },
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<RefundRecord[]>([]);

const pagination = ref({ showPageSize: true, showTotal: true, current: 1, pageSize: 10, total: 0 });
const pageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  loadData();
};

const columns = [
  { title: '退款单号', dataIndex: 'refund_no', width: 180 },
  { title: '售后单ID', dataIndex: 'after_sale_id', width: 100 },
  { title: '退款金额', slotName: 'amount', width: 120 },
  { title: '退款状态', slotName: 'status', width: 100 },
  { title: '退款渠道', dataIndex: 'refund_channel', width: 100 },
  { title: '重试次数', dataIndex: 'retry_count', width: 80 },
  { title: '操作', slotName: 'optional', width: 100 },
];

const getStatusName = (status: string) => {
  const names: Record<string, string> = {
    pending: '待处理',
    processing: '处理中',
    success: '退款成功',
    failed: '退款失败',
  };
  return names[status] || status;
};

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    pending: 'orange',
    processing: 'blue',
    success: 'green',
    failed: 'red',
  };
  return colors[status] || 'arcoblue';
};

const detailVisible = ref(false);
const currentRecord = ref<RefundRecord>({} as RefundRecord);

const loadData = async () => {
  loading.value = true;
  try {
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
    };
    if (formData.form.status) {
      params.status = formData.form.status;
    }
    if (formData.form.refund_no) {
      params.refund_no = formData.form.refund_no;
    }
    const { data } = await axios.get('/after-sale/refund/list', { params });
    if (data.message === 'success') {
      tableData.value = data.data?.list || [];
      pagination.value.total = data.data?.total || 0;
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleDetail = (record: RefundRecord) => {
  currentRecord.value = record;
  detailVisible.value = true;
};

onMounted(() => {
  loadData();
});
</script>

<style lang="scss" scoped>
.amount {
  color: #f53f3f;
  font-weight: 600;
}
.action-icon {
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  &:hover {
    background-color: var(--color-fill-2);
  }
}
</style>

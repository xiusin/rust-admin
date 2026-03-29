<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="order_no" label="订单号">
              <a-input v-model="formData.form.order_no" placeholder="请输入订单号" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="支付状态">
              <a-select v-model="formData.form.status" placeholder="请选择状态" allow-clear>
                <a-option value="pending">待支付</a-option>
                <a-option value="success">支付成功</a-option>
                <a-option value="failed">支付失败</a-option>
                <a-option value="closed">已关闭</a-option>
                <a-option value="refunded">已退款</a-option>
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
        <template #paymentMethod="{ record }">
          <a-tag :color="getPaymentMethodColor(record.payment_method)">
            {{ getPaymentMethodName(record.payment_method) }}
          </a-tag>
        </template>
        <template #amount="{ record }">
          <span class="amount">¥{{ parseFloat(record.amount).toFixed(2) }}</span>
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

    <a-modal v-model:visible="detailVisible" title="支付详情" :footer="false" :width="600">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="订单号">{{ currentRecord.order_no }}</a-descriptions-item>
        <a-descriptions-item label="第三方交易号">{{ currentRecord.transaction_id || "-" }}</a-descriptions-item>
        <a-descriptions-item label="用户ID">{{ currentRecord.consumer_id }}</a-descriptions-item>
        <a-descriptions-item label="支付方式">
          <a-tag :color="getPaymentMethodColor(currentRecord.payment_method)">
            {{ getPaymentMethodName(currentRecord.payment_method) }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="支付金额">
          <span class="amount">¥{{ parseFloat(currentRecord.amount || 0).toFixed(2) }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="支付状态">
          <a-tag :color="getStatusColor(currentRecord.status)">
            {{ getStatusName(currentRecord.status) }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="支付时间">{{ currentRecord.paid_at || "-" }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.created_at || "-" }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import axios from "@/api";

interface PaymentRecord {
  id: number;
  order_no: string;
  transaction_id: string;
  consumer_id: number;
  payment_method: string;
  payment_type: string;
  amount: number;
  status: string;
  paid_at: string;
  created_at: string;
}

const formData = reactive({
  form: {
    order_no: "",
    status: null as string | null
  }
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<PaymentRecord[]>([]);

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
  { title: "订单号", dataIndex: "order_no", width: 200 },
  { title: "用户ID", dataIndex: "consumer_id", width: 100 },
  { title: "支付方式", slotName: "paymentMethod", width: 120 },
  { title: "支付金额", slotName: "amount", width: 120 },
  { title: "支付状态", slotName: "status", width: 100 },
  { title: "支付时间", dataIndex: "paid_at", width: 160 },
  { title: "创建时间", dataIndex: "created_at", width: 160 },
  { title: "操作", slotName: "optional", width: 100 }
];

const getPaymentMethodName = (method: string) => {
  if (!method) return "-";
  const m = method.toLowerCase();
  const names: Record<string, string> = {
    wechat: "微信支付",
    alipay: "支付宝",
    unionpay: "银联",
    balance: "余额",
    yeepay: "易宝支付"
  };
  return names[m] || method;
};

const getPaymentMethodColor = (method: string) => {
  if (!method) return "gray";
  const m = method.toLowerCase();
  const colors: Record<string, string> = {
    wechat: "green",
    alipay: "blue",
    unionpay: "orange",
    balance: "gray",
    yeepay: "purple"
  };
  return colors[m] || "arcoblue";
};

const getStatusName = (status: string) => {
  const names: Record<string, string> = {
    pending: "待支付",
    success: "支付成功",
    failed: "支付失败",
    closed: "已关闭",
    refunded: "已退款"
  };
  return names[status] || status;
};

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    pending: "orange",
    success: "green",
    failed: "red",
    closed: "gray",
    refunded: "purple"
  };
  return colors[status] || "arcoblue";
};

const detailVisible = ref(false);
const currentRecord = ref<PaymentRecord>({} as PaymentRecord);

const loadData = async () => {
  loading.value = true;
  try {
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (formData.form.status) {
      params.status = formData.form.status;
    }
    const { data } = await axios.get("/payment/list", { params });
    if (data.message === "success") {
      tableData.value = data.data?.list || [];
      pagination.value.total = data.data?.total || 0;
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleDetail = (record: PaymentRecord) => {
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

<template>
  <div class="after-sale-container">
    <a-card>
      <template #title>
        <a-space>
          <span>售后管理</span>
          <a-tag color="blue">{{ statistics.pending_count }} 待处理</a-tag>
          <a-tag color="orange">{{ statistics.processing_count }} 处理中</a-tag>
        </a-space>
      </template>
      <template #extra>
        <a-space>
          <a-select v-model="searchForm.status" placeholder="状态筛选" allow-clear style="width: 120px">
            <a-option value="pending">待审核</a-option>
            <a-option value="approved">已通过</a-option>
            <a-option value="rejected">已拒绝</a-option>
            <a-option value="returning">退货中</a-option>
            <a-option value="refunding">退款中</a-option>
            <a-option value="completed">已完成</a-option>
            <a-option value="closed">已关闭</a-option>
          </a-select>
          <a-select v-model="searchForm.type" placeholder="类型筛选" allow-clear style="width: 120px">
            <a-option value="refund_only">仅退款</a-option>
            <a-option value="return_refund">退货退款</a-option>
            <a-option value="exchange">换货</a-option>
          </a-select>
          <a-range-picker v-model="dateRange" style="width: 260px" />
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            搜索
          </a-button>
          <a-button @click="handleReset">重置</a-button>
        </a-space>
      </template>

      <a-table :data="afterSaleList" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="售后单号" data-index="after_sale_no" :width="180" />
          <a-table-column title="订单号" data-index="order_no" :width="180" />
          <a-table-column title="用户ID" data-index="consumer_id" :width="100" />
          <a-table-column title="类型" data-index="type" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getTypeColor(record.type)">{{ getTypeName(record.type) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="原因" data-index="reason" :width="200" ellipsis />
          <a-table-column title="退款金额" data-index="total_refund_amount" :width="120">
            <template #cell="{ record }">
              <span class="text-danger">¥{{ record.total_refund_amount }}</span>
            </template>
          </a-table-column>
          <a-table-column title="状态" data-index="status" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getStatusColor(record.status)">{{ getStatusName(record.status) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="创建时间" data-index="created_at" :width="180" />
          <a-table-column title="操作" :width="200" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
                <a-button v-if="record.status === 'pending'" type="text" size="small" status="success" @click="handleAudit(record)">审核</a-button>
                <a-button v-if="record.status === 'approved' && record.type !== 'refund_only'" type="text" size="small" @click="handleLogistics(record)">物流</a-button>
                <a-button v-if="record.status === 'refunding'" type="text" size="small" status="warning" @click="handleRefund(record)">退款</a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="售后详情" :width="800" :footer="false">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="售后单号">{{ currentAfterSale.after_sale_no }}</a-descriptions-item>
        <a-descriptions-item label="订单号">{{ currentAfterSale.order_no }}</a-descriptions-item>
        <a-descriptions-item label="用户ID">{{ currentAfterSale.consumer_id }}</a-descriptions-item>
        <a-descriptions-item label="类型">{{ getTypeName(currentAfterSale.type) }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="getStatusColor(currentAfterSale.status)">{{ getStatusName(currentAfterSale.status) }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="退款金额">¥{{ currentAfterSale.total_refund_amount }}</a-descriptions-item>
        <a-descriptions-item label="原因" :span="2">{{ currentAfterSale.reason }}</a-descriptions-item>
        <a-descriptions-item label="描述" :span="2">{{ currentAfterSale.description || '-' }}</a-descriptions-item>
      </a-descriptions>

      <a-divider>售后商品</a-divider>
      <a-table :data="currentAfterSale.items || []" :pagination="false" size="small">
        <template #columns>
          <a-table-column title="商品名称" data-index="product_name" />
          <a-table-column title="SKU" data-index="sku_name" />
          <a-table-column title="数量" data-index="quantity" />
          <a-table-column title="单价" data-index="unit_price" />
          <a-table-column title="退款金额" data-index="refund_amount" />
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="auditVisible" title="审核售后" @ok="submitAudit" @cancel="auditVisible = false">
      <a-form :model="auditForm" layout="vertical">
        <a-form-item label="审核结果">
          <a-radio-group v-model="auditForm.agree">
            <a-radio :value="true">通过</a-radio>
            <a-radio :value="false">拒绝</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item v-if="!auditForm.agree" label="拒绝原因">
          <a-textarea v-model="auditForm.reject_reason" placeholder="请输入拒绝原因" :max-length="500" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="refundVisible" title="退款处理" @ok="submitRefund" @cancel="refundVisible = false">
      <a-form :model="refundForm" layout="vertical">
        <a-form-item label="退款渠道">
          <a-select v-model="refundForm.refund_channel">
            <a-option value="original">原路退回</a-option>
            <a-option value="balance">退至余额</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="退款金额">
          <a-input-number v-model="refundForm.refund_amount" :precision="2" :min="0" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="logisticsVisible" title="物流信息" :width="600" :footer="false">
      <a-descriptions v-if="currentLogistics" :column="2" bordered>
        <a-descriptions-item label="物流公司">{{ currentLogistics.logistics_company }}</a-descriptions-item>
        <a-descriptions-item label="物流单号">{{ currentLogistics.tracking_no }}</a-descriptions-item>
        <a-descriptions-item label="发货人">{{ currentLogistics.sender_name }}</a-descriptions-item>
        <a-descriptions-item label="发货电话">{{ currentLogistics.sender_phone }}</a-descriptions-item>
        <a-descriptions-item label="发货地址" :span="2">{{ currentLogistics.sender_address }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="currentLogistics.status === 'received' ? 'green' : 'blue'">
            {{ currentLogistics.status === 'received' ? '已签收' : '运输中' }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="发货时间">{{ currentLogistics.shipped_at }}</a-descriptions-item>
      </a-descriptions>
      <a-empty v-else description="暂无物流信息" />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, reactive } from 'vue';
import { Message } from '@arco-design/web-vue';
import { afterSaleApi, AfterSaleModel, AfterSaleStatistics, AfterSaleLogisticsModel } from '@/api/modules/consumer/afterSale';

const loading = ref(false);
const afterSaleList = ref<AfterSaleModel[]>([]);
const statistics = ref<AfterSaleStatistics>({
  pending_count: 0,
  processing_count: 0,
  completed_count: 0,
  rejected_count: 0,
  total_refund_amount: '0',
});

const pagination = ref({
  current: 1,
  pageSize: 10,
  total: 0,
});

const searchForm = reactive({
  status: '',
  type: '',
  start_time: 0,
  end_time: 0,
});
const dateRange = ref<string[]>([]);

const detailVisible = ref(false);
const auditVisible = ref(false);
const refundVisible = ref(false);
const logisticsVisible = ref(false);

const currentAfterSale = ref<AfterSaleModel>({} as AfterSaleModel);
const currentLogistics = ref<AfterSaleLogisticsModel | null>(null);

const auditForm = reactive({
  after_sale_id: 0,
  agree: true,
  reject_reason: '',
  processor_id: 1,
  processor_name: 'admin',
});

const refundForm = reactive({
  after_sale_id: 0,
  refund_channel: 'original',
  refund_amount: 0,
});

const getTypeName = (type: string) => {
  const map: Record<string, string> = {
    refund_only: '仅退款',
    return_refund: '退货退款',
    exchange: '换货',
  };
  return map[type] || type;
};

const getTypeColor = (type: string) => {
  const map: Record<string, string> = {
    refund_only: 'red',
    return_refund: 'orange',
    exchange: 'blue',
  };
  return map[type] || 'gray';
};

const getStatusName = (status: string) => {
  const map: Record<string, string> = {
    pending: '待审核',
    approved: '已通过',
    rejected: '已拒绝',
    returning: '退货中',
    returned: '已退货',
    refunding: '退款中',
    refunded: '已退款',
    completed: '已完成',
    closed: '已关闭',
  };
  return map[status] || status;
};

const getStatusColor = (status: string) => {
  const map: Record<string, string> = {
    pending: 'orange',
    approved: 'green',
    rejected: 'red',
    returning: 'blue',
    returned: 'cyan',
    refunding: 'purple',
    refunded: 'arcoblue',
    completed: 'green',
    closed: 'gray',
  };
  return map[status] || 'gray';
};

const loadData = async () => {
  loading.value = true;
  try {
    const [listRes, statsRes] = await Promise.all([
      afterSaleApi.list({
        page_num: pagination.value.current,
        page_size: pagination.value.pageSize,
        status: searchForm.status || undefined,
        type: searchForm.type || undefined,
        start_time: searchForm.start_time || undefined,
        end_time: searchForm.end_time || undefined,
      }),
      afterSaleApi.getStatistics(),
    ]);
    afterSaleList.value = listRes.data?.list || [];
    pagination.value.total = listRes.data?.total || 0;
    statistics.value = statsRes.data || statistics.value;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handleSearch = () => {
  if (dateRange.value && dateRange.value.length === 2) {
    searchForm.start_time = new Date(dateRange.value[0]).getTime() / 1000;
    searchForm.end_time = new Date(dateRange.value[1]).getTime() / 1000;
  }
  pagination.value.current = 1;
  loadData();
};

const handleReset = () => {
  searchForm.status = '';
  searchForm.type = '';
  searchForm.start_time = 0;
  searchForm.end_time = 0;
  dateRange.value = [];
  pagination.value.current = 1;
  loadData();
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleDetail = async (record: AfterSaleModel) => {
  try {
    const res = await afterSaleApi.getDetail(record.id);
    currentAfterSale.value = res.data || record;
    detailVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const handleAudit = (record: AfterSaleModel) => {
  auditForm.after_sale_id = record.id;
  auditForm.agree = true;
  auditForm.reject_reason = '';
  auditVisible.value = true;
};

const submitAudit = async () => {
  try {
    await afterSaleApi.audit(auditForm);
    Message.success('审核成功');
    auditVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleLogistics = async (record: AfterSaleModel) => {
  try {
    const res = await afterSaleApi.getLogistics(record.id);
    currentLogistics.value = res.data;
    logisticsVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const handleRefund = (record: AfterSaleModel) => {
  refundForm.after_sale_id = record.id;
  refundForm.refund_amount = parseFloat(record.total_refund_amount);
  refundForm.refund_channel = 'original';
  refundVisible.value = true;
};

const submitRefund = async () => {
  try {
    await afterSaleApi.createRefund({
      after_sale_id: refundForm.after_sale_id,
      refund_channel: refundForm.refund_channel,
      refund_amount: refundForm.refund_amount.toString(),
    });
    Message.success('退款处理成功');
    refundVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.after-sale-container {
  padding: 20px;
}
.text-danger {
  color: #f53f3f;
}
</style>

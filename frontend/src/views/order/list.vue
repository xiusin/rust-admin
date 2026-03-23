<template>
  <div class="page-container">
    <a-card>
      <template #title>订单列表</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.keyword" placeholder="订单号/用户ID" allow-clear style="width: 180px" />
          <a-select v-model="searchForm.status" placeholder="订单状态" allow-clear style="width: 120px">
            <a-option value="pending">待支付</a-option>
            <a-option value="paid">已支付</a-option>
            <a-option value="shipped">已发货</a-option>
            <a-option value="completed">已完成</a-option>
            <a-option value="cancelled">已取消</a-option>
          </a-select>
          <a-range-picker v-model="searchForm.dateRange" style="width: 240px" />
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            搜索
          </a-button>
          <a-button @click="handleExport">
            <template #icon><icon-download /></template>
            导出
          </a-button>
        </a-space>
      </template>
      <a-table :data="tableData" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="订单号" data-index="order_no" :width="180" />
          <a-table-column title="用户ID" data-index="consumer_id" :width="100" />
          <a-table-column title="商品信息" :width="250">
            <template #cell="{ record }">
              <div v-for="item in record.items.slice(0, 2)" :key="item.id" class="goods-item">
                {{ item.name }} x {{ item.quantity }}
              </div>
              <div v-if="record.items.length > 2" class="text-muted">+{{ record.items.length - 2 }}件商品</div>
            </template>
          </a-table-column>
          <a-table-column title="订单金额" :width="120">
            <template #cell="{ record }">
              <span class="text-danger">¥{{ record.total_amount }}</span>
            </template>
          </a-table-column>
          <a-table-column title="实付金额" :width="120">
            <template #cell="{ record }">
              <span class="text-danger">¥{{ record.pay_amount }}</span>
            </template>
          </a-table-column>
          <a-table-column title="订单状态" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getStatusColor(record.status)">{{ getStatusText(record.status) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="收货信息" :width="200">
            <template #cell="{ record }">
              <div>{{ record.receiver_name }} {{ record.receiver_phone }}</div>
              <div class="text-muted">{{ record.receiver_address }}</div>
            </template>
          </a-table-column>
          <a-table-column title="创建时间" data-index="created_at" :width="160" />
          <a-table-column title="操作" :width="180" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
                <a-button v-if="record.status === 'paid'" type="text" size="small" status="success" @click="handleShip(record)">发货</a-button>
                <a-button v-if="record.status === 'pending'" type="text" size="small" status="danger" @click="handleCancel(record)">取消</a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="订单详情" :footer="false" :width="800">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="订单号">{{ currentRecord.order_no }}</a-descriptions-item>
        <a-descriptions-item label="用户ID">{{ currentRecord.consumer_id }}</a-descriptions-item>
        <a-descriptions-item label="订单状态">
          <a-tag :color="getStatusColor(currentRecord.status)">{{ getStatusText(currentRecord.status) }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="订单金额">
          <span class="text-danger">¥{{ currentRecord.total_amount }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="实付金额">
          <span class="text-danger">¥{{ currentRecord.pay_amount }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="支付方式">{{ currentRecord.pay_method || '-' }}</a-descriptions-item>
        <a-descriptions-item label="收货人">{{ currentRecord.receiver_name }}</a-descriptions-item>
        <a-descriptions-item label="联系电话">{{ currentRecord.receiver_phone }}</a-descriptions-item>
        <a-descriptions-item label="收货地址" :span="2">{{ currentRecord.receiver_address }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.created_at }}</a-descriptions-item>
        <a-descriptions-item label="支付时间">{{ currentRecord.paid_at || '-' }}</a-descriptions-item>
      </a-descriptions>
      <a-divider>商品信息</a-divider>
      <a-table :data="currentRecord.items || []" :pagination="false">
        <template #columns>
          <a-table-column title="商品名称" data-index="name" />
          <a-table-column title="单价" :width="120">
            <template #cell="{ record }">¥{{ record.price }}</template>
          </a-table-column>
          <a-table-column title="数量" data-index="quantity" :width="80" />
          <a-table-column title="小计" :width="120">
            <template #cell="{ record }">¥{{ (record.price * record.quantity).toFixed(2) }}</template>
          </a-table-column>
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="shipVisible" title="订单发货" @ok="submitShip" @cancel="shipVisible = false">
      <a-form :model="shipForm" layout="vertical">
        <a-form-item label="快递公司" required>
          <a-select v-model="shipForm.express_company" placeholder="请选择快递公司">
            <a-option value="SF">顺丰速运</a-option>
            <a-option value="YTO">圆通快递</a-option>
            <a-option value="ZTO">中通快递</a-option>
            <a-option value="STO">申通快递</a-option>
            <a-option value="YD">韵达快递</a-option>
            <a-option value="EMS">EMS</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="快递单号" required>
          <a-input v-model="shipForm.express_no" placeholder="请输入快递单号" />
        </a-form-item>
        <a-form-item label="备注">
          <a-textarea v-model="shipForm.remark" placeholder="发货备注" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';

interface OrderItem {
  id: number;
  name: string;
  price: number;
  quantity: number;
}

interface OrderRecord {
  id: number;
  order_no: string;
  consumer_id: number;
  items: OrderItem[];
  total_amount: string;
  pay_amount: string;
  status: string;
  receiver_name: string;
  receiver_phone: string;
  receiver_address: string;
  pay_method?: string;
  created_at: string;
  paid_at?: string;
}

const loading = ref(false);
const tableData = ref<OrderRecord[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });
const searchForm = reactive({
  keyword: '',
  status: '',
  dateRange: [],
});
const detailVisible = ref(false);
const shipVisible = ref(false);
const currentRecord = ref<OrderRecord>({} as OrderRecord);
const shipForm = reactive({
  order_id: 0,
  express_company: '',
  express_no: '',
  remark: '',
});

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    pending: 'orange',
    paid: 'blue',
    shipped: 'cyan',
    completed: 'green',
    cancelled: 'gray',
  };
  return colors[status] || 'gray';
};

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    pending: '待支付',
    paid: '已支付',
    shipped: '已发货',
    completed: '已完成',
    cancelled: '已取消',
  };
  return texts[status] || status;
};

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    tableData.value = [
      { id: 1, order_no: 'ORD2026032200001', consumer_id: 1001, items: [{ id: 1, name: 'iPhone 15 Pro 手机壳', price: 99, quantity: 1 }, { id: 2, name: '钢化膜', price: 29, quantity: 2 }], total_amount: '157.00', pay_amount: '157.00', status: 'paid', receiver_name: '张三', receiver_phone: '13800138001', receiver_address: '北京市朝阳区xxx街道xxx小区1号楼101', pay_method: 'wechat', created_at: '2026-03-22 10:00:00', paid_at: '2026-03-22 10:05:00' },
      { id: 2, order_no: 'ORD2026032200002', consumer_id: 1002, items: [{ id: 3, name: 'MacBook Pro 14寸保护套', price: 299, quantity: 1 }], total_amount: '299.00', pay_amount: '299.00', status: 'shipped', receiver_name: '李四', receiver_phone: '13800138002', receiver_address: '上海市浦东新区xxx路xxx号', pay_method: 'alipay', created_at: '2026-03-21 15:30:00', paid_at: '2026-03-21 15:35:00' },
      { id: 3, order_no: 'ORD2026032200003', consumer_id: 1003, items: [{ id: 4, name: 'AirPods Pro 2 保护套', price: 49, quantity: 1 }], total_amount: '49.00', pay_amount: '0.00', status: 'pending', receiver_name: '王五', receiver_phone: '13800138003', receiver_address: '广州市天河区xxx大道xxx号', created_at: '2026-03-22 11:00:00' },
    ];
    pagination.value.total = 3;
  } finally {
    loading.value = false;
  }
};

const handleSearch = () => {
  pagination.value.current = 1;
  loadData();
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleExport = () => {
  Message.success('订单导出中，请稍后...');
};

const handleDetail = (record: OrderRecord) => {
  currentRecord.value = record;
  detailVisible.value = true;
};

const handleShip = (record: OrderRecord) => {
  shipForm.order_id = record.id;
  shipForm.express_company = '';
  shipForm.express_no = '';
  shipForm.remark = '';
  shipVisible.value = true;
};

const submitShip = async () => {
  if (!shipForm.express_company || !shipForm.express_no) {
    Message.warning('请填写快递公司和快递单号');
    return;
  }
  Message.success('发货成功');
  shipVisible.value = false;
  loadData();
};

const handleCancel = (record: OrderRecord) => {
  Modal.confirm({
    title: '确认取消',
    content: `确定要取消订单"${record.order_no}"吗？`,
    onOk: () => {
      Message.success('订单已取消');
      loadData();
    },
  });
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
.text-danger {
  color: #f53f3f;
  font-weight: 500;
}
.text-muted {
  color: #86909c;
  font-size: 12px;
}
.goods-item {
  line-height: 1.8;
}
</style>

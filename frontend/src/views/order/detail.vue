<template>
  <div class="page-container">
    <a-card>
      <template #title>订单详情</template>
      <a-input-search v-model="orderNo" placeholder="请输入订单号" search-button style="width: 400px" @search="handleSearch" />
    </a-card>

    <a-card v-if="orderInfo.id" style="margin-top: 16px">
      <a-descriptions title="订单信息" :column="2" bordered>
        <a-descriptions-item label="订单号">{{ orderInfo.order_no }}</a-descriptions-item>
        <a-descriptions-item label="用户ID">{{ orderInfo.consumer_id }}</a-descriptions-item>
        <a-descriptions-item label="订单状态">
          <a-tag :color="getStatusColor(orderInfo.status)">{{ getStatusText(orderInfo.status) }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="订单金额">
          <span class="text-danger">¥{{ orderInfo.total_amount }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="实付金额">
          <span class="text-danger">¥{{ orderInfo.pay_amount }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="支付方式">{{ orderInfo.pay_method || "-" }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ orderInfo.created_at }}</a-descriptions-item>
        <a-descriptions-item label="支付时间">{{ orderInfo.paid_at || "-" }}</a-descriptions-item>
      </a-descriptions>

      <a-descriptions title="收货信息" :column="2" bordered style="margin-top: 16px">
        <a-descriptions-item label="收货人">{{ orderInfo.receiver_name }}</a-descriptions-item>
        <a-descriptions-item label="联系电话">{{ orderInfo.receiver_phone }}</a-descriptions-item>
        <a-descriptions-item label="收货地址" :span="2">{{ orderInfo.receiver_address }}</a-descriptions-item>
      </a-descriptions>

      <a-descriptions v-if="orderInfo.express_no" title="物流信息" :column="2" bordered style="margin-top: 16px">
        <a-descriptions-item label="快递公司">{{ orderInfo.express_company }}</a-descriptions-item>
        <a-descriptions-item label="快递单号">{{ orderInfo.express_no }}</a-descriptions-item>
        <a-descriptions-item label="发货时间">{{ orderInfo.shipped_at }}</a-descriptions-item>
        <a-descriptions-item label="收货时间">{{ orderInfo.received_at || "-" }}</a-descriptions-item>
      </a-descriptions>

      <a-divider>商品信息</a-divider>
      <a-table :data="orderInfo.items || []" :pagination="false">
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
    </a-card>

    <a-empty v-else-if="searched" style="margin-top: 100px" description="未找到该订单" />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { Message } from "@arco-design/web-vue";

const orderNo = ref("");
const searched = ref(false);
const orderInfo = reactive<any>({});

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    pending: "orange",
    paid: "blue",
    shipped: "cyan",
    completed: "green",
    cancelled: "gray"
  };
  return colors[status] || "gray";
};

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    pending: "待支付",
    paid: "已支付",
    shipped: "已发货",
    completed: "已完成",
    cancelled: "已取消"
  };
  return texts[status] || status;
};

const handleSearch = async () => {
  if (!orderNo.value) {
    Message.warning("请输入订单号");
    return;
  }
  searched.value = true;
  Object.assign(orderInfo, {
    id: 1,
    order_no: orderNo.value,
    consumer_id: 1001,
    status: "shipped",
    total_amount: "157.00",
    pay_amount: "157.00",
    pay_method: "wechat",
    receiver_name: "张三",
    receiver_phone: "13800138001",
    receiver_address: "北京市朝阳区xxx街道xxx小区1号楼101",
    express_company: "顺丰速运",
    express_no: "SF1234567890",
    created_at: "2026-03-22 10:00:00",
    paid_at: "2026-03-22 10:05:00",
    shipped_at: "2026-03-22 14:00:00",
    items: [
      { id: 1, name: "iPhone 15 Pro 手机壳", price: 99, quantity: 1 },
      { id: 2, name: "钢化膜", price: 29, quantity: 2 }
    ]
  });
};
</script>

<style scoped>
.page-container {
  padding: 20px;
}
.text-danger {
  color: #f53f3f;
  font-weight: 500;
}
</style>

<template>
  <div class="page-container">
    <a-card>
      <template #title>物流跟踪</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.keyword" placeholder="订单号/快递单号" allow-clear style="width: 180px" />
          <a-select v-model="searchForm.status" placeholder="状态" allow-clear style="width: 100px">
            <a-option value="in_transit">运输中</a-option>
            <a-option value="delivered">已签收</a-option>
            <a-option value="exception">异常</a-option>
          </a-select>
          <a-button type="primary" @click="handleSearch">搜索</a-button>
        </a-space>
      </template>
      <a-table :data="tableData" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="订单号" data-index="order_no" :width="180" />
          <a-table-column title="快递公司" data-index="company_name" :width="120" />
          <a-table-column title="快递单号" data-index="tracking_no" :width="180" />
          <a-table-column title="收货人" data-index="receiver" :width="100" />
          <a-table-column title="收货地址" data-index="address" :width="200" ellipsis />
          <a-table-column title="状态" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getStatusColor(record.status)">{{ getStatusText(record.status) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="发货时间" data-index="ship_time" :width="160" />
          <a-table-column title="操作" :width="120" fixed="right">
            <template #cell="{ record }">
              <a-button type="text" size="small" @click="handleDetail(record)">查看轨迹</a-button>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="物流轨迹" :footer="false" :width="600">
      <a-timeline>
        <a-timeline-item v-for="(item, index) in currentTraces" :key="index" :label="item.time">
          {{ item.context }}
        </a-timeline-item>
      </a-timeline>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';

interface TrackRecord {
  id: number;
  order_no: string;
  company_name: string;
  tracking_no: string;
  receiver: string;
  address: string;
  status: string;
  ship_time: string;
}

const loading = ref(false);
const tableData = ref<TrackRecord[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });
const searchForm = reactive({
  keyword: '',
  status: '',
});
const detailVisible = ref(false);
const currentTraces = ref<{ time: string; context: string }[]>([]);

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    in_transit: 'blue',
    delivered: 'green',
    exception: 'red',
  };
  return colors[status] || 'gray';
};

const getStatusText = (status: string) => {
  const texts: Record<string, string> = {
    in_transit: '运输中',
    delivered: '已签收',
    exception: '异常',
  };
  return texts[status] || status;
};

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    tableData.value = [
      { id: 1, order_no: 'ORD2026032200001', company_name: '顺丰速运', tracking_no: 'SF1234567890', receiver: '张三', address: '北京市朝阳区xxx街道xxx小区', status: 'in_transit', ship_time: '2026-03-22 14:00:00' },
      { id: 2, order_no: 'ORD2026032200002', company_name: '圆通快递', tracking_no: 'YTO9876543210', receiver: '李四', address: '上海市浦东新区xxx路xxx号', status: 'delivered', ship_time: '2026-03-21 10:00:00' },
    ];
    pagination.value.total = 2;
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

const handleDetail = (record: TrackRecord) => {
  currentTraces.value = [
    { time: '2026-03-22 18:30:00', context: '【北京市】快件已到达 北京朝阳区营业点' },
    { time: '2026-03-22 16:00:00', context: '【北京市】快件已发出，下一站：北京朝阳区营业点' },
    { time: '2026-03-22 14:30:00', context: '【北京市】快件已到达 北京转运中心' },
    { time: '2026-03-22 10:00:00', context: '【上海市】快件已发出，下一站：北京转运中心' },
    { time: '2026-03-22 08:00:00', context: '【上海市】快件已揽收' },
  ];
  detailVisible.value = true;
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

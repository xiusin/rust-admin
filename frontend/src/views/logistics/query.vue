<template>
  <div class="page-container">
    <a-card>
      <template #title>物流查询</template>
      <a-form :model="searchForm" layout="inline" style="margin-bottom: 16px">
        <a-form-item label="快递公司">
          <a-select v-model="searchForm.company" placeholder="请选择" style="width: 150px">
            <a-option value="SF">顺丰速运</a-option>
            <a-option value="YTO">圆通快递</a-option>
            <a-option value="ZTO">中通快递</a-option>
            <a-option value="STO">申通快递</a-option>
            <a-option value="YD">韵达快递</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="快递单号">
          <a-input v-model="searchForm.tracking_no" placeholder="请输入快递单号" style="width: 200px" />
        </a-form-item>
        <a-form-item>
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            查询
          </a-button>
        </a-form-item>
      </a-form>

      <a-card v-if="trackingInfo.tracking_no" title="物流信息">
        <a-descriptions :column="2" bordered>
          <a-descriptions-item label="快递公司">{{ trackingInfo.company_name }}</a-descriptions-item>
          <a-descriptions-item label="快递单号">{{ trackingInfo.tracking_no }}</a-descriptions-item>
          <a-descriptions-item label="发货时间">{{ trackingInfo.ship_time }}</a-descriptions-item>
          <a-descriptions-item label="预计到达">{{ trackingInfo.estimated_time }}</a-descriptions-item>
          <a-descriptions-item label="当前状态">
            <a-tag color="blue">{{ trackingInfo.status }}</a-tag>
          </a-descriptions-item>
          <a-descriptions-item label="收货地址">{{ trackingInfo.address }}</a-descriptions-item>
        </a-descriptions>

        <a-divider>物流轨迹</a-divider>
        <a-timeline>
          <a-timeline-item v-for="(item, index) in trackingInfo.traces" :key="index" :label="item.time">
            {{ item.context }}
          </a-timeline-item>
        </a-timeline>
      </a-card>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { Message } from '@arco-design/web-vue';

const searchForm = reactive({
  company: '',
  tracking_no: '',
});

const trackingInfo = reactive({
  tracking_no: '',
  company_name: '',
  ship_time: '',
  estimated_time: '',
  status: '',
  address: '',
  traces: [] as { time: string; context: string }[],
});

const handleSearch = async () => {
  if (!searchForm.company || !searchForm.tracking_no) {
    Message.warning('请选择快递公司并输入快递单号');
    return;
  }
  Object.assign(trackingInfo, {
    tracking_no: searchForm.tracking_no,
    company_name: '顺丰速运',
    ship_time: '2026-03-22 14:00:00',
    estimated_time: '2026-03-24 18:00:00',
    status: '运输中',
    address: '北京市朝阳区xxx街道xxx小区',
    traces: [
      { time: '2026-03-22 18:30:00', context: '【北京市】快件已到达 北京朝阳区营业点' },
      { time: '2026-03-22 16:00:00', context: '【北京市】快件已发出，下一站：北京朝阳区营业点' },
      { time: '2026-03-22 14:30:00', context: '【北京市】快件已到达 北京转运中心' },
      { time: '2026-03-22 10:00:00', context: '【上海市】快件已发出，下一站：北京转运中心' },
      { time: '2026-03-22 08:00:00', context: '【上海市】快件已揽收' },
    ],
  });
};
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

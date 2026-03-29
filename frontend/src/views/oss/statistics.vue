<template>
  <div class="page-container">
    <a-row :gutter="16">
      <a-col :span="6">
        <a-card>
          <a-statistic title="存储用量" :value="statistics.usedStorage" suffix="GB" />
          <a-progress :percent="statistics.usedPercent" size="small" style="margin-top: 8px" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="文件总数" :value="statistics.totalFiles" suffix="个" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="本月流量" :value="statistics.monthTraffic" suffix="GB" />
        </a-card>
      </a-col>
      <a-col :span="6">
        <a-card>
          <a-statistic title="本月请求次数" :value="statistics.monthRequests" />
        </a-card>
      </a-col>
    </a-row>

    <a-row :gutter="16" style="margin-top: 16px">
      <a-col :span="12">
        <a-card>
          <template #title>存储分布</template>
          <a-table :data="storageData" :pagination="false">
            <template #columns>
              <a-table-column title="存储类型" data-index="type" />
              <a-table-column title="文件数" data-index="count" />
              <a-table-column title="占用空间" data-index="size" />
              <a-table-column title="占比" :width="200">
                <template #cell="{ record }">
                  <a-progress :percent="record.percent" size="small" />
                </template>
              </a-table-column>
            </template>
          </a-table>
        </a-card>
      </a-col>
      <a-col :span="12">
        <a-card>
          <template #title>流量趋势</template>
          <div id="chart" style="height: 200px"></div>
        </a-card>
      </a-col>
    </a-row>

    <a-card style="margin-top: 16px">
      <template #title>热门文件TOP10</template>
      <a-table :data="hotFiles" :pagination="false">
        <template #columns>
          <a-table-column title="排名" :width="60">
            <template #cell="{ rowIndex }">
              <a-tag v-if="rowIndex < 3" :color="['gold', 'silver', '#cd7f32'][rowIndex]">{{ rowIndex + 1 }}</a-tag>
              <span v-else>{{ rowIndex + 1 }}</span>
            </template>
          </a-table-column>
          <a-table-column title="文件名" data-index="name" />
          <a-table-column title="文件大小" data-index="size" :width="100" />
          <a-table-column title="访问次数" data-index="views" :width="100" />
          <a-table-column title="流量消耗" data-index="traffic" :width="120" />
        </template>
      </a-table>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";

const statistics = reactive({
  usedStorage: 125.6,
  usedPercent: 62,
  totalFiles: 15680,
  monthTraffic: 856.3,
  monthRequests: 1256800
});

const storageData = ref([
  { type: "图片", count: 12500, size: "85.2GB", percent: 68 },
  { type: "视频", count: 280, size: "32.5GB", percent: 26 },
  { type: "文档", count: 2900, size: "7.9GB", percent: 6 }
]);

const hotFiles = ref([
  { name: "banner_home_2026.png", size: "512KB", views: 125680, traffic: "62.5GB" },
  { name: "product_001.jpg", size: "256KB", views: 89560, traffic: "22.4GB" },
  { name: "intro_video.mp4", size: "15.2MB", views: 45230, traffic: "687GB" },
  { name: "product_002.jpg", size: "198KB", views: 38920, traffic: "7.6GB" },
  { name: "category_icon.png", size: "32KB", views: 35680, traffic: "1.1GB" }
]);

onMounted(() => {});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

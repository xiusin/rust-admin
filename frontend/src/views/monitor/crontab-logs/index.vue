<template>
  <div class="snow-page">
    <div class="snow-inner">
      <!-- 搜索区域 -->
      <a-form ref="searchFormRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="job_name" label="任务名称">
              <a-input v-model="form.job_name" placeholder="请输入任务名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="执行状态">
              <a-select v-model="form.status" placeholder="请选择执行状态" allow-clear>
                <a-option v-for="item in statusOptions" :key="item.value" :value="item.value">{{ item.name }}</a-option>
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

      <!-- 操作栏 -->
      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" status="danger" size="small" :disabled="selectedKeys.length === 0">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getJobLogList"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-dropdown @select="onDensity">
              <a-tooltip content="密度">
                <div class="action-icon"><icon-line-height size="18" /></div>
              </a-tooltip>
              <template #content>
                <a-doption v-for="item in densityType" :value="item.value" :key="item.value" :disabled="item.value === density">{{
                  item.label
                }}</a-doption>
              </template>
            </a-dropdown>
            <a-tooltip content="列设置">
              <a-popover trigger="click" position="br" @popup-visible-change="popupVisibleChange">
                <div class="action-icon"><icon-settings size="18" /></div>
                <template #content>
                  <div id="tableSetting">
                    <div v-for="(item, index) in columns" :key="item.dataIndex" class="setting">
                      <div class="setting-box-icon"><icon-drag-arrow /></div>
                      <div><a-checkbox v-model="item.checked" @change="onCheckbox($event, item, index)"></a-checkbox></div>
                      <div class="title">{{ item.title }}</div>
                    </div>
                  </div>
                </template>
              </a-popover>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <!-- 任务日志表格 -->
      <a-table
        row-key="log_id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columnsShow"
        :data="list"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedKeys"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status === '0'">成功</a-tag>
          <a-tag bordered size="small" color="red" v-else>失败</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onDetail(record)">详情</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <!-- 日志详情弹窗 -->
    <a-modal v-model:visible="detailOpen" :footer="false" width="600px">
      <template #title> 日志详情 </template>
      <div v-if="currentLog">
        <a-descriptions :column="1" bordered>
          <a-descriptions-item label="日志ID">{{ currentLog.log_id }}</a-descriptions-item>
          <a-descriptions-item label="任务ID">{{ currentLog.job_id }}</a-descriptions-item>
          <a-descriptions-item label="任务名称">{{ currentLog.job_name }}</a-descriptions-item>
          <a-descriptions-item label="执行状态">
            <a-tag bordered size="small" color="arcoblue" v-if="currentLog.status === '0'">成功</a-tag>
            <a-tag bordered size="small" color="red" v-else>失败</a-tag>
          </a-descriptions-item>
          <a-descriptions-item label="执行次数">{{ currentLog.run_count }}</a-descriptions-item>
          <a-descriptions-item label="执行耗时">{{ currentLog.elapsed_time }} ms</a-descriptions-item>
          <a-descriptions-item label="执行结果">{{ currentLog.job_message || "无" }}</a-descriptions-item>
          <a-descriptions-item label="执行时间">{{ currentLog.created_at }}</a-descriptions-item>
        </a-descriptions>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { getJobLogListAPI } from "@/api/modules/monitor/job";
import { useRouteConfigStore } from "@/store/modules/route-config";
import { deepClone } from "@/utils";
import Sortable from "sortablejs";

let route = useRoute();
const routerStore = useRouteConfigStore();

const statusOptions = ref([
  { name: "成功", value: "0" },
  { name: "失败", value: "1" }
]);

// 密度设置
const densityType = ref([
  { value: "mini", label: "迷你" },
  { value: "small", label: "偏小" },
  { value: "medium", label: "中等" },
  { value: "large", label: "偏大" }
]);
const density = ref("small");
const onDensity = (e: string) => {
  density.value = e;
};

// 表格列配置
const columns = ref([
  { title: "日志ID", dataIndex: "log_id", checked: true },
  { title: "任务ID", dataIndex: "job_id", checked: true },
  { title: "任务名称", dataIndex: "job_name", checked: true },
  { title: "执行状态", dataIndex: "status", slotName: "status", align: "center", checked: true },
  { title: "执行次数", dataIndex: "run_count", align: "center", checked: true },
  { title: "执行耗时(ms)", dataIndex: "elapsed_time", align: "center", checked: true },
  { title: "执行结果", dataIndex: "job_message", ellipsis: true, tooltip: true, checked: true },
  { title: "执行时间", dataIndex: "created_at", checked: true },
  { title: "操作", slotName: "optional", align: "center", checked: true }
]);
const columnsShow = ref<any[]>([]);
const deepColumns = () => {
  columnsShow.value = deepClone(columns.value);
};
deepColumns();

const onCheckbox = (checked: any, row: any, index: any) => {
  if (!checked) {
    columnsShow.value = columnsShow.value.filter((item: any) => item.dataIndex != row.dataIndex);
  } else {
    columnsShow.value.splice(index, 0, row);
  }
};

const popupVisibleChange = (visible: boolean) => {
  if (visible) {
    nextTick(() => {
      const el = document.getElementById("tableSetting") as HTMLElement;
      new Sortable(el, {
        onEnd(e: any) {
          const { oldIndex, newIndex } = e;
          exchangeArray(columns.value, oldIndex, newIndex);
          exchangeArray(columnsShow.value, oldIndex, newIndex);
        }
      });
    });
  }
};

const exchangeArray = (arr: Array<any>, oldIndex: number, newIndex: number) => {
  const temp = arr[newIndex];
  arr[newIndex] = arr[oldIndex];
  arr[oldIndex] = temp;
};

// 搜索表单
const form = ref({
  job_name: "",
  status: null as string | null
});
const searchFormRef = ref();
const search = () => {
  pagination.value.current = 1;
  getJobLogList();
};
const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.value.current = 1;
  getJobLogList();
};

// 批量删除
const selectedKeys = ref<string[]>([]);

// 获取列表
const loading = ref(false);
const pagination = ref({
  pageSize: 10,
  current: 1,
  showPageSize: true,
  showTotal: true,
  total: 0
});
const pageChange = (page: number) => {
  pagination.value.current = page;
  getJobLogList();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  getJobLogList();
};
const list = ref([]);
const getJobLogList = async () => {
  try {
    loading.value = true;
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (route.query.job_id) params.job_id = route.query.job_id;
    if (form.value.job_name) params.job_name = form.value.job_name;
    if (form.value.status) params.status = form.value.status;
    const res = await getJobLogListAPI(params);
    list.value = res.data.list || [];
    pagination.value.total = res.data.total || 0;
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

// 详情
const detailOpen = ref(false);
const currentLog = ref<any>(null);
const onDetail = (record: any) => {
  currentLog.value = deepClone(record);
  detailOpen.value = true;
};

const init = () => {
  if (route.query.job_id) {
    form.value.job_name = route.query.job_name as string;
  }
  getJobLogList();
};

init();

routerStore.setTabsTitle(`任务日志${route.query.job_name ? " - " + route.query.job_name : ""}`);
</script>

<style lang="scss" scoped>
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
.setting {
  display: flex;
  align-items: center;
  width: 200px;
  .setting-box-icon {
    margin-right: 4px;
    cursor: move;
  }
  .title {
    margin-left: 8px;
  }
}
</style>

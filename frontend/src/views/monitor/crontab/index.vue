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
            <a-form-item field="status" label="任务状态">
              <a-select v-model="form.status" placeholder="请选择状态" allow-clear>
                <a-option v-for="item in openState" :key="item.value" :value="item.value">{{ item.name }}</a-option>
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
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedKeys.length === 0" @click="onBatchDelete">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getJobList"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-dropdown @select="onDensity">
              <a-tooltip content="密度">
                <div class="action-icon"><icon-line-height size="18" /></div>
              </a-tooltip>
              <template #content>
                <a-doption v-for="item in densityType" :value="item.value" :key="item.value" :disabled="item.value === density">{{ item.label }}</a-doption>
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

      <!-- 定时任务表格 -->
      <a-table
        row-key="job_id"
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
        <template #task_type="{ record }">
          <a-tag v-if="record.task_type === 'CRON'" bordered size="small" color="purple">CRON</a-tag>
          <a-tag v-else bordered size="small" color="blue">间隔</a-tag>
        </template>
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status === '0'">启用</a-tag>
          <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" status="success" @click="onExecute(record)">执行</a-button>
            <a-button type="text" size="mini" @click="onLogs(record)">日志</a-button>
            <a-button type="text" size="mini" @click="onUpdate(record)">修改</a-button>
            <a-popconfirm type="warning" content="确定删除该任务吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal :width="dialogWidth()" v-model:visible="open" @close="afterClose" @ok="handleOk" @cancel="afterClose">
      <template #title> {{ title }} </template>
      <div>
        <a-form ref="formRef" auto-label-width :layout="formLayout" :rules="rules" :model="addFrom">
          <a-form-item field="job_name" label="任务名称" validate-trigger="blur">
            <a-input v-model="addFrom.job_name" placeholder="请输入任务名称" allow-clear />
          </a-form-item>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="task_type" label="任务类型" validate-trigger="blur">
                <a-select placeholder="请选择任务类型" v-model="addFrom.task_type" allow-clear>
                  <a-option v-for="item in taskTypeOption" :key="item.value" :value="item.value">{{ item.name }}</a-option>
                </a-select>
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="cron_expression" label="Cron表达式" validate-trigger="blur">
                <a-input v-model="addFrom.cron_expression" placeholder="请输入Cron表达式" allow-clear>
                  <template #append>
                    <a-button type="text" size="mini" @click="onValidateCron">验证</a-button>
                  </template>
                </a-input>
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="task_count" label="执行次数" validate-trigger="blur">
                <a-input-number
                  v-model="addFrom.task_count"
                  placeholder="请输入执行次数"
                  :min="0"
                  :step="1"
                  :precision="0"
                  allow-clear
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="run_count" label="已运行次数" validate-trigger="blur">
                <a-input-number
                  v-model="addFrom.run_count"
                  placeholder="请输入已运行次数"
                  :min="0"
                  :step="1"
                  :precision="0"
                  allow-clear
                  disabled
                />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="job_params" label="任务参数" validate-trigger="blur">
            <a-textarea v-model="addFrom.job_params" placeholder="请输入任务参数(JSON格式)" allow-clear />
          </a-form-item>
          <a-form-item field="status" label="任务状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="addFrom.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
          <a-form-item field="remark" label="备注" validate-trigger="blur">
            <a-textarea v-model="addFrom.remark" placeholder="请输入备注" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>

    <!-- Cron验证结果弹窗 -->
    <a-modal v-model:visible="cronModalVisible" :footer="false" width="500px">
      <template #title> Cron表达式验证结果 </template>
      <div v-if="cronValidateResult">
        <a-alert v-if="cronValidateResult.validate" type="success" message="表达式有效"></a-alert>
        <a-alert v-else type="error" message="表达式无效"></a-alert>
        <div v-if="cronValidateResult.next_ten && cronValidateResult.next_ten.length > 0" style="margin-top: 16px;">
          <h4>未来10次执行时间：</h4>
          <a-list :bordered="false">
            <a-list-item v-for="(time, index) in cronValidateResult.next_ten" :key="index">
              {{ index + 1 }}. {{ time }}
            </a-list-item>
          </a-list>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { deepClone } from "@/utils";
import {
  getJobListAPI,
  addJobAPI,
  editJobAPI,
  deleteJobAPI,
  validateCronAPI,
  handExecuteJobAPI
} from "@/api/modules/monitor/job";
import { useLayoutModel } from "@/hooks/useLayoutModel";
import Sortable from "sortablejs";

const router = useRouter();
const openState = ref(dictFilter("status"));
const { dialogWidth, formLayout } = useLayoutModel();

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
  { title: "任务ID", dataIndex: "job_id", checked: true },
  { title: "任务名称", dataIndex: "job_name", checked: true },
  { title: "任务类型", dataIndex: "task_type", slotName: "task_type", align: "center", checked: true },
  { title: "Cron表达式", dataIndex: "cron_expression", checked: true },
  { title: "执行次数", dataIndex: "task_count", align: "center", checked: true },
  { title: "已运行次数", dataIndex: "run_count", align: "center", checked: true },
  { title: "状态", dataIndex: "status", slotName: "status", align: "center", checked: true },
  { title: "备注", dataIndex: "remark", checked: true },
  { title: "创建时间", dataIndex: "created_at", checked: true },
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
  job_group: "",
  status: null as string | null
});
const searchFormRef = ref();
const search = () => {
  pagination.value.current = 1;
  getJobList();
};
const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.value.current = 1;
  getJobList();
};

// 任务类型选项
const taskTypeOption = ref([
  { name: "CRON", value: "CRON" },
  { name: "间隔", value: "INTERVAL" }
]);

// 新增/编辑
const open = ref(false);
const rules = {
  job_name: [{ required: true, message: "请输入任务名称" }],
  task_type: [{ required: true, message: "请选择任务类型" }],
  cron_expression: [{ required: true, message: "请输入Cron表达式" }]
};
const addFrom = ref<any>({
  job_id: "",
  job_name: "",
  job_group: "",
  task_type: "CRON",
  task_count: 0,
  run_count: 0,
  cron_expression: "",
  job_params: "",
  status: "0",
  remark: ""
});
const title = ref("");
const formRef = ref();
const formType = ref(0); // 0新增 1编辑

const onAdd = () => {
  title.value = "添加任务";
  formType.value = 0;
  open.value = true;
};

const onUpdate = (row: any) => {
  title.value = "修改任务";
  formType.value = 1;
  addFrom.value = deepClone({
    ...row,
    job_id: String(row.job_id)
  });
  open.value = true;
};

const handleOk = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    const apiData = {
      ...addFrom.value,
      job_id: addFrom.value.job_id ? String(addFrom.value.job_id) : undefined,
      task_count: Number(addFrom.value.task_count) || 0,
      run_count: Number(addFrom.value.run_count) || 0
    };
    if (formType.value === 1) {
      await editJobAPI(apiData);
      arcoMessage("success", "修改成功");
    } else {
      await addJobAPI(apiData);
      arcoMessage("success", "新增成功");
    }
    open.value = false;
    getJobList();
  } catch (e) {
    console.error(e);
  }
};

const afterClose = () => {
  formRef.value?.resetFields();
  addFrom.value = {
    job_id: "",
    job_name: "",
    job_group: "",
    task_type: "CRON",
    task_count: 0,
    run_count: 0,
    cron_expression: "",
    job_params: "",
    status: "0",
    remark: ""
  };
};

// 删除
const onDelete = async (row: any) => {
  try {
    await deleteJobAPI({ job_id: String(row.job_id) });
    arcoMessage("success", "删除成功");
    getJobList();
  } catch (e) {
    console.error(e);
  }
};

// 批量删除
const selectedKeys = ref<string[]>([]);
const onBatchDelete = async () => {
  try {
    for (const job_id of selectedKeys.value) {
      await deleteJobAPI({ job_id });
    }
    arcoMessage("success", "批量删除成功");
    selectedKeys.value = [];
    getJobList();
  } catch (e) {
    console.error(e);
  }
};

// 手动执行
const onExecute = async (row: any) => {
  try {
    await handExecuteJobAPI({ job_id: String(row.job_id) });
    arcoMessage("success", "任务执行成功");
    getJobList();
  } catch (e) {
    console.error(e);
  }
};

// 查看日志
const onLogs = (row: any) => {
  router.push({
    path: "/monitor/crontab-logs",
    query: {
      job_id: row.job_id,
      job_name: row.job_name
    }
  });
};

// Cron验证
const cronModalVisible = ref(false);
const cronValidateResult = ref<any>(null);
const onValidateCron = async () => {
  if (!addFrom.value.cron_expression) {
    arcoMessage("warning", "请输入Cron表达式");
    return;
  }
  try {
    const res = await validateCronAPI({ cron_expression: addFrom.value.cron_expression });
    cronValidateResult.value = res.data;
    cronModalVisible.value = true;
  } catch (e) {
    console.error(e);
  }
};

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
  getJobList();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  getJobList();
};
const list = ref([]);
const getJobList = async () => {
  try {
    loading.value = true;
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (form.value.job_name) params.job_name = form.value.job_name;
    if (form.value.job_group) params.job_group = form.value.job_group;
    if (form.value.status) params.status = form.value.status;
    const res = await getJobListAPI(params);
    list.value = res.data.list || [];
    pagination.value.total = res.data.total || 0;
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

getJobList();
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

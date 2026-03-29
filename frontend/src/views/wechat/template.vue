<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="title" label="模板名称">
              <a-input v-model="formData.form.title" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="platform" label="所属平台">
              <a-select v-model="formData.form.platform" placeholder="请选择平台" allow-clear>
                <a-option value="mp">公众号</a-option>
                <a-option value="mini">小程序</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="状态">
              <a-select v-model="formData.form.status" placeholder="请选择状态" allow-clear>
                <a-option value="active">启用</a-option>
                <a-option value="inactive">停用</a-option>
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
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" @click="handleAdd">
              <template #icon><icon-plus /></template>
              新增模板
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="loadData"><icon-refresh size="18" /></div>
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
                      <div>
                        <a-checkbox v-model="item.checked" @change="onCheckbox($event, item, index)"></a-checkbox>
                      </div>
                      <div class="title">{{ item.title }}</div>
                    </div>
                  </div>
                </template>
              </a-popover>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>
      <a-table
        row-key="id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columnsShow"
        :data="tableData"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #platform="{ record }">
          <a-tag :color="record.platform === 'mp' ? 'green' : 'blue'">
            {{ record.platform === "mp" ? "公众号" : "小程序" }}
          </a-tag>
        </template>
        <template #status="{ record }">
          <a-badge
            :status="record.status === 'active' ? 'success' : 'default'"
            :text="record.status === 'active' ? '启用' : '停用'"
          />
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
            <a-button type="text" size="small" status="danger" @click="handleDelete(record)">删除</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false" :width="600">
      <a-form :model="form" layout="vertical">
        <a-form-item label="模板ID" required>
          <a-input v-model="form.template_id" placeholder="请输入微信模板ID" />
        </a-form-item>
        <a-form-item label="模板名称" required>
          <a-input v-model="form.title" placeholder="请输入模板名称" />
        </a-form-item>
        <a-form-item label="所属平台">
          <a-radio-group v-model="form.platform">
            <a-radio value="mp">公众号</a-radio>
            <a-radio value="mini">小程序</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item label="模板类型">
          <a-select v-model="form.type" placeholder="请选择模板类型">
            <a-option value="order">订单通知</a-option>
            <a-option value="refund">退款通知</a-option>
            <a-option value="shipping">发货通知</a-option>
            <a-option value="activity">活动通知</a-option>
            <a-option value="other">其他</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="模板内容">
          <a-textarea v-model="form.content" placeholder="请输入模板内容" :auto-size="{ minRows: 4, maxRows: 8 }" />
        </a-form-item>
        <a-form-item label="状态">
          <a-radio-group v-model="form.status">
            <a-radio value="active">启用</a-radio>
            <a-radio value="inactive">停用</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, nextTick } from "vue";
import { Message, Modal } from "@arco-design/web-vue";
import Sortable from "sortablejs";
import { deepClone } from "@/utils";

interface TemplateRecord {
  id: number;
  template_id: string;
  title: string;
  platform: string;
  type: string;
  status: string;
  created_at: string;
}

const formData = reactive({
  form: {
    title: "",
    platform: null as string | null,
    status: null as string | null
  }
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<TemplateRecord[]>([]);

const pagination = ref({ showPageSize: true, showTotal: true, current: 1, pageSize: 10, total: 0 });
const pageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  loadData();
};

interface Column {
  title: string;
  dataIndex: string;
  checked: boolean;
  slotName?: string;
  align?: string;
  width?: number;
}

const columnsShow = ref<Column[]>([]);
const columns = ref<Column[]>([
  { title: "模板ID", dataIndex: "template_id", checked: true, width: 180 },
  { title: "模板名称", dataIndex: "title", checked: true, width: 200 },
  { title: "所属平台", dataIndex: "platform", checked: true, slotName: "platform", width: 100 },
  { title: "模板类型", dataIndex: "type", checked: true, width: 120 },
  { title: "状态", dataIndex: "status", checked: true, slotName: "status", width: 100 },
  { title: "创建时间", dataIndex: "created_at", checked: true, width: 160 },
  { title: "操作", slotName: "optional", align: "center", checked: true, width: 180 }
]);

const deepColumns = () => {
  columnsShow.value = deepClone(columns.value);
};
deepColumns();

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

const exchangeArray = (cols: Array<any>, oldIndex: number, newIndex: number) => {
  let temp = cols[newIndex];
  cols[newIndex] = cols[oldIndex];
  cols[oldIndex] = temp;
};

const modalVisible = ref(false);
const modalTitle = ref("新增模板");
const form = reactive({
  id: 0,
  template_id: "",
  title: "",
  platform: "mp",
  type: "",
  content: "",
  status: "active"
});

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    tableData.value = [
      {
        id: 1,
        template_id: "TEMPLATE_ID_001",
        title: "订单支付成功通知",
        platform: "mp",
        type: "order",
        status: "active",
        created_at: "2026-01-15 10:00:00"
      },
      {
        id: 2,
        template_id: "TEMPLATE_ID_002",
        title: "发货通知",
        platform: "mini",
        type: "shipping",
        status: "active",
        created_at: "2026-02-20 14:30:00"
      }
    ];
    pagination.value.total = 2;
  } finally {
    loading.value = false;
  }
};

const handleAdd = () => {
  modalTitle.value = "新增模板";
  Object.assign(form, { id: 0, template_id: "", title: "", platform: "mp", type: "", content: "", status: "active" });
  modalVisible.value = true;
};

const handleEdit = (record: TemplateRecord) => {
  modalTitle.value = "编辑模板";
  Object.assign(form, record);
  modalVisible.value = true;
};

const handleDelete = (record: TemplateRecord) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除模板"${record.title}"吗？`,
    onOk: () => {
      Message.success("删除成功");
      loadData();
    }
  });
};

const handleSubmit = async () => {
  Message.success(form.id ? "编辑成功" : "新增成功");
  modalVisible.value = false;
  loadData();
};

onMounted(() => {
  loadData();
});
</script>

<style lang="scss" scoped>
.search-btn {
  margin-bottom: 20px;
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

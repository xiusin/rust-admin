<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="modelId" label="所属模型">
              <a-select v-model="searchForm.modelId" placeholder="请选择模型" allow-clear>
                <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="title" label="内容标题">
              <a-input v-model="searchForm.title" placeholder="请输入标题" allow-clear />
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

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" status="success" size="small" :disabled="selectedIds.length === 0" @click="onBatchRestore">
              <template #icon><icon-undo /></template>
              批量恢复
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedIds.length === 0" @click="onBatchClear">
              <template #icon><icon-delete /></template>
              彻底删除
            </a-button>
            <a-popconfirm content="确定要清空回收站吗？此操作不可恢复！" type="warning" @ok="onClearAll">
              <a-button type="primary" status="danger" size="small">
                <template #icon><icon-delete /></template>
                清空回收站
              </a-button>
            </a-popconfirm>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%' }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #status="{ record }">
          <a-tag color="gray" size="small">已删除</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onRestore(record)">恢复</a-button>
            <a-popconfirm content="确定要彻底删除吗？此操作不可恢复！" type="warning" @ok="onClear([record.id])">
              <a-button type="text" status="danger" size="mini">彻底删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { contentApi, type CmsContentList } from "@/api/modules/cms/content";
import { modelApi, type CmsModel } from "@/api/modules/cms/model";

const loading = ref(false);
const tableData = ref<CmsContentList[]>([]);
const selectedIds = ref<number[]>([]);
const models = ref<CmsModel[]>([]);
const searchFormRef = ref();

const searchForm = reactive({
  modelId: null as number | null,
  title: ""
});

const pagination = reactive({
  current: 1,
  pageSize: 20,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { title: "标题", dataIndex: "title", width: 200, ellipsis: true, tooltip: true },
  { title: "所属模型", dataIndex: "modelName", width: 120 },
  { title: "分类", dataIndex: "categoryName", width: 100 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 80, align: "center" },
  { title: "删除时间", dataIndex: "updatedAt", width: 160 },
  { title: "操作", slotName: "optional", width: 150, fixed: "right", align: "center" }
];

const loadModels = async () => {
  try {
    models.value = await modelApi.simpleList();
  } catch (error) {
    console.error(error);
  }
};

const getList = async () => {
  if (!searchForm.modelId) {
    tableData.value = [];
    pagination.total = 0;
    return;
  }
  loading.value = true;
  try {
    const data = await contentApi.recycleList({
      modelId: searchForm.modelId,
      pageNum: pagination.current,
      pageSize: pagination.pageSize
    });
    tableData.value = data.list || [];
    pagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.current = 1;
  tableData.value = [];
  pagination.total = 0;
};

const refresh = () => {
  getList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  getList();
};

const onRestore = async (record: CmsContentList) => {
  try {
    await contentApi.restore(record.id);
    arcoMessage("success", "恢复成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchRestore = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await Promise.all(selectedIds.value.map(id => contentApi.restore(id)));
    arcoMessage("success", "批量恢复成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onClear = async (ids: number[]) => {
  try {
    await contentApi.clearRecycle(ids);
    arcoMessage("success", "删除成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchClear = async () => {
  if (selectedIds.value.length === 0) return;
  Modal.warning({
    title: "确认彻底删除",
    content: `确定要彻底删除选中的${selectedIds.value.length}条内容吗？此操作不可恢复！`,
    hideCancel: false,
    onOk: async () => {
      await onClear(selectedIds.value);
    }
  });
};

const onClearAll = async () => {
  try {
    await contentApi.clearRecycle();
    arcoMessage("success", "清空回收站成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadModels();
});
</script>

<style scoped lang="scss">
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
</style>

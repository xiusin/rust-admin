<template>
  <div class="model-list-page">
    <div class="page-wrapper">
      <div class="page-header">
        <div class="header-left">
          <a-form ref="searchFormRef" :model="searchForm" class="search-form">
            <a-space size="medium">
              <a-input-search
                v-model="searchForm.name"
                placeholder="搜索模型名称"
                style="width: 240px"
                @search="search"
                allow-clear
              />
              <a-input
                v-model="searchForm.code"
                placeholder="模型编码"
                style="width: 180px"
                allow-clear
              />
              <a-select
                v-model="searchForm.isEnabled"
                placeholder="状态"
                style="width: 140px"
                allow-clear
              >
                <a-option :value="true">启用</a-option>
                <a-option :value="false">禁用</a-option>
              </a-select>
              <a-button type="primary" @click="search">
                <template #icon><icon-search /></template>
                查询
              </a-button>
              <a-button @click="reset">
                <template #icon><icon-refresh /></template>
                重置
              </a-button>
            </a-space>
          </a-form>
        </div>
        <div class="header-right">
          <a-space size="medium">
            <a-tooltip content="刷新数据">
              <a-button shape="circle" @click="refresh">
                <icon-refresh />
              </a-button>
            </a-tooltip>
            <a-tooltip :content="viewMode === 'card' ? '切换表格视图' : '切换卡片视图'">
              <a-button shape="circle" @click="toggleViewMode">
                <icon-apps v-if="viewMode === 'card'" />
                <icon-list v-else />
              </a-button>
            </a-tooltip>
          </a-space>
        </div>
      </div>

      <div class="page-toolbar">
        <div class="toolbar-left">
          <a-space size="medium">
            <a-button type="primary" size="middle" @click="onAdd">
              <template #icon><icon-plus /></template>
              新建模型
            </a-button>
            <div v-if="selectedIds.length > 0" class="batch-actions">
              <a-tag color="arcoblue" :closable="false">已选 {{ selectedIds.length }} 项</a-tag>
              <a-button status="danger" size="small" @click="onBatchDelete">
                <template #icon><icon-delete /></template>
                批量删除
              </a-button>
            </div>
          </a-space>
        </div>
      </div>

      <div class="content-area">
        <div v-if="viewMode === 'card'" class="model-card-list">
          <a-row :gutter="[16, 16]">
            <a-col :xs="24" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in tableData" :key="item.id">
              <ModelCard
                :data="item"
                @edit="onEdit"
                @delete="onDelete"
                @design="onDesign"
                @content="onContent"
                @copy="onCopy"
                @toggle-status="onToggleStatus"
              />
            </a-col>
          </a-row>
        </div>

        <ModelTable
          v-else
          :loading="loading"
          :data="tableData"
          :pagination="pagination"
          :selected-keys="selectedIds"
          @page-change="handlePageChange"
          @page-size-change="handlePageSizeChange"
          @selection-change="onSelectionChange"
          @edit="onEdit"
          @delete="onDelete"
          @design="onDesign"
          @content="onContent"
          @copy="onCopy"
          @toggle-status="onToggleStatus"
        />
      </div>
    </div>

    <a-modal
      v-model:visible="modalVisible"
      :width="580"
      @ok="onSubmit"
      @cancel="onCancel"
      :footer="true"
    >
      <template #title>
        <div class="modal-title">
          <icon-plus v-if="!form.id" />
          <icon-edit v-else />
          <span>{{ modalTitle }}</span>
        </div>
      </template>
      <div class="modal-content">
        <a-form ref="formRef" auto-label-width :rules="rules" :model="form" layout="vertical">
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="name" label="模型名称" validate-trigger="blur">
                <a-input v-model="form.name" placeholder="请输入模型名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="code" label="模型编码" validate-trigger="blur">
                <a-input v-model="form.code" placeholder="请输入模型编码" allow-clear :disabled="!!form.id" />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="tableName" label="表名称" validate-trigger="blur">
            <a-input v-model="form.tableName" placeholder="请输入表名称" allow-clear :disabled="!!form.id" />
          </a-form-item>
          <a-form-item field="description" label="描述">
            <a-textarea
              v-model="form.description"
              placeholder="请输入模型描述"
              allow-clear
              :auto-size="{ minRows: 3, maxRows: 5 }"
            />
          </a-form-item>
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="icon" label="图标">
                <a-input v-model="form.icon" placeholder="请输入图标名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="sort" label="排序">
                <a-input-number
                  v-model="form.sort"
                  :min="0"
                  :max="9999"
                  style="width: 100%"
                  placeholder="请输入排序值"
                  mode="button"
                />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="isEnabled" label="模型状态">
            <a-switch type="round" v-model="form.isEnabled">
              <template #checked>启用</template>
              <template #unchecked>禁用</template>
            </a-switch>
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import { modelApi, type CmsModelList, type ModelAddParams, type ModelEditParams } from "@/api/modules/cms/model";
import ModelCard from "./components/ModelCard.vue";
import ModelTable from "./components/ModelTable.vue";

const router = useRouter();
const loading = ref(false);
const tableData = ref<CmsModelList[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增模型");
const formRef = ref();
const selectedIds = ref<number[]>([]);
const searchFormRef = ref();
const viewMode = ref<"card" | "table">("card");

const searchForm = reactive({
  name: "",
  code: "",
  isEnabled: null as boolean | null
});

const form = reactive({
  id: 0,
  name: "",
  code: "",
  tableName: "",
  description: "",
  icon: "",
  sort: 0,
  isEnabled: true
});

const rules = {
  name: [{ required: true, message: "请输入模型名称" }],
  code: [{ required: true, message: "请输入模型编码" }],
  tableName: [{ required: true, message: "请输入表名称" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const getList = async () => {
  loading.value = true;
  try {
    const data = await modelApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      code: searchForm.code || undefined,
      isEnabled: searchForm.isEnabled ?? undefined
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
  getList();
};

const refresh = () => {
  getList();
};

const toggleViewMode = () => {
  viewMode.value = viewMode.value === "card" ? "table" : "card";
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  getList();
};

const onSelectionChange = (keys: (string | number)[]) => {
  selectedIds.value = keys as number[];
};

const onAdd = () => {
  modalTitle.value = "新增模型";
  form.id = 0;
  form.name = "";
  form.code = "";
  form.tableName = "";
  form.description = "";
  form.icon = "";
  form.sort = 0;
  form.isEnabled = true;
  modalVisible.value = true;
};

const onEdit = (record: CmsModelList) => {
  modalTitle.value = "编辑模型";
  form.id = record.id;
  form.name = record.name;
  form.code = record.code;
  form.tableName = record.tableName;
  form.description = record.description || "";
  form.icon = record.icon || "";
  form.sort = record.sort;
  form.isEnabled = record.isEnabled;
  modalVisible.value = true;
};

const onDelete = async (record: CmsModelList) => {
  try {
    await modelApi.delete(record.id);
    arcoMessage("success", "删除成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await Promise.all(selectedIds.value.map(id => modelApi.delete(id)));
    arcoMessage("success", "批量删除成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDesign = (record: CmsModelList) => {
  router.push({
    path: "/cms/model/design",
    query: { id: record.id }
  });
};

const onContent = (record: CmsModelList) => {
  router.push({
    path: "/cms/content/list",
    query: { modelId: record.id }
  });
};

const onCopy = async (record: CmsModelList) => {
  try {
    await modelApi.copy(record.id);
    arcoMessage("success", "复制成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onToggleStatus = async (record: CmsModelList) => {
  try {
    if (record.isEnabled) {
      await modelApi.disable(record.id);
      arcoMessage("success", "禁用成功");
    } else {
      await modelApi.enable(record.id);
      arcoMessage("success", "启用成功");
    }
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  const state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await modelApi.edit(form as ModelEditParams);
      arcoMessage("success", "修改成功");
    } else {
      await modelApi.add(form as ModelAddParams);
      arcoMessage("success", "新增成功");
    }
    modalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onCancel = () => {
  modalVisible.value = false;
  formRef.value?.resetFields();
};

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.model-list-page {
  height: 100%;
  background: var(--color-bg-1);

  .page-wrapper {
    height: 100%;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--color-bg-2);
    border-radius: 8px;
    padding: 16px;
    border: 1px solid var(--color-border-1);

    .header-left {
      flex: 1;

      .search-form {
        display: flex;
        align-items: center;
      }
    }

    .header-right {
      display: flex;
      align-items: center;
    }
  }

  .page-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);

    .toolbar-left {
      display: flex;
      align-items: center;

      .batch-actions {
        display: flex;
        align-items: center;
        gap: 8px;
        padding-left: 12px;
        border-left: 1px solid var(--color-border-1);
      }
    }
  }

  .content-area {
    flex: 1;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);
    padding: 16px;
    overflow-y: auto;
  }

  .model-card-list {
    min-height: 400px;
  }
}

.modal-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
}

.modal-content {
  padding: 8px 0;
}
</style>

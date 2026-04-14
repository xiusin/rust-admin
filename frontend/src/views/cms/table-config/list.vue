<template>
  <div class="table-config-page">
    <div class="page-wrapper">
      <div class="page-header">
        <div class="header-left">
          <a-select
            v-model="searchForm.modelId"
            placeholder="请选择模型"
            allow-clear
            @change="onModelChange"
            style="width: 280px"
          >
            <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
          </a-select>
          <a-button type="primary" @click="search">
            <template #icon><icon-search /></template>
            查询
          </a-button>
          <a-button @click="reset">
            <template #icon><icon-refresh /></template>
            重置
          </a-button>
        </div>
        <div class="header-right">
          <a-tooltip content="刷新数据">
            <a-button shape="circle" @click="refresh">
              <icon-refresh />
            </a-button>
          </a-tooltip>
        </div>
      </div>

      <div class="page-toolbar">
        <div class="toolbar-left">
          <a-space size="medium">
            <a-button type="primary" size="middle" :disabled="!searchForm.modelId" @click="onAdd">
              <template #icon><icon-plus /></template>
              新建表格配置
            </a-button>
          </a-space>
        </div>
      </div>

      <div class="content-area">
        <div v-if="tableData.length > 0" class="config-card-list">
          <a-row :gutter="[16, 16]">
            <a-col :xs="24" :sm="12" :md="8" :lg="6" v-for="item in tableData" :key="item.id">
              <ConfigCard
                :data="item"
                @edit="onEdit"
                @delete="onDelete"
                @preview="onPreview"
                @copy="onCopy"
                @set-default="onSetDefault"
              />
            </a-col>
          </a-row>
        </div>
        <div v-else class="empty-list">
          <a-empty :description="searchForm.modelId ? '暂无表格配置' : '请选择模型查看表格配置'">
            <template v-if="searchForm.modelId">
              <a-button type="primary" @click="onAdd">
                <template #icon><icon-plus /></template>
                创建第一个配置
              </a-button>
            </template>
          </a-empty>
        </div>
      </div>

      <a-modal v-model:visible="previewVisible" :width="900" :footer="false">
        <template #title>
          <div class="modal-title">
            <icon-table />
            <span>表格预览</span>
          </div>
        </template>
        <PreviewModal v-if="previewConfig" :config="previewConfig" />
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import { tableApi, type TableConfigItem } from "@/api/modules/cms/table";
import { modelApi, type CmsModel } from "@/api/modules/cms/model";
import ConfigCard from "./components/ConfigCard.vue";
import PreviewModal from "./components/PreviewModal.vue";

const router = useRouter();
const loading = ref(false);
const tableData = ref<TableConfigItem[]>([]);
const models = ref<CmsModel[]>([]);
const previewVisible = ref(false);
const previewConfig = ref<TableConfigItem | null>(null);
const searchFormRef = ref();

const searchForm = reactive({
  modelId: null as number | null
});

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
    return;
  }
  loading.value = true;
  try {
    tableData.value = await tableApi.list(searchForm.modelId);
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const onModelChange = () => {
  getList();
};

const search = () => {
  getList();
};

const reset = () => {
  searchFormRef.value?.resetFields();
  tableData.value = [];
};

const refresh = () => {
  getList();
};

const onAdd = () => {
  router.push({
    path: "/cms/table-config/builder",
    query: { modelId: searchForm.modelId }
  });
};

const onEdit = (record: TableConfigItem) => {
  router.push({
    path: "/cms/table-config/builder",
    query: { id: record.id, modelId: searchForm.modelId }
  });
};

const onDelete = async (record: TableConfigItem) => {
  try {
    await tableApi.delete(record.id);
    arcoMessage("success", "删除成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onPreview = async (record: TableConfigItem) => {
  try {
    previewConfig.value = await tableApi.detail(record.id);
    previewVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onSetDefault = async (record: TableConfigItem) => {
  try {
    await tableApi.setDefault(record.id);
    arcoMessage("success", "设置成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onCopy = async (record: TableConfigItem) => {
  try {
    await tableApi.copy(record.id);
    arcoMessage("success", "复制成功");
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
.table-config-page {
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
      display: flex;
      align-items: center;
      gap: 12px;
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
  }

  .content-area {
    flex: 1;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);
    padding: 16px;
    overflow-y: auto;
  }

  .config-card-list {
    min-height: 400px;
  }

  .empty-list {
    display: flex;
    align-items: center;
    justify-content: center;
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
</style>

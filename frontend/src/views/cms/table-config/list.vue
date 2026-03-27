<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="modelId" label="所属模型">
              <a-select v-model="searchForm.modelId" placeholder="请选择模型" allow-clear @change="onModelChange">
                <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
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

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="onAdd" :disabled="!searchForm.modelId">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
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
        <a-empty description="暂无表格配置" />
      </div>

      <a-modal v-model:visible="previewVisible" :width="900" :footer="false" title="表格预览">
        <PreviewModal v-if="previewConfig" :config="previewConfig" />
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { tableApi, type TableConfigItem } from '@/api/modules/cms/table';
import { modelApi, type CmsModel } from '@/api/modules/cms/model';
import ConfigCard from './components/ConfigCard.vue';
import PreviewModal from './components/PreviewModal.vue';

const router = useRouter();
const loading = ref(false);
const tableData = ref<TableConfigItem[]>([]);
const models = ref<CmsModel[]>([]);
const previewVisible = ref(false);
const previewConfig = ref<TableConfigItem | null>(null);
const searchFormRef = ref();

const searchForm = reactive({
  modelId: null as number | null,
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
    path: '/cms/table-config/builder',
    query: { modelId: searchForm.modelId },
  });
};

const onEdit = (record: TableConfigItem) => {
  router.push({
    path: '/cms/table-config/builder',
    query: { id: record.id, modelId: searchForm.modelId },
  });
};

const onDelete = async (record: TableConfigItem) => {
  try {
    await tableApi.delete(record.id);
    arcoMessage('success', '删除成功');
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
    arcoMessage('success', '设置成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onCopy = async (record: TableConfigItem) => {
  try {
    await tableApi.copy(record.id);
    arcoMessage('success', '复制成功');
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

.config-card-list {
  min-height: 400px;
}

.empty-list {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 400px;
}
</style>

<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="模型名称">
              <a-input v-model="searchForm.name" placeholder="请输入模型名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="code" label="模型编码">
              <a-input v-model="searchForm.code" placeholder="请输入模型编码" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="isEnabled" label="状态">
              <a-select v-model="searchForm.isEnabled" placeholder="请选择状态" allow-clear>
                <a-option :value="true">启用</a-option>
                <a-option :value="false">禁用</a-option>
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
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedIds.length === 0" @click="onBatchDelete">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-tooltip :content="viewMode === 'card' ? '表格视图' : '卡片视图'">
              <div class="action-icon" @click="toggleViewMode">
                <icon-apps v-if="viewMode === 'card'" size="18" />
                <icon-list v-else size="18" />
              </div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <div v-if="viewMode === 'card'" class="model-card-list">
        <a-row :gutter="[16, 16]">
          <a-col :xs="24" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in tableData" :key="item.id">
            <ModelCard :data="item" @edit="onEdit" @delete="onDelete" @design="onDesign" @content="onContent" @copy="onCopy" @toggle-status="onToggleStatus" />
          </a-col>
        </a-row>
      </div>

      <ModelTable v-else :loading="loading" :data="tableData" :pagination="pagination" :selected-keys="selectedIds" @page-change="handlePageChange" @page-size-change="handlePageSizeChange" @selection-change="onSelectionChange" @edit="onEdit" @delete="onDelete" @design="onDesign" @content="onContent" @copy="onCopy" @toggle-status="onToggleStatus" />

      <a-modal v-model:visible="modalVisible" :width="600" @ok="onSubmit" @cancel="onCancel">
        <template #title>{{ modalTitle }}</template>
        <div>
          <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
            <a-form-item field="name" label="模型名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入模型名称" allow-clear />
            </a-form-item>
            <a-form-item field="code" label="模型编码" validate-trigger="blur">
              <a-input v-model="form.code" placeholder="请输入模型编码" allow-clear :disabled="!!form.id" />
            </a-form-item>
            <a-form-item field="tableName" label="表名称" validate-trigger="blur">
              <a-input v-model="form.tableName" placeholder="请输入表名称" allow-clear :disabled="!!form.id" />
            </a-form-item>
            <a-form-item field="description" label="描述">
              <a-textarea v-model="form.description" placeholder="请输入描述" allow-clear :auto-size="{ minRows: 2, maxRows: 4 }" />
            </a-form-item>
            <a-form-item field="icon" label="图标">
              <a-input v-model="form.icon" placeholder="请输入图标名称" allow-clear />
            </a-form-item>
            <a-form-item field="sort" label="排序">
              <a-input-number v-model="form.sort" :min="0" :max="9999" :style="{ width: '150px' }" placeholder="请输入排序" mode="button" />
            </a-form-item>
            <a-form-item field="isEnabled" label="状态">
              <a-switch type="round" v-model="form.isEnabled">
                <template #checked>启用</template>
                <template #unchecked>禁用</template>
              </a-switch>
            </a-form-item>
          </a-form>
        </div>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { modelApi, type CmsModelList, type ModelAddParams, type ModelEditParams } from '@/api/modules/cms/model';
import ModelCard from './components/ModelCard.vue';
import ModelTable from './components/ModelTable.vue';

const router = useRouter();
const loading = ref(false);
const tableData = ref<CmsModelList[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增模型');
const formRef = ref();
const selectedIds = ref<number[]>([]);
const searchFormRef = ref();
const viewMode = ref<'card' | 'table'>('card');

const searchForm = reactive({
  name: '',
  code: '',
  isEnabled: null as boolean | null,
});

const form = reactive({
  id: 0,
  name: '',
  code: '',
  tableName: '',
  description: '',
  icon: '',
  sort: 0,
  isEnabled: true,
});

const rules = {
  name: [{ required: true, message: '请输入模型名称' }],
  code: [{ required: true, message: '请输入模型编码' }],
  tableName: [{ required: true, message: '请输入表名称' }],
};

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const getList = async () => {
  loading.value = true;
  try {
    const data = await modelApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      code: searchForm.code || undefined,
      isEnabled: searchForm.isEnabled ?? undefined,
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
  viewMode.value = viewMode.value === 'card' ? 'table' : 'card';
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
  modalTitle.value = '新增模型';
  form.id = 0;
  form.name = '';
  form.code = '';
  form.tableName = '';
  form.description = '';
  form.icon = '';
  form.sort = 0;
  form.isEnabled = true;
  modalVisible.value = true;
};

const onEdit = (record: CmsModelList) => {
  modalTitle.value = '编辑模型';
  form.id = record.id;
  form.name = record.name;
  form.code = record.code;
  form.tableName = record.tableName;
  form.description = record.description || '';
  form.icon = record.icon || '';
  form.sort = record.sort;
  form.isEnabled = record.isEnabled;
  modalVisible.value = true;
};

const onDelete = async (record: CmsModelList) => {
  try {
    await modelApi.delete(record.id);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await Promise.all(selectedIds.value.map((id) => modelApi.delete(id)));
    arcoMessage('success', '批量删除成功');
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDesign = (record: CmsModelList) => {
  router.push({
    path: '/cms/model/design',
    query: { id: record.id },
  });
};

const onContent = (record: CmsModelList) => {
  router.push({
    path: '/cms/content/list',
    query: { modelId: record.id },
  });
};

const onCopy = async (record: CmsModelList) => {
  try {
    await modelApi.copy(record.id);
    arcoMessage('success', '复制成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onToggleStatus = async (record: CmsModelList) => {
  try {
    if (record.isEnabled) {
      await modelApi.disable(record.id);
      arcoMessage('success', '禁用成功');
    } else {
      await modelApi.enable(record.id);
      arcoMessage('success', '启用成功');
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
      arcoMessage('success', '修改成功');
    } else {
      await modelApi.add(form as ModelAddParams);
      arcoMessage('success', '新增成功');
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

.model-card-list {
  min-height: 400px;
}
</style>

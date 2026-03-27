<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="标签名称">
              <a-input v-model="searchForm.name" placeholder="请输入标签名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="code" label="标签编码">
              <a-input v-model="searchForm.code" placeholder="请输入标签编码" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
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
            <a-button type="primary" size="small" @click="onBatchAdd">
              <template #icon><icon-plus-multiple /></template>
              批量添加
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
            <a-tooltip content="标签云">
              <div class="action-icon" @click="showTagCloud"><icon-apps size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <TagTable
        :loading="loading"
        :data="tableData"
        :pagination="pagination"
        :selected-keys="selectedIds"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="onSelectionChange"
        @edit="onEdit"
        @delete="onDelete"
        @toggle-status="onToggleStatus"
      />

      <a-modal v-model:visible="modalVisible" :width="500" @ok="onSubmit" @cancel="onCancel">
        <template #title>{{ modalTitle }}</template>
        <div>
          <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
            <a-form-item field="name" label="标签名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入标签名称" allow-clear />
            </a-form-item>
            <a-form-item field="code" label="标签编码" validate-trigger="blur">
              <a-input v-model="form.code" placeholder="请输入标签编码" allow-clear :disabled="!!form.id" />
            </a-form-item>
            <a-form-item field="color" label="标签颜色">
              <a-input v-model="form.color" placeholder="请输入颜色值" allow-clear />
            </a-form-item>
            <a-form-item field="sort" label="排序">
              <a-input-number v-model="form.sort" :min="0" :max="9999" style="width: 150px" />
            </a-form-item>
            <a-form-item field="status" label="状态">
              <a-switch type="round" v-model="form.status">
                <template #checked>启用</template>
                <template #unchecked>禁用</template>
              </a-switch>
            </a-form-item>
            <a-form-item field="description" label="描述">
              <a-textarea v-model="form.description" placeholder="请输入描述" allow-clear />
            </a-form-item>
          </a-form>
        </div>
      </a-modal>

      <a-modal v-model:visible="batchModalVisible" :width="600" title="批量添加标签" @ok="onBatchSubmit" @cancel="batchModalVisible = false">
        <div>
          <a-form ref="batchFormRef" :model="batchForm">
            <a-form-item label="标签列表">
              <a-textarea
                v-model="batchForm.tagsText"
                placeholder="请输入标签，每行一个标签，格式：标签名称|标签编码|颜色（可选）"
                :auto-size="{ minRows: 10, maxRows: 20 }"
              />
            </a-form-item>
          </a-form>
          <div class="batch-tips">
            <p>示例：</p>
            <pre>技术|tech|#1890ff
生活|life|#52c41a
旅行|travel|#fa8c16</pre>
          </div>
        </div>
      </a-modal>

      <a-modal v-model:visible="tagCloudVisible" :width="800" :footer="false" title="标签云">
        <TagCloud :tags="tagCloudData" @select="onTagSelect" />
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { tagApi, type CmsTagItem, type TagAddParams, type TagEditParams } from '@/api/modules/cms/tag';
import TagTable from './components/TagTable.vue';
import TagCloud from './components/TagCloud.vue';

const loading = ref(false);
const tableData = ref<CmsTagItem[]>([]);
const selectedIds = ref<number[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增标签');
const formRef = ref();
const searchFormRef = ref();
const batchModalVisible = ref(false);
const batchFormRef = ref();
const tagCloudVisible = ref(false);
const tagCloudData = ref<CmsTagItem[]>([]);

const searchForm = reactive({
  name: '',
  code: '',
  status: null as boolean | null,
});

const form = reactive({
  id: 0,
  name: '',
  code: '',
  color: '',
  sort: 0,
  status: true,
  description: '',
});

const batchForm = reactive({
  tagsText: '',
});

const rules = {
  name: [{ required: true, message: '请输入标签名称' }],
  code: [{ required: true, message: '请输入标签编码' }],
};

const pagination = reactive({
  current: 1,
  pageSize: 20,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const getList = async () => {
  loading.value = true;
  try {
    const data = await tagApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      code: searchForm.code || undefined,
      status: searchForm.status ?? undefined,
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
  modalTitle.value = '新增标签';
  form.id = 0;
  form.name = '';
  form.code = '';
  form.color = '';
  form.sort = 0;
  form.status = true;
  form.description = '';
  modalVisible.value = true;
};

const onEdit = (record: CmsTagItem) => {
  modalTitle.value = '编辑标签';
  form.id = record.id;
  form.name = record.name;
  form.code = record.code;
  form.color = record.color || '';
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || '';
  modalVisible.value = true;
};

const onDelete = async (record: CmsTagItem) => {
  try {
    await tagApi.delete(record.id);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  Modal.warning({
    title: '确认删除',
    content: `确定要删除选中的${selectedIds.value.length}个标签吗？`,
    hideCancel: false,
    onOk: async () => {
      try {
        await tagApi.batchDelete(selectedIds.value);
        arcoMessage('success', '批量删除成功');
        selectedIds.value = [];
        getList();
      } catch (error) {
        console.error(error);
      }
    },
  });
};

const onToggleStatus = async (record: CmsTagItem) => {
  try {
    await tagApi.updateStatus(record.id, !record.status);
    arcoMessage('success', record.status ? '禁用成功' : '启用成功');
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
      await tagApi.edit(form as TagEditParams);
      arcoMessage('success', '修改成功');
    } else {
      await tagApi.add(form as TagAddParams);
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

const onBatchAdd = () => {
  batchForm.tagsText = '';
  batchModalVisible.value = true;
};

const onBatchSubmit = async () => {
  if (!batchForm.tagsText.trim()) {
    arcoMessage('warning', '请输入标签');
    return;
  }
  const lines = batchForm.tagsText.split('\n').filter(Boolean);
  const tags = lines.map((line) => {
    const parts = line.split('|');
    return {
      name: parts[0]?.trim() || '',
      code: parts[1]?.trim() || parts[0]?.trim() || '',
      color: parts[2]?.trim() || undefined,
    };
  }).filter((t) => t.name && t.code);

  if (tags.length === 0) {
    arcoMessage('warning', '请输入有效的标签');
    return;
  }

  try {
    await tagApi.batchAdd({ tags });
    arcoMessage('success', '批量添加成功');
    batchModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const showTagCloud = async () => {
  try {
    tagCloudData.value = await tagApi.cloud(100);
    tagCloudVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onTagSelect = (tag: CmsTagItem) => {
  searchForm.name = tag.name;
  tagCloudVisible.value = false;
  search();
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

.batch-tips {
  margin-top: 12px;
  padding: 12px;
  background: var(--color-fill-1);
  border-radius: 4px;

  pre {
    margin: 8px 0 0;
    padding: 8px;
    background: var(--color-bg-2);
    border-radius: 4px;
    font-size: 12px;
  }
}
</style>

<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="品牌名称">
              <a-input v-model="searchForm.name" placeholder="请输入品牌名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="品牌状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option value="0">启用</a-option>
                <a-option value="1">禁用</a-option>
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
          </a-space>
        </a-col>
      </a-row>

      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #logo="{ record }">
          <a-image-preview>
            <div class="brand-logo" v-if="record.logo">
              <img :src="record.logo" :alt="record.name" />
            </div>
            <div class="brand-logo empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" bordered size="small">
            {{ record.status === '0' ? '启用' : '禁用' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-popconfirm type="warning" content="确定删除该品牌吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :width="600" @ok="onSubmit" @cancel="onCancel">
      <template #title>{{ modalTitle }}</template>
      <div>
        <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
          <a-form-item field="name" label="品牌名称" validate-trigger="blur">
            <a-input v-model="form.name" placeholder="请输入品牌名称" allow-clear />
          </a-form-item>
          <a-form-item field="logo" label="品牌Logo" validate-trigger="blur">
            <a-input v-model="form.logo" placeholder="请输入Logo URL" allow-clear />
          </a-form-item>
          <a-form-item field="website" label="官方网站" validate-trigger="blur">
            <a-input v-model="form.website" placeholder="请输入官方网站" allow-clear />
          </a-form-item>
          <a-form-item field="sort" label="排序" validate-trigger="blur">
            <a-input-number v-model="form.sort" :min="0" :max="9999" :style="{ width: '150px' }" placeholder="请输入排序" mode="button" />
          </a-form-item>
          <a-form-item field="status" label="状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
          <a-form-item field="description" label="品牌描述" validate-trigger="blur">
            <a-textarea v-model="form.description" placeholder="请输入品牌描述" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { brandApi, BrandListItem } from '@/api/modules/product/brand';

const loading = ref(false);
const tableData = ref<BrandListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增品牌');
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();

const searchForm = reactive({
  name: '',
  status: null as string | null,
});

const form = reactive({
  id: 0,
  name: '',
  logo: '',
  website: '',
  sort: 0,
  status: '0',
  description: '',
});

const rules = {
  name: [{ required: true, message: '请输入品牌名称' }],
};

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60, fixed: 'left' },
  { title: '品牌Logo', dataIndex: 'logo', slotName: 'logo', width: 100 },
  { title: '品牌名称', dataIndex: 'name', width: 150 },
  { title: '官方网站', dataIndex: 'website', width: 180 },
  { title: '排序', dataIndex: 'sort', width: 80, align: 'center' },
  { title: '状态', dataIndex: 'status', slotName: 'status', width: 100, align: 'center' },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 150, fixed: 'right', align: 'center' },
];

const getList = async () => {
  loading.value = true;
  try {
    const data = await brandApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status || undefined,
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

const onAdd = () => {
  modalTitle.value = '新增品牌';
  form.id = 0;
  form.name = '';
  form.logo = '';
  form.website = '';
  form.sort = 0;
  form.status = '0';
  form.description = '';
  modalVisible.value = true;
};

const onEdit = (record: BrandListItem) => {
  modalTitle.value = '编辑品牌';
  form.id = record.id;
  form.name = record.name;
  form.logo = record.logo || '';
  form.website = record.website || '';
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || '';
  modalVisible.value = true;
};

const onDelete = async (record: BrandListItem) => {
  try {
    await brandApi.delete([record.id]);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await brandApi.delete(selectedIds.value.map((id) => Number(id)));
    arcoMessage('success', '批量删除成功');
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    if (form.id) {
      await brandApi.edit(form);
      arcoMessage('success', '修改成功');
    } else {
      await brandApi.add(form);
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

.brand-logo {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  &.empty {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f2f3f5;
    color: #86909c;
    font-size: 20px;
  }
}
</style>

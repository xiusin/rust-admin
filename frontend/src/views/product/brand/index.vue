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
                <a-option value="0">正常</a-option>
                <a-option value="1">停用</a-option>
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
            <a-button type="primary" size="small" status="danger" @click="onBatchDelete" :disabled="selectedIds.length === 0">
              <template #icon><icon-delete /></template>
              批量删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getList"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        ref="tableRef"
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 800 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="handleSelectionChange"
      >
        <template #logo="{ record }">
          <a-avatar v-if="record.logo" :size="40" :image-url="record.logo" />
          <a-avatar v-else :size="40"><icon-image /></a-avatar>
        </template>
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status === '0'">正常</a-tag>
          <a-tag bordered size="small" color="red" v-else>停用</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-if="record.status === '0'">禁用</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-else>启用</a-button>
            <a-popconfirm type="warning" content="确定删除该品牌吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="600" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-form-item field="name" label="品牌名称" :rules="[{ required: true, message: '请输入品牌名称' }]">
          <a-input v-model="form.name" placeholder="请输入品牌名称" />
        </a-form-item>
        <a-form-item field="logo" label="品牌Logo">
          <a-input v-model="form.logo" placeholder="请输入Logo URL" />
        </a-form-item>
        <a-form-item field="description" label="品牌描述">
          <a-textarea v-model="form.description" placeholder="请输入品牌描述" :rows="3" />
        </a-form-item>
        <a-form-item field="sort" label="排序">
          <a-input-number v-model="form.sort" placeholder="请输入排序" :min="0" />
        </a-form-item>
        <a-form-item field="status" label="状态">
          <a-radio-group v-model="form.status">
            <a-radio value="0">正常</a-radio>
            <a-radio value="1">停用</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { brandApi, BrandListItem } from '@/api/modules/product/brand';

const loading = ref(false);
const tableData = ref<BrandListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增品牌');
const formRef = ref();
const selectedIds = ref<number[]>([]);

const searchForm = reactive({
  name: '',
  status: '',
});

const form = reactive({
  id: 0,
  name: '',
  logo: '',
  description: '',
  sort: 0,
  status: '0',
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60 },
  {
    title: '品牌Logo',
    dataIndex: 'logo',
    slotName: 'logo',
    width: 100,
  },
  {
    title: '品牌名称',
    dataIndex: 'name',
    width: 200,
  },
  {
    title: '品牌描述',
    dataIndex: 'description',
    ellipsis: true,
  },
  {
    title: '商品数量',
    dataIndex: 'productCount',
    width: 100,
  },
  {
    title: '排序',
    dataIndex: 'sort',
    width: 80,
  },
  {
    title: '状态',
    dataIndex: 'status',
    slotName: 'status',
    width: 100,
  },
  {
    title: '创建时间',
    dataIndex: 'createdAt',
    width: 180,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 200,
    fixed: 'right',
  },
];

const getList = async () => {
  loading.value = true;
  try {
    const res = await brandApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status || undefined,
    });
    if (res.code === 200) {
      tableData.value = res.data?.list || [];
      pagination.total = res.data?.total || 0;
    }
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
  searchForm.name = '';
  searchForm.status = '';
  pagination.current = 1;
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

const handleSelectionChange = (keys: (string | number)[]) => {
  selectedIds.value = keys as number[];
};

const onAdd = () => {
  modalTitle.value = '新增品牌';
  form.id = 0;
  form.name = '';
  form.logo = '';
  form.description = '';
  form.sort = 0;
  form.status = '0';
  modalVisible.value = true;
};

const onEdit = (record: BrandListItem) => {
  modalTitle.value = '编辑品牌';
  form.id = record.id;
  form.name = record.name;
  form.logo = record.logo || '';
  form.description = record.description || '';
  form.sort = record.sort;
  form.status = record.status;
  modalVisible.value = true;
};

const onDelete = async (record: BrandListItem) => {
  try {
    const res = await brandApi.delete([record.id]);
    if (res.code === 200) {
      Message.success('删除成功');
      getList();
    } else {
      Message.error(res.message || '删除失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    const res = await brandApi.delete(selectedIds.value);
    if (res.code === 200) {
      Message.success('批量删除成功');
      getList();
    } else {
      Message.error(res.message || '删除失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onUpdateStatus = async (record: BrandListItem) => {
  const newStatus = record.status === '0' ? '1' : '0';
  try {
    const res = await brandApi.updateStatus(record.id, newStatus);
    if (res.code === 200) {
      Message.success(newStatus === '0' ? '启用成功' : '禁用成功');
      getList();
    } else {
      Message.error(res.message || '操作失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  try {
    let res;
    if (form.id) {
      res = await brandApi.edit({
        id: form.id,
        name: form.name,
        logo: form.logo || undefined,
        description: form.description || undefined,
        sort: form.sort,
        status: form.status,
      });
    } else {
      res = await brandApi.add({
        name: form.name,
        logo: form.logo || undefined,
        description: form.description || undefined,
        sort: form.sort,
        status: form.status,
      });
    }

    if (res.code === 200) {
      Message.success(form.id ? '编辑成功' : '新增成功');
      modalVisible.value = false;
      getList();
    } else {
      Message.error(res.message || '操作失败');
    }
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

<style scoped lang="less">
.snow-page {
  padding: 16px;
  height: 100%;
  box-sizing: border-box;
}

.snow-inner {
  background: var(--color-bg-2);
  padding: 16px;
  border-radius: 4px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.search-btn {
  display: flex;
  align-items: center;
}

.action-icon {
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  &:hover {
    background: var(--color-fill-2);
  }
}
</style>

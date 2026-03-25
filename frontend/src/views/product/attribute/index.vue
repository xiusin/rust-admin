<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="模板名称">
              <a-input v-model="searchForm.name" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="模板状态">
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
        :scroll="{ x: '100%', y: '100%', minWidth: 1200 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" bordered size="small">
            {{ record.status === '0' ? '启用' : '禁用' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-popconfirm type="warning" content="确定删除该模板吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="800" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="name" label="模板名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="categoryId" label="关联分类" validate-trigger="blur">
              <a-tree-select
                v-model="form.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择关联分类"
                style="width: 100%"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="sort" label="排序" validate-trigger="blur">
              <a-input-number v-model="form.sort" :min="0" :max="9999" :style="{ width: '150px' }" placeholder="请输入排序" mode="button" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="status" label="状态" validate-trigger="blur">
              <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
                <template #checked> 启用 </template>
                <template #unchecked> 禁用 </template>
              </a-switch>
            </a-form-item>
          </a-col>
        </a-row>

        <a-divider>属性列表</a-divider>

        <a-table :data="form.attributes" :pagination="false" :bordered="{ cell: true }">
          <template #columns>
            <a-table-column title="属性名称">
              <template #cell="{ record }">
                <a-input v-model="record.name" placeholder="属性名" />
              </template>
            </a-table-column>
            <a-table-column title="类型" :width="120">
              <template #cell="{ record }">
                <a-select v-model="record.attrType" style="width: 100%">
                  <a-option :value="0">文本</a-option>
                  <a-option :value="1">单选</a-option>
                  <a-option :value="2">多选</a-option>
                </a-select>
              </template>
            </a-table-column>
            <a-table-column title="必填" :width="60">
              <template #cell="{ record }">
                <a-switch type="round" :checked-value="1" :unchecked-value="0" v-model="record.isRequired" size="small" />
              </template>
            </a-table-column>
            <a-table-column title="可筛选" :width="60">
              <template #cell="{ record }">
                <a-switch type="round" :checked-value="1" :unchecked-value="0" v-model="record.isFilter" size="small" />
              </template>
            </a-table-column>
            <a-table-column title="操作" :width="60">
              <template #cell="{ rowIndex }">
                <a-button type="text" status="danger" size="small" @click="removeAttribute(rowIndex)">
                  <icon-delete />
                </a-button>
              </template>
            </a-table-column>
          </template>
        </a-table>
        <a-button type="primary" size="small" @click="addAttribute" style="margin-top: 12px">
          <template #icon><icon-plus /></template>
          添加属性
        </a-button>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { attributeTemplateApi, AttributeTemplateListItem } from '@/api/modules/product/attributeTemplate';
import { categoryApi } from '@/api/modules/product/category';

interface AttributeFormItem {
  id?: number;
  name: string;
  attrType: number;
  isRequired: number;
  isFilter: number;
  sort: number;
  status: string;
}

const loading = ref(false);
const tableData = ref<AttributeTemplateListItem[]>([]);
const categoryTree = ref<any[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增模板');
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
  categoryId: 0,
  sort: 0,
  status: '0',
  attributes: [] as AttributeFormItem[],
});

const rules = {
  name: [{ required: true, message: '请输入模板名称' }],
  categoryId: [{ required: true, message: '请选择关联分类' }],
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
  { title: '模板名称', dataIndex: 'name', width: 200 },
  { title: '关联分类', dataIndex: 'categoryName', width: 150 },
  { title: '属性数量', dataIndex: 'attributeCount', width: 100, align: 'center' },
  { title: '排序', dataIndex: 'sort', width: 80, align: 'center' },
  { title: '状态', slotName: 'status', width: 100, align: 'center' },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 150, fixed: 'right' },
];

const getList = async () => {
  loading.value = true;
  try {
    const data = await attributeTemplateApi.list({
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

const getCategoryTree = async () => {
  try {
    const data = await categoryApi.tree();
    categoryTree.value = data || [];
  } catch (error) {
    console.error(error);
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
  modalTitle.value = '新增属性模板';
  form.id = 0;
  form.name = '';
  form.categoryId = 0;
  form.sort = 0;
  form.status = '0';
  form.attributes = [];
  modalVisible.value = true;
};

const onEdit = async (record: AttributeTemplateListItem) => {
  modalTitle.value = '编辑属性模板';
  try {
    const data = await attributeTemplateApi.detail(record.id);
    form.id = data.id;
    form.name = data.name;
    form.categoryId = data.categoryId;
    form.sort = data.sort;
    form.status = data.status;
    form.attributes = (data.attributes || []).map((a: any) => ({
      id: a.id,
      name: a.name,
      attrType: a.attrType,
      isRequired: a.isRequired,
      isFilter: a.isFilter,
      sort: a.sort,
      status: a.status,
    }));
    modalVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: AttributeTemplateListItem) => {
  try {
    await attributeTemplateApi.delete([record.id]);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await attributeTemplateApi.delete(selectedIds.value.map((id) => Number(id)));
    arcoMessage('success', '批量删除成功');
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const addAttribute = () => {
  form.attributes.push({
    name: '',
    attrType: 1,
    isRequired: 0,
    isFilter: 0,
    sort: form.attributes.length,
    status: '0',
  });
};

const removeAttribute = (index: number) => {
  form.attributes.splice(index, 1);
};

const onSubmit = async () => {
  let state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await attributeTemplateApi.edit(form);
      arcoMessage('success', '修改成功');
    } else {
      await attributeTemplateApi.add(form);
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
  getCategoryTree();
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

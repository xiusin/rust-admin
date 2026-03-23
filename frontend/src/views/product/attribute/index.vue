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
            <a-form-item field="categoryId" label="关联分类">
              <a-tree-select
                v-model="searchForm.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择分类"
                allow-clear
              />
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
              新增模板
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
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="handleSelectionChange"
      >
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'">{{ record.status === '0' ? '启用' : '禁用' }}</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-if="record.status === '0'">禁用</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-else>启用</a-button>
            <a-popconfirm type="warning" content="确定删除该模板吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="900" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="name" label="模板名称" :rules="[{ required: true, message: '请输入模板名称' }]">
              <a-input v-model="form.name" placeholder="请输入模板名称" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="categoryId" label="关联分类" :rules="[{ required: true, message: '请选择关联分类' }]">
              <a-tree-select
                v-model="form.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择关联分类"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="sort" label="排序">
              <a-input-number v-model="form.sort" placeholder="请输入排序" :min="0" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="status" label="状态">
              <a-radio-group v-model="form.status">
                <a-radio value="0">启用</a-radio>
                <a-radio value="1">禁用</a-radio>
              </a-radio-group>
            </a-form-item>
          </a-col>
        </a-row>

        <a-divider>属性列表</a-divider>

        <div class="attribute-list">
          <div class="attribute-header">
            <a-button type="primary" size="small" @click="addAttribute">
              <template #icon><icon-plus /></template>
              添加属性
            </a-button>
          </div>
          <a-table :data="form.attributes" :pagination="false" :bordered="true" style="margin-top: 16px">
            <template #columns>
              <a-table-column title="属性名称" :width="150">
                <template #cell="{ record }">
                  <a-input v-model="record.name" placeholder="属性名" />
                </template>
              </a-table-column>
              <a-table-column title="属性类型" :width="120">
                <template #cell="{ record }">
                  <a-select v-model="record.attrType" placeholder="类型">
                    <a-option :value="0">文本</a-option>
                    <a-option :value="1">单选</a-option>
                    <a-option :value="2">多选</a-option>
                  </a-select>
                </template>
              </a-table-column>
              <a-table-column title="必填" :width="60">
                <template #cell="{ record }">
                  <a-switch v-model="record.isRequired" :checked-value="1" :unchecked-value="0" size="small" />
                </template>
              </a-table-column>
              <a-table-column title="筛选" :width="60">
                <template #cell="{ record }">
                  <a-switch v-model="record.isFilter" :checked-value="1" :unchecked-value="0" size="small" />
                </template>
              </a-table-column>
              <a-table-column title="属性值">
                <template #cell="{ record }">
                  <div class="attr-values">
                    <a-tag
                      v-for="(v, idx) in record.values"
                      :key="idx"
                      closable
                      @close="removeAttributeValue(record, idx)"
                    >
                      {{ v.value }}
                    </a-tag>
                    <a-input
                      v-if="record.attrType > 0"
                      v-model="record.newValue"
                      placeholder="输入属性值"
                      size="small"
                      style="width: 100px"
                      @press-enter="addAttributeValue(record)"
                    />
                    <a-button
                      v-if="record.attrType > 0"
                      type="text"
                      size="small"
                      @click="addAttributeValue(record)"
                    >
                      添加
                    </a-button>
                  </div>
                </template>
              </a-table-column>
              <a-table-column title="操作" :width="80">
                <template #cell="{ rowIndex }">
                  <a-button type="text" status="danger" size="small" @click="removeAttribute(rowIndex)">
                    <icon-delete />
                  </a-button>
                </template>
              </a-table-column>
            </template>
          </a-table>
        </div>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
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
  values: { value: string; sort: number }[];
  newValue: string;
}

const loading = ref(false);
const tableData = ref<AttributeTemplateListItem[]>([]);
const categoryTree = ref<any[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增属性模板');
const formRef = ref();
const selectedIds = ref<number[]>([]);

const searchForm = reactive({
  name: '',
  categoryId: undefined as number | undefined,
  status: '',
});

const form = reactive({
  id: 0,
  name: '',
  categoryId: 0,
  sort: 0,
  status: '0',
  attributes: [] as AttributeFormItem[],
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60 },
  {
    title: '模板名称',
    dataIndex: 'name',
    width: 200,
  },
  {
    title: '关联分类',
    dataIndex: 'categoryName',
    width: 150,
  },
  {
    title: '属性数量',
    dataIndex: 'attributeCount',
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
    const res = await attributeTemplateApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      categoryId: searchForm.categoryId,
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

const getCategoryTree = async () => {
  try {
    const res = await categoryApi.tree();
    if (res.code === 200) {
      categoryTree.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchForm.name = '';
  searchForm.categoryId = undefined;
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

const resetForm = () => {
  form.id = 0;
  form.name = '';
  form.categoryId = 0;
  form.sort = 0;
  form.status = '0';
  form.attributes = [];
};

const onAdd = () => {
  modalTitle.value = '新增属性模板';
  resetForm();
  modalVisible.value = true;
};

const onEdit = async (record: AttributeTemplateListItem) => {
  modalTitle.value = '编辑属性模板';
  try {
    const res = await attributeTemplateApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
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
        values: (a.values || []).map((v: any) => ({ value: v.value, sort: v.sort })),
        newValue: '',
      }));
      modalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: AttributeTemplateListItem) => {
  try {
    const res = await attributeTemplateApi.delete([record.id]);
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
    const res = await attributeTemplateApi.delete(selectedIds.value);
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

const onUpdateStatus = async (record: AttributeTemplateListItem) => {
  const newStatus = record.status === '0' ? '1' : '0';
  try {
    const res = await attributeTemplateApi.edit({
      id: record.id,
      name: record.name,
      categoryId: record.categoryId,
      sort: record.sort,
      status: newStatus,
    });
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

const addAttribute = () => {
  form.attributes.push({
    name: '',
    attrType: 1,
    isRequired: 0,
    isFilter: 0,
    sort: form.attributes.length,
    status: '0',
    values: [],
    newValue: '',
  });
};

const removeAttribute = (index: number) => {
  form.attributes.splice(index, 1);
};

const addAttributeValue = (record: AttributeFormItem) => {
  if (record.newValue && record.newValue.trim()) {
    record.values.push({
      value: record.newValue.trim(),
      sort: record.values.length,
    });
    record.newValue = '';
  }
};

const removeAttributeValue = (record: AttributeFormItem, index: number) => {
  record.values.splice(index, 1);
};

const onSubmit = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  const submitData = {
    ...form,
    attributes: form.attributes.map((a) => ({
      name: a.name,
      attrType: a.attrType,
      isRequired: a.isRequired,
      isFilter: a.isFilter,
      sort: a.sort,
      status: a.status,
      values: a.values,
    })),
  };

  try {
    let res;
    if (form.id) {
      res = await attributeTemplateApi.edit(submitData);
    } else {
      res = await attributeTemplateApi.add(submitData);
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
  getCategoryTree();
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

.attribute-list {
  .attribute-header {
    margin-bottom: 8px;
  }
}

.attr-values {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
  align-items: center;
}
</style>

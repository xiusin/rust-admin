<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="分类名称">
              <a-input v-model="searchForm.name" placeholder="请输入分类名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="分类状态">
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
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getTree"><icon-refresh size="18" /></div>
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
        :pagination="false"
      >
        <template #name="{ record }">
          <span :style="{ paddingLeft: `${(record.level - 1) * 20}px` }">
            <icon-drive-file v-if="record.children && record.children.length > 0" />
            <icon-file v-else />
            {{ record.name }}
          </span>
        </template>
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status === '0'">正常</a-tag>
          <a-tag bordered size="small" color="red" v-else>停用</a-tag>
        </template>
        <template #showInNav="{ record }">
          <a-tag bordered size="small" color="green" v-if="record.showInNav === 1">是</a-tag>
          <a-tag bordered size="small" color="gray" v-else>否</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onAddChild(record)" v-if="record.level < 4">新增下级</a-button>
            <a-popconfirm type="warning" content="确定删除该分类吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="600" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-form-item field="name" label="分类名称" :rules="[{ required: true, message: '请输入分类名称' }]">
          <a-input v-model="form.name" placeholder="请输入分类名称" />
        </a-form-item>
        <a-form-item field="parentId" label="上级分类">
          <a-tree-select
            v-model="form.parentId"
            :data="treeSelectData"
            placeholder="请选择上级分类"
            allow-clear
          />
        </a-form-item>
        <a-form-item field="icon" label="分类图标">
          <a-input v-model="form.icon" placeholder="请输入图标URL" />
        </a-form-item>
        <a-form-item field="image" label="分类图片">
          <a-input v-model="form.image" placeholder="请输入图片URL" />
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
        <a-form-item field="showInNav" label="显示在导航">
          <a-radio-group v-model="form.showInNav">
            <a-radio :value="1">是</a-radio>
            <a-radio :value="0">否</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { categoryApi, CategoryTreeItem } from '@/api/modules/product/category';

const loading = ref(false);
const tableData = ref<CategoryTreeItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增分类');
const formRef = ref();

const searchForm = reactive({
  name: '',
  status: '',
});

const form = reactive({
  id: 0,
  name: '',
  parentId: 0,
  icon: '',
  image: '',
  sort: 0,
  status: '0',
  showInNav: 0,
});

const columns = [
  {
    title: '分类名称',
    dataIndex: 'name',
    slotName: 'name',
    width: 300,
  },
  {
    title: '图标',
    dataIndex: 'icon',
    width: 100,
  },
  {
    title: '排序',
    dataIndex: 'sort',
    width: 80,
  },
  {
    title: '层级',
    dataIndex: 'level',
    width: 80,
  },
  {
    title: '状态',
    dataIndex: 'status',
    slotName: 'status',
    width: 100,
  },
  {
    title: '显示在导航',
    dataIndex: 'showInNav',
    slotName: 'showInNav',
    width: 120,
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

const treeSelectData = ref<any[]>([]);

const getTree = async () => {
  loading.value = true;
  try {
    const res = await categoryApi.tree({ status: searchForm.status || undefined });
    if (res.code === 200) {
      tableData.value = res.data || [];
      treeSelectData.value = buildTreeSelectData(res.data || []);
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const buildTreeSelectData = (data: CategoryTreeItem[]): any[] => {
  return data.map(item => ({
    key: item.id,
    title: item.name,
    value: item.id,
    children: item.children ? buildTreeSelectData(item.children) : undefined,
  }));
};

const search = () => {
  getTree();
};

const reset = () => {
  searchForm.name = '';
  searchForm.status = '';
  getTree();
};

const onAdd = () => {
  modalTitle.value = '新增分类';
  form.id = 0;
  form.name = '';
  form.parentId = 0;
  form.icon = '';
  form.image = '';
  form.sort = 0;
  form.status = '0';
  form.showInNav = 0;
  modalVisible.value = true;
};

const onEdit = (record: CategoryTreeItem) => {
  modalTitle.value = '编辑分类';
  form.id = record.id;
  form.name = record.name;
  form.parentId = record.parentId;
  form.icon = record.icon || '';
  form.image = record.image || '';
  form.sort = record.sort;
  form.status = record.status;
  form.showInNav = record.showInNav;
  modalVisible.value = true;
};

const onAddChild = (record: CategoryTreeItem) => {
  modalTitle.value = '新增下级分类';
  form.id = 0;
  form.name = '';
  form.parentId = record.id;
  form.icon = '';
  form.image = '';
  form.sort = 0;
  form.status = '0';
  form.showInNav = 0;
  modalVisible.value = true;
};

const onDelete = async (record: CategoryTreeItem) => {
  try {
    const res = await categoryApi.delete([record.id]);
    if (res.code === 200) {
      Message.success('删除成功');
      getTree();
    } else {
      Message.error(res.message || '删除失败');
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
      res = await categoryApi.edit({
        id: form.id,
        name: form.name,
        icon: form.icon || undefined,
        image: form.image || undefined,
        sort: form.sort,
        status: form.status,
        showInNav: form.showInNav,
      });
    } else {
      res = await categoryApi.add({
        name: form.name,
        parentId: form.parentId || undefined,
        icon: form.icon || undefined,
        image: form.image || undefined,
        sort: form.sort,
        status: form.status,
        showInNav: form.showInNav,
      });
    }

    if (res.code === 200) {
      Message.success(form.id ? '编辑成功' : '新增成功');
      modalVisible.value = false;
      getTree();
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
  getTree();
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

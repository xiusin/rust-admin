<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="小程序名称">
              <a-input v-model="formData.form.name" placeholder="请输入小程序名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="app_id" label="AppID">
              <a-input v-model="formData.form.app_id" placeholder="请输入AppID" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" @click="loadData">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button @click="onReset">
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
            <a-button type="primary" @click="handleAdd">
              <template #icon><icon-plus /></template>
              新增小程序
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="loadData"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>
      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :columns="columns"
        :data="tableData"
        :pagination="false"
      >
        <template #status="{ record }">
          <a-switch :model-value="record.status === 1" @change="handleStatusChange(record)" />
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
            <a-button type="text" size="small" status="danger" @click="handleDelete(record)">删除</a-button>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false">
      <a-form :model="form" layout="vertical">
        <a-form-item label="AppID" required>
          <a-input v-model="form.app_id" placeholder="请输入小程序AppID" />
        </a-form-item>
        <a-form-item label="AppSecret" required>
          <a-input-password v-model="form.app_secret" placeholder="请输入小程序AppSecret" />
        </a-form-item>
        <a-form-item label="小程序名称" required>
          <a-input v-model="form.name" placeholder="请输入小程序名称" />
        </a-form-item>
        <a-form-item label="原始ID">
          <a-input v-model="form.original_id" placeholder="请输入原始ID" />
        </a-form-item>
        <a-form-item label="状态">
          <a-radio-group v-model="form.status">
            <a-radio :value="1">启用</a-radio>
            <a-radio :value="0">停用</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import axios from '@/api';

interface MiniRecord {
  id: number;
  app_id: string;
  name: string;
  app_secret: string;
  original_id?: string;
  status: number;
  remark?: string;
  created_at: string;
}

const formData = reactive({
  form: {
    name: '',
    app_id: '',
  },
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<MiniRecord[]>([]);

const columns = [
  { title: 'AppID', dataIndex: 'app_id', width: 180 },
  { title: '小程序名称', dataIndex: 'name', width: 200 },
  { title: '原始ID', dataIndex: 'original_id', width: 180 },
  { title: '状态', dataIndex: 'status', slotName: 'status', width: 100 },
  { title: '创建时间', dataIndex: 'created_at', width: 160 },
  { title: '操作', slotName: 'optional', align: 'center', width: 180 },
];

const modalVisible = ref(false);
const modalTitle = ref('新增小程序');
const form = reactive({
  id: 0,
  app_id: '',
  app_secret: '',
  name: '',
  original_id: '',
  status: 1,
});

const loadData = async () => {
  loading.value = true;
  try {
    const { data } = await axios.get('/wechat-mini/list');
    if (data.message === 'success') {
      tableData.value = data.data || [];
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleAdd = () => {
  modalTitle.value = '新增小程序';
  Object.assign(form, { id: 0, app_id: '', app_secret: '', name: '', original_id: '', status: 1 });
  modalVisible.value = true;
};

const handleEdit = (record: MiniRecord) => {
  modalTitle.value = '编辑小程序';
  Object.assign(form, record);
  modalVisible.value = true;
};

const handleDelete = (record: MiniRecord) => {
  Modal.confirm({
    title: '确认删除',
    content: `确定要删除小程序"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete('/wechat-mini/del', { params: { id: record.id } });
        Message.success('删除成功');
        loadData();
      } catch (e) {
        console.error(e);
      }
    },
  });
};

const handleStatusChange = async (record: MiniRecord) => {
  try {
    await axios.put('/wechat-mini/toggle', null, { params: { id: record.id } });
    Message.success(`${record.name} 状态已切换`);
    loadData();
  } catch (e) {
    console.error(e);
  }
};

const handleSubmit = async () => {
  if (!form.app_id || !form.app_secret || !form.name) {
    Message.warning('请填写完整信息');
    return;
  }
  try {
    const submitData = {
      name: form.name,
      app_id: form.app_id,
      app_secret: form.app_secret,
      original_id: form.original_id || null,
      status: form.status,
    };

    if (form.id) {
      await axios.put('/wechat-mini/edit', { ...submitData, id: form.id });
      Message.success('编辑成功');
    } else {
      await axios.post('/wechat-mini/add', submitData);
      Message.success('新增成功');
    }
    modalVisible.value = false;
    loadData();
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style lang="scss" scoped>
.search-btn {
  margin-bottom: 20px;
}
.action-icon {
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 4px;
  &:hover {
    background-color: var(--color-fill-2);
  }
}
</style>

<template>
  <div class="page-container">
    <a-card>
      <template #title>物流公司</template>
      <template #extra>
        <a-button type="primary" @click="handleAdd">
          <template #icon><icon-plus /></template>
          新增公司
        </a-button>
      </template>
      <a-table :data="tableData" :loading="loading" :pagination="false">
        <template #columns>
          <a-table-column title="公司编码" data-index="code" :width="120" />
          <a-table-column title="公司名称" data-index="name" :width="180" />
          <a-table-column title="Logo" :width="80">
            <template #cell="{ record }">
              <a-image v-if="record.logo" :src="record.logo" :width="40" :height="40" fit="cover" />
              <span v-else>-</span>
            </template>
          </a-table-column>
          <a-table-column title="状态" :width="100">
            <template #cell="{ record }">
              <a-switch :model-value="record.status === 1" @change="handleStatusChange(record)" />
            </template>
          </a-table-column>
          <a-table-column title="排序" data-index="sort" :width="80" />
          <a-table-column title="创建时间" data-index="created_at" :width="160" />
          <a-table-column title="操作" :width="150" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
                <a-button type="text" size="small" status="danger" @click="handleDelete(record)">删除</a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false">
      <a-form :model="form" layout="vertical">
        <a-form-item label="公司编码" required>
          <a-input v-model="form.code" placeholder="如：SF、YTO、ZTO" />
        </a-form-item>
        <a-form-item label="公司名称" required>
          <a-input v-model="form.name" placeholder="请输入公司全称" />
        </a-form-item>
        <a-form-item label="Logo URL">
          <a-input v-model="form.logo" placeholder="请输入Logo URL" />
        </a-form-item>
        <a-form-item label="排序">
          <a-input-number v-model="form.sort" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="是否启用">
          <a-switch v-model="form.is_active" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message, Modal } from '@arco-design/web-vue';
import axios from '@/api';

interface CompanyRecord {
  id: number;
  code: string;
  name: string;
  logo: string;
  status: number;
  sort: number;
  created_at: string;
}

const loading = ref(false);
const tableData = ref<CompanyRecord[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增公司');
const form = reactive({
  id: 0,
  code: '',
  name: '',
  logo: '',
  sort: 0,
  is_active: true,
});

const loadData = async () => {
  loading.value = true;
  try {
    const { data } = await axios.get('/logistics-company/list');
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
  modalTitle.value = '新增公司';
  Object.assign(form, { id: 0, code: '', name: '', logo: '', sort: 0, is_active: true });
  modalVisible.value = true;
};

const handleEdit = (record: CompanyRecord) => {
  modalTitle.value = '编辑公司';
  Object.assign(form, {
    ...record,
    is_active: record.status === 1,
  });
  modalVisible.value = true;
};

const handleDelete = (record: CompanyRecord) => {
  Modal.confirm({
    title: '确认删除',
    content: `确定要删除物流公司"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete('/logistics-company/del', { params: { id: record.id } });
        Message.success('删除成功');
        loadData();
      } catch (e) {
        console.error(e);
      }
    },
  });
};

const handleStatusChange = async (record: CompanyRecord) => {
  try {
    await axios.put('/logistics-company/toggle', null, { params: { id: record.id } });
    Message.success(`${record.name} 状态已切换`);
    loadData();
  } catch (e) {
    console.error(e);
  }
};

const handleSubmit = async () => {
  if (!form.code || !form.name) {
    Message.warning('请填写完整信息');
    return;
  }
  try {
    const submitData = {
      code: form.code,
      name: form.name,
      logo: form.logo || null,
      sort: form.sort,
      status: form.is_active ? 1 : 0,
    };

    if (form.id) {
      await axios.put('/logistics-company/edit', { ...submitData, id: form.id });
      Message.success('编辑成功');
    } else {
      await axios.post('/logistics-company/add', submitData);
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

<style scoped>
.page-container {
  padding: 20px;
}
</style>

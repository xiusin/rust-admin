<template>
  <div class="freight-container">
    <a-card>
      <template #title>运费模板</template>
      <template #extra>
        <a-space>
          <a-button type="primary" @click="handleCreate">
            <template #icon><icon-plus /></template>
            新建模板
          </a-button>
        </a-space>
      </template>

      <a-table :data="freightList" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="ID" data-index="id" :width="80" />
          <a-table-column title="模板名称" data-index="name" :width="200" />
          <a-table-column title="计费方式" data-index="calc_type" :width="120">
            <template #cell="{ record }">
              <a-tag :color="record.calc_type === 'weight' ? 'blue' : 'green'">
                {{ record.calc_type === 'weight' ? '按重量' : '按件数' }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="首重/首件" data-index="first_unit" :width="100" />
          <a-table-column title="首费(元)" data-index="first_fee" :width="100" />
          <a-table-column title="续重/续件" data-index="continue_unit" :width="100" />
          <a-table-column title="续费(元)" data-index="continue_fee" :width="100" />
          <a-table-column title="状态" data-index="status" :width="80">
            <template #cell="{ record }">
              <a-tag :color="record.status === '1' ? 'green' : 'red'">
                {{ record.status === '1' ? '启用' : '禁用' }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="创建时间" data-index="created_at" :width="180" />
          <a-table-column title="操作" :width="150" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
                <a-popconfirm content="确定删除该模板吗？" @ok="handleDelete(record)">
                  <a-button type="text" size="small" status="danger">删除</a-button>
                </a-popconfirm>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="formVisible" :title="form.id ? '编辑模板' : '新建模板'" @ok="handleSubmit" @cancel="formVisible = false">
      <a-form :model="form" layout="vertical">
        <a-form-item label="模板名称" required>
          <a-input v-model="form.name" placeholder="请输入模板名称" />
        </a-form-item>
        <a-form-item label="计费方式">
          <a-radio-group v-model="form.calc_type">
            <a-radio value="weight">按重量</a-radio>
            <a-radio value="piece">按件数</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item label="首重/首件">
          <a-input-number v-model="form.first_unit" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="首费(元)">
          <a-input-number v-model="form.first_fee" :min="0" :precision="2" style="width: 100%" />
        </a-form-item>
        <a-form-item label="续重/续件">
          <a-input-number v-model="form.continue_unit" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="续费(元)">
          <a-input-number v-model="form.continue_fee" :min="0" :precision="2" style="width: 100%" />
        </a-form-item>
        <a-form-item label="状态">
          <a-radio-group v-model="form.status">
            <a-radio value="1">启用</a-radio>
            <a-radio value="0">禁用</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';

interface FreightTemplate {
  id: number;
  name: string;
  calc_type: string;
  first_unit: number;
  first_fee: number;
  continue_unit: number;
  continue_fee: number;
  status: string;
  created_at: string;
}

const loading = ref(false);
const freightList = ref<FreightTemplate[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const formVisible = ref(false);
const form = reactive({
  id: 0,
  name: '',
  calc_type: 'weight',
  first_unit: 1,
  first_fee: 0,
  continue_unit: 1,
  continue_fee: 0,
  status: '1',
});

const loadData = async () => {
  loading.value = true;
  try {
    // TODO: 调用实际API
    freightList.value = [];
    pagination.value.total = 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleCreate = () => {
  Object.assign(form, {
    id: 0,
    name: '',
    calc_type: 'weight',
    first_unit: 1,
    first_fee: 0,
    continue_unit: 1,
    continue_fee: 0,
    status: '1',
  });
  formVisible.value = true;
};

const handleEdit = (record: FreightTemplate) => {
  Object.assign(form, record);
  formVisible.value = true;
};

const handleSubmit = async () => {
  if (!form.name) {
    Message.warning('请输入模板名称');
    return;
  }
  try {
    // TODO: 调用实际API
    Message.success('保存成功');
    formVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleDelete = async (record: FreightTemplate) => {
  try {
    // TODO: 调用实际API
    Message.success('删除成功');
    loadData();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.freight-container {
  padding: 20px;
}
</style>

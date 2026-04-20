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
          <a-table-column title="计费方式" data-index="calculation_type" :width="120">
            <template #cell="{ record }">
              <a-tag :color="record.calculation_type === 'by_weight' ? 'blue' : 'green'">
                {{ record.calculation_type === "by_weight" ? "按重量" : "按距离" }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="首重/首件" data-index="first_weight" :width="100" />
          <a-table-column title="首费(元)" data-index="first_price" :width="100" />
          <a-table-column title="续重/续件" data-index="additional_weight" :width="100" />
          <a-table-column title="续费(元)" data-index="additional_price" :width="100" />
          <a-table-column title="状态" data-index="status" :width="80">
            <template #cell="{ record }">
              <a-tag :color="record.status === 'active' ? 'green' : 'red'">
                {{ record.status === "active" ? "启用" : "禁用" }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="创建时间" data-index="created_at" :width="180">
            <template #cell="{ record }">
              {{ record.created_at?.replace('T', ' ').substring(0, 19) }}
            </template>
          </a-table-column>
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

    <a-modal
      v-model:visible="formVisible"
      :title="form.id ? '编辑模板' : '新建模板'"
      @ok="handleSubmit"
      @cancel="formVisible = false"
    >
      <a-form :model="form" layout="vertical">
        <a-form-item label="模板名称" required>
          <a-input v-model="form.name" placeholder="请输入模板名称" />
        </a-form-item>
        <a-form-item label="计费方式">
          <a-radio-group v-model="form.calculation_type">
            <a-radio value="by_weight">按重量</a-radio>
            <a-radio value="by_distance">按距离</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item label="首重/首件">
          <a-input-number v-model="form.first_weight" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="首费(元)">
          <a-input-number v-model="form.first_price" :min="0" :precision="2" style="width: 100%" />
        </a-form-item>
        <a-form-item label="续重/续件">
          <a-input-number v-model="form.additional_weight" :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item label="续费(元)">
          <a-input-number v-model="form.additional_price" :min="0" :precision="2" style="width: 100%" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import { freightApi, FreightTemplate, CreateTemplateParams } from "@/api/modules/consumer/freight";

const loading = ref(false);
const freightList = ref<FreightTemplate[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const formVisible = ref(false);
const form = reactive<{
  id: number;
  name: string;
  calculation_type: string;
  first_weight: number;
  first_price: number;
  additional_weight: number;
  additional_price: number;
}>({
  id: 0,
  name: "",
  calculation_type: "by_weight",
  first_weight: 1,
  first_price: 0,
  additional_weight: 1,
  additional_price: 0,
});

const loadData = async () => {
  loading.value = true;
  try {
    const res = await freightApi.listTemplates({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    });
    if (res.code === 200 || res.message === "success") {
      freightList.value = res.data.list || [];
      pagination.value.total = res.data.total || 0;
    }
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
    name: "",
    calculation_type: "by_weight",
    first_weight: 1,
    first_price: 0,
    additional_weight: 1,
    additional_price: 0,
  });
  formVisible.value = true;
};

const handleEdit = (record: FreightTemplate) => {
  Object.assign(form, {
    id: record.id,
    name: record.name,
    calculation_type: record.calculation_type,
    first_weight: Number(record.first_weight) || 0,
    first_price: Number(record.first_price) || 0,
    additional_weight: Number(record.additional_weight) || 0,
    additional_price: Number(record.additional_price) || 0,
  });
  formVisible.value = true;
};

const handleSubmit = async () => {
  if (!form.name) {
    Message.warning("请输入模板名称");
    return;
  }
  try {
    const params: CreateTemplateParams = {
      name: form.name,
      calculation_type: form.calculation_type,
      first_weight: form.first_weight,
      first_price: form.first_price,
      additional_weight: form.additional_weight,
      additional_price: form.additional_price,
    };

    if (form.id) {
      await freightApi.updateTemplate({ ...params, id: form.id });
      Message.success("编辑成功");
    } else {
      await freightApi.createTemplate(params);
      Message.success("创建成功");
    }
    
    formVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleDelete = async (record: FreightTemplate) => {
  try {
    await freightApi.deleteTemplate(record.id);
    Message.success("删除成功");
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

<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="配置名称">
              <a-input v-model="formData.form.name" placeholder="请输入配置名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="provider" label="服务商">
              <a-select v-model="formData.form.provider" placeholder="请选择服务商" allow-clear>
                <a-option value="aliyun">阿里云</a-option>
                <a-option value="tencent">腾讯云</a-option>
                <a-option value="huawei">华为云</a-option>
              </a-select>
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
              新增配置
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
        column-resizable
        :loading="loading"
        :size="'small'"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columns"
        :data="tableData"
        :pagination="false"
      >
        <template #provider="{ record }">
          <a-tag>{{ getProviderName(record.provider) }}</a-tag>
        </template>
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
        <a-form-item label="配置名称" required>
          <a-input v-model="form.name" placeholder="请输入配置名称" />
        </a-form-item>
        <a-form-item label="服务商" required>
          <a-select v-model="form.provider" placeholder="请选择服务商">
            <a-option value="aliyun">阿里云</a-option>
            <a-option value="tencent">腾讯云</a-option>
            <a-option value="huawei">华为云</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="AccessKey ID" required>
          <a-input v-model="form.access_key" placeholder="请输入AccessKey ID" />
        </a-form-item>
        <a-form-item label="AccessKey Secret" required>
          <a-input-password v-model="form.access_secret" placeholder="请输入AccessKey Secret" />
        </a-form-item>
        <a-form-item label="短信签名" required>
          <a-input v-model="form.sign_name" placeholder="请输入短信签名" />
        </a-form-item>
        <a-form-item label="区域">
          <a-input v-model="form.region" placeholder="如：cn-hangzhou" />
        </a-form-item>
        <a-form-item label="是否启用">
          <a-switch v-model="form.is_active" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message, Modal } from "@arco-design/web-vue";
import axios from "@/api";

interface ConfigRecord {
  id: number;
  name: string;
  provider: string;
  access_key: string;
  access_secret: string;
  sign_name: string;
  region: string;
  is_default: number;
  status: number;
  remark: string;
  created_at: string;
}

const formData = reactive({
  form: {
    name: "",
    provider: null as string | null
  }
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<ConfigRecord[]>([]);

const columns = [
  { title: "配置名称", dataIndex: "name", width: 200 },
  { title: "服务商", dataIndex: "provider", slotName: "provider", width: 150 },
  { title: "AccessKey", dataIndex: "access_key", width: 200, ellipsis: true },
  { title: "签名", dataIndex: "sign_name", width: 150 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 100 },
  { title: "创建时间", dataIndex: "created_at", width: 160 },
  { title: "操作", slotName: "optional", align: "center", width: 150 }
];

const getProviderName = (provider: string) => {
  const names: Record<string, string> = {
    aliyun: "阿里云",
    tencent: "腾讯云",
    huawei: "华为云"
  };
  return names[provider] || provider;
};

const modalVisible = ref(false);
const modalTitle = ref("新增配置");
const form = reactive({
  id: 0,
  name: "",
  provider: "aliyun",
  access_key: "",
  access_secret: "",
  sign_name: "",
  region: "",
  is_active: true
});

const loadData = async () => {
  loading.value = true;
  try {
    const { data } = await axios.get("/sms-config/list");
    if (data.message === "success") {
      tableData.value = data.data || [];
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleAdd = () => {
  modalTitle.value = "新增配置";
  Object.assign(form, {
    id: 0,
    name: "",
    provider: "aliyun",
    access_key: "",
    access_secret: "",
    sign_name: "",
    region: "",
    is_active: true
  });
  modalVisible.value = true;
};

const handleEdit = (record: ConfigRecord) => {
  modalTitle.value = "编辑配置";
  Object.assign(form, {
    ...record,
    is_active: record.status === 1
  });
  modalVisible.value = true;
};

const handleDelete = (record: ConfigRecord) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除配置"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete("/sms-config/del", { params: { id: record.id } });
        Message.success("删除成功");
        loadData();
      } catch (e) {
        console.error(e);
      }
    }
  });
};

const handleStatusChange = async (record: ConfigRecord) => {
  try {
    await axios.put("/sms-config/toggle", null, { params: { id: record.id } });
    Message.success(`${record.name} 状态已切换`);
    loadData();
  } catch (e) {
    console.error(e);
  }
};

const handleSubmit = async () => {
  if (!form.name || !form.provider || !form.access_key || !form.access_secret || !form.sign_name) {
    Message.warning("请填写完整信息");
    return;
  }
  try {
    const submitData = {
      name: form.name,
      provider: form.provider,
      access_key: form.access_key,
      access_secret: form.access_secret,
      sign_name: form.sign_name,
      region: form.region || null,
      status: form.is_active ? 1 : 0
    };

    if (form.id) {
      await axios.put("/sms-config/edit", { ...submitData, id: form.id });
      Message.success("编辑成功");
    } else {
      await axios.post("/sms-config/add", submitData);
      Message.success("新增成功");
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

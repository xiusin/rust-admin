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
            <a-form-item field="storage_type" label="存储类型">
              <a-select v-model="formData.form.storage_type" placeholder="请选择存储类型" allow-clear>
                <a-option value="aliyun">阿里云OSS</a-option>
                <a-option value="tencent">腾讯云COS</a-option>
                <a-option value="qiniu">七牛云</a-option>
                <a-option value="minio">MinIO</a-option>
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
        :loading="loading"
        :bordered="{ cell: true }"
        :columns="columns"
        :data="tableData"
        :pagination="false"
      >
        <template #storage_type="{ record }">
          <a-tag>{{ getStorageTypeName(record.storage_type) }}</a-tag>
        </template>
        <template #is_default="{ record }">
          <a-tag :color="record.is_default === 1 ? 'green' : 'gray'">
            {{ record.is_default === 1 ? "默认" : "否" }}
          </a-tag>
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

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false" :width="600">
      <a-form :model="form" layout="vertical">
        <a-form-item label="配置名称" required>
          <a-input v-model="form.name" placeholder="请输入配置名称" />
        </a-form-item>
        <a-form-item label="存储类型" required>
          <a-select v-model="form.storage_type" placeholder="请选择存储类型">
            <a-option value="aliyun">阿里云OSS</a-option>
            <a-option value="tencent">腾讯云COS</a-option>
            <a-option value="qiniu">七牛云</a-option>
            <a-option value="minio">MinIO</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="Endpoint" required>
          <a-input v-model="form.endpoint" placeholder="如：oss-cn-hangzhou.aliyuncs.com" />
        </a-form-item>
        <a-form-item label="Bucket" required>
          <a-input v-model="form.bucket" placeholder="请输入Bucket名称" />
        </a-form-item>
        <a-form-item label="AccessKey" required>
          <a-input v-model="form.access_key" placeholder="请输入AccessKey" />
        </a-form-item>
        <a-form-item label="SecretKey" required>
          <a-input-password v-model="form.secret_key" placeholder="请输入SecretKey" />
        </a-form-item>
        <a-form-item label="Region">
          <a-input v-model="form.region" placeholder="如：cn-hangzhou" />
        </a-form-item>
        <a-form-item label="自定义域名">
          <a-input v-model="form.domain" placeholder="如：https://cdn.example.com" />
        </a-form-item>
        <a-form-item label="设为默认">
          <a-switch v-model="form.is_default" />
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

interface OssRecord {
  id: number;
  name: string;
  storage_type: string;
  endpoint: string;
  bucket: string;
  access_key: string;
  secret_key: string;
  region?: string;
  domain?: string;
  is_default: number;
  status: number;
  remark?: string;
  created_at: string;
}

const formData = reactive({
  form: {
    name: "",
    storage_type: null as string | null
  }
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<OssRecord[]>([]);

const columns = [
  { title: "配置名称", dataIndex: "name", width: 150 },
  { title: "存储类型", dataIndex: "storage_type", slotName: "storage_type", width: 120 },
  { title: "Endpoint", dataIndex: "endpoint", width: 200, ellipsis: true },
  { title: "Bucket", dataIndex: "bucket", width: 150 },
  { title: "默认", dataIndex: "is_default", slotName: "is_default", width: 80 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 100 },
  { title: "创建时间", dataIndex: "created_at", width: 160 },
  { title: "操作", slotName: "optional", align: "center", width: 150 }
];

const getStorageTypeName = (type: string) => {
  const names: Record<string, string> = {
    aliyun: "阿里云OSS",
    tencent: "腾讯云COS",
    qiniu: "七牛云",
    minio: "MinIO"
  };
  return names[type] || type;
};

const modalVisible = ref(false);
const modalTitle = ref("新增配置");
const form = reactive({
  id: 0,
  name: "",
  storage_type: "aliyun",
  endpoint: "",
  bucket: "",
  access_key: "",
  secret_key: "",
  region: "",
  domain: "",
  is_default: false,
  is_active: true
});

const loadData = async () => {
  loading.value = true;
  try {
    const { data } = await axios.get("/oss-config/list");
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
    storage_type: "aliyun",
    endpoint: "",
    bucket: "",
    access_key: "",
    secret_key: "",
    region: "",
    domain: "",
    is_default: false,
    is_active: true
  });
  modalVisible.value = true;
};

const handleEdit = (record: OssRecord) => {
  modalTitle.value = "编辑配置";
  Object.assign(form, {
    ...record,
    is_default: record.is_default === 1,
    is_active: record.status === 1
  });
  modalVisible.value = true;
};

const handleDelete = (record: OssRecord) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除配置"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete("/oss-config/del", { params: { id: record.id } });
        Message.success("删除成功");
        loadData();
      } catch (e) {
        console.error(e);
      }
    }
  });
};

const handleStatusChange = async (record: OssRecord) => {
  try {
    await axios.put("/oss-config/toggle", null, { params: { id: record.id } });
    Message.success(`${record.name} 状态已切换`);
    loadData();
  } catch (e) {
    console.error(e);
  }
};

const handleSubmit = async () => {
  if (!form.name || !form.storage_type || !form.endpoint || !form.bucket || !form.access_key || !form.secret_key) {
    Message.warning("请填写完整信息");
    return;
  }
  try {
    const submitData = {
      name: form.name,
      storage_type: form.storage_type,
      endpoint: form.endpoint,
      bucket: form.bucket,
      access_key: form.access_key,
      secret_key: form.secret_key,
      region: form.region || null,
      domain: form.domain || null,
      is_default: form.is_default ? 1 : 0,
      status: form.is_active ? 1 : 0
    };

    if (form.id) {
      await axios.put("/oss-config/edit", { ...submitData, id: form.id });
      Message.success("编辑成功");
    } else {
      await axios.post("/oss-config/add", submitData);
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

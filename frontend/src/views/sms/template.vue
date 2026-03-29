<template>
  <div class="page-container">
    <a-card>
      <template #title>短信模板</template>
      <template #extra>
        <a-button type="primary" @click="handleAdd">
          <template #icon><icon-plus /></template>
          新增模板
        </a-button>
      </template>
      <a-table :data="tableData" :loading="loading" :pagination="false">
        <template #columns>
          <a-table-column title="模板编码" data-index="template_code" :width="150" />
          <a-table-column title="模板名称" data-index="name" :width="180" />
          <a-table-column title="模板内容" data-index="content" :width="300" ellipsis />
          <a-table-column title="模板类型" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getTypeColor(record.template_type)">{{ getTypeName(record.template_type) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="状态" :width="100">
            <template #cell="{ record }">
              <a-badge :status="record.status === 1 ? 'success' : 'default'" :text="record.status === 1 ? '启用' : '停用'" />
            </template>
          </a-table-column>
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

    <a-modal v-model:visible="modalVisible" :title="modalTitle" @ok="handleSubmit" @cancel="modalVisible = false" :width="600">
      <a-form :model="form" layout="vertical">
        <a-form-item label="模板编码" required>
          <a-input v-model="form.template_code" placeholder="如：SMS_001" />
        </a-form-item>
        <a-form-item label="模板名称" required>
          <a-input v-model="form.name" placeholder="请输入模板名称" />
        </a-form-item>
        <a-form-item label="模板类型" required>
          <a-select v-model="form.template_type" placeholder="请选择模板类型">
            <a-option value="verification">验证码</a-option>
            <a-option value="notification">通知</a-option>
            <a-option value="marketing">营销</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="模板内容" required>
          <a-textarea
            v-model="form.content"
            placeholder="请输入模板内容，变量用${变量名}表示"
            :auto-size="{ minRows: 4, maxRows: 8 }"
          />
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
import { ref, reactive, onMounted } from "vue";
import { Message, Modal } from "@arco-design/web-vue";
import axios from "@/api";

interface TemplateRecord {
  id: number;
  template_code: string;
  name: string;
  content: string;
  template_type: string;
  params: any;
  status: number;
  remark: string;
  created_at: string;
}

const loading = ref(false);
const tableData = ref<TemplateRecord[]>([]);

const modalVisible = ref(false);
const modalTitle = ref("新增模板");
const form = reactive({
  id: 0,
  template_code: "",
  name: "",
  template_type: "verification",
  content: "",
  params: null as any,
  status: 1
});

const getTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    verification: "blue",
    notification: "green",
    marketing: "orange"
  };
  return colors[type] || "gray";
};

const getTypeName = (type: string) => {
  const names: Record<string, string> = {
    verification: "验证码",
    notification: "通知",
    marketing: "营销"
  };
  return names[type] || type;
};

const loadData = async () => {
  loading.value = true;
  try {
    const { data } = await axios.get("/sms-template/list");
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
  modalTitle.value = "新增模板";
  Object.assign(form, {
    id: 0,
    template_code: "",
    name: "",
    template_type: "verification",
    content: "",
    params: null,
    status: 1
  });
  modalVisible.value = true;
};

const handleEdit = (record: TemplateRecord) => {
  modalTitle.value = "编辑模板";
  Object.assign(form, record);
  modalVisible.value = true;
};

const handleDelete = (record: TemplateRecord) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除模板"${record.name}"吗？`,
    onOk: async () => {
      try {
        await axios.delete("/sms-template/del", { params: { id: record.id } });
        Message.success("删除成功");
        loadData();
      } catch (e) {
        console.error(e);
      }
    }
  });
};

const handleSubmit = async () => {
  if (!form.template_code || !form.name || !form.template_type || !form.content) {
    Message.warning("请填写完整信息");
    return;
  }
  try {
    const submitData = {
      template_code: form.template_code,
      name: form.name,
      template_type: form.template_type,
      content: form.content,
      params: form.params,
      status: form.status
    };

    if (form.id) {
      await axios.put("/sms-template/edit", { ...submitData, id: form.id });
      Message.success("编辑成功");
    } else {
      await axios.post("/sms-template/add", submitData);
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

<style scoped>
.page-container {
  padding: 20px;
}
</style>

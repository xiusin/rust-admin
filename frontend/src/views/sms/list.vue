<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="formRef" :model="formData.form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="phone" label="手机号">
              <a-input v-model="formData.form.phone" placeholder="请输入手机号" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="sms_type" label="短信类型">
              <a-select v-model="formData.form.sms_type" placeholder="请选择类型" allow-clear>
                <a-option value="verification">验证码</a-option>
                <a-option value="notification">通知</a-option>
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
            <a-button type="primary" status="success" @click="handleSend">
              <template #icon><icon-send /></template>
              发送短信
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
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)">
            {{ getStatusName(record.status) }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="sendVisible" title="发送短信" @ok="submitSend" @cancel="sendVisible = false">
      <a-form :model="sendForm" layout="vertical">
        <a-form-item label="手机号" required>
          <a-input v-model="sendForm.phone" placeholder="请输入手机号" />
        </a-form-item>
        <a-form-item label="短信类型" required>
          <a-select v-model="sendForm.sms_type" placeholder="请选择短信类型">
            <a-option value="verification">验证码</a-option>
            <a-option value="notification">通知</a-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="detailVisible" title="短信详情" :footer="false" :width="600">
      <a-descriptions :column="1" bordered>
        <a-descriptions-item label="ID">{{ currentRecord.id }}</a-descriptions-item>
        <a-descriptions-item label="手机号">{{ currentRecord.phone }}</a-descriptions-item>
        <a-descriptions-item label="短信类型">{{ currentRecord.sms_type }}</a-descriptions-item>
        <a-descriptions-item label="短信内容">{{ currentRecord.content }}</a-descriptions-item>
        <a-descriptions-item label="验证码">{{ currentRecord.code }}</a-descriptions-item>
        <a-descriptions-item label="渠道">{{ currentRecord.channel }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="getStatusColor(currentRecord.status)">
            {{ getStatusName(currentRecord.status) }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="发送时间">{{ currentRecord.sent_at }}</a-descriptions-item>
        <a-descriptions-item label="过期时间">{{ currentRecord.expires_at }}</a-descriptions-item>
        <a-descriptions-item v-if="currentRecord.error_message" label="错误信息">
          <span class="text-danger">{{ currentRecord.error_message }}</span>
        </a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import axios from "axios";

interface SmsRecord {
  id: number;
  phone: string;
  sms_type: string;
  content: string | null;
  code: string | null;
  channel: string;
  status: string;
  error_message: string | null;
  sent_at: string | null;
  expires_at: string | null;
}

const formData = reactive({
  form: {
    phone: "",
    sms_type: null as string | null
  }
});

const formRef = ref();
const onReset = () => {
  formRef.value.resetFields();
  loadData();
};

const loading = ref(false);
const tableData = ref<SmsRecord[]>([]);

const pagination = ref({ showPageSize: true, showTotal: true, current: 1, pageSize: 10, total: 0 });
const pageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  loadData();
};

const columns = [
  { title: "ID", dataIndex: "id", width: 100 },
  { title: "手机号", dataIndex: "phone", width: 140 },
  { title: "短信类型", dataIndex: "sms_type", width: 120 },
  { title: "短信内容", dataIndex: "content", width: 250, ellipsis: true },
  { title: "验证码", dataIndex: "code", width: 100 },
  { title: "渠道", dataIndex: "channel", width: 100 },
  { title: "状态", slotName: "status", width: 100 },
  { title: "发送时间", dataIndex: "sent_at", width: 160 },
  { title: "操作", slotName: "optional", width: 100 }
];

const getStatusName = (status: string) => {
  const names: Record<string, string> = {
    sent: "已发送",
    verified: "已验证",
    failed: "发送失败",
    expired: "已过期"
  };
  return names[status] || status;
};

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    sent: "blue",
    verified: "green",
    failed: "red",
    expired: "gray"
  };
  return colors[status] || "arcoblue";
};

const sendVisible = ref(false);
const detailVisible = ref(false);
const currentRecord = ref<SmsRecord>({} as SmsRecord);
const sendForm = reactive({
  phone: "",
  sms_type: "verification"
});

const loadData = async () => {
  loading.value = true;
  try {
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (formData.form.phone) params.phone = formData.form.phone;
    if (formData.form.sms_type) params.sms_type = formData.form.sms_type;

    const { data } = await axios.get("/sms/logs", { params });
    if (data.message === "success") {
      tableData.value = data.data?.list || [];
      pagination.value.total = data.data?.total || 0;
    }
  } catch (e) {
    console.error(e);
    Message.error("加载失败");
  } finally {
    loading.value = false;
  }
};

const handleSend = () => {
  Object.assign(sendForm, { phone: "", sms_type: "verification" });
  sendVisible.value = true;
};

const submitSend = async () => {
  if (!sendForm.phone) {
    Message.warning("请输入手机号");
    return;
  }
  try {
    const { data } = await axios.post("/sms/send-code", {
      phone: sendForm.phone,
      sms_type: sendForm.sms_type
    });
    if (data.message === "success") {
      Message.success("短信发送成功");
      sendVisible.value = false;
      loadData();
    }
  } catch (e: any) {
    Message.error(e.response?.data?.message || "发送失败");
  }
};

const handleDetail = (record: SmsRecord) => {
  currentRecord.value = record;
  detailVisible.value = true;
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
.text-danger {
  color: #f53f3f;
}
</style>

<template>
  <div class="page-container">
    <a-row :gutter="16">
      <a-col :xs="24" :lg="12">
        <a-card title="许可证列表" class="license-card">
          <template #extra>
            <a-button type="primary" size="small" @click="loadData">
              <template #icon><icon-refresh /></template>
              刷新
            </a-button>
          </template>

          <a-table
            row-key="id"
            :loading="loading"
            :bordered="{ cell: true }"
            :columns="licenseColumns"
            :data="licenseData"
            :pagination="pagination"
            @page-change="handlePageChange"
          >
            <template #licenseKey="{ record }">
              <a-tooltip :content="record.licenseKey">
                <span class="license-key">{{ maskLicenseKey(record.licenseKey) }}</span>
              </a-tooltip>
              <a-button type="text" size="small" @click="copyLicenseKey(record.licenseKey)">
                <template #icon><icon-copy /></template>
              </a-button>
            </template>
            <template #status="{ record }">
              <a-badge :status="getStatusType(record.status)" :text="record.statusName" />
            </template>
            <template #optional="{ record }">
              <a-button type="text" size="small" @click="handleLicenseDetail(record)">详情</a-button>
            </template>
          </a-table>
        </a-card>
      </a-col>

      <a-col :xs="24" :lg="12">
        <a-card title="设备绑定列表" class="device-card">
          <template #extra>
            <a-button type="primary" size="small" @click="loadDevices">
              <template #icon><icon-refresh /></template>
              刷新
            </a-button>
          </template>

          <a-table
            v-if="selectedLicense"
            row-key="deviceId"
            :loading="deviceLoading"
            :bordered="{ cell: true }"
            :columns="deviceColumns"
            :data="deviceData"
            :pagination="devicePagination"
            @page-change="handleDevicePageChange"
          >
            <template #status="{ record }">
              <a-tag :color="getDeviceStatusColor(record.status)">
                {{ getDeviceStatusText(record.status) }}
              </a-tag>
            </template>
            <template #lastVerifyTime="{ record }">
              {{ record.lastVerifyTime || "-" }}
            </template>
            <template #optional="{ record }">
              <a-button v-if="record.status === 1" type="text" size="small" status="danger" @click="handleUnbindDevice(record)">
                解绑
              </a-button>
              <a-tag v-else-if="record.status === 0" color="gray">已离线</a-tag>
            </template>
          </a-table>

          <a-empty v-else description="请先选择一个许可证查看设备列表">
            <template #image>
              <icon-monitor size="48" color="var(--color-text-4)" />
            </template>
          </a-empty>
        </a-card>
      </a-col>
    </a-row>

    <a-modal v-model:visible="licenseDetailVisible" title="许可证详情" :footer="null" :width="700">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="许可证密钥" :span="2">
          <div class="license-key-display">
            <span>{{ currentLicense.licenseKey }}</span>
            <a-button type="text" size="small" @click="copyLicenseKey(currentLicense.licenseKey)">
              <template #icon><icon-copy /></template>
              复制
            </a-button>
          </div>
        </a-descriptions-item>
        <a-descriptions-item label="插件名称">{{ currentLicense.pluginName }}</a-descriptions-item>
        <a-descriptions-item label="订阅方案">{{ currentLicense.planName }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-badge :status="getStatusType(currentLicense.status)" :text="currentLicense.statusName" />
        </a-descriptions-item>
        <a-descriptions-item label="设备限额">
          {{ currentLicense.usedDevices || 0 }} / {{ currentLicense.maxDevices === -1 ? "无限制" : currentLicense.maxDevices }}
        </a-descriptions-item>
        <a-descriptions-item label="生效时间">{{ currentLicense.startTime || "-" }}</a-descriptions-item>
        <a-descriptions-item label="到期时间">
          <span :class="{ 'text-danger': currentLicense.isExpired }">
            {{ currentLicense.endTime || "永久有效" }}
          </span>
        </a-descriptions-item>
        <a-descriptions-item label="剩余天数">
          <span :class="{ 'text-danger': currentLicense.isExpired }"> {{ currentLicense.daysRemaining || 0 }} 天 </span>
        </a-descriptions-item>
        <a-descriptions-item label="验证次数">{{ currentLicense.verifyCount || 0 }}</a-descriptions-item>
      </a-descriptions>

      <a-divider>设备使用情况</a-divider>

      <a-progress
        v-if="currentLicense.maxDevices !== -1"
        :percent="currentLicense.maxDevices > 0 ? ((currentLicense.usedDevices || 0) / currentLicense.maxDevices) * 100 : 0"
        :format="percent => `${currentLicense.usedDevices || 0} / ${currentLicense.maxDevices}`"
        :color="getUsageColor(currentLicense.usedDevices || 0, currentLicense.maxDevices)"
      />

      <a-row v-if="currentLicense.maxDevices === -1" :gutter="16" style="margin-top: 16px">
        <a-col :span="8">
          <a-statistic title="已使用设备" :value="currentLicense.usedDevices || 0" />
        </a-col>
        <a-col :span="8">
          <a-statistic title="状态" value="无限制" />
        </a-col>
        <a-col :span="8">
          <a-statistic title="验证次数" :value="currentLicense.verifyCount || 0" />
        </a-col>
      </a-row>

      <a-divider />
      <a-space style="display: flex; justify-content: flex-end">
        <a-button type="primary" @click="handleBindDevice">
          <template #icon><icon-plus /></template>
          绑定设备
        </a-button>
      </a-space>
    </a-modal>

    <a-modal v-model:visible="bindDeviceVisible" title="绑定设备" @ok="submitBindDevice" @cancel="bindDeviceVisible = false">
      <a-form :model="bindForm" layout="vertical">
        <a-form-item label="许可证">
          <a-select v-model="bindForm.licenseId" placeholder="请选择许可证" @change="handleLicenseSelectChange">
            <a-option v-for="item in licenseData" :key="item.id" :value="item.id">
              {{ item.pluginName }} - {{ item.planName }}
            </a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="设备ID" required>
          <a-input v-model="bindForm.deviceId" placeholder="请输入设备ID" />
        </a-form-item>
        <a-form-item label="设备名称">
          <a-input v-model="bindForm.deviceName" placeholder="请输入设备名称" />
        </a-form-item>
        <a-form-item label="设备类型">
          <a-select v-model="bindForm.deviceType" placeholder="请选择设备类型">
            <a-option value="windows">Windows</a-option>
            <a-option value="macos">macOS</a-option>
            <a-option value="linux">Linux</a-option>
            <a-option value="ios">iOS</a-option>
            <a-option value="android">Android</a-option>
            <a-option value="web">Web</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="操作系统版本">
          <a-input v-model="bindForm.osVersion" placeholder="如：Windows 10 21H2" />
        </a-form-item>
        <a-form-item label="应用版本">
          <a-input v-model="bindForm.appVersion" placeholder="如：1.0.0" />
        </a-form-item>
        <a-form-item label="MAC地址">
          <a-input v-model="bindForm.macAddress" placeholder="如：00:1A:2B:3C:4D:5E" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message, Modal } from "@arco-design/web-vue";

interface LicenseRecord {
  id: number;
  licenseKey: string;
  pluginId: number;
  pluginName: string;
  planId: number;
  planName: string;
  status: number;
  statusName: string;
  startTime?: string;
  endTime?: string;
  isExpired: boolean;
  daysRemaining: number;
  verifyCount: number;
  maxDevices: number;
  usedDevices: number;
}

interface DeviceRecord {
  id: number;
  deviceId: string;
  deviceName: string;
  deviceType: string;
  osVersion?: string;
  appVersion?: string;
  macAddress?: string;
  ipAddress?: string;
  status: number;
  lastVerifyTime?: string;
  bindTime: string;
}

const loading = ref(false);
const deviceLoading = ref(false);
const licenseData = ref<LicenseRecord[]>([]);
const deviceData = ref<DeviceRecord[]>([]);
const selectedLicense = ref<LicenseRecord | null>(null);
const licenseDetailVisible = ref(false);
const bindDeviceVisible = ref(false);

const currentLicense = ref<LicenseRecord>({} as LicenseRecord);

const pagination = ref({ current: 1, pageSize: 10, total: 0 });
const devicePagination = ref({ current: 1, pageSize: 10, total: 0 });

const licenseColumns = [
  { title: "许可证密钥", slotName: "licenseKey", width: 220 },
  { title: "插件", dataIndex: "pluginName", width: 140 },
  { title: "方案", dataIndex: "planName", width: 100 },
  { title: "状态", slotName: "status", width: 80 },
  { title: "到期时间", dataIndex: "endTime", width: 160 },
  { title: "操作", slotName: "optional", width: 80 }
];

const deviceColumns = [
  { title: "设备ID", dataIndex: "deviceId", width: 120 },
  { title: "设备名称", dataIndex: "deviceName", width: 120 },
  { title: "设备类型", dataIndex: "deviceType", width: 90 },
  { title: "状态", slotName: "status", width: 80 },
  { title: "最后验证", slotName: "lastVerifyTime", width: 160 },
  { title: "绑定时间", dataIndex: "bindTime", width: 160 },
  { title: "操作", slotName: "optional", width: 80 }
];

const bindForm = reactive({
  licenseId: 0,
  deviceId: "",
  deviceName: "",
  deviceType: "",
  osVersion: "",
  appVersion: "",
  macAddress: ""
});

const mockLicenseData: LicenseRecord[] = [
  {
    id: 1,
    licenseKey: "550e8400-e29b-41d4-a716-446655440001",
    pluginId: 1,
    pluginName: "智能优惠券",
    planId: 2,
    planName: "专业版",
    status: 1,
    statusName: "启用",
    startTime: "2024-03-20 10:35:00",
    endTime: "2025-03-20 10:35:00",
    isExpired: false,
    daysRemaining: 361,
    verifyCount: 156,
    maxDevices: 5,
    usedDevices: 2
  },
  {
    id: 2,
    licenseKey: "550e8400-e29b-41d4-a716-446655440002",
    pluginId: 2,
    pluginName: "限时秒杀",
    planId: 1,
    planName: "基础版",
    status: 1,
    statusName: "启用",
    startTime: "2024-02-15 14:20:00",
    endTime: "2025-02-15 14:20:00",
    isExpired: false,
    daysRemaining: 329,
    verifyCount: 89,
    maxDevices: 1,
    usedDevices: 1
  },
  {
    id: 3,
    licenseKey: "550e8400-e29b-41d4-a716-446655440003",
    pluginId: 3,
    pluginName: "数据统计分析",
    planId: 3,
    planName: "企业版",
    status: 0,
    statusName: "禁用",
    startTime: "2024-01-10 09:00:00",
    endTime: "2024-01-10 09:00:00",
    isExpired: true,
    daysRemaining: 0,
    verifyCount: 23,
    maxDevices: -1,
    usedDevices: 0
  }
];

const mockDeviceData: DeviceRecord[] = [
  {
    id: 1,
    deviceId: "device-001",
    deviceName: "办公电脑",
    deviceType: "windows",
    osVersion: "Windows 10 21H2",
    appVersion: "2.1.0",
    macAddress: "00:1A:2B:3C:4D:5E",
    ipAddress: "192.168.1.100",
    status: 1,
    lastVerifyTime: "2025-03-24 10:30:00",
    bindTime: "2024-03-20 10:40:00"
  },
  {
    id: 2,
    deviceId: "device-002",
    deviceName: "MacBook Pro",
    deviceType: "macos",
    osVersion: "macOS Sonoma 14.0",
    appVersion: "2.1.0",
    macAddress: "A1:B2:C3:D4:E5:F6",
    ipAddress: "192.168.1.101",
    status: 1,
    lastVerifyTime: "2025-03-24 09:15:00",
    bindTime: "2024-03-21 14:20:00"
  },
  {
    id: 3,
    deviceId: "device-003",
    deviceName: "备用服务器",
    deviceType: "linux",
    osVersion: "Ubuntu 22.04 LTS",
    appVersion: "2.1.0",
    macAddress: "11:22:33:44:55:66",
    ipAddress: "192.168.1.102",
    status: 0,
    lastVerifyTime: "2024-12-01 08:30:00",
    bindTime: "2024-03-22 09:00:00"
  }
];

const getStatusType = (status: number) => {
  switch (status) {
    case 1:
      return "success";
    case 0:
      return "danger";
    default:
      return "default";
  }
};

const getDeviceStatusColor = (status: number) => {
  switch (status) {
    case 1:
      return "green";
    case 0:
      return "gray";
    default:
      return "default";
  }
};

const getDeviceStatusText = (status: number) => {
  switch (status) {
    case 1:
      return "在线";
    case 0:
      return "离线";
    default:
      return "未知";
  }
};

const getUsageColor = (used: number, max: number) => {
  if (max === -1) return "green";
  const ratio = used / max;
  if (ratio >= 0.9) return "red";
  if (ratio >= 0.7) return "orange";
  return "green";
};

const maskLicenseKey = (key: string) => {
  if (!key) return "-";
  if (key.length <= 8) return "****";
  return `${key.slice(0, 4)}...${key.slice(-4)}`;
};

const copyLicenseKey = (key: string) => {
  if (!key) return;
  navigator.clipboard
    .writeText(key)
    .then(() => {
      Message.success("许可证密钥已复制到剪贴板");
    })
    .catch(() => {
      Message.error("复制失败，请手动复制");
    });
};

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    licenseData.value = mockLicenseData;
    pagination.value.total = mockLicenseData.length;
  } finally {
    loading.value = false;
  }
};

const loadDevices = async () => {
  if (!selectedLicense.value) return;
  deviceLoading.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    deviceData.value = mockDeviceData;
    devicePagination.value.total = mockDeviceData.length;
  } finally {
    deviceLoading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleDevicePageChange = (page: number) => {
  devicePagination.value.current = page;
  loadDevices();
};

const handleLicenseDetail = (record: LicenseRecord) => {
  currentLicense.value = record;
  selectedLicense.value = record;
  licenseDetailVisible.value = true;
  loadDevices();
};

const handleLicenseSelectChange = (licenseId: number) => {
  const selected = licenseData.value.find(item => item.id === licenseId);
  if (selected) {
    selectedLicense.value = selected;
  }
};

const handleBindDevice = () => {
  bindForm.licenseId = currentLicense.value.id;
  bindForm.deviceId = "";
  bindForm.deviceName = "";
  bindForm.deviceType = "";
  bindForm.osVersion = "";
  bindForm.appVersion = "";
  bindForm.macAddress = "";
  bindDeviceVisible.value = true;
};

const submitBindDevice = async () => {
  if (!bindForm.licenseId) {
    Message.warning("请选择许可证");
    return;
  }
  if (!bindForm.deviceId) {
    Message.warning("请输入设备ID");
    return;
  }
  try {
    await new Promise(resolve => setTimeout(resolve, 500));
    Message.success("设备绑定成功");
    bindDeviceVisible.value = false;
    loadDevices();
  } catch {
    Message.error("设备绑定失败，请重试");
  }
};

const handleUnbindDevice = (record: DeviceRecord) => {
  Modal.confirm({
    title: "确认解绑",
    content: `确定要解绑设备"${record.deviceName || record.deviceId}"吗？解绑后该设备将无法使用此许可证。`,
    onOk: async () => {
      try {
        await new Promise(resolve => setTimeout(resolve, 500));
        Message.success("设备已解绑");
        loadDevices();
      } catch {
        Message.error("解绑失败，请重试");
      }
    }
  });
};

onMounted(() => {
  loadData();
});
</script>

<style scoped lang="scss">
.page-container {
  padding: 20px;
}

.license-card,
.device-card {
  height: 100%;
  min-height: 500px;
}

.license-key {
  font-family: "Monaco", "Menlo", monospace;
  font-size: 12px;
  cursor: pointer;
}

.license-key-display {
  display: flex;
  align-items: center;
  gap: 8px;
  font-family: "Monaco", "Menlo", monospace;
  font-size: 12px;
  word-break: break-all;
}

.text-danger {
  color: #f53f3f !important;
}
</style>

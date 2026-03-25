<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-row :gutter="16" class="header-row">
        <a-col :span="12">
          <a-space size="medium">
            <h3>已绑定设备</h3>
            <a-tag>{{ deviceList.length }} / {{ maxDevices }}</a-tag>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="fetchDeviceList"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-alert v-if="deviceList.length >= maxDevices && maxDevices > 0" type="warning" style="margin-bottom: 16px">
        <template #title>设备数量已达上限</template>
        您已绑定 {{ deviceList.length }} 台设备，已达到最大限制 {{ maxDevices }} 台。如需绑定新设备，请先解绑不需要的设备。
      </a-alert>

      <a-table
        row-key="deviceId"
        :loading="loading"
        :bordered="{ cell: true }"
        :columns="columns"
        :data="deviceList"
        :pagination="false"
      >
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)" bordered size="small">
            <template #icon><icon-circle :size="10" /></template>
            {{ record.statusName }}
          </a-tag>
        </template>
        <template #onlineStatus="{ record }">
          <a-badge
            :status="record.online ? 'success' : 'default'"
            :text="record.online ? '在线' : '离线'"
          />
        </template>
        <template #lastActiveTime="{ record }">
          {{ formatTime(record.lastActiveTime) }}
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" status="danger" @click="handleUnbind(record)">
              <template #icon><icon-close /></template>
              解绑
            </a-button>
          </a-space>
        </template>
      </a-table>

      <div v-if="deviceList.length === 0 && !loading" class="empty-state">
        <icon-drive-file name="empty" :size="64" />
        <p>暂无绑定的设备</p>
      </div>

      <a-modal
        v-model:visible="unbindModalVisible"
        title="解绑设备"
        :width="400"
        @ok="handleConfirmUnbind"
        @cancel="unbindModalVisible = false"
      >
        <a-result status="warning" title="确认解绑此设备？">
          <template #subtitle>
            <div class="unbind-info">
              <p>设备名称：{{ currentDevice?.deviceName }}</p>
              <p>设备类型：{{ currentDevice?.deviceType }}</p>
              <p>最后活跃：{{ formatTime(currentDevice?.lastActiveTime) }}</p>
            </div>
          </template>
        </a-result>
        <a-input-password v-model="confirmText" placeholder="请输入 '解绑' 确认" style="margin-top: 16px" />
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { license } from '@/api/modules/plugin-market/license';
import { Message, Modal } from '@arco-design/web-vue';

interface DeviceItem {
  deviceId: string;
  deviceName: string;
  deviceType: string;
  osVersion: string;
  status: number;
  statusName: string;
  online: boolean;
  lastActiveTime: string;
  bindTime: string;
}

const loading = ref(false);
const deviceList = ref<DeviceItem[]>([]);
const maxDevices = ref(5);
const unbindModalVisible = ref(false);
const currentDevice = ref<DeviceItem | null>(null);
const confirmText = ref('');

const columns = [
  { title: '设备名称', dataIndex: 'deviceName', width: 180 },
  { title: '设备类型', dataIndex: 'deviceType', width: 120 },
  { title: '操作系统', dataIndex: 'osVersion', width: 150 },
  { title: '状态', slotName: 'status', width: 100 },
  { title: '在线状态', slotName: 'onlineStatus', width: 100 },
  { title: '最后活跃', slotName: 'lastActiveTime', width: 180 },
  { title: '绑定时间', dataIndex: 'bindTime', width: 180 },
  { title: '操作', slotName: 'optional', width: 100, fixed: 'right' },
];

const getStatusColor = (status: number) => {
  const colors: Record<number, string> = { 0: 'gray', 1: 'green', 2: 'red' };
  return colors[status] || 'gray';
};

const formatTime = (time?: string) => {
  if (!time) return '-';
  const date = new Date(time);
  return date.toLocaleString();
};

const mockDeviceList: DeviceItem[] = [
  {
    deviceId: 'device-001-xxxx-xxxx',
    deviceName: 'MacBook Pro - macOS 14.0',
    deviceType: 'Mac',
    osVersion: 'macOS 14.0 Sonoma',
    status: 1,
    statusName: '正常',
    online: true,
    lastActiveTime: new Date().toISOString(),
    bindTime: '2024-03-15T10:30:00Z',
  },
  {
    deviceId: 'device-002-xxxx-xxxx',
    deviceName: 'Windows PC',
    deviceType: 'Windows PC',
    osVersion: 'Windows 11',
    status: 1,
    statusName: '正常',
    online: false,
    lastActiveTime: new Date(Date.now() - 86400000).toISOString(),
    bindTime: '2024-03-10T08:15:00Z',
  },
  {
    deviceId: 'device-003-xxxx-xxxx',
    deviceName: 'iPhone 15 Pro',
    deviceType: 'iPhone',
    osVersion: 'iOS 17.4',
    status: 1,
    statusName: '正常',
    online: false,
    lastActiveTime: new Date(Date.now() - 172800000).toISOString(),
    bindTime: '2024-02-28T14:20:00Z',
  },
];

const fetchDeviceList = async () => {
  loading.value = true;
  try {
    const res = await license.list({ pageSize: 100 });
    if (res.list && res.list.length > 0) {
      const licenseData = res.list[0];
      deviceList.value = mockDeviceList;
      maxDevices.value = licenseData.maxDevices || 5;
    } else {
      deviceList.value = mockDeviceList;
    }
  } catch (error) {
    console.error(error);
    deviceList.value = mockDeviceList;
  } finally {
    loading.value = false;
  }
};

const handleUnbind = (record: DeviceItem) => {
  currentDevice.value = record;
  confirmText.value = '';
  unbindModalVisible.value = true;
};

const handleConfirmUnbind = async () => {
  if (confirmText.value !== '解绑') {
    Message.warning('请输入 "解绑" 确认操作');
    return;
  }

  if (!currentDevice.value) return;

  try {
    await license.unbind({
      licenseId: 0,
      deviceId: currentDevice.value.deviceId,
    });
    Message.success('设备解绑成功');
    unbindModalVisible.value = false;
    fetchDeviceList();
  } catch (error) {
    console.error(error);
    Message.error('设备解绑失败，请稍后重试');
  }
};

onMounted(() => {
  fetchDeviceList();
});
</script>

<style scoped lang="scss">
.header-row {
  margin-bottom: 20px;
  h3 {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
  }
}

.action-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background-color: var(--color-fill-2);
  }
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 0;
  color: var(--color-text-3);

  p {
    margin-top: 16px;
    font-size: 14px;
  }
}

.unbind-info {
  text-align: left;
  padding: 8px 16px;

  p {
    margin: 4px 0;
    font-size: 14px;
    color: var(--color-text-2);
  }
}
</style>

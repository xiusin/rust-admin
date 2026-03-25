<template>
  <div class="page-container">
    <a-card>
      <template #title>
        <div class="card-title">
          <span>订阅管理</span>
          <a-switch v-model="globalAutoRenew" size="small" @change="handleGlobalAutoRenewChange" />
          <span class="title-tip">全局自动续费</span>
        </div>
      </template>
      <template #extra>
        <a-space>
          <a-button type="primary" @click="loadData">
            <template #icon><icon-refresh /></template>
            刷新
          </a-button>
        </a-space>
      </template>

      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
      >
        <template #status="{ record }">
          <a-badge :status="getStatusType(record.status)" :text="record.statusName" />
        </template>
        <template #autoRenew="{ record }">
          <a-switch
            v-model="record.autoRenew"
            size="small"
            :disabled="record.isExpired"
            @change="handleAutoRenewChange(record)"
          />
        </template>
        <template #expireTime="{ record }">
          <span :class="{ 'text-danger': record.isExpired, 'text-warning': !record.isExpired && record.daysRemaining <= 7 }">
            {{ record.expireTime || '永久有效' }}
          </span>
          <span v-if="!record.isExpired && record.daysRemaining <= 7" class="expire-tip">
            ({{ record.daysRemaining }}天后到期)
          </span>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="small" @click="handleDetail(record)">详情</a-button>
            <a-dropdown trigger="click">
              <a-button type="text" size="small">
                更多
                <template #icon><icon-down /></template>
              </a-button>
              <template #menu>
                <a-doption v-if="!record.isExpired" @click="handleRenew(record)">
                  <template #icon><icon-refresh /></template>
                  续费
                </a-doption>
                <a-doption v-if="!record.isExpired" @click="handleUpgrade(record)">
                  <template #icon><icon-up-circle /></template>
                  升级
                </a-doption>
                <a-doption v-if="!record.isExpired" @click="handleDowngrade(record)">
                  <template #icon><icon-down-circle /></template>
                  降级
                </a-doption>
                <a-doption v-if="record.autoRenew && !record.isExpired" status="warning" @click="handleCancelAutoRenew(record)">
                  <template #icon><icon-close /></template>
                  取消自动续费
                </a-doption>
                <a-doption v-if="record.isExpired" status="danger" @click="handleCancel(record)">
                  <template #icon><icon-delete /></template>
                  取消订阅
                </a-doption>
              </template>
            </a-dropdown>
          </a-space>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="订阅详情" :footer="null" :width="600">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="插件名称">{{ currentRecord.pluginName }}</a-descriptions-item>
        <a-descriptions-item label="订阅方案">{{ currentRecord.planName }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-badge :status="getStatusType(currentRecord.status)" :text="currentRecord.statusName" />
        </a-descriptions-item>
        <a-descriptions-item label="自动续费">
          <a-switch v-model="currentRecord.autoRenew" size="small" disabled />
        </a-descriptions-item>
        <a-descriptions-item label="开始时间">{{ currentRecord.startTime || '-' }}</a-descriptions-item>
        <a-descriptions-item label="到期时间">{{ currentRecord.expireTime || '永久有效' }}</a-descriptions-item>
        <a-descriptions-item label="剩余天数">
          <span :class="{ 'text-danger': currentRecord.isExpired }">
            {{ currentRecord.daysRemaining || 0 }} 天
          </span>
        </a-descriptions-item>
        <a-descriptions-item label="已使用设备">
          {{ currentRecord.usedDevices || 0 }} / {{ currentRecord.maxDevices === -1 ? '无限制' : currentRecord.maxDevices }}
        </a-descriptions-item>
        <a-descriptions-item label="下次扣款金额" :span="2">
          ¥{{ currentRecord.nextRenewalAmount || 0 }}
        </a-descriptions-item>
      </a-descriptions>
      <a-divider />
      <a-space style="display: flex; justify-content: flex-end">
        <a-button v-if="currentRecord.isExpired" type="primary" @click="handleRenew(currentRecord)">
          <template #icon><icon-refresh /></template>
          续费
        </a-button>
        <a-button v-if="!currentRecord.isExpired" type="outline" @click="handleUpgrade(currentRecord)">
          <template #icon><icon-up-circle /></template>
          升级方案
        </a-button>
      </a-space>
    </a-modal>

    <a-modal v-model:visible="renewVisible" title="续费订阅" @ok="submitRenew" @cancel="renewVisible = false">
      <a-form :model="renewForm" layout="vertical">
        <a-form-item label="插件">
          <a-input :value="currentRecord.pluginName" disabled />
        </a-form-item>
        <a-form-item label="当前方案">
          <a-input :value="currentRecord.planName" disabled />
        </a-form-item>
        <a-form-item label="续费时长" required>
          <a-select v-model="renewForm.extendDays" placeholder="请选择续费时长">
            <a-option :value="30">1个月</a-option>
            <a-option :value="90">3个月</a-option>
            <a-option :value="180">6个月</a-option>
            <a-option :value="365">1年</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="自动续费">
          <a-switch v-model="renewForm.autoRenew" />
          <span class="form-tip">开启后将在到期前自动续费</span>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="upgradeVisible" title="变更订阅方案" :footer="null" :width="700">
      <div class="change-plan-content">
        <a-alert class="change-tip">
          {{ upgradeType === 'upgrade' ? '升级后将立即生效，按剩余时间折算抵扣' : '降级将在当前周期结束后生效' }}
        </a-alert>
        <a-radio-group v-model="upgradeType" class="change-type" @change="handleChangeTypeChange">
          <a-radio value="upgrade">升级</a-radio>
          <a-radio value="downgrade">降级</a-radio>
        </a-radio-group>
        <a-list :data="changePlans" :bordered="false" class="plan-list">
          <template #item="{ item }">
            <a-list-item
              class="plan-item"
              :class="{
                selected: changeForm.planId === item.id,
                current: item.id === currentRecord.planId,
                disabled: upgradeType === 'upgrade' && item.sort <= currentRecord.planSort,
              }"
            >
              <a-radio
                :value="item.id"
                :disabled="upgradeType === 'upgrade' && item.sort <= currentRecord.planSort"
                @change="changeForm.planId = item.id"
              >
                <div class="plan-content">
                  <div class="plan-header">
                    <span class="plan-name">
                      {{ item.name }}
                      <a-tag v-if="item.id === currentRecord.planId" size="small" color="arcoblue">当前</a-tag>
                    </span>
                    <span class="plan-price">¥{{ item.price }}/月</span>
                  </div>
                  <div class="plan-desc">{{ item.description }}</div>
                  <div class="plan-features" v-if="item.features?.length">
                    <a-tag v-for="f in item.features.slice(0, 4)" :key="f.code" size="small">
                      {{ f.name }}
                    </a-tag>
                    <a-tag v-if="item.features.length > 4" size="small">+{{ item.features.length - 4 }}</a-tag>
                  </div>
                </div>
              </a-radio>
            </a-list-item>
          </template>
        </a-list>
        <div class="change-actions">
          <a-button @click="upgradeVisible = false">取消</a-button>
          <a-button type="primary" @click="submitChangePlan">
            确认{{ upgradeType === 'upgrade' ? '升级' : '降级' }}
          </a-button>
        </div>
      </div>
    </a-modal>

    <a-modal v-model:visible="reminderVisible" title="到期提醒设置" @ok="submitReminder" @cancel="reminderVisible = false">
      <a-form :model="reminderForm" layout="vertical">
        <a-form-item label="插件">
          <a-input :value="currentRecord.pluginName" disabled />
        </a-form-item>
        <a-form-item label="提醒方式">
          <a-checkbox-group v-model="reminderForm.methods">
            <a-checkbox value="system">系统通知</a-checkbox>
            <a-checkbox value="email">邮件通知</a-checkbox>
            <a-checkbox value="sms">短信通知</a-checkbox>
          </a-checkbox-group>
        </a-form-item>
        <a-form-item label="提前提醒时间">
          <a-select v-model="reminderForm.days" placeholder="请选择提前提醒时间">
            <a-option :value="1">提前1天</a-option>
            <a-option :value="3">提前3天</a-option>
            <a-option :value="7">提前7天</a-option>
            <a-option :value="14">提前14天</a-option>
            <a-option :value="30">提前30天</a-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';

interface SubscriptionRecord {
  id: number;
  pluginId: number;
  pluginName: string;
  pluginCover?: string;
  planId: number;
  planName: string;
  planSort: number;
  startTime?: string;
  expireTime?: string;
  isExpired: boolean;
  daysRemaining: number;
  autoRenew: boolean;
  status: number;
  statusName: string;
  maxDevices: number;
  usedDevices: number;
  nextRenewalAmount: number;
}

interface PlanItem {
  id: number;
  pluginId: number;
  name: string;
  description: string;
  periodType: number;
  periodTypeName: string;
  periodDays: number;
  price: number;
  originalPrice: number;
  features: { code: string; name: string; included: boolean }[];
  maxDevices: number;
  sort: number;
  status: number;
}

const loading = ref(false);
const globalAutoRenew = ref(false);
const tableData = ref<SubscriptionRecord[]>([]);
const detailVisible = ref(false);
const renewVisible = ref(false);
const upgradeVisible = ref(false);
const reminderVisible = ref(false);
const currentRecord = ref<SubscriptionRecord>({} as SubscriptionRecord);
const changePlans = ref<PlanItem[]>([]);
const upgradeType = ref<'upgrade' | 'downgrade'>('upgrade');

const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const renewForm = reactive({
  extendDays: 30,
  autoRenew: true,
});

const changeForm = reactive({
  planId: 0,
});

const reminderForm = reactive({
  methods: ['system'] as string[],
  days: 7,
});

const columns = [
  { title: '插件名称', dataIndex: 'pluginName', width: 180 },
  { title: '订阅方案', dataIndex: 'planName', width: 120 },
  { title: '状态', slotName: 'status', width: 100 },
  { title: '自动续费', slotName: 'autoRenew', width: 100 },
  { title: '到期时间', slotName: 'expireTime', width: 180 },
  { title: '操作', slotName: 'optional', width: 160, fixed: 'right' },
];

const mockData: SubscriptionRecord[] = [
  {
    id: 1,
    pluginId: 1,
    pluginName: '智能优惠券',
    planId: 2,
    planName: '专业版',
    planSort: 2,
    startTime: '2024-03-20 10:35:00',
    expireTime: '2025-03-25 10:35:00',
    isExpired: false,
    daysRemaining: 361,
    autoRenew: true,
    status: 1,
    statusName: '有效',
    maxDevices: 5,
    usedDevices: 2,
    nextRenewalAmount: 299,
  },
  {
    id: 2,
    pluginId: 2,
    pluginName: '限时秒杀',
    planId: 1,
    planName: '基础版',
    planSort: 1,
    startTime: '2024-02-15 14:20:00',
    expireTime: '2024-03-15 14:20:00',
    isExpired: true,
    daysRemaining: 0,
    autoRenew: false,
    status: 0,
    statusName: '已过期',
    maxDevices: 1,
    usedDevices: 0,
    nextRenewalAmount: 0,
  },
  {
    id: 3,
    pluginId: 3,
    pluginName: '数据统计分析',
    planId: 3,
    planName: '企业版',
    planSort: 3,
    startTime: '2024-01-10 09:00:00',
    expireTime: '2025-01-10 09:00:00',
    isExpired: false,
    daysRemaining: 293,
    autoRenew: true,
    status: 1,
    statusName: '有效',
    maxDevices: -1,
    usedDevices: 3,
    nextRenewalAmount: 799,
  },
  {
    id: 4,
    pluginId: 4,
    pluginName: 'AI智能客服',
    planId: 2,
    planName: '高级版',
    planSort: 2,
    startTime: '2023-12-01 08:30:00',
    expireTime: '2024-12-01 08:30:00',
    isExpired: false,
    daysRemaining: 3,
    autoRenew: false,
    status: 1,
    statusName: '有效',
    maxDevices: 5,
    usedDevices: 1,
    nextRenewalAmount: 0,
  },
];

const mockPlans: PlanItem[] = [
  {
    id: 1,
    pluginId: 1,
    name: '基础版',
    description: '适合个人开发者或小型店铺使用',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 99,
    originalPrice: 99,
    features: [
      { code: 'basic', name: '基础功能', included: true },
      { code: 'template', name: '5个优惠券模板', included: true },
    ],
    maxDevices: 1,
    sort: 1,
    status: 1,
  },
  {
    id: 2,
    pluginId: 1,
    name: '专业版',
    description: '适合中型商家使用',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 299,
    originalPrice: 399,
    features: [
      { code: 'basic', name: '基础功能', included: true },
      { code: 'template', name: '无限优惠券模板', included: true },
      { code: 'api', name: 'API调用', included: true },
    ],
    maxDevices: 5,
    sort: 2,
    status: 1,
  },
  {
    id: 3,
    pluginId: 1,
    name: '企业版',
    description: '适合大型企业使用',
    periodType: 0,
    periodTypeName: '月付',
    periodDays: 30,
    price: 799,
    originalPrice: 999,
    features: [
      { code: 'basic', name: '基础功能', included: true },
      { code: 'template', name: '无限优惠券模板', included: true },
      { code: 'api', name: '无限API调用', included: true },
      { code: 'priority', name: '专属客服', included: true },
    ],
    maxDevices: -1,
    sort: 3,
    status: 1,
  },
];

const getStatusType = (status: number) => {
  switch (status) {
    case 1:
      return 'success';
    case 0:
      return 'danger';
    default:
      return 'default';
  }
};

const loadData = async () => {
  loading.value = true;
  try {
    await new Promise((resolve) => setTimeout(resolve, 500));
    tableData.value = mockData;
    pagination.value.total = mockData.length;
  } finally {
    loading.value = false;
  }
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleGlobalAutoRenewChange = (checked: boolean | number | string) => {
  Message.info(`全局自动续费已${checked ? '开启' : '关闭'}`);
};

const handleAutoRenewChange = (record: SubscriptionRecord) => {
  Message.success(`${record.pluginName} 自动续费已${record.autoRenew ? '开启' : '关闭'}`);
};

const handleDetail = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  detailVisible.value = true;
};

const handleRenew = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  renewForm.extendDays = 30;
  renewForm.autoRenew = record.autoRenew;
  renewVisible.value = true;
  detailVisible.value = false;
};

const submitRenew = async () => {
  if (!renewForm.extendDays) {
    Message.warning('请选择续费时长');
    return;
  }
  try {
    await new Promise((resolve) => setTimeout(resolve, 500));
    Message.success('续费成功');
    renewVisible.value = false;
    loadData();
  } catch {
    Message.error('续费失败，请重试');
  }
};

const handleUpgrade = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  changePlans.value = mockPlans;
  upgradeType.value = 'upgrade';
  changeForm.planId = record.planId;
  upgradeVisible.value = true;
  detailVisible.value = false;
};

const handleDowngrade = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  changePlans.value = mockPlans;
  upgradeType.value = 'downgrade';
  changeForm.planId = record.planId;
  upgradeVisible.value = true;
  detailVisible.value = false;
};

const handleChangeTypeChange = () => {
  changeForm.planId = currentRecord.value.planId;
};

const submitChangePlan = async () => {
  if (!changeForm.planId || changeForm.planId === currentRecord.value.planId) {
    Message.warning(`请选择要${upgradeType.value === 'upgrade' ? '升级' : '降级'}的方案`);
    return;
  }
  try {
    await new Promise((resolve) => setTimeout(resolve, 500));
    Message.success(`${upgradeType.value === 'upgrade' ? '升级' : '降级'}成功`);
    upgradeVisible.value = false;
    loadData();
  } catch {
    Message.error(`${upgradeType.value === 'upgrade' ? '升级' : '降级'}失败，请重试`);
  }
};

const handleCancelAutoRenew = (record: SubscriptionRecord) => {
  record.autoRenew = false;
  Message.success(`${record.pluginName} 自动续费已取消`);
};

const handleCancel = () => {
  Message.warning('取消订阅功能开发中');
};

onMounted(() => {
  loadData();
});
</script>

<style scoped lang="scss">
.page-container {
  padding: 20px;
}

.card-title {
  display: flex;
  align-items: center;
  gap: 12px;

  .title-tip {
    font-size: 13px;
    color: var(--color-text-4);
  }
}

.text-danger {
  color: #f53f3f !important;
}

.text-warning {
  color: #ff7d00 !important;
}

.expire-tip {
  font-size: 12px;
  color: #ff7d00;
  margin-left: 4px;
}

.form-tip {
  margin-left: 8px;
  font-size: 12px;
  color: var(--color-text-4);
}

.change-plan-content {
  .change-tip {
    margin-bottom: 16px;
  }

  .change-type {
    margin-bottom: 16px;
  }

  .plan-list {
    max-height: 350px;
    overflow-y: auto;

    .plan-item {
      padding: 12px;
      margin-bottom: 8px;
      border: 1px solid var(--color-fill-3);
      border-radius: 8px;
      transition: all 0.3s;

      &.selected {
        border-color: #1650ff;
        background-color: rgba(22, 80, 255, 0.05);
      }

      &.current {
        border-color: var(--color-primary);
      }

      &.disabled {
        opacity: 0.5;
        cursor: not-allowed;
      }

      .plan-content {
        .plan-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 8px;

          .plan-name {
            font-weight: 600;
            font-size: 15px;
            display: flex;
            align-items: center;
            gap: 8px;
          }

          .plan-price {
            color: #f53f3f;
            font-weight: 600;
          }
        }

        .plan-desc {
          color: var(--color-text-4);
          font-size: 13px;
          margin-bottom: 8px;
        }

        .plan-features {
          display: flex;
          flex-wrap: wrap;
          gap: 4px;
        }
      }
    }
  }

  .change-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 16px;
  }
}
</style>

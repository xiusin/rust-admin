<template>
  <div class="page-container">
    <a-card>
      <template #title>
        <a-tabs v-model:active-key="activeTab" @change="handleTabChange">
          <a-tab-pane key="all" title="全部" />
          <a-tab-pane key="active" title="有效" />
          <a-tab-pane key="expired" title="已过期" />
        </a-tabs>
      </template>
      <template #extra>
        <a-button type="primary" @click="loadData">
          <template #icon><icon-refresh /></template>
          刷新
        </a-button>
      </template>

      <a-row :gutter="[16, 16]" v-if="!loading && tableData.length > 0">
        <a-col :xs="24" :sm="12" :md="8" :lg="6" v-for="item in tableData" :key="item.id">
          <a-card class="plugin-card" hoverable @click="handleDetail(item)">
            <div class="card-header">
              <a-avatar shape="square" :image-url="item.pluginCover" :size="56" />
              <div class="plugin-info">
                <div class="plugin-name">{{ item.pluginName }}</div>
                <div class="plugin-version">v{{ item.pluginVersion }}</div>
              </div>
              <a-tag :color="item.isExpired ? 'red' : 'green'" class="status-tag">
                {{ item.isExpired ? "已过期" : "有效" }}
              </a-tag>
            </div>

            <a-divider :margin="12" />

            <div class="card-body">
              <div class="info-row">
                <span class="label">订阅方案</span>
                <span class="value">{{ item.planName }}</span>
              </div>
              <div class="info-row">
                <span class="label">到期时间</span>
                <span class="value" :class="{ 'text-danger': item.isExpired }">
                  {{ item.expireTime || "永久有效" }}
                </span>
              </div>
              <div class="info-row" v-if="!item.isExpired">
                <span class="label">剩余天数</span>
                <span class="value text-primary">{{ item.daysRemaining }} 天</span>
              </div>
            </div>

            <a-divider :margin="12" />

            <div class="card-footer">
              <a-space>
                <a-button v-if="item.isExpired" type="primary" size="small" @click.stop="handleRenew(item)">
                  <template #icon><icon-refresh /></template>
                  续费
                </a-button>
                <a-button v-if="!item.isExpired" type="outline" size="small" @click.stop="handleUpgrade(item)">
                  <template #icon><icon-up-circle /></template>
                  升级
                </a-button>
                <a-button type="text" size="small" @click.stop="handleDetail(item)"> 详情 </a-button>
              </a-space>
            </div>
          </a-card>
        </a-col>
      </a-row>

      <a-empty v-else-if="!loading && tableData.length === 0" description="暂无订阅插件">
        <template #image>
          <icon-subscription color="var(--color-text-4)" />
        </template>
      </a-empty>

      <div v-if="loading" class="loading-container">
        <a-spin size="large" />
      </div>
    </a-card>

    <a-modal v-model:visible="detailVisible" title="插件详情" :footer="null" :width="600">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="插件名称">{{ currentRecord.pluginName }}</a-descriptions-item>
        <a-descriptions-item label="版本">{{ currentRecord.pluginVersion }}</a-descriptions-item>
        <a-descriptions-item label="订阅方案">{{ currentRecord.planName }}</a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="currentRecord.isExpired ? 'red' : 'green'">
            {{ currentRecord.isExpired ? "已过期" : "有效" }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="开始时间">{{ currentRecord.startTime || "-" }}</a-descriptions-item>
        <a-descriptions-item label="到期时间">{{ currentRecord.expireTime || "永久有效" }}</a-descriptions-item>
        <a-descriptions-item label="剩余天数" :span="2">
          <span :class="{ 'text-danger': currentRecord.isExpired }"> {{ currentRecord.daysRemaining || 0 }} 天 </span>
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

    <a-modal v-model:visible="renewVisible" title="续费插件" @ok="submitRenew" @cancel="renewVisible = false">
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
      </a-form>
    </a-modal>

    <a-modal v-model:visible="upgradeVisible" title="升级方案" :footer="null" :width="700">
      <div v-if="currentRecord.pluginName" class="upgrade-content">
        <a-alert class="upgrade-tip"> 升级后将立即生效，按剩余时间折算抵扣 </a-alert>
        <a-list :data="upgradePlans" :bordered="false" class="plan-list">
          <template #item="{ item }">
            <a-list-item class="plan-item" :class="{ selected: upgradeForm.planId === item.id }">
              <a-radio :value="item.id" :disabled="item.sort <= currentRecord.planSort" @change="upgradeForm.planId = item.id">
                <div class="plan-content">
                  <div class="plan-header">
                    <span class="plan-name">{{ item.name }}</span>
                    <span class="plan-price">¥{{ item.price }}/月</span>
                  </div>
                  <div class="plan-desc">{{ item.description }}</div>
                  <div class="plan-features" v-if="item.features?.length">
                    <a-tag v-for="f in item.features.slice(0, 3)" :key="f.code" size="small">
                      {{ f.name }}
                    </a-tag>
                    <a-tag v-if="item.features.length > 3" size="small">更多...</a-tag>
                  </div>
                </div>
              </a-radio>
            </a-list-item>
          </template>
        </a-list>
        <div class="upgrade-actions">
          <a-button @click="upgradeVisible = false">取消</a-button>
          <a-button type="primary" @click="submitUpgrade">确认升级</a-button>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import { subscription } from "@/api/modules/plugin-market/license";
import { plan } from "@/api/modules/plugin-market/market";

interface SubscriptionRecord {
  id: number;
  pluginId: number;
  pluginName: string;
  pluginCover?: string;
  pluginVersion: string;
  planId: number;
  planName: string;
  planSort?: number;
  startTime?: string;
  expireTime?: string;
  isExpired: boolean;
  daysRemaining: number;
  status: number;
  statusName: string;
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
const activeTab = ref("all");
const tableData = ref<SubscriptionRecord[]>([]);
const detailVisible = ref(false);
const renewVisible = ref(false);
const upgradeVisible = ref(false);
const currentRecord = ref<SubscriptionRecord>({} as SubscriptionRecord);
const upgradePlans = ref<PlanItem[]>([]);

const renewForm = reactive({
  subscriptionId: 0,
  extendDays: 30
});

const upgradeForm = reactive({
  planId: 0
});

const loadData = async () => {
  loading.value = true;
  try {
    const res = await subscription.list({ pageNum: pagination.value.current, pageSize: pagination.value.pageSize });
    const data = res.data?.list || [];
    if (activeTab.value === "all") {
      tableData.value = data;
    } else if (activeTab.value === "active") {
      tableData.value = data.filter((item: SubscriptionRecord) => !item.isExpired);
    } else {
      tableData.value = data.filter((item: SubscriptionRecord) => item.isExpired);
    }
    pagination.value.total = res.data?.total || data.length;
  } catch (error) {
    Message.error("获取数据失败");
  } finally {
    loading.value = false;
  }
};

const handleTabChange = (key: string) => {
  activeTab.value = key;
  pagination.value.current = 1;
  loadData();
};

const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const handleDetail = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  detailVisible.value = true;
};

const handleRenew = (record: SubscriptionRecord) => {
  currentRecord.value = record;
  renewForm.subscriptionId = record.id;
  renewForm.extendDays = 30;
  renewVisible.value = true;
  detailVisible.value = false;
};

const submitRenew = async () => {
  if (!renewForm.extendDays) {
    Message.warning("请选择续费时长");
    return;
  }
  try {
    await subscription.renew({ subscriptionId: currentRecord.value.id, extendDays: renewForm.extendDays });
    Message.success("续费成功");
    renewVisible.value = false;
    loadData();
  } catch {
    Message.error("续费失败，请重试");
  }
};

const handleUpgrade = async (record: SubscriptionRecord) => {
  currentRecord.value = record;
  try {
    const res = await plan.list(record.pluginId);
    upgradePlans.value = res.data || [];
    upgradeForm.planId = record.planId;
    upgradeVisible.value = true;
    detailVisible.value = false;
  } catch {
    Message.error("获取方案失败");
  }
};

const submitUpgrade = async () => {
  if (!upgradeForm.planId || upgradeForm.planId === currentRecord.value.planId) {
    Message.warning("请选择要升级的方案");
    return;
  }
  Message.warning("升级功能开发中");
};

onMounted(() => {
  loadData();
});
</script>

<style scoped lang="scss">
.page-container {
  padding: 20px;
}

.plugin-card {
  height: 100%;
  transition: all 0.3s;

  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .card-header {
    display: flex;
    align-items: center;
    gap: 12px;

    .plugin-info {
      flex: 1;
      min-width: 0;

      .plugin-name {
        font-size: 16px;
        font-weight: 600;
        color: var(--color-text-1);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .plugin-version {
        font-size: 12px;
        color: var(--color-text-4);
      }
    }

    .status-tag {
      flex-shrink: 0;
    }
  }

  .card-body {
    .info-row {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 6px 0;

      .label {
        color: var(--color-text-4);
        font-size: 13px;
      }

      .value {
        color: var(--color-text-1);
        font-size: 13px;
      }
    }
  }

  .card-footer {
    display: flex;
    justify-content: flex-end;
  }
}

.loading-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 200px;
}

.text-danger {
  color: #f53f3f !important;
}

.text-primary {
  color: #1650ff !important;
}

.upgrade-content {
  .upgrade-tip {
    margin-bottom: 16px;
  }

  .plan-list {
    max-height: 400px;
    overflow-y: auto;
    margin: 0 -16px;
    padding: 0 16px;

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

      .plan-content {
        .plan-header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 8px;

          .plan-name {
            font-weight: 600;
            font-size: 15px;
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

  .upgrade-actions {
    display: flex;
    justify-content: flex-end;
    gap: 12px;
    margin-top: 16px;
  }
}
</style>

<template>
  <div class="snow-page">
    <div class="snow-inner card-redeem-container">
      <a-card class="redeem-card">
        <div class="card-header">
          <icon-gift size="48" class="header-icon" />
          <h2>卡密兑换</h2>
          <p class="subtitle">输入卡号和密码兑换插件</p>
        </div>

        <a-form ref="formRef" :model="form" layout="vertical" class="redeem-form">
          <a-form-item field="cardNo" label="卡号" :rules="[{ required: true, message: '请输入卡号' }]">
            <a-input v-model="form.cardNo" placeholder="请输入12位卡号" :max-length="20" allow-clear>
              <template #prefix><icon-id-card /></template>
            </a-input>
          </a-form-item>

          <a-form-item field="cardPwd" label="密码" :rules="[{ required: true, message: '请输入卡密密码' }]">
            <a-input-password v-model="form.cardPwd" placeholder="请输入卡密密码" allow-clear>
              <template #prefix><icon-lock /></template>
            </a-input-password>
          </a-form-item>

          <a-button type="primary" :loading="checking" long @click="handlePreview">
            <template #icon><icon-preview /></template>
            预览插件信息
          </a-button>
        </a-form>

        <a-divider v-if="previewInfo">兑换信息预览</a-divider>

        <a-card v-if="previewInfo" class="preview-card" :bordered="false">
          <a-descriptions :column="2" bordered size="large">
            <a-descriptions-item label="插件名称" :span="2">
              <a-space>
                <icon-app name="plugin" :size="16" />
                {{ previewInfo.pluginName }}
              </a-space>
            </a-descriptions-item>
            <a-descriptions-item label="插件版本">{{ previewInfo.pluginVersion }}</a-descriptions-item>
            <a-descriptions-item label="价格">{{ previewInfo.price }} 元</a-descriptions-item>
            <a-descriptions-item label="计划名称">{{ previewInfo.planName }}</a-descriptions-item>
            <a-descriptions-item label="有效期">{{ previewInfo.periodDays }} 天</a-descriptions-item>
          </a-descriptions>

          <a-alert v-if="previewInfo.features && previewInfo.features.length > 0" style="margin-top: 16px">
            <template #title>包含功能</template>
            <template #content>
              <ul class="feature-list">
                <li v-for="(feature, index) in previewInfo.features" :key="index">
                  {{ feature }}
                </li>
              </ul>
            </template>
          </a-alert>

          <a-button type="primary" status="success" :loading="redeeming" long style="margin-top: 24px" @click="handleRedeem">
            <template #icon><icon-check-circle /></template>
            确认兑换
          </a-button>
        </a-card>

        <a-alert v-if="errorMessage" type="error" style="margin-top: 16px">
          <template #title>兑换失败</template>
          {{ errorMessage }}
        </a-alert>
      </a-card>

      <a-modal v-model:visible="successModalVisible" title="兑换成功" :footer="null" :width="500">
        <a-result status="success" title="兑换成功！">
          <template #subtitle>
            <div class="success-details">
              <a-descriptions :column="1" bordered size="large">
                <a-descriptions-item label="订单号">{{ orderInfo?.orderNo }}</a-descriptions-item>
                <a-descriptions-item label="插件名称">{{ orderInfo?.pluginName }}</a-descriptions-item>
                <a-descriptions-item label="计划名称">{{ orderInfo?.planName }}</a-descriptions-item>
                <a-descriptions-item label="有效期">{{ orderInfo?.periodDays }} 天</a-descriptions-item>
                <a-descriptions-item label="兑换时间">{{ orderInfo?.redeemTime }}</a-descriptions-item>
              </a-descriptions>

              <a-space style="margin-top: 24px; width: 100%; justify-content: center">
                <a-button type="primary" @click="goToLicense">
                  <template #icon><icon-launch /></template>
                  查看许可证
                </a-button>
                <a-button @click="successModalVisible = false">
                  <template #icon><icon-refresh /></template>
                  继续兑换
                </a-button>
              </a-space>
            </div>
          </template>
        </a-result>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { card } from "@/api/modules/plugin-market/card";
import { Message } from "@arco-design/web-vue";

interface PreviewInfo {
  pluginId: number;
  pluginName: string;
  pluginVersion: string;
  planId: number;
  planName: string;
  price: number;
  periodDays: number;
  features: string[];
}

interface OrderInfo {
  orderNo: string;
  pluginName: string;
  planName: string;
  periodDays: number;
  redeemTime: string;
  licenseKey: string;
}

const router = useRouter();
const formRef = ref();
const form = reactive({
  cardNo: "",
  cardPwd: ""
});

const checking = ref(false);
const redeeming = ref(false);
const previewInfo = ref<PreviewInfo | null>(null);
const errorMessage = ref("");
const successModalVisible = ref(false);
const orderInfo = ref<OrderInfo | null>(null);

const mockPreviewInfo: PreviewInfo = {
  pluginId: 1,
  pluginName: "智能优惠券",
  pluginVersion: "2.1.0",
  planId: 2,
  planName: "专业版",
  price: 299,
  periodDays: 30,
  features: ["多种优惠券类型", "无限优惠券模板", "API调用（10000次/日）", "优先客服支持"]
};

const handlePreview = async () => {
  try {
    await formRef.value.validateField("cardNo");
    await formRef.value.validateField("cardPwd");
  } catch {
    return;
  }

  if (form.cardNo.length < 6) {
    Message.warning("请输入有效的卡号");
    return;
  }

  checking.value = true;
  errorMessage.value = "";
  previewInfo.value = null;

  try {
    previewInfo.value = mockPreviewInfo;
  } catch (error: any) {
    errorMessage.value = error?.message || "获取插件信息失败，请检查卡号密码";
    Message.error("预览失败");
  } finally {
    checking.value = false;
  }
};

const handleRedeem = async () => {
  if (!previewInfo.value) {
    Message.warning("请先预览插件信息");
    return;
  }

  redeeming.value = true;
  errorMessage.value = "";

  try {
    const response = await card.redeem({
      cardNo: form.cardNo,
      cardPwd: form.cardPwd
    });

    orderInfo.value = {
      orderNo: response.orderNo || `ORD${Date.now()}`,
      pluginName: previewInfo.value.pluginName,
      planName: previewInfo.value.planName,
      periodDays: previewInfo.value.periodDays,
      redeemTime: new Date().toLocaleString(),
      licenseKey: response.licenseKey || ""
    };

    successModalVisible.value = true;
    Message.success("兑换成功");
    form.cardNo = "";
    form.cardPwd = "";
    previewInfo.value = null;
  } catch (error: any) {
    errorMessage.value = error?.message || "兑换失败，请检查卡号密码是否正确";
    Message.error("兑换失败");
  } finally {
    redeeming.value = false;
  }
};

const goToLicense = () => {
  successModalVisible.value = false;
  router.push("/verify/device");
};
</script>

<style scoped lang="scss">
.card-redeem-container {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 40px 20px;
  min-height: calc(100vh - 120px);
}

.redeem-card {
  width: 100%;
  max-width: 550px;
  border-radius: 8px;
}

.card-header {
  text-align: center;
  margin-bottom: 32px;

  .header-icon {
    color: rgb(var(--primary-6));
    margin-bottom: 16px;
  }

  h2 {
    margin: 0 0 8px;
    font-size: 24px;
    font-weight: 600;
  }

  .subtitle {
    color: var(--color-text-3);
    margin: 0;
  }
}

.redeem-form {
  margin-bottom: 16px;
}

.preview-card {
  background-color: var(--color-fill-1);
  margin-top: 16px;
}

.feature-list {
  margin: 8px 0 0;
  padding-left: 20px;

  li {
    margin: 4px 0;
    color: var(--color-text-2);
  }
}

.success-details {
  text-align: center;
}
</style>

<template>
  <div class="snow-page">
    <div class="snow-inner activate-container">
      <a-card class="activate-card">
        <div class="card-header">
          <icon-shield-check size="48" class="header-icon" />
          <h2>插件激活</h2>
          <p class="subtitle">输入许可证密钥激活您的插件</p>
        </div>

        <a-form ref="formRef" :model="form" layout="vertical" class="activate-form">
          <a-form-item field="licenseKey" label="许可证密钥" :rules="[{ required: true, message: '请输入许可证密钥' }]">
            <a-input v-model="form.licenseKey" placeholder="请输入32位许可证密钥" :max-length="64" allow-clear>
              <template #prefix><icon-lock /></template>
            </a-input>
          </a-form-item>

          <a-divider>设备信息</a-divider>

          <a-descriptions :column="1" bordered size="large" class="device-info">
            <a-descriptions-item label="设备ID">
              <a-tooltip :content="deviceInfo.deviceId">
                <span class="device-id">{{ deviceInfo.deviceId }}</span>
              </a-tooltip>
            </a-descriptions-item>
            <a-descriptions-item label="设备名称">{{ deviceInfo.deviceName }}</a-descriptions-item>
            <a-descriptions-item label="设备类型">{{ deviceInfo.deviceType }}</a-descriptions-item>
            <a-descriptions-item label="操作系统">{{ deviceInfo.osVersion }}</a-descriptions-item>
          </a-descriptions>

          <a-divider>验证码</a-divider>

          <a-form-item field="code" label="激活验证码" :rules="[{ required: true, message: '请输入6位验证码' }]">
            <a-input v-model="form.code" placeholder="请输入6位验证码" :max-length="6" style="width: 200px" allow-clear>
              <template #prefix><icon-message /></template>
            </a-input>
          </a-form-item>

          <a-space direction="vertical" :size="16" style="width: 100%">
            <a-button type="primary" :loading="sendingCode" :disabled="countdown > 0" long @click="handleSendCode">
              <template #icon><icon-send /></template>
              {{ countdown > 0 ? `${countdown}秒后可重发` : "发送激活码" }}
            </a-button>

            <a-button type="primary" status="success" :loading="activating" :disabled="!canActivate" long @click="handleActivate">
              <template #icon><icon-check-circle /></template>
              激活插件
            </a-button>
          </a-space>
        </a-form>

        <a-alert v-if="resultMessage" :type="resultType" style="margin-top: 16px">
          <template #title>{{ resultMessage }}</template>
        </a-alert>
      </a-card>

      <a-modal v-model:visible="successModalVisible" title="激活成功" :footer="null" :width="400">
        <div class="success-content">
          <icon-check-circle size="64" class="success-icon" />
          <p class="success-text">您的插件已成功激活！</p>
          <a-descriptions :column="1" size="large">
            <a-descriptions-item label="许可证密钥">
              {{ form.licenseKey }}
            </a-descriptions-item>
            <a-descriptions-item label="激活时间">{{ activatedTime }}</a-descriptions-item>
            <a-descriptions-item label="有效期至">{{ expireTime }}</a-descriptions-item>
          </a-descriptions>
        </div>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted, onUnmounted } from "vue";
import { verify } from "@/api/modules/plugin-market/verify";
import { license } from "@/api/modules/plugin-market/license";
import { Message } from "@arco-design/web-vue";

const formRef = ref();
const form = reactive({
  licenseKey: "",
  code: ""
});

const deviceInfo = reactive({
  deviceId: "",
  deviceName: "",
  deviceType: "",
  osVersion: ""
});

const sendingCode = ref(false);
const activating = ref(false);
const countdown = ref(0);
const resultMessage = ref("");
const resultType = ref<"success" | "error" | "warning" | "info">("info");
const successModalVisible = ref(false);
const activatedTime = ref("");
const expireTime = ref("");

let countdownTimer: number | undefined;

const canActivate = computed(() => {
  return form.licenseKey.length >= 32 && form.code.length === 6;
});

const generateDeviceId = () => {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, c => {
    const r = (Math.random() * 16) | 0;
    const v = c === "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
};

const getDeviceInfo = () => {
  const ua = navigator.userAgent;
  let osVersion = "Unknown OS";
  let deviceType = "Desktop";

  if (ua.includes("Windows")) {
    osVersion = "Windows";
    if (ua.includes("Windows NT 10.0")) osVersion += " 10/11";
    else if (ua.includes("Windows NT 6.3")) osVersion += " 8.1";
    deviceType = "Windows PC";
  } else if (ua.includes("Mac OS X")) {
    const match = ua.match(/Mac OS X (\d+[._]\d+)/);
    osVersion = match ? `macOS ${match[1].replace("_", ".")}` : "macOS";
    deviceType = "Mac";
  } else if (ua.includes("Linux")) {
    osVersion = "Linux";
    deviceType = "Linux PC";
  } else if (ua.includes("Android")) {
    const match = ua.match(/Android (\d+(\.\d+)?)/);
    osVersion = match ? `Android ${match[1]}` : "Android";
    deviceType = "Android Device";
  } else if (ua.includes("iOS") || ua.includes("iPhone") || ua.includes("iPad")) {
    const match = ua.match(/OS (\d+[._]\d+)/);
    osVersion = match ? `iOS ${match[1].replace("_", ".")}` : "iOS";
    deviceType = ua.includes("iPad") ? "iPad" : "iPhone";
  }

  deviceInfo.deviceId = generateDeviceId();
  deviceInfo.deviceName = `${deviceType} - ${osVersion}`;
  deviceInfo.deviceType = deviceType;
  deviceInfo.osVersion = osVersion;
};

const handleSendCode = async () => {
  if (!form.licenseKey || form.licenseKey.length < 32) {
    Message.warning("请输入有效的许可证密钥");
    return;
  }

  sendingCode.value = true;
  try {
    await verify.sendCode({
      licenseId: 0,
      pluginId: 0,
      purpose: 1,
      deviceHash: deviceInfo.deviceId
    });
    Message.success("激活码已发送，请查收");
    countdown.value = 60;
    countdownTimer = window.setInterval(() => {
      countdown.value--;
      if (countdown.value <= 0) {
        clearInterval(countdownTimer);
      }
    }, 1000);
  } catch (error) {
    console.error(error);
    Message.error("发送激活码失败，请稍后重试");
  } finally {
    sendingCode.value = false;
  }
};

const handleActivate = async () => {
  try {
    await formRef.value.validate();
  } catch {
    return;
  }

  if (form.code.length !== 6) {
    Message.warning("请输入6位验证码");
    return;
  }

  activating.value = true;
  resultMessage.value = "";

  try {
    const checkResult = await verify.checkCode({
      licenseId: 0,
      code: form.code,
      deviceHash: deviceInfo.deviceId
    });

    if (checkResult.success) {
      await license.bind({
        licenseId: 0,
        deviceId: deviceInfo.deviceId,
        deviceName: deviceInfo.deviceName,
        deviceType: deviceInfo.deviceType,
        osVersion: deviceInfo.osVersion
      });

      activatedTime.value = new Date().toLocaleString();
      expireTime.value = "一年后";

      resultType.value = "success";
      resultMessage.value = "激活成功！";
      successModalVisible.value = true;
      Message.success("插件激活成功");
    } else {
      resultType.value = "error";
      resultMessage.value = checkResult.message || "激活失败，请检查验证码";
    }
  } catch (error: any) {
    resultType.value = "error";
    resultMessage.value = error?.message || "激活失败，请稍后重试";
    Message.error("激活失败");
  } finally {
    activating.value = false;
  }
};

onMounted(() => {
  getDeviceInfo();
});

onUnmounted(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer);
  }
});
</script>

<style scoped lang="scss">
.activate-container {
  display: flex;
  justify-content: center;
  align-items: flex-start;
  padding: 40px 20px;
  min-height: calc(100vh - 120px);
}

.activate-card {
  width: 100%;
  max-width: 500px;
  border-radius: 8px;
}

.card-header {
  text-align: center;
  margin-bottom: 32px;

  .header-icon {
    color: rgb(var(--success-6));
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

.activate-form {
  .device-info {
    margin: 16px 0;

    .device-id {
      font-family: monospace;
      color: var(--color-primary);
    }
  }
}

.result-message {
  margin-top: 16px;
  padding: 12px;
  border-radius: 4px;

  &.success {
    background-color: rgb(var(--success-1));
    border: 1px solid rgb(var(--success-3));
  }

  &.error {
    background-color: rgb(var(--danger-1));
    border: 1px solid rgb(var(--danger-3));
  }
}

.success-content {
  text-align: center;

  .success-icon {
    color: rgb(var(--success-6));
    margin-bottom: 16px;
  }

  .success-text {
    font-size: 16px;
    margin-bottom: 24px;
  }
}
</style>

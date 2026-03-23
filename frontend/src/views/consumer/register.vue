<template>
  <div class="consumer-container">
    <a-card>
      <template #title>用户注册</template>
      <a-form :model="form" layout="vertical">
        <a-form-item label="手机号" required>
          <a-input v-model="form.phone" placeholder="请输入手机号" />
        </a-form-item>
        <a-form-item label="密码" required>
          <a-input-password v-model="form.password" placeholder="请输入密码" />
        </a-form-item>
        <a-form-item label="验证码" required>
          <a-input-search v-model="form.sms_code" placeholder="请输入验证码" search-text="发送验证码" @search="sendCode" />
        </a-form-item>
        <a-form-item>
          <a-button type="primary" @click="handleRegister">注册</a-button>
        </a-form-item>
      </a-form>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { Message } from '@arco-design/web-vue';
import { consumerApi } from '@/api/modules/consumer';
import { smsApi } from '@/api/modules/consumer/sms';

const form = ref({
  phone: '',
  password: '',
  sms_code: '',
});

const sendCode = async () => {
  if (!form.value.phone) {
    Message.warning('请输入手机号');
    return;
  }
  try {
    await smsApi.sendCode({ phone: form.value.phone, sms_type: 'verification' });
    Message.success('验证码已发送');
  } catch (error: any) {
    Message.error(error.message || '发送失败');
  }
};

const handleRegister = async () => {
  if (!form.value.phone || !form.value.password || !form.value.sms_code) {
    Message.warning('请填写完整信息');
    return;
  }
  try {
    await consumerApi.register(form.value);
    Message.success('注册成功');
  } catch (error: any) {
    Message.error(error.message || '注册失败');
  }
};
</script>

<style scoped>
.consumer-container {
  padding: 20px;
}
</style>
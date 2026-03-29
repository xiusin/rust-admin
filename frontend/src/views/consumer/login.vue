<template>
  <div class="consumer-login-container">
    <a-card>
      <template #title>用户登录</template>
      <a-form :model="form" layout="vertical">
        <a-form-item label="手机号" required>
          <a-input v-model="form.phone" placeholder="请输入手机号" />
        </a-form-item>
        <a-form-item label="密码" required>
          <a-input-password v-model="form.password" placeholder="请输入密码" />
        </a-form-item>
        <a-form-item>
          <a-button type="primary" @click="handleLogin">登录</a-button>
        </a-form-item>
      </a-form>
    </a-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { Message } from "@arco-design/web-vue";
import { useRouter } from "vue-router";
import { consumerApi } from "@/api/modules/consumer";

const router = useRouter();
const form = ref({
  phone: "",
  password: ""
});

const handleLogin = async () => {
  if (!form.value.phone || !form.value.password) {
    Message.warning("请填写完整信息");
    return;
  }
  try {
    const res = await consumerApi.login(form.value);
    if (res.data) {
      localStorage.setItem("consumer_token", res.token);
      localStorage.setItem("consumer_info", JSON.stringify(res.consumer));
      Message.success("登录成功");
      router.push("/consumer/profile");
    }
  } catch (error: any) {
    Message.error(error.message || "登录失败");
  }
};
</script>

<style scoped>
.consumer-login-container {
  padding: 20px;
  max-width: 400px;
  margin: 0 auto;
}
</style>

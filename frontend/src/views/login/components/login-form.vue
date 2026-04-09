<template>
  <div class="login-form-container">
    <div class="login-form-wrapper">
      <a-form
        :rules="rules"
        :model="form"
        layout="vertical"
        @submit="onSubmit"
        class="login-form"
      >
        <a-form-item field="username" :hide-asterisk="true">
          <a-input
            v-model="form.username"
            allow-clear
            placeholder="请输入账号"
            class="custom-input"
            size="large"
          >
            <template #prefix>
              <icon-user class="input-icon" />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item field="password" :hide-asterisk="true">
          <a-input-password
            v-model="form.password"
            allow-clear
            placeholder="请输入密码"
            class="custom-input"
            size="large"
          >
            <template #prefix>
              <icon-lock class="input-icon" />
            </template>
          </a-input-password>
        </a-form-item>
        <a-form-item field="remember" class="form-options">
          <div class="options-wrapper">
            <a-checkbox v-model="form.remember" class="remember-checkbox">
              记住密码
            </a-checkbox>
            <a-link class="forgot-link" hover>忘记密码？</a-link>
          </div>
        </a-form-item>
        <a-form-item class="submit-item">
          <a-button
            long
            type="primary"
            html-type="submit"
            size="large"
            :loading="loading"
            class="submit-btn"
          >
            {{ loading ? '登录中...' : '登录' }}
          </a-button>
        </a-form-item>
      </a-form>
      <div class="divider">
        <span>或</span>
      </div>
      <div class="social-login">
        <div class="social-item" v-for="(social, index) in socialList" :key="index">
          <span class="social-icon">{{ social.icon }}</span>
        </div>
      </div>
      <div class="register-link">
        还没有账号？
        <a-link class="link-text" hover>立即注册</a-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { Message } from "@arco-design/web-vue";
import { useUserStore } from "@/store/modules/user-info";
import { useRouteConfigStore } from "@/store/modules/route-config";
import { useSystemStore } from "@/store/modules/system";

const userStores = useUserStore();
const routeStore = useRouteConfigStore();
const router = useRouter();
const loading = ref(false);

const form = ref({
  username: "admin",
  password: "123456",
  remember: false
});

const rules = ref({
  username: [
    {
      required: true,
      message: "请输入账号",
      trigger: "blur"
    }
  ],
  password: [
    {
      required: true,
      message: "请输入密码",
      trigger: "blur"
    },
    {
      minLength: 6,
      message: "密码长度不能少于6位",
      trigger: "blur"
    }
  ]
});

const socialList = [
  { icon: "微信", name: "wechat" },
  { icon: "QQ", name: "qq" },
  { icon: "G", name: "google" }
];

const onSubmit = async ({ errors }: any) => {
  if (errors) return;
  await onLogin();
};

const onLogin = async () => {
  try {
    loading.value = true;
    await userStores.login(form.value);
    Message.success("登录成功");
    await userStores.setUserInfo();
    useSystemStore().setDictData();
    router.replace("/home");
  } catch (error: any) {
    Message.error(error?.message || "登录失败");
  } finally {
    loading.value = false;
  }
};
</script>

<style lang="scss" scoped>
.login-form-container {
  width: 100%;
  animation: formFadeIn 0.8s ease-out 0.4s backwards;
}

@keyframes formFadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.login-form-wrapper {
  width: 100%;
}

.login-form {
  :deep(.arco-form-item) {
    margin-bottom: 24px;
  }

  :deep(.arco-form-item-label) {
    padding-bottom: 8px;
  }
}

.custom-input {
  :deep(.arco-input-wrapper) {
    border-radius: 12px;
    transition: all 0.3s ease;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
  }

  :deep(.arco-input-wrapper:hover) {
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.15);
  }

  :deep(.arco-input-wrapper.arco-input-focus) {
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }
}

.input-icon {
  color: #86909c;
  font-size: 18px;
  transition: color 0.3s ease;
}

.custom-input:focus-within .input-icon {
  color: #667eea;
}

.form-options {
  margin-bottom: 32px !important;
}

.options-wrapper {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.remember-checkbox {
  :deep(.arco-checkbox-label) {
    color: #4e5969;
    font-size: 14px;
  }
}

.forgot-link {
  font-size: 14px;
  color: #667eea;
  transition: all 0.3s ease;
}

.forgot-link:hover {
  color: #764ba2;
}

.submit-item {
  margin-bottom: 0 !important;
}

.submit-btn {
  height: 48px;
  font-size: 16px;
  font-weight: 600;
  border-radius: 12px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  box-shadow: 0 4px 16px rgba(102, 126, 234, 0.3);
  transition: all 0.3s ease;
}

.submit-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.submit-btn:active {
  transform: translateY(0);
}

.divider {
  display: flex;
  align-items: center;
  margin: 32px 0;
  color: #86909c;
  font-size: 14px;

  &::before,
  &::after {
    content: "";
    flex: 1;
    height: 1px;
    background: #e5e6eb;
  }

  span {
    padding: 0 16px;
  }
}

.social-login {
  display: flex;
  justify-content: center;
  gap: 16px;
  margin-bottom: 24px;
}

.social-item {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 12px;
  background: #f7f8fa;
  border: 1px solid #e5e6eb;
  cursor: pointer;
  transition: all 0.3s ease;
}

.social-item:hover {
  background: #fff;
  border-color: #667eea;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.15);
}

.social-icon {
  font-size: 20px;
  font-weight: 600;
  color: #4e5969;
}

.register-link {
  text-align: center;
  font-size: 14px;
  color: #86909c;
}

.link-text {
  color: #667eea;
  font-weight: 500;
  transition: color 0.3s ease;
}

.link-text:hover {
  color: #764ba2;
}
</style>

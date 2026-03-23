<template>
  <div>
    <div class="login_form_box">
      <a-form :rules="rules" :model="form" layout="vertical" @submit="onSubmit">
        <a-form-item field="username" :hide-asterisk="true">
          <a-input v-model="form.username" allow-clear placeholder="请输入账号：admin/common">
            <template #prefix>
              <icon-user />
            </template>
          </a-input>
        </a-form-item>
        <a-form-item field="password" :hide-asterisk="true">
          <a-input-password v-model="form.password" allow-clear placeholder="请输入密码">
            <template #prefix>
              <icon-lock />
            </template>
          </a-input-password>
        </a-form-item>
        <a-form-item field="remember">
          <div class="remember">
            <a-checkbox v-model="form.remember">记住密码</a-checkbox>
            <div class="forgot-password">忘记密码</div>
          </div>
        </a-form-item>
        <a-form-item>
          <a-button long type="primary" html-type="submit">登录</a-button>
        </a-form-item>
      </a-form>
    </div>
    <div class="register">注册账号</div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { Message } from "@arco-design/web-vue";
import { useUserStore } from "@/store/modules/user-info";
import { useRouteConfigStore } from "@/store/modules/route-config";
import { useSystemStore } from "@/store/modules/system";
let userStores = useUserStore();
const routeStore = useRouteConfigStore();
const router = useRouter();
const form = ref({
  username: "admin",
  password: "123456",
  remember: false
});
const rules = ref({
  username: [
    {
      required: true,
      message: "请输入账号"
    }
  ],
  password: [
    {
      required: true,
      message: "请输入密码"
    }
  ]
});

// 提交表单
const onSubmit = async ({ errors }: any) => {
  if (errors) return;
  onLogin();
};

// 登录
const onLogin = async () => {
  try {
    // 调用store的login方法
    await userStores.login(form.value);
    Message.success("登录成功");
    // 获取用户信息
    await userStores.setUserInfo();
    // 设置字典
    useSystemStore().setDictData();
    // 跳转首页
    router.replace("/home");
  } catch (error: any) {
    Message.error(error?.message || "登录失败");
  }
};
</script>

<style lang="scss" scoped>
.login_form_box {
  margin-top: 28px;
  .verifyCode {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }
  .remember {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    .forgot-password {
      color: $color-primary;
      cursor: pointer;
    }
  }
}
.register {
  font-size: $font-size-body-1;
  color: $color-text-3;
  text-align: center;
  cursor: pointer;
}
</style>
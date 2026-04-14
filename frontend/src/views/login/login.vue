<template>
  <div class="login-container">
    <div class="login-wrapper">
      <div class="login-card">
        <div class="bg-gradient"></div>
        <div class="login-content">
          <div class="login-left" :class="{ 'animate-fadeInLeft': isPc }">
            <LoginBanner v-if="isPc" />
          </div>
          <div class="login-right" :class="{ 'animate-fadeInRight': isPc }">
            <div class="login-box">
              <div class="login-header">
                <div class="logo-section">
                  <s-svg-icon name="snow" size="32" />
                  <h1 class="app-title">{{ title }}</h1>
                </div>
                <h2 class="welcome-text">欢迎回来</h2>
                <p class="welcome-desc">登录您的电商 SaaS 平台</p>
              </div>
              <LoginForm />
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import LoginBanner from "@/views/login/components/login-banner.vue";
import LoginForm from "@/views/login/components/login-form.vue";
import { useDevicesSize } from "@/hooks/useDevicesSize";

const { isPc } = useDevicesSize();
const title = import.meta.env.VITE_GLOB_APP_TITLE;
</script>

<style lang="scss" scoped>
.login-container {
  position: relative;
  width: 100%;
  min-height: 100vh;
  overflow: hidden;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.bg-gradient {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.3) 0%, transparent 50%),
              radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.3) 0%, transparent 50%),
              radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.2) 0%, transparent 50%);
  z-index: 0;
}

.login-wrapper {
  position: relative;
  z-index: 1;
  width: 100%;
  max-width: 900px;
  padding: 16px;
}

.login-card {
  position: relative;
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  box-shadow: 0 12px 30px rgba(0, 0, 0, 0.2);
  overflow: hidden;
  backdrop-filter: blur(15px);
}

.login-content {
  display: flex;
  min-height: 400px;
}

.login-right {
  width: 360px;
  padding: 32px 24px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.login-header {
  margin-bottom: 24px;
}

.logo-section {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 16px;
  color: #667eea;
}

.app-title {
  font-size: 20px;
  font-weight: 700;
  margin: 0;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.welcome-text {
  font-size: 24px;
  font-weight: 700;
  color: #1d2129;
  margin: 0 0 6px 0;
}

.welcome-desc {
  font-size: 12px;
  color: #86909c;
  margin: 0;
}

.animate-fadeInLeft {
  animation: fadeInLeft 0.8s ease-out forwards;
}

.animate-fadeInRight {
  animation: fadeInRight 0.8s ease-out forwards;
}

@keyframes fadeInLeft {
  from {
    opacity: 0;
    transform: translateX(-30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@keyframes fadeInRight {
  from {
    opacity: 0;
    transform: translateX(30px);
  }
  to {
    opacity: 1;
    transform: translateX(0);
  }
}

@media (max-width: 960px) {
  .login-content {
    flex-direction: column;
    min-height: auto;
  }

  .login-left {
    display: none;
  }

  .login-right {
    width: 100%;
    padding: 40px 32px;
  }

  .login-wrapper {
    padding: 16px;
  }
}
</style>

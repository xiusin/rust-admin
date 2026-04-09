<template>
  <div class="banner-container">
    <div class="banner-bg-circles">
      <div class="circle circle-1"></div>
      <div class="circle circle-2"></div>
      <div class="circle circle-3"></div>
    </div>
    <div class="banner-content">
      <div class="banner-header">
        <s-svg-icon name="snow" size="36" />
        <span class="banner-title">{{ title }}</span>
      </div>
      <div class="banner-main">
        <div class="banner-illustration">
          <s-svg-icon name="数据时代" size="100%" />
        </div>
        <div class="banner-features">
          <div class="feature-item" v-for="(feature, index) in features" :key="index">
            <div class="feature-icon">{{ feature.icon }}</div>
            <div class="feature-text">
              <h3>{{ feature.title }}</h3>
              <p>{{ feature.desc }}</p>
            </div>
          </div>
        </div>
      </div>
      <div class="banner-footer">
        <p>电商 SaaS 平台 · 专业 · 高效 · 安全</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
const title = import.meta.env.VITE_GLOB_APP_TITLE;

const features = [
  {
    icon: "⚡",
    title: "高效管理",
    desc: "一站式电商运营解决方案"
  },
  {
    icon: "📊",
    title: "数据洞察",
    desc: "实时数据分析与智能报表"
  },
  {
    icon: "🔒",
    title: "安全可靠",
    desc: "企业级安全防护体系"
  }
];
</script>

<style lang="scss" scoped>
.banner-container {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  padding: 60px 50px;
  overflow: hidden;
}

.banner-bg-circles {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow: hidden;
  pointer-events: none;
}

.circle {
  position: absolute;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.1);
  animation: float 6s ease-in-out infinite;
}

.circle-1 {
  width: 300px;
  height: 300px;
  top: -100px;
  right: -100px;
  animation-delay: 0s;
}

.circle-2 {
  width: 200px;
  height: 200px;
  bottom: 20%;
  left: -50px;
  animation-delay: 2s;
}

.circle-3 {
  width: 150px;
  height: 150px;
  top: 40%;
  right: 10%;
  animation-delay: 4s;
}

@keyframes float {
  0%, 100% {
    transform: translateY(0) scale(1);
  }
  50% {
    transform: translateY(-20px) scale(1.05);
  }
}

.banner-content {
  position: relative;
  z-index: 1;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.banner-header {
  display: flex;
  align-items: center;
  gap: 14px;
  margin-bottom: 40px;
  color: white;
  animation: slideDown 0.8s ease-out 0.2s backwards;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.banner-title {
  font-size: 32px;
  font-weight: 700;
  letter-spacing: -0.5px;
}

.banner-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.banner-illustration {
  width: 100%;
  max-width: 400px;
  margin: 0 auto 40px;
  animation: scaleIn 0.8s ease-out 0.4s backwards;
}

@keyframes scaleIn {
  from {
    opacity: 0;
    transform: scale(0.9);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

.banner-features {
  display: flex;
  flex-direction: column;
  gap: 24px;
  animation: slideUp 0.8s ease-out 0.6s backwards;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.feature-item {
  display: flex;
  align-items: flex-start;
  gap: 16px;
  padding: 16px 20px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  backdrop-filter: blur(10px);
  transition: all 0.3s ease;
}

.feature-item:hover {
  background: rgba(255, 255, 255, 0.15);
  transform: translateX(8px);
}

.feature-icon {
  font-size: 32px;
  line-height: 1;
  flex-shrink: 0;
}

.feature-text {
  flex: 1;
}

.feature-text h3 {
  margin: 0 0 6px 0;
  font-size: 16px;
  font-weight: 600;
  color: white;
}

.feature-text p {
  margin: 0;
  font-size: 13px;
  color: rgba(255, 255, 255, 0.8);
  line-height: 1.5;
}

.banner-footer {
  margin-top: auto;
  padding-top: 30px;
  text-align: center;
  animation: fadeIn 0.8s ease-out 0.8s backwards;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

.banner-footer p {
  margin: 0;
  font-size: 14px;
  color: rgba(255, 255, 255, 0.7);
  letter-spacing: 1px;
}
</style>

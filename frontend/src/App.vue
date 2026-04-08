<script setup lang="ts">
import { ref, computed } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute();
const collapsed = ref(false);

const breadcrumbItems = computed(() => {
  const items = [
    { path: '/', title: '首页' }
  ];
  
  if (route.path !== '/') {
    const pathParts = route.path.split('/').filter(Boolean);
    let currentPath = '';
    
    pathParts.forEach(part => {
      currentPath += `/${part}`;
      items.push({
        path: currentPath,
        title: part.charAt(0).toUpperCase() + part.slice(1)
      });
    });
  }
  
  return items;
});

const toggleSidebar = () => {
  collapsed.value = !collapsed.value;
};
</script>

<template>
  <el-container class="app-container">
    <!-- 顶部导航栏 -->
    <el-header height="64px" class="app-header">
      <div class="header-left">
        <el-button type="text" @click="toggleSidebar" class="sidebar-toggle">
          <el-icon :class="{ 'rotate': !collapsed }">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
            </svg>
          </el-icon>
        </el-button>
        <div class="logo" :class="{ 'collapsed': collapsed }">管理系统</div>
      </div>
      <div class="header-right">
        <el-dropdown>
          <span class="user-info">
            <el-avatar size="small" :class="{ 'pulse': true }">U</el-avatar>
            <span :class="{ 'fade': collapsed }">用户</span>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>
                <span class="dropdown-item">个人中心</span>
              </el-dropdown-item>
              <el-dropdown-item>
                <span class="dropdown-item">退出登录</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </el-header>
    
    <el-container>
      <!-- 左侧菜单 -->
      <el-aside :width="collapsed ? '64px' : '200px'" class="app-sidebar" :class="{ 'collapsed': collapsed }">
        <el-menu
          :default-active="route.path"
          class="sidebar-menu"
          :collapse="collapsed"
          router
          :collapse-transition="false"
        >
          <el-menu-item index="/">
            <el-icon class="menu-icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg></el-icon>
            <template #title><span class="menu-title">首页</span></template>
          </el-menu-item>
          <el-sub-menu index="/about">
            <template #title>
              <el-icon class="menu-icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg></el-icon>
              <span class="menu-title">关于</span>
            </template>
            <el-menu-item index="/about">
              <span class="menu-title">关于我们</span>
            </el-menu-item>
            <el-menu-item index="/about/team">
              <span class="menu-title">团队介绍</span>
            </el-menu-item>
          </el-sub-menu>
          <el-menu-item index="/dashboard">
            <el-icon class="menu-icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg></el-icon>
            <template #title><span class="menu-title">仪表盘</span></template>
          </el-menu-item>
          <el-menu-item index="/settings">
            <el-icon class="menu-icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg></el-icon>
            <template #title><span class="menu-title">设置</span></template>
          </el-menu-item>
          <el-menu-item index="/users">
            <el-icon class="menu-icon"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg></el-icon>
            <template #title><span class="menu-title">用户管理</span></template>
          </el-menu-item>
        </el-menu>
      </el-aside>
      
      <!-- 主内容区 -->
      <el-main class="app-main">
        <!-- 面包屑导航 -->
        <el-breadcrumb separator="/" class="breadcrumb">
          <el-breadcrumb-item v-for="(item, index) in breadcrumbItems" :key="item.path" :to="item.path">
            <span class="breadcrumb-item" :class="{ 'active': index === breadcrumbItems.length - 1 }">
              {{ item.title }}
            </span>
          </el-breadcrumb-item>
        </el-breadcrumb>
        
        <!-- 路由视图 -->
        <transition name="fade" mode="out-in">
          <router-view />
        </transition>
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.app-container {
  height: 100vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.app-header {
  background-color: var(--bg-primary);
  box-shadow: var(--shadow-sm);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 var(--spacing-lg);
  height: 64px;
  z-index: 10;
}

.header-left {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.sidebar-toggle {
  color: var(--text-primary);
  font-size: 20px;
  transition: color var(--transition-fast);
}

.sidebar-toggle:hover {
  color: var(--primary);
}

.logo {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--primary);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.user-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
  cursor: pointer;
}

.user-info:hover {
  background-color: var(--bg-secondary);
}

.user-info span {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-weight: 500;
}

.app-sidebar {
  background-color: var(--bg-tertiary);
  transition: all var(--transition);
  overflow: hidden;
}

.sidebar-menu {
  border-right: none;
  height: 100%;
  background-color: transparent;
}

.sidebar-menu .el-menu-item,
.sidebar-menu .el-sub-menu__title {
  color: var(--text-secondary);
  height: 56px;
  line-height: 56px;
  margin: 0 var(--spacing-sm);
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
  font-size: var(--font-size-sm);
  font-weight: 500;
}

.sidebar-menu .el-menu-item:hover,
.sidebar-menu .el-sub-menu__title:hover {
  background-color: rgba(64, 158, 255, 0.1);
  color: var(--primary);
}

.sidebar-menu .el-menu-item.is-active {
  background-color: var(--primary);
  color: #fff;
  box-shadow: var(--shadow-sm);
}

.sidebar-menu .el-menu-item.is-active:hover {
  background-color: var(--primary-dark);
}

.app-main {
  padding: var(--spacing-lg);
  overflow-y: auto;
  background-color: var(--bg-secondary);
  flex: 1;
}

.breadcrumb {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
}

/* 动画效果 */
/* 旋转动画 */
.rotate {
  transition: transform var(--transition);
  transform: rotate(90deg);
}

/* 淡入淡出动画 */
.fade {
  transition: opacity var(--transition-fast);
}

.collapsed .fade {
  opacity: 0;
  pointer-events: none;
}

/* 菜单图标动画 */
.menu-icon {
  transition: all var(--transition-fast);
  font-size: 18px;
}

.sidebar-menu .el-menu-item:hover .menu-icon,
.sidebar-menu .el-sub-menu__title:hover .menu-icon {
  transform: scale(1.1);
}

.sidebar-menu .el-menu-item.is-active .menu-icon {
  transform: scale(1.1);
}

/* 菜单项文字动画 */
.menu-title {
  transition: all var(--transition-fast);
}

/* 面包屑导航动画 */
.breadcrumb-item {
  transition: all var(--transition-fast);
}

.breadcrumb-item.active {
  color: var(--primary);
  font-weight: 500;
}

/* 下拉菜单项动画 */
.dropdown-item {
  transition: all var(--transition-fast);
  display: block;
  padding: var(--spacing-xs) 0;
}

.el-dropdown-item:hover .dropdown-item {
  color: var(--primary);
  transform: translateX(4px);
}

/* 头像脉冲动画 */
.pulse {
  position: relative;
}

.pulse::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  border-radius: 50%;
  border: 2px solid var(--primary);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% {
    transform: scale(1);
    opacity: 1;
  }
  70% {
    transform: scale(1.2);
    opacity: 0;
  }
  100% {
    transform: scale(1);
    opacity: 0;
  }
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .app-header {
    padding: 0 var(--spacing-md);
  }
  
  .logo {
    font-size: var(--font-size);
  }
  
  .app-sidebar {
    position: fixed;
    left: 0;
    top: 64px;
    height: calc(100vh - 64px);
    z-index: 100;
    transform: translateX(-100%);
    box-shadow: var(--shadow-lg);
    transition: transform var(--transition) !important;
  }
  
  .app-sidebar:not(.collapsed) {
    transform: translateX(0);
  }
  
  .app-main {
    padding: var(--spacing-md);
  }
  
  .breadcrumb {
    margin-bottom: var(--spacing-md);
  }
}
</style>

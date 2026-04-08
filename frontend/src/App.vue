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
    <el-header height="60px" class="app-header">
      <div class="header-left">
        <el-button type="text" @click="toggleSidebar" class="sidebar-toggle">
          <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/></svg></el-icon>
        </el-button>
        <div class="logo">管理系统</div>
      </div>
      <div class="header-right">
        <el-dropdown>
          <span class="user-info">
            <el-avatar size="small">U</el-avatar>
            <span>用户</span>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item>个人中心</el-dropdown-item>
              <el-dropdown-item>退出登录</el-dropdown-item>
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
        >
          <el-menu-item index="/">
            <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg></el-icon>
            <template #title>首页</template>
          </el-menu-item>
          <el-sub-menu index="/about">
            <template #title>
              <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg></el-icon>
              <span>关于</span>
            </template>
            <el-menu-item index="/about">关于我们</el-menu-item>
            <el-menu-item index="/about/team">团队介绍</el-menu-item>
          </el-sub-menu>
          <el-menu-item index="/dashboard">
            <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg></el-icon>
            <template #title>仪表盘</template>
          </el-menu-item>
          <el-menu-item index="/settings">
            <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg></el-icon>
            <template #title>设置</template>
          </el-menu-item>
          <el-menu-item index="/users">
            <el-icon><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg></el-icon>
            <template #title>用户管理</template>
          </el-menu-item>
        </el-menu>
      </el-aside>
      
      <!-- 主内容区 -->
      <el-main class="app-main">
        <!-- 面包屑导航 -->
        <el-breadcrumb separator="/" class="breadcrumb">
          <el-breadcrumb-item v-for="item in breadcrumbItems" :key="item.path" :to="item.path">
            {{ item.title }}
          </el-breadcrumb-item>
        </el-breadcrumb>
        
        <!-- 路由视图 -->
        <router-view />
      </el-main>
    </el-container>
  </el-container>
</template>

<style scoped>
.app-container {
  height: 100vh;
  overflow: hidden;
}

.app-header {
  background-color: #fff;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 20px;
}

.header-left {
  display: flex;
  align-items: center;
}

.sidebar-toggle {
  margin-right: 20px;
}

.logo {
  font-size: 18px;
  font-weight: bold;
  color: #409EFF;
}

.header-right {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  cursor: pointer;
}

.user-info span {
  margin-left: 10px;
}

.app-sidebar {
  background-color: #304156;
  transition: width 0.3s;
}

.sidebar-menu {
  border-right: none;
  height: 100%;
}

.sidebar-menu .el-menu-item,
.sidebar-menu .el-sub-menu__title {
  color: #bfcbd9;
  height: 60px;
  line-height: 60px;
  margin: 0 10px;
  border-radius: 4px;
}

.sidebar-menu .el-menu-item:hover,
.sidebar-menu .el-sub-menu__title:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.sidebar-menu .el-menu-item.is-active {
  background-color: #409EFF;
  color: #fff;
}

.app-main {
  padding: 20px;
  overflow-y: auto;
  background-color: #f5f7fa;
}

.breadcrumb {
  margin-bottom: 20px;
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .app-sidebar {
    position: fixed;
    left: 0;
    top: 60px;
    height: calc(100vh - 60px);
    z-index: 100;
    transform: translateX(-100%);
  }
  
  .app-sidebar:not(.collapsed) {
    transform: translateX(0);
  }
  
  .app-main {
    margin-left: 0;
  }
}
</style>

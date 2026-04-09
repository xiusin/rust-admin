<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();
const collapsed = ref(false);
const showMobileMenu = ref(false);
const isMobile = ref(false);

// 检查是否为移动设备
const checkMobile = () => {
  isMobile.value = window.innerWidth <= 768;
  if (isMobile.value) {
    collapsed.value = true;
  }
};

// 菜单数据
const menuItems = [
  {
    index: '/',
    title: '首页',
    icon: 'home',
    path: '/'
  },
  {
    index: '/dashboard',
    title: '仪表盘',
    icon: 'dashboard',
    path: '/dashboard'
  },
  {
    index: '/system',
    title: '系统管理',
    icon: 'settings',
    children: [
      {
        index: '/users',
        title: '用户管理',
        icon: 'user',
        path: '/users'
      },
      {
        index: '/depts',
        title: '部门管理',
        icon: 'building',
        path: '/depts'
      },
      {
        index: '/roles',
        title: '角色管理',
        icon: 'role',
        path: '/roles'
      },
      {
        index: '/menus',
        title: '菜单管理',
        icon: 'menu',
        path: '/menus'
      }
    ]
  },
  {
    index: '/permissions',
    title: '权限管理',
    icon: 'lock',
    children: [
      {
        index: '/api-permissions',
        title: 'API权限',
        icon: 'api',
        path: '/api-permissions'
      },
      {
        index: '/permission-categories',
        title: '权限分类',
        icon: 'category',
        path: '/permission-categories'
      }
    ]
  },
  {
    index: '/monitoring',
    title: '系统监控',
    icon: 'monitor',
    children: [
      {
        index: '/login-logs',
        title: '登录日志',
        icon: 'log',
        path: '/login-logs'
      },
      {
        index: '/operation-logs',
        title: '操作日志',
        icon: 'operation',
        path: '/operation-logs'
      },
      {
        index: '/server-info',
        title: '服务器信息',
        icon: 'server',
        path: '/server-info'
      }
    ]
  },
  {
    index: '/jobs',
    title: '任务管理',
    icon: 'job',
    path: '/jobs'
  },
  {
    index: '/about',
    title: '关于',
    icon: 'about',
    children: [
      {
        index: '/about',
        title: '关于我们',
        icon: 'info',
        path: '/about'
      },
      {
        index: '/about/team',
        title: '团队介绍',
        icon: 'team',
        path: '/about/team'
      }
    ]
  }
];

// 面包屑导航
const breadcrumbItems = computed(() => {
  const items = [
    { path: '/', title: '首页' }
  ];
  
  if (route.path !== '/') {
    const pathParts = route.path.split('/').filter(Boolean);
    let currentPath = '';
    
    pathParts.forEach(part => {
      currentPath += `/${part}`;
      
      // 查找对应的菜单标题
      let title = part.charAt(0).toUpperCase() + part.slice(1);
      
      // 递归查找菜单标题
      const findMenuTitle = (menuList: any[], currentPath: string) => {
        for (const menu of menuList) {
          if (menu.path === currentPath) {
            title = menu.title;
            return true;
          }
          if (menu.children) {
            if (findMenuTitle(menu.children, currentPath)) {
              return true;
            }
          }
        }
        return false;
      };
      
      findMenuTitle(menuItems, currentPath);
      
      items.push({
        path: currentPath,
        title
      });
    });
  }
  
  return items;
});

// 切换侧边栏
const toggleSidebar = () => {
  collapsed.value = !collapsed.value;
  if (isMobile.value) {
    showMobileMenu.value = !showMobileMenu.value;
  }
};

// 关闭移动菜单
const closeMobileMenu = () => {
  if (isMobile.value) {
    showMobileMenu.value = false;
  }
};

// 监听路由变化，关闭移动菜单
watch(() => route.path, () => {
  closeMobileMenu();
});

// 监听窗口大小变化
onMounted(() => {
  checkMobile();
  window.addEventListener('resize', checkMobile);
});

// 菜单项点击事件
const handleMenuClick = () => {
  closeMobileMenu();
};

// 图标映射
const getIcon = (iconName: string) => {
  const icons: Record<string, string> = {
    home: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M10 20v-6h4v6h5v-8h3L12 3 2 12h3v8z"/></svg>',
    dashboard: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 13h8V3H3v10zm0 8h8v-6H3v6zm10 0h8V11h-8v10zm0-18v6h8V3h-8z"/></svg>',
    settings: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    user: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/></svg>',
    building: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-5 14h-4v-4h4v4zm0-6h-4v-2h4v2zm0-4h-4V5h4v2z"/></svg>',
    role: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>',
    menu: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/></svg>',
    lock: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 17c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2s-2 .9-2 2v6c0 1.1.9 2 2 2zm6-9h-1V6c0-2.76-2.24-5-5-5S7 3.24 7 6v2H6c-1.1 0-2 .9-2 2v10c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V10c0-1.1-.9-2-2-2zM8.9 6c0-1.71 1.39-3.1 3.1-3.1s3.1 1.39 3.1 3.1v2H8.9V6z"/></svg>',
    api: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    category: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M11 7h6v2h-6zm0 4h6v2h-6zm0 4h6v2h-6zM7 7h2v2H7zm0 4h2v2H7zm0 4h2v2H7zM21 3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H3V5h18v14z"/></svg>',
    monitor: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M20 3H4c-1.1 0-2 .9-2 2v11c0 1.1.9 2 2 2h3l-1 1v1h10v-1l-1-1h3c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 13H4V5h16v11z"/></svg>',
    log: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 15h-2v-2h2v2zm0-4h-2V7h2v6z"/></svg>',
    operation: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19.14 12.94c.04-.3.06-.61.06-.94 0-.32-.02-.64-.07-.94l2.03-1.58c.18-.14.23-.41.12-.61l-1.92-3.32c-.12-.22-.37-.29-.59-.22l-2.39.96c-.5-.38-1.03-.7-1.62-.94l-.36-2.54c-.04-.24-.24-.41-.48-.41h-3.84c-.24 0-.43.17-.47.41l-.36 2.54c-.59.24-1.13.57-1.62.94l-2.39-.96c-.22-.08-.47 0-.59.22L2.74 8.87c-.12.21-.08.47.12.61l2.03 1.58c-.05.3-.09.63-.09.94s.02.64.07.94l-2.03 1.58c-.18.14-.23.41-.12.61l1.92 3.32c.12.22.37.29.59.22l2.39-.96c.5.38 1.03.7 1.62.94l.36 2.54c.05.24.24.41.48.41h3.84c.24 0 .44-.17.47-.41l.36-2.54c.59-.24 1.13-.56 1.62-.94l2.39.96c.22.08.47 0 .59-.22l1.92-3.32c.12-.22.07-.47-.12-.61l-2.01-1.58zM12 15.6c-1.98 0-3.6-1.62-3.6-3.6s1.62-3.6 3.6-3.6 3.6 1.62 3.6 3.6-1.62 3.6-3.6 3.6z"/></svg>',
    server: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M4 2h16c1.1 0 2 .9 2 2v16c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2zm0 2v16h16V4H4zm2 4h12v2H6zm0 4h12v2H6zm0 4h12v2H6z"/></svg>',
    job: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 14h-2v-4H8v-2h2V9h2v2h2v2h-2v4z"/></svg>',
    about: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>',
    info: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/></svg>',
    team: '<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M16 11c1.66 0 2.99-1.34 2.99-3S17.66 5 16 5c-1.66 0-3 1.34-3 3s1.34 3 3 3zm-8 0c1.66 0 2.99-1.34 2.99-3S9.66 5 8 5C6.34 5 5 6.34 5 8s1.34 3 3 3zm0 2c-2.33 0-7 1.17-7 3.5V19h14v-2.5c0-2.33-4.67-3.5-7-3.5zm8 0c-.29 0-.62.02-.97.05 1.16.84 1.97 1.97 1.97 3.45V19h6v-2.5c0-2.33-4.67-3.5-7-3.5z"/></svg>'
  };
  return icons[iconName] || icons.home;
};
</script>

<template>
  <el-container class="app-container">
    <!-- 顶部导航栏 -->
    <el-header height="64px" class="app-header">
      <div class="header-left">
        <el-button type="text" @click="toggleSidebar" class="sidebar-toggle">
          <el-icon :class="{ 'rotate': !collapsed && !isMobile }">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
              <path d="M3 18h18v-2H3v2zm0-5h18v-2H3v2zm0-7v2h18V6H3z"/>
            </svg>
          </el-icon>
        </el-button>
        <div class="logo" :class="{ 'collapsed': collapsed && !isMobile }">管理系统</div>
      </div>
      <div class="header-right">
        <div class="header-actions">
          <!-- 搜索框 -->
          <el-input
            v-if="!collapsed || !isMobile"
            placeholder="搜索"
            class="search-input"
            size="small"
          >
            <template #prefix>
              <el-icon class="search-icon">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M15.5 14h-.79l-.28-.27C15.41 12.59 16 11.11 16 9.5 16 5.91 13.09 3 9.5 3S3 5.91 3 9.5 5.91 16 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z"/>
                </svg>
              </el-icon>
            </template>
          </el-input>
          
          <!-- 通知图标 -->
          <el-badge :value="3" class="notification-badge">
            <el-button type="text" class="notification-button">
              <el-icon class="notification-icon">
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                  <path d="M12 22c1.1 0 2-.9 2-2h-4c0 1.1.89 2 2 2zm6-6v-5c0-3.07-1.64-5.64-4.5-6.32V4c0-.83-.67-1.5-1.5-1.5s-1.5.67-1.5 1.5v.68C7.63 5.36 6 7.92 6 11v5l-2 2v1h16v-1l-2-2z"/>
                </svg>
              </el-icon>
            </el-button>
          </el-badge>
        </div>
        
        <!-- 用户信息 -->
        <el-dropdown @click="handleMenuClick">
          <span class="user-info">
            <el-avatar size="small" :class="{ 'pulse': true }">U</el-avatar>
            <span v-if="!collapsed || !isMobile" class="user-name">管理员</span>
          </span>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item @click="handleMenuClick">
                <el-icon class="dropdown-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
                  </svg>
                </el-icon>
                <span class="dropdown-item">个人中心</span>
              </el-dropdown-item>
              <el-dropdown-item @click="handleMenuClick">
                <el-icon class="dropdown-icon">
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                    <path d="M17 7l-1.41 1.41L18.17 11H8v2h10.17l-2.58 2.58L17 17l5-5z"/>
                  </svg>
                </el-icon>
                <span class="dropdown-item">退出登录</span>
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </el-header>
    
    <el-container>
      <!-- 左侧菜单 -->
      <el-aside 
        :width="collapsed && !isMobile ? '64px' : '240px'" 
        class="app-sidebar" 
        :class="{ 'collapsed': collapsed && !isMobile, 'mobile-open': showMobileMenu }"
      >
        <el-menu
          :default-active="route.path"
          class="sidebar-menu"
          :collapse="collapsed && !isMobile"
          router
          @select="handleMenuClick"
        >
          <template v-for="menu in menuItems" :key="menu.index">
            <el-menu-item v-if="!menu.children" :index="menu.path">
              <el-icon class="menu-icon" v-html="getIcon(menu.icon)"></el-icon>
              <template #title><span class="menu-title">{{ menu.title }}</span></template>
            </el-menu-item>
            <el-sub-menu v-else :index="menu.index">
              <template #title>
                <el-icon class="menu-icon" v-html="getIcon(menu.icon)"></el-icon>
                <span class="menu-title">{{ menu.title }}</span>
              </template>
              <el-menu-item v-for="child in menu.children" :key="child.index" :index="child.path">
                <el-icon class="menu-icon" v-html="getIcon(child.icon)"></el-icon>
                <template #title><span class="menu-title">{{ child.title }}</span></template>
              </el-menu-item>
            </el-sub-menu>
          </template>
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
        
        <!-- 页面标题 -->
        <div class="page-header">
          <h2>{{ breadcrumbItems[breadcrumbItems.length - 1]?.title || '首页' }}</h2>
        </div>
        
        <!-- 路由视图 -->
        <transition name="page-transition" mode="out-in">
          <router-view />
        </transition>
      </el-main>
    </el-container>
    
    <!-- 移动端遮罩 -->
    <div v-if="showMobileMenu" class="mobile-overlay" @click="closeMobileMenu"></div>
    
    <!-- 移动端底部导航 -->
    <div v-if="isMobile" class="mobile-nav">
      <div 
        v-for="menu in menuItems.filter(item => !item.children || item.children.length === 0)" 
        :key="menu.index"
        class="mobile-nav-item"
        :class="{ 'active': route.path === menu.path }"
        @click="menu.path && router.push(menu.path)"
      >
        <el-icon class="mobile-nav-icon" v-html="getIcon(menu.icon)"></el-icon>
        <span class="mobile-nav-text">{{ menu.title }}</span>
      </div>
    </div>
  </el-container>
</template>

<style scoped>
.app-container {
  height: 100vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  position: relative;
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
  transition: all var(--transition);
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
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.sidebar-toggle:hover {
  color: var(--primary);
  background-color: var(--bg-secondary);
}

.logo {
  font-size: var(--font-size-lg);
  font-weight: 600;
  color: var(--primary);
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.logo.collapsed {
  font-size: var(--font-size);
}

.header-right {
  display: flex;
  align-items: center;
  gap: var(--spacing-md);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
}

.search-input {
  width: 200px;
  transition: width var(--transition);
}

.search-input:focus-within {
  width: 250px;
}

.search-icon {
  color: var(--text-placeholder);
  transition: color var(--transition-fast);
}

.search-input:focus-within .search-icon {
  color: var(--primary);
}

.notification-badge {
  position: relative;
  transition: transform var(--transition-fast);
}

.notification-badge:hover {
  transform: scale(1.1);
}

.notification-button {
  color: var(--text-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.notification-button:hover {
  color: var(--primary);
  background-color: var(--bg-secondary);
}

.notification-icon {
  font-size: 18px;
}

.user-info {
  display: flex;
  align-items: center;
  gap: var(--spacing-sm);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius-md);
  transition: background-color var(--transition-fast);
  cursor: pointer;
  position: relative;
}

.user-info:hover {
  background-color: var(--bg-secondary);
}

.user-name {
  font-size: var(--font-size-sm);
  color: var(--text-primary);
  font-weight: 500;
  transition: all var(--transition-fast);
}

.dropdown-icon {
  font-size: 16px;
  margin-right: var(--spacing-xs);
  transition: color var(--transition-fast);
}

.el-dropdown-item:hover .dropdown-icon {
  color: var(--primary);
}

.app-sidebar {
  background-color: var(--bg-tertiary);
  transition: all var(--transition);
  overflow: hidden;
  position: relative;
}

.app-sidebar.collapsed {
  width: 64px !important;
}

.app-sidebar.mobile-open {
  transform: translateX(0) !important;
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
  display: flex;
  align-items: center;
}

.sidebar-menu .el-menu-item:hover,
.sidebar-menu .el-sub-menu__title:hover {
  background-color: rgba(64, 158, 255, 0.1);
  color: var(--primary);
  transform: translateX(4px);
}

.sidebar-menu .el-menu-item.is-active {
  background-color: var(--primary);
  color: #fff;
  box-shadow: var(--shadow-sm);
  transform: translateX(4px);
}

.sidebar-menu .el-menu-item.is-active:hover {
  background-color: var(--primary-dark);
}

.menu-icon {
  transition: all var(--transition-fast);
  font-size: 18px;
  margin-right: var(--spacing-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
}

.sidebar-menu .el-menu-item:hover .menu-icon,
.sidebar-menu .el-sub-menu__title:hover .menu-icon {
  transform: scale(1.1);
}

.sidebar-menu .el-menu-item.is-active .menu-icon {
  transform: scale(1.1);
}

.menu-title {
  transition: all var(--transition-fast);
  white-space: nowrap;
}

.app-main {
  padding: var(--spacing-lg);
  overflow-y: auto;
  background-color: var(--bg-secondary);
  flex: 1;
  transition: all var(--transition);
}

.breadcrumb {
  margin-bottom: var(--spacing-lg);
  padding: var(--spacing-sm) var(--spacing-md);
  background-color: var(--bg-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-sm);
  transition: all var(--transition);
}

.breadcrumb-item {
  transition: all var(--transition-fast);
  font-size: var(--font-size-sm);
}

.breadcrumb-item.active {
  color: var(--primary);
  font-weight: 500;
}

.page-header {
  margin-bottom: var(--spacing-lg);
  transition: all var(--transition);
}

.page-header h2 {
  font-size: var(--font-size-xl);
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  transition: all var(--transition);
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

/* 页面切换动画 */
.page-transition-enter-active,
.page-transition-leave-active {
  transition: all var(--transition);
}

.page-transition-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.page-transition-leave-to {
  opacity: 0;
  transform: translateY(-20px);
}

/* 移动端遮罩 */
.mobile-overlay {
  position: fixed;
  top: 64px;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  z-index: 99;
  animation: fadeIn var(--transition);
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* 移动端底部导航 */
.mobile-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: var(--bg-primary);
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.1);
  display: flex;
  justify-content: space-around;
  align-items: center;
  padding: var(--spacing-sm) 0;
  z-index: 100;
  animation: slideUp var(--transition);
}

@keyframes slideUp {
  from {
    transform: translateY(100%);
  }
  to {
    transform: translateY(0);
  }
}

.mobile-nav-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--spacing-xs);
  padding: var(--spacing-xs) var(--spacing-sm);
  border-radius: var(--radius);
  transition: all var(--transition-fast);
  color: var(--text-secondary);
  cursor: pointer;
  min-width: 60px;
}

.mobile-nav-item.active {
  color: var(--primary);
  background-color: var(--bg-tertiary);
}

.mobile-nav-item:hover {
  color: var(--primary);
  background-color: var(--bg-secondary);
}

.mobile-nav-icon {
  font-size: 20px;
  transition: all var(--transition-fast);
}

.mobile-nav-item.active .mobile-nav-icon {
  transform: scale(1.1);
}

.mobile-nav-text {
  font-size: var(--font-size-xs);
  transition: all var(--transition-fast);
  white-space: nowrap;
}

/* 响应式布局 */
@media screen and (max-width: 768px) {
  .app-header {
    padding: 0 var(--spacing-md);
  }
  
  .logo {
    font-size: var(--font-size);
  }
  
  .search-input {
    width: 150px;
  }
  
  .search-input:focus-within {
    width: 180px;
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
  
  .app-main {
    padding: var(--spacing-md);
    padding-bottom: 80px; /* 为底部导航留出空间 */
  }
  
  .breadcrumb {
    margin-bottom: var(--spacing-md);
  }
  
  .page-header h2 {
    font-size: var(--font-size-lg);
  }
  
  .header-actions {
    gap: var(--spacing-xs);
  }
  
  .notification-button,
  .sidebar-toggle {
    width: 28px;
    height: 28px;
  }
  
  .notification-icon,
  .sidebar-toggle .el-icon {
    font-size: 16px;
  }
}

/* 平板设备响应式 */
@media screen and (min-width: 769px) and (max-width: 1024px) {
  .app-sidebar {
    width: 200px !important;
  }
  
  .app-main {
    padding: var(--spacing-md);
  }
  
  .search-input {
    width: 180px;
  }
  
  .search-input:focus-within {
    width: 220px;
  }
}
</style>

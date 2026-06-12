<template>
  <div :class="asideDark ? 'aside dark' : 'aside'">
    <Logo />
    <a-layout-sider
      :collapsed="collapsed"
      breakpoint="xl"
      class="layout_side"
      :width="180"
      :collapsed-width="48"
    >
      <a-scrollbar class="menu-scrollbar">
        <Menu :route-tree="routeTree" />
      </a-scrollbar>
    </a-layout-sider>
  </div>
</template>

<script setup lang="ts">
import Logo from "@/layout/components/Logo/index.vue";
import Menu from "@/layout/components/Menu/index.vue";
import { storeToRefs } from "pinia";
import { useThemeConfig } from "@/store/modules/theme-config";
import { useRouteConfigStore } from "@/store/modules/route-config";
const themeStore = useThemeConfig();
const { collapsed, asideDark } = storeToRefs(themeStore);
const routerStore = useRouteConfigStore();
const { routeTree } = storeToRefs(routerStore);
</script>

<style lang="scss" scoped>
.aside {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: $color-bg-1;
  border-right: $border-1 solid $color-border-2;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.dark {
  background: #1f1f23;
  border-right: $border-1 solid #333;
}

.layout_side {
  flex: 1;
  overflow: hidden;
  transition: all $transition-normal;
}

.menu-scrollbar {
  height: 100%;
  padding: $spacing-sm 0;
}

// 优化滚动条样式
:deep(.arco-scrollbar-thumb-direction-vertical .arco-scrollbar-thumb-bar) {
  width: 4px;
  margin-left: 8px;
  border-radius: 2px;
  background: $color-border-3;
  transition: background 0.3s ease;
  &:hover {
    background: $color-primary;
  }
}

// 优化菜单样式
:deep(.arco-menu-vertical) {
  background: transparent;
  .arco-menu-item {
    margin: 4px 8px;
    border-radius: $radius-box-2;
    transition: all 0.2s ease;
    &:hover {
      background: $color-fill-2;
      transform: translateX(4px);
    }
    &.arco-menu-item-active {
      background: rgba(var(--primary-6), 0.1);
      color: $color-primary;
      font-weight: 500;
      box-shadow: 0 2px 8px rgba(var(--primary-6), 0.2);
    }
  }
  .arco-menu-inline-header {
    margin: 8px 8px 4px;
    font-weight: 600;
    font-size: 11px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    color: $color-text-3;
  }
}

  .arco-menu-item-selected {
    background: linear-gradient(135deg, $color-primary 0%, $color-primary-dark 100%) !important;
    color: $color-text-white !important;
    box-shadow: 0 4px 12px rgba($color-primary, 0.3);

    .arco-menu-icon {
      color: $color-text-white !important;
    }

    &:hover {
      background: linear-gradient(135deg, $color-primary-dark 0%, $color-primary-8 100%) !important;
    }
  }
}

// 暗色模式菜单优化
:deep(.arco-menu-dark) {
  .arco-menu-item,
  .arco-menu-pop-header,
  .arco-menu-inline-header {
    &:hover {
      background: rgba(255, 255, 255, 0.08);
    }
  }

  .arco-menu-item-selected {
    background: linear-gradient(135deg, $color-primary 0%, $color-primary-dark 100%) !important;
    box-shadow: 0 4px 12px rgba($color-primary, 0.4);
  }
}

// 折叠菜单优化
:deep(.arco-menu-vertical.arco-menu-collapsed) {
  .arco-menu-has-icon {
    justify-content: center;
    padding: 0;
  }

  .arco-menu-icon {
    padding: 12px 0;
    margin-right: 0;
    font-size: 20px;
  }

  .arco-menu-title {
    display: none;
  }
  .arco-menu-item {
    margin: 4px 0;
    &:hover {
      transform: none;
      background: $color-fill-2;
    }
  }
}

// 优化sider背景
.arco-layout-sider {
  background: transparent;
  transition: width 0.3s ease;
}
</style>

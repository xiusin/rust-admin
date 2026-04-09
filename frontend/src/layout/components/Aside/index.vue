<template>
  <div :class="asideDark ? 'aside dark' : 'aside'">
    <Logo />
    <a-layout-sider 
      :collapsed="collapsed" 
      breakpoint="xl" 
      class="layout_side" 
      :width="240"
      :collapsed-width="64"
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
  background: $color-bg-white;
  transition: background $transition-normal;
}

.dark {
  background: linear-gradient(180deg, #1d2333 0%, #0f1419 100%);
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
  width: 6px;
  border-radius: $radius-full;
  background: $color-fill-3;
  transition: background $transition-fast;

  &:hover {
    background: $color-fill-4;
  }
}

// 优化侧边栏样式
:deep(.arco-layout-sider) {
  background: transparent !important;
  border-right: 1px solid $color-border-1;
  box-shadow: $shadow-xs;
  transition: all $transition-normal;
}

:deep(.arco-layout-sider-dark) {
  border-right: 1px solid rgba(255, 255, 255, 0.08);
}

// 优化菜单样式
:deep(.arco-menu) {
  background: transparent !important;
  padding: $spacing-sm;
}

:deep(.arco-menu-vertical) {
  .arco-menu-item,
  .arco-menu-pop-header,
  .arco-menu-inline-header {
    height: 44px;
    line-height: 44px;
    margin: $spacing-xs 0;
    border-radius: $radius-md;
    transition: all $transition-fast;

    &:hover {
      background: rgba($color-primary, 0.06);
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
}
</style>

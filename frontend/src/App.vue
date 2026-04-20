<template>
  <div>
    <router-view />
  </div>
</template>

<script setup lang="ts">
import { useThemeMethods } from "@/hooks/useThemeMethods";
import { useGlobalWebSocket } from "@/hooks/useWebSocket";

// 初始化主题
const onTheme = () => {
  let { initTheme } = useThemeMethods();
  initTheme();
};
onTheme();

// 初始化全局 WebSocket 连接
useGlobalWebSocket();
</script>

<style lang="scss" scoped></style>

<template>
  <a-layout-header class="header">
    <HeaderLeft />
    <HeaderRight />
  </a-layout-header>
</template>
<script setup lang="ts">
import HeaderLeft from "@/layout/components/Header/components/header-left/index.vue";
import HeaderRight from "@/layout/components/Header/components/header-right/index.vue";
</script>

<style lang="scss" scoped>
.header {
  position: relative;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  height: 40px;
  padding: 0 $padding;
  background: $color-bg-1;
  border-bottom: $border-1 solid $color-border-2;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  z-index: 100;
  transition: all 0.3s ease;
}
</style>

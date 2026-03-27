<template>
  <a-form :model="modelValue" layout="vertical">
    <a-form-item label="使用TypeScript">
      <a-switch v-model="modelValue.useTypeScript" />
    </a-form-item>
    <a-form-item label="使用Composition API">
      <a-switch v-model="modelValue.useCompositionApi" />
    </a-form-item>
    <a-form-item label="生成API">
      <a-switch v-model="modelValue.generateApi" />
    </a-form-item>
    <a-form-item label="生成视图">
      <a-switch v-model="modelValue.generateViews" />
    </a-form-item>
    <a-form-item label="生成组件">
      <a-switch v-model="modelValue.generateComponents" />
    </a-form-item>
    <a-form-item label="生成路由">
      <a-switch v-model="modelValue.generateRoutes" />
    </a-form-item>
    <a-form-item label="生成Store">
      <a-switch v-model="modelValue.generateStore" />
    </a-form-item>
    <a-form-item label="作者">
      <a-input v-model="modelValue.author" placeholder="请输入作者" />
    </a-form-item>
    <a-form-item label="版本">
      <a-input v-model="modelValue.version" placeholder="请输入版本" />
    </a-form-item>
  </a-form>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { CodeGenOptions } from '@/api/modules/cms/codeGen';

interface Props {
  modelValue: CodeGenOptions;
  disabled?: boolean;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  'update:modelValue': [value: CodeGenOptions];
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val),
});
</script>

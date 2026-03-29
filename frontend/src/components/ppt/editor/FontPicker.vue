<template>
  <div class="font-picker">
    <a-select
      v-model="selectedFont"
      :style="{ width: fullWidth ? '100%' : '200px' }"
      placeholder="选择字体"
      @change="handleChange"
    >
      <a-option v-for="font in fonts" :key="font.value" :value="font.value">
        <span :style="{ fontFamily: font.value }">{{ font.label }}</span>
      </a-option>
    </a-select>
    
    <a-popover v-if="showPreview" trigger="hover">
      <template #content>
        <div class="font-preview" :style="{ fontFamily: modelValue }">
          {{ previewText }}
        </div>
      </template>
      <icon-font-colors class="preview-icon" />
    </a-popover>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { IconFontColors } from '@arco-design/web-vue/es/icon';

const props = withDefaults(defineProps<{
  modelValue: string;
  fullWidth?: boolean;
  showPreview?: boolean;
  previewText?: string;
}>(), {
  fullWidth: false,
  showPreview: true,
  previewText: '你好世界 Hello World 123',
});

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
}>();

const selectedFont = ref(props.modelValue);

const fonts = [
  { label: '微软雅黑', value: 'Microsoft YaHei' },
  { label: '宋体', value: 'SimSun' },
  { label: '黑体', value: 'SimHei' },
  { label: '楷体', value: 'KaiTi' },
  { label: '仿宋', value: 'FangSong' },
  { label: 'Arial', value: 'Arial' },
  { label: 'Times New Roman', value: 'Times New Roman' },
  { label: 'Georgia', value: 'Georgia' },
  { label: 'Verdana', value: 'Verdana' },
  { label: 'Courier New', value: 'Courier New' },
  { label: 'Impact', value: 'Impact' },
  { label: 'Comic Sans MS', value: 'Comic Sans MS' },
];

watch(() => props.modelValue, (val) => {
  selectedFont.value = val;
});

function handleChange(value: string) {
  emit('update:modelValue', value);
  emit('change', value);
}
</script>

<style scoped>
.font-picker {
  display: inline-flex;
  align-items: center;
  gap: 8px;
}

.preview-icon {
  font-size: 18px;
  color: var(--color-text-3);
  cursor: pointer;
}

.font-preview {
  padding: 12px;
  font-size: 16px;
  line-height: 1.5;
  min-width: 200px;
}
</style>

<template>
  <div class="text-editor">
    <a-form layout="vertical" size="small">
      <a-form-item label="文本内容">
        <a-textarea
          v-model="textContent"
          :auto-size="{ minRows: 3, maxRows: 6 }"
          placeholder="输入文本内容..."
          @change="handleContentChange"
        />
      </a-form-item>

      <a-divider>字体样式</a-divider>

      <a-form-item label="字体">
        <a-select v-model="fontFamily" @change="handleStyleChange">
          <a-option value="Microsoft YaHei">微软雅黑</a-option>
          <a-option value="SimSun">宋体</a-option>
          <a-option value="SimHei">黑体</a-option>
          <a-option value="KaiTi">楷体</a-option>
          <a-option value="Source Han Sans SC">思源黑体</a-option>
          <a-option value="Noto Serif SC">思源宋体</a-option>
          <a-option value="Arial">Arial</a-option>
          <a-option value="Helvetica">Helvetica</a-option>
          <a-option value="Times New Roman">Times New Roman</a-option>
        </a-select>
      </a-form-item>

      <a-row :gutter="8">
        <a-col :span="12">
          <a-form-item label="字号">
            <a-input-number
              v-model="fontSize"
              :min="8"
              :max="200"
              @change="handleStyleChange"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item label="字重">
            <a-select v-model="fontWeight" @change="handleStyleChange">
              <a-option value="normal">正常</a-option>
              <a-option value="bold">粗体</a-option>
              <a-option value="lighter">更细</a-option>
              <a-option value="100">100</a-option>
              <a-option value="200">200</a-option>
              <a-option value="300">300</a-option>
              <a-option value="400">400</a-option>
              <a-option value="500">500</a-option>
              <a-option value="600">600</a-option>
              <a-option value="700">700</a-option>
              <a-option value="800">800</a-option>
              <a-option value="900">900</a-option>
            </a-select>
          </a-form-item>
        </a-col>
      </a-row>

      <a-row :gutter="8">
        <a-col :span="12">
          <a-form-item label="颜色">
            <a-input v-model="color" @change="handleStyleChange">
              <template #prefix>
                <div class="color-preview" :style="{ backgroundColor: color }" />
              </template>
            </a-input>
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item label="背景色">
            <a-input v-model="backgroundColor" @change="handleStyleChange">
              <template #prefix>
                <div class="color-preview" :style="{ backgroundColor: backgroundColor || 'transparent' }" />
              </template>
            </a-input>
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item label="文字样式">
        <a-checkbox-group v-model="textStyles" @change="handleStyleChange">
          <a-checkbox value="italic">斜体</a-checkbox>
          <a-checkbox value="underline">下划线</a-checkbox>
          <a-checkbox value="line-through">删除线</a-checkbox>
        </a-checkbox-group>
      </a-form-item>

      <a-divider>对齐方式</a-divider>

      <a-form-item label="水平对齐">
        <a-radio-group v-model="textAlign" @change="handleStyleChange">
          <a-radio value="left">
            <template #icon><icon-align-left /></template>
            左
          </a-radio>
          <a-radio value="center">
            <template #icon><icon-align-center /></template>
            中
          </a-radio>
          <a-radio value="right">
            <template #icon><icon-align-right /></template>
            右
          </a-radio>
        </a-radio-group>
      </a-form-item>

      <a-form-item label="垂直对齐">
        <a-radio-group v-model="verticalAlign" @change="handleStyleChange">
          <a-radio value="top">上</a-radio>
          <a-radio value="middle">中</a-radio>
          <a-radio value="bottom">下</a-radio>
        </a-radio-group>
      </a-form-item>

      <a-divider>间距</a-divider>

      <a-row :gutter="8">
        <a-col :span="12">
          <a-form-item label="行高">
            <a-input-number
              v-model="lineHeight"
              :min="0.5"
              :max="3"
              :step="0.1"
              :precision="1"
              @change="handleStyleChange"
            />
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item label="字间距">
            <a-input-number
              v-model="letterSpacing"
              :min="0"
              :max="10"
              :step="0.5"
              :precision="1"
              @change="handleStyleChange"
            />
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item label="阴影">
        <a-input v-model="shadow" placeholder="0px 0px 10px rgba(0,0,0,0.5)" @change="handleStyleChange" />
      </a-form-item>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from 'vue';
import type { CanvasElement, ElementStyle } from './types';

const props = defineProps<{
  element: CanvasElement;
}>();

const emit = defineEmits<{
  (e: 'update', updates: Partial<CanvasElement>): void;
}>();

const textContent = ref('');
const fontFamily = ref('Microsoft YaHei');
const fontSize = ref(24);
const fontWeight = ref('normal');
const color = ref('#000000');
const backgroundColor = ref('');
const textStyles = ref<string[]>([]);
const textAlign = ref<'left' | 'center' | 'right'>('left');
const verticalAlign = ref<'top' | 'middle' | 'bottom'>('middle');
const lineHeight = ref(1.5);
const letterSpacing = ref(0);
const shadow = ref('');

watch(() => props.element, (el) => {
  if (el.type === 'text' && el.content.text !== undefined) {
    textContent.value = el.content.text;
  }
  
  if (el.style) {
    fontFamily.value = el.style.fontFamily || 'Microsoft YaHei';
    fontSize.value = el.style.fontSize || 24;
    fontWeight.value = el.style.fontWeight || 'normal';
    color.value = el.style.color || '#000000';
    backgroundColor.value = el.style.backgroundColor || '';
    textAlign.value = el.style.textAlign || 'left';
    verticalAlign.value = el.style.verticalAlign || 'middle';
    lineHeight.value = el.style.lineHeight || 1.5;
    letterSpacing.value = el.style.letterSpacing || 0;
    shadow.value = el.style.shadow || '';
    
    textStyles.value = [];
    if (el.style.fontStyle === 'italic') textStyles.value.push('italic');
    if (el.style.textDecoration?.includes('underline')) textStyles.value.push('underline');
    if (el.style.textDecoration?.includes('line-through')) textStyles.value.push('line-through');
  }
}, { immediate: true, deep: true });

const handleContentChange = () => {
  emit('update', {
    content: {
      ...props.element.content,
      text: textContent.value,
    },
  });
};

const handleStyleChange = () => {
  const style: ElementStyle = {
    fontFamily: fontFamily.value,
    fontSize: fontSize.value,
    fontWeight: fontWeight.value,
    color: color.value,
    backgroundColor: backgroundColor.value || undefined,
    textAlign: textAlign.value,
    verticalAlign: verticalAlign.value,
    lineHeight: lineHeight.value,
    letterSpacing: letterSpacing.value,
    shadow: shadow.value || undefined,
    fontStyle: textStyles.value.includes('italic') ? 'italic' : 'normal',
    textDecoration: [
      textStyles.value.includes('underline') ? 'underline' : '',
      textStyles.value.includes('line-through') ? 'line-through' : '',
    ].filter(Boolean).join(' ') || undefined,
  };

  emit('update', { style });
};
</script>

<style scoped lang="scss">
.text-editor {
  :deep(.arco-form-item) {
    margin-bottom: 12px;
  }

  .color-preview {
    width: 16px;
    height: 16px;
    border-radius: 2px;
    border: 1px solid var(--color-border);
  }
}
</style>

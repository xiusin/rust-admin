<template>
  <div class="shape-editor">
    <a-form layout="vertical" size="small">
      <a-form-item label="形状类型">
        <a-select v-model="shapeType" @change="handleShapeChange">
          <a-option value="rectangle">矩形</a-option>
          <a-option value="circle">圆形</a-option>
          <a-option value="triangle">三角形</a-option>
          <a-option value="line">线条</a-option>
          <a-option value="arrow">箭头</a-option>
        </a-select>
      </a-form-item>

      <a-divider>填充</a-divider>

      <a-form-item label="填充类型">
        <a-radio-group v-model="fillType" @change="handleStyleChange">
          <a-radio value="solid">纯色</a-radio>
          <a-radio value="gradient">渐变</a-radio>
          <a-radio value="none">无填充</a-radio>
        </a-radio-group>
      </a-form-item>

      <a-form-item label="填充颜色" v-if="fillType === 'solid'">
        <a-input v-model="fillColor" @change="handleStyleChange">
          <template #prefix>
            <div class="color-preview" :style="{ backgroundColor: fillColor }" />
          </template>
        </a-input>
      </a-form-item>

      <div v-if="fillType === 'gradient'">
        <a-form-item label="渐变类型">
          <a-select v-model="gradientType" @change="handleStyleChange">
            <a-option value="linear">线性渐变</a-option>
            <a-option value="radial">径向渐变</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="渐变角度" v-if="gradientType === 'linear'">
          <a-input-number
            v-model="gradientAngle"
            :min="0"
            :max="360"
            @change="handleStyleChange"
          >
            <template #suffix>°</template>
          </a-input-number>
        </a-form-item>
        <a-form-item label="渐变颜色">
          <div class="gradient-colors">
            <a-input
              v-for="(color, index) in gradientColors"
              :key="index"
              v-model="gradientColors[index]"
              @change="handleStyleChange"
            >
              <template #prefix>
                <div class="color-preview" :style="{ backgroundColor: color }" />
              </template>
            </a-input>
          </div>
        </a-form-item>
      </div>

      <a-divider>边框</a-divider>

      <a-row :gutter="8">
        <a-col :span="12">
          <a-form-item label="边框颜色">
            <a-input v-model="borderColor" @change="handleStyleChange">
              <template #prefix>
                <div class="color-preview" :style="{ backgroundColor: borderColor || 'transparent' }" />
              </template>
            </a-input>
          </a-form-item>
        </a-col>
        <a-col :span="12">
          <a-form-item label="边框宽度">
            <a-input-number
              v-model="borderWidth"
              :min="0"
              :max="20"
              @change="handleStyleChange"
            />
          </a-form-item>
        </a-col>
      </a-row>

      <a-form-item label="边框样式">
        <a-select v-model="borderStyle" @change="handleStyleChange">
          <a-option value="solid">实线</a-option>
          <a-option value="dashed">虚线</a-option>
          <a-option value="dotted">点线</a-option>
        </a-select>
      </a-form-item>

      <a-divider>圆角</a-divider>

      <a-form-item label="圆角半径" v-if="shapeType === 'rectangle'">
        <a-slider
          v-model="borderRadius"
          :min="0"
          :max="100"
          @change="handleStyleChange"
        />
      </a-form-item>

      <a-divider>阴影</a-divider>

      <a-form-item label="阴影">
        <a-input v-model="shadow" placeholder="0px 0px 10px rgba(0,0,0,0.5)" @change="handleStyleChange" />
      </a-form-item>

      <a-divider>线条设置</a-divider>

      <div v-if="shapeType === 'line' || shapeType === 'arrow'">
        <a-form-item label="线条端点">
          <a-select v-model="lineCap" @change="handleStyleChange">
            <a-option value="butt">平头</a-option>
            <a-option value="round">圆头</a-option>
            <a-option value="square">方头</a-option>
          </a-select>
        </a-form-item>
        
        <a-form-item label="线条连接">
          <a-select v-model="lineJoin" @change="handleStyleChange">
            <a-option value="miter">尖角</a-option>
            <a-option value="round">圆角</a-option>
            <a-option value="bevel">斜角</a-option>
          </a-select>
        </a-form-item>
      </div>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import type { CanvasElement, ElementStyle, ElementContent } from './types';

const props = defineProps<{
  element: CanvasElement;
}>();

const emit = defineEmits<{
  (e: 'update', updates: Partial<CanvasElement>): void;
}>();

const shapeType = ref<'rectangle' | 'circle' | 'triangle' | 'line' | 'arrow'>('rectangle');
const fillType = ref<'solid' | 'gradient' | 'none'>('solid');
const fillColor = ref('#1890ff');
const gradientType = ref<'linear' | 'radial'>('linear');
const gradientAngle = ref(0);
const gradientColors = ref(['#1890ff', '#096dd9']);
const borderColor = ref('');
const borderWidth = ref(0);
const borderStyle = ref('solid');
const borderRadius = ref(0);
const shadow = ref('');
const lineCap = ref<'butt' | 'round' | 'square'>('butt');
const lineJoin = ref<'miter' | 'round' | 'bevel'>('miter');

watch(() => props.element, (el) => {
  if (el.type === 'shape' && el.content.shapeType !== undefined) {
    shapeType.value = el.content.shapeType;
  }
  
  if (el.style) {
    if (el.style.backgroundColor) {
      fillColor.value = el.style.backgroundColor;
      fillType.value = 'solid';
    } else {
      fillType.value = 'none';
    }
    
    borderColor.value = el.style.borderColor || '';
    borderWidth.value = el.style.borderWidth || 0;
    borderRadius.value = el.style.borderRadius || 0;
    shadow.value = el.style.shadow || '';
  }
}, { immediate: true, deep: true });

const handleShapeChange = () => {
  emit('update', {
    content: {
      ...props.element.content,
      shapeType: shapeType.value,
    },
  });
};

const handleStyleChange = () => {
  const style: ElementStyle = {
    ...props.element.style,
    backgroundColor: fillType.value === 'solid' ? fillColor.value : undefined,
    borderColor: borderColor.value || undefined,
    borderWidth: borderWidth.value || undefined,
    borderRadius: borderRadius.value || undefined,
    shadow: shadow.value || undefined,
  };
  
  emit('update', { style });
};
</script>

<style scoped lang="scss">
.shape-editor {
  :deep(.arco-form-item) {
    margin-bottom: 12px;
  }

  .color-preview {
    width: 16px;
    height: 16px;
    border-radius: 2px;
    border: 1px solid var(--color-border);
  }

  .gradient-colors {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
}
</style>

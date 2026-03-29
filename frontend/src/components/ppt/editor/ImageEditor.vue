<template>
  <div class="image-editor">
    <a-form layout="vertical" size="small">
      <a-form-item label="图片">
        <div class="image-preview">
          <img :src="imageUrl" v-if="imageUrl" alt="预览" />
          <div v-else class="no-image">
            <icon-image />
            <span>暂无图片</span>
          </div>
        </div>
        <a-space style="margin-top: 8px; width: 100%;">
          <a-upload
            :show-file-list="false"
            :auto-upload="false"
            accept="image/*"
            @change="handleImageUpload"
          >
            <template #upload-button>
              <a-button long>
                <template #icon><icon-upload /></template>
                替换图片
              </a-button>
            </template>
          </a-upload>
        </a-space>
      </a-form-item>

      <a-divider>裁剪</a-divider>

      <a-form-item>
        <a-button long @click="startCrop" :disabled="!imageUrl">
          <template #icon><icon-scissor /></template>
          开始裁剪
        </a-button>
      </a-form-item>

      <a-divider>滤镜</a-divider>

      <a-form-item label="滤镜效果">
        <a-select v-model="filter" @change="handleFilterChange">
          <a-option value="none">无</a-option>
          <a-option value="grayscale">灰度</a-option>
          <a-option value="sepia">复古</a-option>
          <a-option value="blur">模糊</a-option>
          <a-option value="brightness">亮度增强</a-option>
          <a-option value="contrast">对比度增强</a-option>
          <a-option value="saturate">饱和度增强</a-option>
          <a-option value="invert">反色</a-option>
        </a-select>
      </a-form-item>

      <a-form-item label="滤镜强度" v-if="filter !== 'none'">
        <a-slider
          v-model="filterIntensity"
          :min="0"
          :max="100"
          @change="handleFilterChange"
        />
      </a-form-item>

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

      <a-form-item label="圆角">
        <a-slider
          v-model="borderRadius"
          :min="0"
          :max="50"
          @change="handleStyleChange"
        />
      </a-form-item>

      <a-divider>阴影</a-divider>

      <a-form-item label="阴影">
        <a-input v-model="shadow" placeholder="0px 0px 10px rgba(0,0,0,0.5)" @change="handleStyleChange" />
      </a-form-item>

      <a-divider>调整</a-divider>

      <a-form-item label="亮度">
        <a-slider
          v-model="brightness"
          :min="0"
          :max="200"
          @change="handleAdjustChange"
        />
      </a-form-item>

      <a-form-item label="对比度">
        <a-slider
          v-model="contrast"
          :min="0"
          :max="200"
          @change="handleAdjustChange"
        />
      </a-form-item>

      <a-form-item label="饱和度">
        <a-slider
          v-model="saturate"
          :min="0"
          :max="200"
          @change="handleAdjustChange"
        />
      </a-form-item>

      <a-divider>操作</a-divider>

      <a-space direction="vertical" style="width: 100%;">
        <a-button long @click="resetAdjustments">
          <template #icon><icon-refresh /></template>
          重置调整
        </a-button>
        <a-button long @click="flipHorizontal">
          <template #icon><icon-swap /></template>
          水平翻转
        </a-button>
        <a-button long @click="flipVertical">
          <template #icon><icon-swap /></template>
          垂直翻转
        </a-button>
      </a-space>
    </a-form>

    <a-modal
      v-model:visible="cropModalVisible"
      title="图片裁剪"
      :width="800"
      @ok="applyCrop"
      @cancel="cancelCrop"
    >
      <div class="crop-container" ref="cropContainer">
        <canvas ref="cropCanvas"></canvas>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue';
import { Message } from '@arco-design/web-vue';
import type { CanvasElement, ElementStyle } from './types';

const props = defineProps<{
  element: CanvasElement;
}>();

const emit = defineEmits<{
  (e: 'update', updates: Partial<CanvasElement>): void;
}>();

const imageUrl = ref('');
const filter = ref('none');
const filterIntensity = ref(100);
const borderColor = ref('');
const borderWidth = ref(0);
const borderRadius = ref(0);
const shadow = ref('');
const brightness = ref(100);
const contrast = ref(100);
const saturate = ref(100);

const cropModalVisible = ref(false);
const cropContainer = ref<HTMLElement>();
const cropCanvas = ref<HTMLCanvasElement>();

watch(() => props.element, (el) => {
  if (el.type === 'image' && el.content.imageUrl !== undefined) {
    imageUrl.value = el.content.imageUrl;
  }
  
  if (el.style) {
    borderColor.value = el.style.borderColor || '';
    borderWidth.value = el.style.borderWidth || 0;
    borderRadius.value = el.style.borderRadius || 0;
    shadow.value = el.style.shadow || '';
    
    if (el.style.filter) {
      parseFilter(el.style.filter);
    }
  }
}, { immediate: true, deep: true });

const parseFilter = (filterStr: string) => {
  if (!filterStr || filterStr === 'none') {
    filter.value = 'none';
    filterIntensity.value = 100;
    brightness.value = 100;
    contrast.value = 100;
    saturate.value = 100;
    return;
  }
  
  const brightnessMatch = filterStr.match(/brightness\((\d+)%?\)/);
  if (brightnessMatch) brightness.value = parseInt(brightnessMatch[1]);
  
  const contrastMatch = filterStr.match(/contrast\((\d+)%?\)/);
  if (contrastMatch) contrast.value = parseInt(contrastMatch[1]);
  
  const saturateMatch = filterStr.match(/saturate\((\d+)%?\)/);
  if (saturateMatch) saturate.value = parseInt(saturateMatch[1]);
  
  const grayscaleMatch = filterStr.match(/grayscale\((\d+)%?\)/);
  if (grayscaleMatch) {
    filter.value = 'grayscale';
    filterIntensity.value = parseInt(grayscaleMatch[1]);
  }
  
  const sepiaMatch = filterStr.match(/sepia\((\d+)%?\)/);
  if (sepiaMatch) {
    filter.value = 'sepia';
    filterIntensity.value = parseInt(sepiaMatch[1]);
  }
  
  const blurMatch = filterStr.match(/blur\((\d+)px\)/);
  if (blurMatch) {
    filter.value = 'blur';
    filterIntensity.value = parseInt(blurMatch[1]);
  }
};

const handleImageUpload = (file: any) => {
  const reader = new FileReader();
  reader.onload = (e) => {
    imageUrl.value = e.target?.result as string;
    emit('update', {
      content: {
        ...props.element.content,
        imageUrl: imageUrl.value,
      },
    });
    Message.success('图片上传成功');
  };
  reader.readAsDataURL(file.file);
};

const handleFilterChange = () => {
  const filterStr = buildFilterString();
  emit('update', {
    style: {
      ...props.element.style,
      filter: filterStr,
    },
  });
};

const handleAdjustChange = () => {
  const filterStr = buildFilterString();
  emit('update', {
    style: {
      ...props.element.style,
      filter: filterStr,
    },
  });
};

const buildFilterString = (): string => {
  const filters: string[] = [];
  
  filters.push(`brightness(${brightness.value}%)`);
  filters.push(`contrast(${contrast.value}%)`);
  filters.push(`saturate(${saturate.value}%)`);
  
  if (filter.value !== 'none') {
    const intensity = filterIntensity.value;
    switch (filter.value) {
      case 'grayscale':
        filters.push(`grayscale(${intensity}%)`);
        break;
      case 'sepia':
        filters.push(`sepia(${intensity}%)`);
        break;
      case 'blur':
        filters.push(`blur(${intensity / 10}px)`);
        break;
      case 'invert':
        filters.push(`invert(${intensity}%)`);
        break;
    }
  }
  
  return filters.join(' ');
};

const handleStyleChange = () => {
  const style: ElementStyle = {
    ...props.element.style,
    borderColor: borderColor.value || undefined,
    borderWidth: borderWidth.value || undefined,
    borderRadius: borderRadius.value || undefined,
    shadow: shadow.value || undefined,
  };
  
  emit('update', { style });
};

const resetAdjustments = () => {
  brightness.value = 100;
  contrast.value = 100;
  saturate.value = 100;
  filter.value = 'none';
  filterIntensity.value = 100;
  handleAdjustChange();
  Message.success('已重置调整');
};

const flipHorizontal = () => {
  Message.info('水平翻转功能开发中...');
};

const flipVertical = () => {
  Message.info('垂直翻转功能开发中...');
};

const startCrop = () => {
  if (!imageUrl.value) {
    Message.warning('请先上传图片');
    return;
  }
  cropModalVisible.value = true;
};

const applyCrop = () => {
  Message.success('裁剪应用成功');
  cropModalVisible.value = false;
};

const cancelCrop = () => {
  cropModalVisible.value = false;
};
</script>

<style scoped lang="scss">
.image-editor {
  :deep(.arco-form-item) {
    margin-bottom: 12px;
  }

  .image-preview {
    width: 100%;
    aspect-ratio: 16 / 9;
    background: var(--color-fill-2);
    border-radius: 4px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;

    img {
      max-width: 100%;
      max-height: 100%;
      object-fit: contain;
    }

    .no-image {
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 8px;
      color: var(--color-text-3);

      svg {
        font-size: 32px;
      }
    }
  }

  .color-preview {
    width: 16px;
    height: 16px;
    border-radius: 2px;
    border: 1px solid var(--color-border);
  }

  .crop-container {
    width: 100%;
    height: 500px;
    background: #000;
    display: flex;
    align-items: center;
    justify-content: center;

    canvas {
      max-width: 100%;
      max-height: 100%;
    }
  }
}
</style>

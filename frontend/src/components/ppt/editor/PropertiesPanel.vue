<template>
  <div class="properties-panel">
    <a-tabs v-model:active-key="activeTab" type="rounded">
      <a-tab-pane key="element" title="元素">
        <div v-if="selectedElement" class="element-properties">
          <a-form :model="elementForm" layout="vertical" size="small">
            <a-collapse :default-active-key="['position', 'style']">
              <a-collapse-item key="position" header="位置与大小">
                <a-row :gutter="8">
                  <a-col :span="12">
                    <a-form-item label="X">
                      <a-input-number
                        v-model="elementForm.x"
                        :precision="0"
                        @change="updatePosition('x', $event)"
                      />
                    </a-form-item>
                  </a-col>
                  <a-col :span="12">
                    <a-form-item label="Y">
                      <a-input-number
                        v-model="elementForm.y"
                        :precision="0"
                        @change="updatePosition('y', $event)"
                      />
                    </a-form-item>
                  </a-col>
                </a-row>
                <a-row :gutter="8">
                  <a-col :span="12">
                    <a-form-item label="宽度">
                      <a-input-number
                        v-model="elementForm.width"
                        :min="10"
                        :precision="0"
                        @change="updateSize('width', $event)"
                      />
                    </a-form-item>
                  </a-col>
                  <a-col :span="12">
                    <a-form-item label="高度">
                      <a-input-number
                        v-model="elementForm.height"
                        :min="10"
                        :precision="0"
                        @change="updateSize('height', $event)"
                      />
                    </a-form-item>
                  </a-col>
                </a-row>
                <a-row :gutter="8">
                  <a-col :span="12">
                    <a-form-item label="旋转">
                      <a-input-number
                        v-model="elementForm.rotation"
                        :min="-360"
                        :max="360"
                        :precision="0"
                        @change="updateRotation"
                      >
                        <template #suffix>°</template>
                      </a-input-number>
                    </a-form-item>
                  </a-col>
                  <a-col :span="12">
                    <a-form-item label="透明度">
                      <a-slider
                        v-model="elementForm.opacity"
                        :min="0"
                        :max="1"
                        :step="0.1"
                        @change="updateOpacity"
                      />
                    </a-form-item>
                  </a-col>
                </a-row>
              </a-collapse-item>

              <a-collapse-item key="style" header="样式">
                <TextEditor
                  v-if="selectedElement.type === 'text'"
                  :element="selectedElement"
                  @update="handleStyleUpdate"
                />
                <ImageEditor
                  v-else-if="selectedElement.type === 'image'"
                  :element="selectedElement"
                  @update="handleStyleUpdate"
                />
                <ShapeEditor
                  v-else-if="selectedElement.type === 'shape'"
                  :element="selectedElement"
                  @update="handleStyleUpdate"
                />
                <div v-else class="empty-editor">
                  <icon-settings />
                  <p>选择元素类型以编辑样式</p>
                </div>
              </a-collapse-item>

              <a-collapse-item key="animation" header="动画">
                <AnimationEditor
                  :element="selectedElement"
                  @update="handleAnimationUpdate"
                />
              </a-collapse-item>
            </a-collapse>
          </a-form>
        </div>
        <div v-else class="empty-state">
          <icon-select-all />
          <p>请选择一个元素</p>
          <p class="hint">点击画布上的元素进行编辑</p>
        </div>
      </a-tab-pane>

      <a-tab-pane key="slide" title="幻灯片">
        <div v-if="slide" class="slide-properties">
          <a-form :model="slideForm" layout="vertical" size="small">
            <a-form-item label="标题">
              <a-input
                v-model="slideForm.title"
                placeholder="输入幻灯片标题"
                @change="updateSlide('title', $event)"
              />
            </a-form-item>

            <a-collapse :default-active-key="['background']">
              <a-collapse-item key="background" header="背景">
                <a-radio-group v-model="slideForm.backgroundType" @change="handleBackgroundTypeChange">
                  <a-radio value="color">纯色</a-radio>
                  <a-radio value="gradient">渐变</a-radio>
                  <a-radio value="image">图片</a-radio>
                </a-radio-group>

                <div v-if="slideForm.backgroundType === 'color'" class="background-color">
                  <a-form-item label="背景颜色">
                    <a-input v-model="slideForm.backgroundColor" @change="updateBackground">
                      <template #prefix>
                        <div
                          class="color-preview"
                          :style="{ backgroundColor: slideForm.backgroundColor }"
                        />
                      </template>
                    </a-input>
                  </a-form-item>
                </div>

                <div v-else-if="slideForm.backgroundType === 'gradient'" class="background-gradient">
                  <a-form-item label="渐变类型">
                    <a-select v-model="slideForm.gradientType" @change="updateBackground">
                      <a-option value="linear">线性渐变</a-option>
                      <a-option value="radial">径向渐变</a-option>
                    </a-select>
                  </a-form-item>
                  <a-form-item label="渐变角度" v-if="slideForm.gradientType === 'linear'">
                    <a-input-number
                      v-model="slideForm.gradientAngle"
                      :min="0"
                      :max="360"
                      @change="updateBackground"
                    />
                  </a-form-item>
                  <a-form-item label="渐变颜色">
                    <div class="gradient-colors">
                      <a-input
                        v-for="(color, index) in slideForm.gradientColors"
                        :key="index"
                        v-model="slideForm.gradientColors[index]"
                        @change="updateBackground"
                      >
                        <template #prefix>
                          <div
                            class="color-preview"
                            :style="{ backgroundColor: color }"
                          />
                        </template>
                      </a-input>
                    </div>
                  </a-form-item>
                </div>

                <div v-else-if="slideForm.backgroundType === 'image'" class="background-image">
                  <a-form-item label="背景图片">
                    <a-upload
                      :show-file-list="false"
                      :auto-upload="false"
                      @change="handleImageUpload"
                    >
                      <template #upload-button>
                        <a-button>
                          <template #icon><icon-upload /></template>
                          上传图片
                        </a-button>
                      </template>
                    </a-upload>
                  </a-form-item>
                  <a-form-item label="透明度">
                    <a-slider
                      v-model="slideForm.backgroundOpacity"
                      :min="0"
                      :max="1"
                      :step="0.1"
                      @change="updateBackground"
                    />
                  </a-form-item>
                </div>
              </a-collapse-item>

              <a-collapse-item key="transition" header="切换效果">
                <a-form-item label="切换类型">
                  <a-select v-model="slideForm.transitionType" @change="updateTransition">
                    <a-option value="none">无</a-option>
                    <a-option value="fade">淡入淡出</a-option>
                    <a-option value="slide">滑动</a-option>
                    <a-option value="zoom">缩放</a-option>
                    <a-option value="flip">翻转</a-option>
                    <a-option value="cube">立方体</a-option>
                  </a-select>
                </a-form-item>
                <a-form-item label="持续时间">
                  <a-input-number
                    v-model="slideForm.transitionDuration"
                    :min="100"
                    :max="5000"
                    :step="100"
                    @change="updateTransition"
                  >
                    <template #suffix>ms</template>
                  </a-input-number>
                </a-form-item>
              </a-collapse-item>

              <a-collapse-item key="notes" header="备注">
                <a-textarea
                  v-model="slideForm.notes"
                  placeholder="输入演讲备注..."
                  :auto-size="{ minRows: 4, maxRows: 8 }"
                  @change="updateSlide('notes', $event)"
                />
              </a-collapse-item>
            </a-collapse>
          </a-form>
        </div>
      </a-tab-pane>
    </a-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue';
import type { CanvasElement, SlideElement } from './types';
import TextEditor from './TextEditor.vue';
import ImageEditor from './ImageEditor.vue';
import ShapeEditor from './ShapeEditor.vue';
import AnimationEditor from './AnimationEditor.vue';

const props = defineProps<{
  selectedElement: CanvasElement | null;
  slide: SlideElement | null;
}>();

const emit = defineEmits<{
  (e: 'updateElement', elementId: string, updates: Partial<CanvasElement>): void;
  (e: 'updateSlide', updates: Partial<SlideElement>): void;
}>();

const activeTab = ref('element');

const elementForm = reactive({
  x: 0,
  y: 0,
  width: 0,
  height: 0,
  rotation: 0,
  opacity: 1,
});

const slideForm = reactive({
  title: '',
  backgroundType: 'color' as 'color' | 'gradient' | 'image',
  backgroundColor: '#ffffff',
  gradientType: 'linear' as 'linear' | 'radial',
  gradientAngle: 0,
  gradientColors: ['#ffffff', '#f0f0f0'],
  backgroundImage: '',
  backgroundOpacity: 1,
  transitionType: 'none',
  transitionDuration: 500,
  transitionDirection: 'right',
  notes: '',
});

watch(() => props.selectedElement, (element) => {
  if (element) {
    elementForm.x = element.x;
    elementForm.y = element.y;
    elementForm.width = element.width;
    elementForm.height = element.height;
    elementForm.rotation = element.rotation;
    elementForm.opacity = element.opacity;
  }
}, { immediate: true });

watch(() => props.slide, (slide) => {
  if (slide) {
    slideForm.title = slide.title || '';
    slideForm.notes = slide.notes || '';
    
    if (slide.background) {
      slideForm.backgroundType = slide.background.type || 'color';
      slideForm.backgroundColor = slide.background.color || '#ffffff';
      slideForm.backgroundOpacity = slide.background.opacity || 1;
      
      if (slide.background.gradient) {
        slideForm.gradientType = slide.background.gradient.type;
        slideForm.gradientAngle = slide.background.gradient.angle || 0;
        slideForm.gradientColors = slide.background.gradient.colors || ['#ffffff', '#f0f0f0'];
      }
      
      if (slide.background.imageUrl) {
        slideForm.backgroundImage = slide.background.imageUrl;
      }
    }
    
    if (slide.transition) {
      slideForm.transitionType = slide.transition.type || 'none';
      slideForm.transitionDuration = slide.transition.duration || 500;
      slideForm.transitionDirection = slide.transition.direction || 'right';
    }
  }
}, { immediate: true });

const updatePosition = (prop: 'x' | 'y', value: number) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, { [prop]: value });
  }
};

const updateSize = (prop: 'width' | 'height', value: number) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, { [prop]: value });
  }
};

const updateRotation = (value: number) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, { rotation: value });
  }
};

const updateOpacity = (value: number) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, { opacity: value });
  }
};

const handleStyleUpdate = (updates: Partial<CanvasElement>) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, updates);
  }
};

const handleAnimationUpdate = (animation: any) => {
  if (props.selectedElement) {
    emit('updateElement', props.selectedElement.id, { animation });
  }
};

const updateSlide = (prop: string, value: any) => {
  emit('updateSlide', { [prop]: value });
};

const handleBackgroundTypeChange = () => {
  updateBackground();
};

const updateBackground = () => {
  const background: any = {
    type: slideForm.backgroundType,
    opacity: slideForm.backgroundOpacity,
  };

  if (slideForm.backgroundType === 'color') {
    background.color = slideForm.backgroundColor;
  } else if (slideForm.backgroundType === 'gradient') {
    background.gradient = {
      type: slideForm.gradientType,
      angle: slideForm.gradientAngle,
      colors: slideForm.gradientColors,
    };
  } else if (slideForm.backgroundType === 'image') {
    background.imageUrl = slideForm.backgroundImage;
  }

  emit('updateSlide', { background });
};

const handleImageUpload = (file: any) => {
  const reader = new FileReader();
  reader.onload = (e) => {
    slideForm.backgroundImage = e.target?.result as string;
    updateBackground();
  };
  reader.readAsDataURL(file.file);
};

const updateTransition = () => {
  emit('updateSlide', {
    transition: {
      type: slideForm.transitionType,
      duration: slideForm.transitionDuration,
      direction: slideForm.transitionDirection,
    },
  });
};
</script>

<style scoped lang="scss">
.properties-panel {
  height: 100%;
  padding: 12px;
  overflow-y: auto;

  :deep(.arco-tabs-content) {
    padding-top: 12px;
  }

  .element-properties,
  .slide-properties {
    :deep(.arco-form-item) {
      margin-bottom: 12px;
    }
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 40px 20px;
    color: var(--color-text-3);
    text-align: center;

    svg {
      font-size: 48px;
      margin-bottom: 16px;
    }

    .hint {
      font-size: 12px;
      margin-top: 8px;
    }
  }

  .empty-editor {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 24px;
    color: var(--color-text-3);
    text-align: center;

    svg {
      font-size: 32px;
      margin-bottom: 8px;
    }
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

  :deep(.arco-collapse-item-content) {
    padding: 12px 0;
  }
}
</style>

<template>
  <div class="animation-editor">
    <a-form layout="vertical" size="small">
      <a-form-item label="动画类型">
        <a-select v-model="animationType" @change="handleAnimationChange">
          <a-option value="none">无动画</a-option>
          <a-option value="fade">淡入</a-option>
          <a-option value="slide">滑动</a-option>
          <a-option value="zoom">缩放</a-option>
          <a-option value="bounce">弹跳</a-option>
          <a-option value="rotate">旋转</a-option>
          <a-option value="flip">翻转</a-option>
        </a-select>
      </a-form-item>

      <div v-if="animationType !== 'none'">
        <a-form-item label="动画方向" v-if="['slide', 'zoom', 'flip'].includes(animationType)">
          <a-radio-group v-model="animationDirection" @change="handleAnimationChange">
            <a-radio value="left">从左</a-radio>
            <a-radio value="right">从右</a-radio>
            <a-radio value="top">从上</a-radio>
            <a-radio value="bottom">从下</a-radio>
          </a-radio-group>
        </a-form-item>

        <a-form-item label="持续时间">
          <a-slider
            v-model="animationDuration"
            :min="100"
            :max="3000"
            :step="100"
            :format-tooltip="(value: number) => `${value}ms`"
            @change="handleAnimationChange"
          />
        </a-form-item>

        <a-form-item label="延迟时间">
          <a-slider
            v-model="animationDelay"
            :min="0"
            :max="2000"
            :step="100"
            :format-tooltip="(value: number) => `${value}ms`"
            @change="handleAnimationChange"
          />
        </a-form-item>

        <a-form-item label="缓动函数">
          <a-select v-model="animationEasing" @change="handleAnimationChange">
            <a-option value="linear">线性</a-option>
            <a-option value="ease">缓动</a-option>
            <a-option value="ease-in">缓入</a-option>
            <a-option value="ease-out">缓出</a-option>
            <a-option value="ease-in-out">缓入缓出</a-option>
            <a-option value="cubic-bezier(0.68, -0.55, 0.265, 1.55)">弹性</a-option>
          </a-select>
        </a-form-item>

        <a-divider>预览</a-divider>

        <div class="animation-preview">
          <div
            class="preview-element"
            :style="previewStyle"
            @animationend="isPlaying = false"
          >
            预览
          </div>
        </div>

        <a-button long @click="playAnimation" :loading="isPlaying">
          <template #icon><icon-play-arrow /></template>
          播放动画
        </a-button>
      </div>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { CanvasElement, ElementAnimation } from './types';

const props = defineProps<{
  element: CanvasElement;
}>();

const emit = defineEmits<{
  (e: 'update', animation: ElementAnimation | undefined): void;
}>();

const animationType = ref<'none' | 'fade' | 'slide' | 'zoom' | 'bounce' | 'rotate' | 'flip'>('none');
const animationDirection = ref<'left' | 'right' | 'top' | 'bottom'>('left');
const animationDuration = ref(500);
const animationDelay = ref(0);
const animationEasing = ref('ease');
const isPlaying = ref(false);

watch(() => props.element.animation, (anim) => {
  if (anim) {
    animationType.value = anim.type || 'none';
    animationDirection.value = anim.direction || 'left';
    animationDuration.value = anim.duration || 500;
    animationDelay.value = anim.delay || 0;
    animationEasing.value = anim.easing || 'ease';
  } else {
    animationType.value = 'none';
  }
}, { immediate: true });

const handleAnimationChange = () => {
  if (animationType.value === 'none') {
    emit('update', undefined);
    return;
  }

  const animation: ElementAnimation = {
    type: animationType.value as any,
    duration: animationDuration.value,
    delay: animationDelay.value,
    direction: animationDirection.value,
    easing: animationEasing.value,
  };

  emit('update', animation);
};

const previewStyle = computed(() => {
  if (animationType.value === 'none' || !isPlaying.value) {
    return {};
  }

  const keyframes = getKeyframes();
  return {
    animation: `${keyframes} ${animationDuration.value}ms ${animationEasing.value} ${animationDelay.value}ms forwards`,
  };
});

const getKeyframes = (): string => {
  const type = animationType.value;
  const dir = animationDirection.value;

  const keyframeMap: Record<string, Record<string, string>> = {
    fade: {
      '0%': 'opacity: 0',
      '100%': 'opacity: 1',
    },
    slide: {
      left: { '0%': 'transform: translateX(-100%)', '100%': 'transform: translateX(0)' },
      right: { '0%': 'transform: translateX(100%)', '100%': 'transform: translateX(0)' },
      top: { '0%': 'transform: translateY(-100%)', '100%': 'transform: translateY(0)' },
      bottom: { '0%': 'transform: translateY(100%)', '100%': 'transform: translateY(0)' },
    },
    zoom: {
      left: { '0%': 'transform: scale(0) translateX(-100%)', '100%': 'transform: scale(1) translateX(0)' },
      right: { '0%': 'transform: scale(0) translateX(100%)', '100%': 'transform: scale(1) translateX(0)' },
      top: { '0%': 'transform: scale(0) translateY(-100%)', '100%': 'transform: scale(1) translateY(0)' },
      bottom: { '0%': 'transform: scale(0) translateY(100%)', '100%': 'transform: scale(1) translateY(0)' },
    },
    bounce: {
      '0%': 'transform: scale(0)',
      '50%': 'transform: scale(1.2)',
      '100%': 'transform: scale(1)',
    },
    rotate: {
      '0%': 'transform: rotate(0deg)',
      '100%': 'transform: rotate(360deg)',
    },
    flip: {
      left: { '0%': 'transform: perspective(400px) rotateY(90deg)', '100%': 'transform: perspective(400px) rotateY(0)' },
      right: { '0%': 'transform: perspective(400px) rotateY(-90deg)', '100%': 'transform: perspective(400px) rotateY(0)' },
      top: { '0%': 'transform: perspective(400px) rotateX(90deg)', '100%': 'transform: perspective(400px) rotateX(0)' },
      bottom: { '0%': 'transform: perspective(400px) rotateX(-90deg)', '100%': 'transform: perspective(400px) rotateX(0)' },
    },
  };

  const animName = `anim_${type}_${Date.now()}`;
  let keyframes = '';

  if (type === 'fade' || type === 'bounce' || type === 'rotate') {
    const frames = keyframeMap[type];
    keyframes = `@keyframes ${animName} { ${Object.entries(frames).map(([k, v]) => `${k} { ${v} }`).join(' ')} }`;
  } else if (['slide', 'zoom', 'flip'].includes(type)) {
    const frames = keyframeMap[type][dir];
    keyframes = `@keyframes ${animName} { ${Object.entries(frames).map(([k, v]) => `${k} { ${v} }`).join(' ')} }`;
  }

  const styleSheet = document.styleSheets[0];
  try {
    styleSheet.insertRule(keyframes, styleSheet.cssRules.length);
  } catch (e) {
    // Rule already exists
  }

  return animName;
};

const playAnimation = () => {
  isPlaying.value = false;
  setTimeout(() => {
    isPlaying.value = true;
  }, 10);
};
</script>

<style scoped lang="scss">
.animation-editor {
  :deep(.arco-form-item) {
    margin-bottom: 12px;
  }

  .animation-preview {
    width: 100%;
    height: 100px;
    background: var(--color-fill-2);
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-bottom: 12px;

    .preview-element {
      padding: 12px 24px;
      background: rgb(var(--primary-6));
      color: white;
      border-radius: 4px;
      font-size: 14px;
    }
  }
}
</style>

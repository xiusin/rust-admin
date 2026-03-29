<template>
  <div class="canvas-editor">
    <EditorToolbar
      @tool-change="handleToolChange"
      @undo="handleUndo"
      @redo="handleRedo"
      @ai-polish="handleAIPolish"
    />
    
    <div class="editor-main">
      <div class="slide-list">
        <div class="slide-list-header">
          <span>幻灯片</span>
          <a-button type="text" size="small" @click="handleAddSlide">
            <template #icon><icon-plus /></template>
          </a-button>
        </div>
        <div class="slide-list-content">
          <div
            v-for="(slide, index) in slides"
            :key="slide.id"
            :class="['slide-item', { active: currentSlideIndex === index }]"
            draggable="true"
            @click="selectSlide(index)"
            @dragstart="handleDragStart($event, index)"
            @dragover.prevent
            @drop="handleDrop($event, index)"
          >
            <div class="slide-thumbnail">
              <canvas :ref="el => setThumbnailRef(el, index)" class="thumbnail-canvas" />
              <span class="slide-number">{{ index + 1 }}</span>
            </div>
            <div class="slide-title">{{ slide.title || `幻灯片 ${index + 1}` }}</div>
          </div>
        </div>
      </div>

      <div class="canvas-container" ref="canvasContainer">
        <canvas ref="canvas" class="main-canvas"></canvas>
        <div class="zoom-controls">
          <a-button-group size="small">
            <a-button @click="zoomOut">
              <template #icon><icon-minus /></template>
            </a-button>
            <a-button>{{ Math.round(zoom * 100) }}%</a-button>
            <a-button @click="zoomIn">
              <template #icon><icon-plus /></template>
            </a-button>
            <a-button @click="resetZoom">重置</a-button>
          </a-button-group>
        </div>
      </div>

      <div class="properties-panel">
        <PropertiesPanel
          :selected-element="selectedElement"
          :slide="currentSlide"
          @update-element="handleUpdateElement"
          @update-slide="handleUpdateSlide"
        />
      </div>
    </div>

    <LayerPanel
      v-if="currentSlide"
      :elements="currentSlide.elements"
      :selected-ids="selectedElementIds"
      @select="handleSelectElement"
      @reorder="handleReorderElements"
      @toggle-visibility="handleToggleVisibility"
      @toggle-lock="handleToggleLock"
      @delete="handleDeleteElement"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from 'vue';
import { Message } from '@arco-design/web-vue';
import { usePPTStore } from '@/store/modules/ppt';
import { useEditorStore } from '@/store/modules/editor';
import { useCanvas } from './useCanvas';
import { useHistory } from './useHistory';
import { useElementOperations } from './useElementOperations';
import EditorToolbar from './EditorToolbar.vue';
import PropertiesPanel from './PropertiesPanel.vue';
import LayerPanel from './LayerPanel.vue';
import type { CanvasElement, SlideElement, EditorTool } from './types';

const pptStore = usePPTStore();
const editorStore = useEditorStore();

const canvasContainer = ref<HTMLElement>();
const canvas = ref<HTMLCanvasElement>();
const thumbnailRefs = ref<Map<number, HTMLCanvasElement>>(new Map());

const { 
  initCanvas, 
  destroyCanvas, 
  zoomIn, 
  zoomOut, 
  resetZoom, 
  render,
  state: canvasState 
} = useCanvas(canvas, canvasContainer);

const { 
  canUndo, 
  canRedo, 
  undo, 
  redo, 
  pushState 
} = useHistory();

const {
  selectElement,
  moveElement,
  resizeElement,
  rotateElement,
  deleteElement,
  copyElement,
  pasteElement,
  alignElements,
  distributeElements,
} = useElementOperations();

const slides = computed(() => pptStore.slides);
const currentSlideIndex = computed(() => pptStore.currentSlideIndex);
const currentSlide = computed(() => pptStore.currentSlide);
const selectedElementIds = computed(() => editorStore.selectedElementIds);
const zoom = computed(() => canvasState.zoom);

const selectedElement = computed<CanvasElement | null>(() => {
  if (!currentSlide.value || selectedElementIds.value.length !== 1) {
    return null;
  }
  return currentSlide.value.elements.find(el => el.id === selectedElementIds.value[0]) || null;
});

const setThumbnailRef = (el: any, index: number) => {
  if (el) {
    thumbnailRefs.value.set(index, el);
  }
};

const handleToolChange = (tool: EditorTool) => {
  editorStore.setTool(tool);
};

const handleUndo = () => {
  const state = undo();
  if (state) {
    pptStore.setSlides(state.slides);
    pptStore.setCurrentSlideIndex(state.currentSlideIndex);
    render();
  }
};

const handleRedo = () => {
  const state = redo();
  if (state) {
    pptStore.setSlides(state.slides);
    pptStore.setCurrentSlideIndex(state.currentSlideIndex);
    render();
  }
};

const handleAddSlide = () => {
  const newSlide: SlideElement = {
    id: `slide-${Date.now()}`,
    title: '',
    elements: [],
    background: {
      type: 'color',
      color: '#ffffff',
      opacity: 1,
    },
  };
  
  pptStore.addSlide(newSlide as any);
  pushState({
    slides: slides.value,
    currentSlideIndex: currentSlideIndex.value,
    timestamp: Date.now(),
    action: '添加幻灯片',
  });
  
  Message.success('添加幻灯片成功');
};

const selectSlide = (index: number) => {
  pptStore.setCurrentSlideIndex(index);
  editorStore.clearSelection();
  render();
};

const handleDragStart = (e: DragEvent, index: number) => {
  if (e.dataTransfer) {
    e.dataTransfer.setData('slideIndex', String(index));
  }
};

const handleDrop = (e: DragEvent, toIndex: number) => {
  const fromIndexStr = e.dataTransfer?.getData('slideIndex');
  if (fromIndexStr) {
    const fromIndex = parseInt(fromIndexStr, 10);
    if (fromIndex !== toIndex) {
      pptStore.reorderSlides(fromIndex, toIndex);
      pushState({
        slides: slides.value,
        currentSlideIndex: currentSlideIndex.value,
        timestamp: Date.now(),
        action: '重新排序幻灯片',
      });
    }
  }
};

const handleSelectElement = (elementId: string, multi: boolean = false) => {
  selectElement(elementId, multi);
  render();
};

const handleUpdateElement = (elementId: string, updates: Partial<CanvasElement>) => {
  if (!currentSlide.value) return;
  
  const element = currentSlide.value.elements.find(el => el.id === elementId);
  if (element) {
    Object.assign(element, updates);
    pushState({
      slides: slides.value,
      currentSlideIndex: currentSlideIndex.value,
      timestamp: Date.now(),
      action: '更新元素',
    });
    render();
  }
};

const handleUpdateSlide = (updates: Partial<SlideElement>) => {
  if (!currentSlide.value) return;
  
  pptStore.updateSlide(currentSlideIndex.value, updates);
  pushState({
    slides: slides.value,
    currentSlideIndex: currentSlideIndex.value,
    timestamp: Date.now(),
    action: '更新幻灯片',
  });
  render();
};

const handleReorderElements = (fromIndex: number, toIndex: number) => {
  if (!currentSlide.value) return;
  
  const elements = currentSlide.value.elements;
  const [element] = elements.splice(fromIndex, 1);
  elements.splice(toIndex, 0, element);
  
  pushState({
    slides: slides.value,
    currentSlideIndex: currentSlideIndex.value,
    timestamp: Date.now(),
    action: '重新排序元素',
  });
  render();
};

const handleToggleVisibility = (elementId: string) => {
  if (!currentSlide.value) return;
  
  const element = currentSlide.value.elements.find(el => el.id === elementId);
  if (element) {
    element.visible = !element.visible;
    render();
  }
};

const handleToggleLock = (elementId: string) => {
  if (!currentSlide.value) return;
  
  const element = currentSlide.value.elements.find(el => el.id === elementId);
  if (element) {
    element.locked = !element.locked;
  }
};

const handleDeleteElement = (elementId: string) => {
  deleteElement(elementId);
  pushState({
    slides: slides.value,
    currentSlideIndex: currentSlideIndex.value,
    timestamp: Date.now(),
    action: '删除元素',
  });
  render();
};

const handleAIPolish = () => {
  Message.info('AI润色功能开发中...');
};

const renderThumbnails = () => {
  nextTick(() => {
    thumbnailRefs.value.forEach((canvasEl, index) => {
      const slide = slides.value[index];
      if (slide && canvasEl) {
        const ctx = canvasEl.getContext('2d');
        if (ctx) {
          ctx.fillStyle = '#ffffff';
          ctx.fillRect(0, 0, canvasEl.width, canvasEl.height);
          
          ctx.fillStyle = '#e0e0e0';
          ctx.font = '24px sans-serif';
          ctx.textAlign = 'center';
          ctx.fillText(String(index + 1), canvasEl.width / 2, canvasEl.height / 2);
        }
      }
    });
  });
};

watch(slides, () => {
  renderThumbnails();
}, { deep: true });

watch(currentSlideIndex, () => {
  render();
});

onMounted(() => {
  initCanvas();
  renderThumbnails();
  
  const handleKeyDown = (e: KeyboardEvent) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'z') {
      e.preventDefault();
      if (e.shiftKey) {
        handleRedo();
      } else {
        handleUndo();
      }
    }
    
    if ((e.ctrlKey || e.metaKey) && e.key === 'y') {
      e.preventDefault();
      handleRedo();
    }
    
    if ((e.ctrlKey || e.metaKey) && e.key === 'c') {
      e.preventDefault();
      if (selectedElement.value) {
        copyElement(selectedElement.value.id);
      }
    }
    
    if ((e.ctrlKey || e.metaKey) && e.key === 'v') {
      e.preventDefault();
      pasteElement();
      render();
    }
    
    if (e.key === 'Delete' || e.key === 'Backspace') {
      if (selectedElementIds.value.length > 0) {
        selectedElementIds.value.forEach(id => handleDeleteElement(id));
      }
    }
  };
  
  window.addEventListener('keydown', handleKeyDown);
});

onUnmounted(() => {
  destroyCanvas();
});
</script>

<style scoped lang="scss">
.canvas-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-2);
}

.editor-main {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.slide-list {
  width: 200px;
  background: var(--color-bg-1);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;

  .slide-list-header {
    padding: 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    font-weight: 500;
  }

  .slide-list-content {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .slide-item {
    padding: 8px;
    margin-bottom: 8px;
    border-radius: 4px;
    cursor: pointer;
    border: 2px solid var(--color-border);
    transition: all 0.2s;

    &:hover {
      border-color: rgb(var(--primary-6));
    }

    &.active {
      border-color: rgb(var(--primary-6));
      background: var(--color-fill-1);
    }

    .slide-thumbnail {
      width: 100%;
      height: 80px;
      background: var(--color-fill-2);
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 4px;
      position: relative;
      overflow: hidden;

      .thumbnail-canvas {
        width: 100%;
        height: 100%;
      }

      .slide-number {
        position: absolute;
        top: 4px;
        left: 4px;
        width: 20px;
        height: 20px;
        background: rgba(0, 0, 0, 0.5);
        color: white;
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 12px;
      }
    }

    .slide-title {
      font-size: 12px;
      text-align: center;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}

.canvas-container {
  flex: 1;
  position: relative;
  overflow: hidden;
  background: #f0f0f0;

  .main-canvas {
    display: block;
  }

  .zoom-controls {
    position: absolute;
    bottom: 16px;
    right: 16px;
    background: var(--color-bg-1);
    border-radius: 4px;
    padding: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  }
}

.properties-panel {
  width: 300px;
  background: var(--color-bg-1);
  border-left: 1px solid var(--color-border);
  overflow-y: auto;
}
</style>

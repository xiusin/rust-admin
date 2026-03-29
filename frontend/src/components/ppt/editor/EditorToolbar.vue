<template>
  <div class="editor-toolbar">
    <div class="toolbar-group">
      <a-tooltip content="选择 (V)">
        <a-button
          :type="currentTool === 'select' ? 'primary' : 'text'"
          @click="setTool('select')"
        >
          <template #icon><icon-pointer /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="平移 (H)">
        <a-button
          :type="currentTool === 'pan' ? 'primary' : 'text'"
          @click="setTool('pan')"
        >
          <template #icon><icon-drag-dot /></template>
        </a-button>
      </a-tooltip>
    </div>

    <a-divider direction="vertical" />

    <div class="toolbar-group">
      <a-tooltip content="文本 (T)">
        <a-button
          :type="currentTool === 'text' ? 'primary' : 'text'"
          @click="setTool('text')"
        >
          <template #icon><icon-font-colors /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="图片 (I)">
        <a-button
          :type="currentTool === 'image' ? 'primary' : 'text'"
          @click="setTool('image')"
        >
          <template #icon><icon-image /></template>
        </a-button>
      </a-tooltip>
      <a-dropdown trigger="click">
        <a-button :type="currentTool === 'shape' ? 'primary' : 'text'">
          <template #icon><icon-apps /></template>
        </a-button>
        <template #content>
          <a-doption @click="selectShape('rectangle')">
            <template #icon><icon-rectangle /></template>
            矩形
          </a-doption>
          <a-doption @click="selectShape('circle')">
            <template #icon><icon-heart /></template>
            圆形
          </a-doption>
          <a-doption @click="selectShape('triangle')">
            <template #icon><icon-caret-up /></template>
            三角形
          </a-doption>
          <a-doption @click="selectShape('line')">
            <template #icon><icon-line /></template>
            线条
          </a-doption>
          <a-doption @click="selectShape('arrow')">
            <template #icon><icon-arrow-right /></template>
            箭头
          </a-doption>
        </template>
      </a-dropdown>
      <a-tooltip content="图表">
        <a-button
          :type="currentTool === 'chart' ? 'primary' : 'text'"
          @click="setTool('chart')"
        >
          <template #icon><icon-bar-chart /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="表格">
        <a-button
          :type="currentTool === 'table' ? 'primary' : 'text'"
          @click="setTool('table')"
        >
          <template #icon><icon-table /></template>
        </a-button>
      </a-tooltip>
    </div>

    <a-divider direction="vertical" />

    <div class="toolbar-group">
      <a-tooltip content="撤销 (Ctrl+Z)">
        <a-button text :disabled="!canUndo" @click="handleUndo">
          <template #icon><icon-undo /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="重做 (Ctrl+Y)">
        <a-button text :disabled="!canRedo" @click="handleRedo">
          <template #icon><icon-redo /></template>
        </a-button>
      </a-tooltip>
    </div>

    <a-divider direction="vertical" />

    <div class="toolbar-group" v-if="hasSelection">
      <a-tooltip content="复制 (Ctrl+C)">
        <a-button text @click="handleCopy">
          <template #icon><icon-copy /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="粘贴 (Ctrl+V)">
        <a-button text :disabled="!hasClipboard" @click="handlePaste">
          <template #icon><icon-paste /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="删除 (Delete)">
        <a-button text @click="handleDelete">
          <template #icon><icon-delete /></template>
        </a-button>
      </a-tooltip>
    </div>

    <a-divider direction="vertical" v-if="hasSelection && isMultiSelect" />

    <div class="toolbar-group" v-if="hasSelection && isMultiSelect">
      <a-dropdown trigger="click">
        <a-button text>
          <template #icon><icon-align-left /></template>
          对齐
        </a-button>
        <template #content>
          <a-doption @click="handleAlign('left')">
            <template #icon><icon-align-left /></template>
            左对齐
          </a-doption>
          <a-doption @click="handleAlign('center')">
            <template #icon><icon-align-center /></template>
            水平居中
          </a-doption>
          <a-doption @click="handleAlign('right')">
            <template #icon><icon-align-right /></template>
            右对齐
          </a-doption>
          <a-divider />
          <a-doption @click="handleAlign('top')">
            <template #icon><icon-align-left /></template>
            顶部对齐
          </a-doption>
          <a-doption @click="handleAlign('middle')">
            <template #icon><icon-align-center /></template>
            垂直居中
          </a-doption>
          <a-doption @click="handleAlign('bottom')">
            <template #icon><icon-align-right /></template>
            底部对齐
          </a-doption>
        </template>
      </a-dropdown>
      <a-dropdown trigger="click">
        <a-button text>
          <template #icon><icon-menu /></template>
          分布
        </a-button>
        <template #content>
          <a-doption @click="handleDistribute('horizontal')">
            水平分布
          </a-doption>
          <a-doption @click="handleDistribute('vertical')">
            垂直分布
          </a-doption>
        </template>
      </a-dropdown>
    </div>

    <a-divider direction="vertical" v-if="hasSelection" />

    <div class="toolbar-group" v-if="hasSelection">
      <a-dropdown trigger="click">
        <a-button text>
          <template #icon><icon-layers /></template>
          图层
        </a-button>
        <template #content>
          <a-doption @click="handleLayer('front')">
            <template #icon><icon-to-top /></template>
            置于顶层
          </a-doption>
          <a-doption @click="handleLayer('forward')">
            <template #icon><icon-arrow-up /></template>
            上移一层
          </a-doption>
          <a-doption @click="handleLayer('backward')">
            <template #icon><icon-arrow-down /></template>
            下移一层
          </a-doption>
          <a-doption @click="handleLayer('back')">
            <template #icon><icon-to-bottom /></template>
            置于底层
          </a-doption>
        </template>
      </a-dropdown>
    </div>

    <div class="toolbar-spacer"></div>

    <div class="toolbar-group">
      <a-tooltip content="显示网格">
        <a-button
          :type="showGrid ? 'primary' : 'text'"
          @click="toggleGrid"
        >
          <template #icon><icon-apps /></template>
        </a-button>
      </a-tooltip>
      <a-tooltip content="对齐网格">
        <a-button
          :type="snapToGrid ? 'primary' : 'text'"
          @click="toggleSnapToGrid"
        >
          <template #icon><icon-magnet /></template>
        </a-button>
      </a-tooltip>
    </div>

    <a-divider direction="vertical" />

    <div class="toolbar-group">
      <a-button type="primary" @click="handleAIPolish">
        <template #icon><icon-robot /></template>
        AI润色
      </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useEditorStore } from '@/store/modules/editor';
import type { EditorTool } from './types';

const emit = defineEmits<{
  (e: 'tool-change', tool: EditorTool): void;
  (e: 'undo'): void;
  (e: 'redo'): void;
  (e: 'copy'): void;
  (e: 'paste'): void;
  (e: 'delete'): void;
  (e: 'align', alignment: 'left' | 'center' | 'right' | 'top' | 'middle' | 'bottom'): void;
  (e: 'distribute', direction: 'horizontal' | 'vertical'): void;
  (e: 'layer', action: 'front' | 'forward' | 'backward' | 'back'): void;
  (e: 'aiPolish'): void;
  (e: 'addElement', type: string, shape?: string): void;
}>();

defineProps<{
  canUndo?: boolean;
  canRedo?: boolean;
}>();

const editorStore = useEditorStore();

const currentTool = computed(() => editorStore.currentTool);
const hasSelection = computed(() => editorStore.hasSelection);
const isMultiSelect = computed(() => editorStore.isMultiSelect);
const hasClipboard = computed(() => !!editorStore.clipboard);
const showGrid = computed(() => editorStore.showGrid);
const snapToGrid = computed(() => editorStore.snapToGrid);

const setTool = (tool: EditorTool) => {
  editorStore.setTool(tool);
  emit('toolChange', tool);
};

const selectShape = (shapeType: string) => {
  editorStore.setTool('shape');
  emit('toolChange', 'shape');
  emit('addElement', 'shape', shapeType);
};

const handleUndo = () => {
  emit('undo');
};

const handleRedo = () => {
  emit('redo');
};

const handleCopy = () => {
  emit('copy');
};

const handlePaste = () => {
  emit('paste');
};

const handleDelete = () => {
  emit('delete');
};

const handleAlign = (alignment: 'left' | 'center' | 'right' | 'top' | 'middle' | 'bottom') => {
  emit('align', alignment);
};

const handleDistribute = (direction: 'horizontal' | 'vertical') => {
  emit('distribute', direction);
};

const handleLayer = (action: 'front' | 'forward' | 'backward' | 'back') => {
  emit('layer', action);
};

const toggleGrid = () => {
  editorStore.toggleGrid();
};

const toggleSnapToGrid = () => {
  editorStore.toggleSnapToGrid();
};

const handleAIPolish = () => {
  emit('aiPolish');
};
</script>

<style scoped lang="scss">
.editor-toolbar {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: var(--color-bg-1);
  border-bottom: 1px solid var(--color-border);
  gap: 8px;

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .toolbar-spacer {
    flex: 1;
  }

  :deep(.arco-btn) {
    padding: 4px 8px;
  }
}
</style>

<template>
  <div class="layer-panel">
    <div class="panel-header">
      <span>图层</span>
      <a-space>
        <a-tooltip content="全部显示">
          <a-button size="small" text @click="showAll">
            <template #icon><icon-eye /></template>
          </a-button>
        </a-tooltip>
        <a-tooltip content="全部锁定">
          <a-button size="small" text @click="lockAll">
            <template #icon><icon-lock /></template>
          </a-button>
        </a-tooltip>
      </a-space>
    </div>

    <div class="layer-list">
      <div
        v-for="(element, index) in sortedElements"
        :key="element.id"
        :class="['layer-item', { active: isSelected(element.id), locked: element.locked }]"
        draggable="true"
        @click="handleSelect(element.id, $event)"
        @dblclick="handleDoubleClick(element)"
        @dragstart="handleDragStart($event, index)"
        @dragover.prevent
        @drop="handleDrop($event, index)"
      >
        <div class="layer-drag-handle">
          <icon-drag-dot-vertical />
        </div>

        <div class="layer-preview">
          <div class="preview-icon">
            <icon-font-colors v-if="element.type === 'text'" />
            <icon-image v-else-if="element.type === 'image'" />
            <icon-apps v-else-if="element.type === 'shape'" />
            <icon-bar-chart v-else-if="element.type === 'chart'" />
            <icon-table v-else-if="element.type === 'table'" />
            <icon-line v-else />
          </div>
        </div>

        <div class="layer-info">
          <a-input
            v-if="editingId === element.id"
            v-model="editingName"
            size="small"
            :auto-focus="true"
            @blur="handleRename(element.id)"
            @keyup.enter="handleRename(element.id)"
            @keyup.esc="editingId = ''"
          />
          <template v-else>
            <span class="layer-name">{{ getElementName(element) }}</span>
            <span class="layer-type">{{ getTypeLabel(element.type) }}</span>
          </template>
        </div>

        <div class="layer-actions">
          <a-tooltip :content="element.visible ? '隐藏' : '显示'">
            <a-button
              size="small"
              text
              @click.stop="handleToggleVisibility(element.id)"
            >
              <template #icon>
                <icon-eye v-if="element.visible" />
                <icon-eye-invisible v-else />
              </template>
            </a-button>
          </a-tooltip>

          <a-tooltip :content="element.locked ? '解锁' : '锁定'">
            <a-button
              size="small"
              text
              @click.stop="handleToggleLock(element.id)"
            >
              <template #icon>
                <icon-lock v-if="element.locked" />
                <icon-unlock v-else />
              </template>
            </a-button>
          </a-tooltip>

          <a-popconfirm
            content="确定删除此图层吗？"
            @ok="handleDelete(element.id)"
          >
            <a-button size="small" text @click.stop>
              <template #icon><icon-delete /></template>
            </a-button>
          </a-popconfirm>
        </div>
      </div>

      <div v-if="elements.length === 0" class="empty-state">
        <icon-layers />
        <p>暂无图层</p>
        <p class="hint">添加元素后将在此显示</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import type { CanvasElement } from './types';

const props = defineProps<{
  elements: CanvasElement[];
  selectedIds: string[];
}>();

const emit = defineEmits<{
  (e: 'select', elementId: string, multi: boolean): void;
  (e: 'reorder', fromIndex: number, toIndex: number): void;
  (e: 'toggle-visibility', elementId: string): void;
  (e: 'toggle-lock', elementId: string): void;
  (e: 'delete', elementId: string): void;
  (e: 'rename', elementId: string, name: string): void;
}>();

const editingId = ref('');
const editingName = ref('');
const draggedIndex = ref(-1);

const sortedElements = computed(() => {
  return [...props.elements].reverse();
});

const isSelected = (elementId: string): boolean => {
  return props.selectedIds.includes(elementId);
};

const getTypeLabel = (type: string): string => {
  const labels: Record<string, string> = {
    text: '文本',
    image: '图片',
    shape: '形状',
    chart: '图表',
    table: '表格',
    line: '线条',
  };
  return labels[type] || type;
};

const getElementName = (element: CanvasElement): string => {
  if (element.type === 'text' && element.content.text) {
    return element.content.text.substring(0, 20) || '文本';
  }
  return `${getTypeLabel(element.type)}`;
};

const handleSelect = (elementId: string, event: MouseEvent) => {
  emit('select', elementId, event.ctrlKey || event.metaKey);
};

const handleDoubleClick = (element: CanvasElement) => {
  editingId.value = element.id;
  editingName.value = getElementName(element);
};

const handleRename = (elementId: string) => {
  if (editingName.value.trim()) {
    emit('rename', elementId, editingName.value.trim());
  }
  editingId.value = '';
};

const handleDragStart = (e: DragEvent, index: number) => {
  draggedIndex.value = index;
  if (e.dataTransfer) {
    e.dataTransfer.effectAllowed = 'move';
    e.dataTransfer.setData('text/plain', String(index));
  }
};

const handleDrop = (e: DragEvent, toIndex: number) => {
  if (draggedIndex.value === -1 || draggedIndex.value === toIndex) return;
  
  const actualFromIndex = props.elements.length - 1 - draggedIndex.value;
  const actualToIndex = props.elements.length - 1 - toIndex;
  
  emit('reorder', actualFromIndex, actualToIndex);
  draggedIndex.value = -1;
};

const handleToggleVisibility = (elementId: string) => {
  emit('toggle-visibility', elementId);
};

const handleToggleLock = (elementId: string) => {
  emit('toggle-lock', elementId);
};

const handleDelete = (elementId: string) => {
  emit('delete', elementId);
};

const showAll = () => {
  props.elements.forEach(el => {
    if (!el.visible) {
      emit('toggle-visibility', el.id);
    }
  });
};

const lockAll = () => {
  props.elements.forEach(el => {
    if (!el.locked) {
      emit('toggle-lock', el.id);
    }
  });
};
</script>

<style scoped lang="scss">
.layer-panel {
  position: absolute;
  right: 320px;
  top: 60px;
  width: 240px;
  max-height: calc(100vh - 120px);
  background: var(--color-bg-1);
  border: 1px solid var(--color-border);
  border-radius: 4px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
  display: flex;
  flex-direction: column;
  z-index: 10;

  .panel-header {
    padding: 12px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--color-border);
    font-weight: 500;
  }

  .layer-list {
    flex: 1;
    overflow-y: auto;
    padding: 8px;
  }

  .layer-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px;
    margin-bottom: 4px;
    border-radius: 4px;
    cursor: pointer;
    transition: all 0.2s;
    border: 1px solid transparent;

    &:hover {
      background: var(--color-fill-1);
    }

    &.active {
      background: var(--color-fill-2);
      border-color: rgb(var(--primary-6));
    }

    &.locked {
      opacity: 0.6;
    }

    .layer-drag-handle {
      cursor: move;
      color: var(--color-text-3);
      font-size: 12px;

      &:hover {
        color: var(--color-text-1);
      }
    }

    .layer-preview {
      width: 32px;
      height: 32px;
      background: var(--color-fill-2);
      border-radius: 4px;
      display: flex;
      align-items: center;
      justify-content: center;

      .preview-icon {
        font-size: 16px;
        color: var(--color-text-2);
      }
    }

    .layer-info {
      flex: 1;
      min-width: 0;

      .layer-name {
        display: block;
        font-size: 13px;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .layer-type {
        display: block;
        font-size: 11px;
        color: var(--color-text-3);
      }
    }

    .layer-actions {
      display: flex;
      gap: 4px;
      opacity: 0;
      transition: opacity 0.2s;

      :deep(.arco-btn) {
        padding: 2px 4px;
      }
    }

    &:hover .layer-actions {
      opacity: 1;
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
      font-size: 32px;
      margin-bottom: 8px;
    }

    .hint {
      font-size: 12px;
      margin-top: 4px;
    }
  }
}
</style>

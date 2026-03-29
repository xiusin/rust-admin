<template>
  <div class="history-panel">
    <div class="panel-header">
      <h3>操作历史</h3>
      <div class="header-actions">
        <a-button size="small" :disabled="!canUndo" @click="handleUndo">
          <template #icon><icon-undo /></template>
          撤销
        </a-button>
        <a-button size="small" :disabled="!canRedo" @click="handleRedo">
          <template #icon><icon-redo /></template>
          重做
        </a-button>
        <a-button size="small" status="danger" @click="handleClear">
          清空
        </a-button>
      </div>
    </div>
    
    <div class="history-list">
      <div
        v-for="(action, index) in historyList"
        :key="action.id"
        class="history-item"
        :class="{ active: index === 0 }"
        @click="handleSelectAction(action)"
      >
        <div class="action-icon">
          <component :is="getActionIcon(action.type)" />
        </div>
        <div class="action-info">
          <div class="action-type">{{ action.description }}</div>
          <div class="action-time">{{ formatTime(action.timestamp) }}</div>
        </div>
      </div>
      
      <div v-if="historyList.length === 0" class="empty-state">
        <icon-history class="empty-icon" />
        <span>暂无操作历史</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useHistoryStore, type HistoryAction } from '@/store/modules/history';
import { IconUndo, IconRedo, IconHistory, IconEdit, IconDelete, IconDragArrow, IconPlus } from '@arco-design/web-vue/es/icon';

const historyStore = useHistoryStore();

const canUndo = computed(() => historyStore.canUndo);
const canRedo = computed(() => historyStore.canRedo);
const historyList = computed(() => historyStore.historyList);

const emit = defineEmits<{
  (e: 'undo', action: HistoryAction): void;
  (e: 'redo', action: HistoryAction): void;
  (e: 'select', action: HistoryAction): void;
}>();

function handleUndo() {
  const action = historyStore.undo();
  if (action) {
    emit('undo', action);
  }
}

function handleRedo() {
  const action = historyStore.redo();
  if (action) {
    emit('redo', action);
  }
}

function handleClear() {
  historyStore.clear();
}

function handleSelectAction(action: HistoryAction) {
  emit('select', action);
}

function getActionIcon(type: string) {
  const icons: Record<string, any> = {
    'edit': IconEdit,
    'delete': IconDelete,
    'move': IconDragArrow,
    'add': IconPlus,
    'default': IconHistory,
  };
  return icons[type] || icons.default;
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}
</script>

<style scoped>
.history-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-2);
  border-left: 1px solid var(--color-border);
}

.panel-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.history-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.history-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 10px 12px;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.2s;
}

.history-item:hover {
  background: var(--color-fill-2);
}

.history-item.active {
  background: var(--color-primary-light-1);
}

.action-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-fill-2);
  border-radius: 6px;
  color: var(--color-text-2);
}

.action-info {
  flex: 1;
  min-width: 0;
}

.action-type {
  font-size: 13px;
  color: var(--color-text-1);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.action-time {
  font-size: 12px;
  color: var(--color-text-3);
  margin-top: 2px;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 40px 20px;
  color: var(--color-text-3);
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
  opacity: 0.5;
}
</style>

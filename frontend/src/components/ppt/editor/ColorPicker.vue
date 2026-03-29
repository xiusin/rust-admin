<template>
  <div class="color-picker">
    <div class="current-color" :style="{ background: modelValue }" @click="showPicker = !showPicker" />
    
    <a-popover v-model:popup-visible="showPicker" trigger="click">
      <div class="picker-content">
        <div class="preset-colors">
          <div
            v-for="color in presetColors"
            :key="color"
            class="color-item"
            :style="{ background: color }"
            :class="{ active: modelValue === color }"
            @click="selectColor(color)"
          />
        </div>
        
        <div class="color-input">
          <a-input v-model="inputColor" placeholder="#000000" @change="handleInput">
            <template #prefix>#</template>
          </a-input>
        </div>
        
        <div class="recent-colors" v-if="recentColors.length > 0">
          <div class="section-label">最近使用</div>
          <div class="recent-list">
            <div
              v-for="color in recentColors"
              :key="color"
              class="color-item small"
              :style="{ background: color }"
              @click="selectColor(color)"
            />
          </div>
        </div>
      </div>
    </a-popover>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';

const props = defineProps<{
  modelValue: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
}>();

const showPicker = ref(false);
const inputColor = ref(props.modelValue?.replace('#', '') || '');
const recentColors = ref<string[]>([]);

const presetColors = [
  '#000000', '#333333', '#666666', '#999999', '#CCCCCC', '#FFFFFF',
  '#FF0000', '#FF6B00', '#FFB800', '#FFDD00', '#FFFF00', '#A8FF00',
  '#00FF00', '#00FFA8', '#00FFFF', '#00A8FF', '#0066FF', '#0000FF',
  '#6600FF', '#A800FF', '#FF00FF', '#FF00A8', '#FF0066', '#FF0033',
];

watch(() => props.modelValue, (val) => {
  inputColor.value = val?.replace('#', '') || '';
});

function selectColor(color: string) {
  emit('update:modelValue', color);
  emit('change', color);
  addToRecent(color);
  showPicker.value = false;
}

function handleInput() {
  const color = `#${inputColor.value}`;
  if (/^#[0-9A-Fa-f]{6}$/.test(color)) {
    emit('update:modelValue', color);
    emit('change', color);
  }
}

function addToRecent(color: string) {
  const index = recentColors.value.indexOf(color);
  if (index > -1) {
    recentColors.value.splice(index, 1);
  }
  recentColors.value.unshift(color);
  if (recentColors.value.length > 10) {
    recentColors.value.pop();
  }
}
</script>

<style scoped>
.color-picker {
  display: inline-block;
}

.current-color {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  border: 1px solid var(--color-border);
  cursor: pointer;
  transition: transform 0.2s;
}

.current-color:hover {
  transform: scale(1.05);
}

.picker-content {
  padding: 12px;
  width: 240px;
}

.preset-colors {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 6px;
}

.color-item {
  width: 32px;
  height: 32px;
  border-radius: 4px;
  cursor: pointer;
  border: 2px solid transparent;
  transition: all 0.2s;
}

.color-item:hover {
  transform: scale(1.1);
}

.color-item.active {
  border-color: var(--color-primary);
}

.color-item.small {
  width: 24px;
  height: 24px;
}

.color-input {
  margin-top: 12px;
}

.recent-colors {
  margin-top: 12px;
}

.section-label {
  font-size: 12px;
  color: var(--color-text-3);
  margin-bottom: 8px;
}

.recent-list {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
}
</style>

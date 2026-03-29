<template>
  <div class="style-selector">
    <div class="style-grid">
      <div
        v-for="style in styleList"
        :key="style.id"
        :class="['style-item', { active: modelValue === style.id }]"
        @click="handleSelect(style.id)"
      >
        <div class="style-preview" :style="{ background: style.gradient }">
          <div class="style-content">
            <span class="style-title">{{ style.name }}</span>
          </div>
        </div>
        <div class="style-name">{{ style.name }}</div>
      </div>
    </div>

    <a-button v-if="modelValue" type="text" size="small" class="clear-btn" @click="handleClear">
      <template #icon><icon-close /></template>
      清除选择
    </a-button>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

defineProps<{
  modelValue?: number | null;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: number | null): void;
}>();

interface StyleOption {
  id: number;
  name: string;
  gradient: string;
}

const styleList = ref<StyleOption[]>([
  { id: 1, name: "商务蓝", gradient: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)" },
  { id: 2, name: "科技黑", gradient: "linear-gradient(135deg, #434343 0%, #000000 100%)" },
  { id: 3, name: "清新绿", gradient: "linear-gradient(135deg, #11998e 0%, #38ef7d 100%)" },
  { id: 4, name: "活力橙", gradient: "linear-gradient(135deg, #f093fb 0%, #f5576c 100%)" },
  { id: 5, name: "优雅紫", gradient: "linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)" },
  { id: 6, name: "温暖黄", gradient: "linear-gradient(135deg, #fa709a 0%, #fee140 100%)" },
  { id: 7, name: "经典红", gradient: "linear-gradient(135deg, #ff0844 0%, #ffb199 100%)" },
  { id: 8, name: "简约灰", gradient: "linear-gradient(135deg, #bdc3c7 0%, #2c3e50 100%)" }
]);

const handleSelect = (id: number) => {
  emit("update:modelValue", id);
};

const handleClear = () => {
  emit("update:modelValue", null);
};
</script>

<style scoped lang="scss">
.style-selector {
  position: relative;
}

.style-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.style-item {
  cursor: pointer;
  border-radius: 4px;
  overflow: hidden;
  border: 2px solid transparent;
  transition: all 0.2s;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  }

  &.active {
    border-color: rgb(var(--primary-6));
  }

  .style-preview {
    height: 60px;
    display: flex;
    align-items: center;
    justify-content: center;

    .style-content {
      color: #fff;
      text-align: center;
      padding: 8px;

      .style-title {
        font-size: 12px;
        font-weight: 500;
        text-shadow: 0 1px 2px rgba(0, 0, 0, 0.3);
      }
    }
  }

  .style-name {
    padding: 4px 8px;
    font-size: 12px;
    text-align: center;
    background: var(--color-fill-1);
    color: var(--color-text-2);
  }
}

.clear-btn {
  position: absolute;
  top: 0;
  right: 0;
}
</style>

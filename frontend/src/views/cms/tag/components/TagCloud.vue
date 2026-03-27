<template>
  <div class="tag-cloud">
    <div class="cloud-container">
      <span
        v-for="tag in tags"
        :key="tag.id"
        class="cloud-tag"
        :style="getTagStyle(tag)"
        @click="emit('select', tag)"
      >
        {{ tag.name }}
        <span class="tag-count">{{ tag.contentCount }}</span>
      </span>
    </div>
    <div v-if="tags.length === 0" class="empty-cloud">
      <a-empty description="暂无标签" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { CmsTagItem } from '@/api/modules/cms/tag';

interface Props {
  tags: CmsTagItem[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  select: [tag: CmsTagItem];
}>();

const maxCount = computed(() => {
  if (props.tags.length === 0) return 1;
  return Math.max(...props.tags.map((t) => t.contentCount || 0));
});

const getTagStyle = (tag: CmsTagItem) => {
  const ratio = (tag.contentCount || 0) / maxCount.value;
  const fontSize = 12 + ratio * 12;
  const opacity = 0.6 + ratio * 0.4;
  return {
    fontSize: `${fontSize}px`,
    opacity,
    color: tag.color || '#1890ff',
    borderColor: tag.color || '#1890ff',
  };
};
</script>

<style scoped lang="scss">
.tag-cloud {
  .cloud-container {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    padding: 16px;
    min-height: 200px;

    .cloud-tag {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      padding: 4px 12px;
      border: 1px solid;
      border-radius: 16px;
      cursor: pointer;
      transition: all 0.3s;

      &:hover {
        transform: scale(1.1);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
      }

      .tag-count {
        font-size: 10px;
        padding: 0 4px;
        background: currentColor;
        color: #fff;
        border-radius: 8px;
        opacity: 0.8;
      }
    }
  }

  .empty-cloud {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }
}
</style>

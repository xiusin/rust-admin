<template>
  <div class="image-renderer">
    <template v-if="imageUrls.length > 0">
      <a-image-preview-group v-if="imageUrls.length > 1">
        <a-space>
          <a-image v-for="(url, index) in displayImages" :key="index" :src="url" :width="width" :height="height" fit="cover" />
        </a-space>
      </a-image-preview-group>
      <a-image v-else :src="imageUrls[0]" :width="width" :height="height" fit="cover" />
    </template>
    <span v-else class="empty-text">{{ emptyText }}</span>
  </div>
</template>

<script setup lang="ts">
interface Props {
  value: string | string[];
  width?: number;
  height?: number;
  maxCount?: number;
  emptyText?: string;
}

const props = withDefaults(defineProps<Props>(), {
  width: 40,
  height: 40,
  maxCount: 3,
  emptyText: "-"
});

const imageUrls = computed(() => {
  if (!props.value) return [];
  if (Array.isArray(props.value)) return props.value;
  if (typeof props.value === "string") {
    try {
      const parsed = JSON.parse(props.value);
      return Array.isArray(parsed) ? parsed : [props.value];
    } catch {
      return props.value.split(",").filter(Boolean);
    }
  }
  return [];
});

const displayImages = computed(() => {
  return imageUrls.value.slice(0, props.maxCount);
});
</script>

<style lang="scss" scoped>
.image-renderer {
  display: flex;
  align-items: center;

  .empty-text {
    color: var(--color-text-3);
  }
}
</style>

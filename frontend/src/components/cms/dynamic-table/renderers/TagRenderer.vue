<template>
  <a-tag :color="tagColor" :size="size">
    {{ displayText }}
  </a-tag>
</template>

<script setup lang="ts">
interface TagColorMap {
  [key: string]: string;
}

interface Props {
  value: any;
  colorMap?: TagColorMap;
  defaultColor?: string;
  size?: "small" | "medium" | "large";
  emptyText?: string;
}

const props = withDefaults(defineProps<Props>(), {
  defaultColor: "gray",
  size: "small",
  emptyText: "-"
});

const displayText = computed(() => {
  if (props.value === null || props.value === undefined || props.value === "") {
    return props.emptyText;
  }
  return String(props.value);
});

const tagColor = computed(() => {
  if (!props.colorMap) return props.defaultColor;
  return props.colorMap[props.value] || props.defaultColor;
});
</script>

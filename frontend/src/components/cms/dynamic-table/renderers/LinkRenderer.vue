<template>
  <a-link :href="href" :target="target" :disabled="!href">
    {{ displayText }}
  </a-link>
</template>

<script setup lang="ts">
interface Props {
  value: string;
  text?: string;
  target?: "_blank" | "_self" | "_parent" | "_top";
  prefix?: string;
  suffix?: string;
}

const props = withDefaults(defineProps<Props>(), {
  target: "_blank"
});

const href = computed(() => {
  if (!props.value) return "";
  let url = props.value;
  if (props.prefix) url = props.prefix + url;
  if (props.suffix) url = url + props.suffix;
  return url;
});

const displayText = computed(() => {
  return props.text || props.value || "-";
});
</script>

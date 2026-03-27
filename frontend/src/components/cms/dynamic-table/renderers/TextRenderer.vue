<template>
  <span class="text-renderer">
    {{ displayText }}
  </span>
</template>

<script setup lang="ts">
interface Props {
  value: any
  emptyText?: string
  maxLength?: number
}

const props = withDefaults(defineProps<Props>(), {
  emptyText: '-',
  maxLength: 50
})

const displayText = computed(() => {
  if (props.value === null || props.value === undefined || props.value === '') {
    return props.emptyText
  }
  
  const text = String(props.value)
  if (text.length > props.maxLength) {
    return text.slice(0, props.maxLength) + '...'
  }
  
  return text
})
</script>

<style lang="scss" scoped>
.text-renderer {
  word-break: break-all;
}
</style>

<template>
  <a-select
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :multiple="multiple"
    :allow-clear="allowClear"
    :allow-search="allowSearch"
    :options="options"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface SelectOption {
  label: string
  value: any
  disabled?: boolean
}

interface Props {
  modelValue: any
  placeholder?: string
  disabled?: boolean
  multiple?: boolean
  allowClear?: boolean
  allowSearch?: boolean
  options?: SelectOption[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  placeholder: '请选择',
  disabled: false,
  multiple: false,
  allowClear: true,
  allowSearch: true,
  options: () => []
})

const emit = defineEmits<{
  'update:modelValue': [value: any]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})
</script>

<template>
  <a-input-number
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :min="min"
    :max="max"
    :step="step"
    :precision="precision"
    :mode="mode"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface Props {
  modelValue: number | undefined
  placeholder?: string
  disabled?: boolean
  min?: number
  max?: number
  step?: number
  precision?: number
  mode?: 'embed' | 'button'
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  placeholder: '请输入数字',
  disabled: false,
  mode: 'embed'
})

const emit = defineEmits<{
  'update:modelValue': [value: number | undefined]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})
</script>

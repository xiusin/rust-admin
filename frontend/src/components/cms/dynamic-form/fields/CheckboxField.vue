<template>
  <a-checkbox-group
    v-model="localValue"
    :disabled="disabled"
    :direction="direction"
    v-bind="$attrs"
  >
    <a-checkbox
      v-for="option in options"
      :key="option.value"
      :value="option.value"
      :disabled="option.disabled"
    >
      {{ option.label }}
    </a-checkbox>
  </a-checkbox-group>
</template>

<script setup lang="ts">
interface CheckboxOption {
  label: string
  value: any
  disabled?: boolean
}

interface Props {
  modelValue: any[]
  disabled?: boolean
  direction?: 'horizontal' | 'vertical'
  options?: CheckboxOption[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
  disabled: false,
  direction: 'horizontal',
  options: () => []
})

const emit = defineEmits<{
  'update:modelValue': [value: any[]]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})
</script>

<template>
  <a-radio-group
    v-model="localValue"
    :disabled="disabled"
    :type="type"
    :direction="direction"
    v-bind="$attrs"
  >
    <component
      :is="type === 'button' ? 'a-radio' : 'a-radio'"
      v-for="option in options"
      :key="option.value"
      :value="option.value"
      :disabled="option.disabled"
    >
      {{ option.label }}
    </component>
  </a-radio-group>
</template>

<script setup lang="ts">
interface RadioOption {
  label: string
  value: any
  disabled?: boolean
}

interface Props {
  modelValue: any
  disabled?: boolean
  type?: 'radio' | 'button'
  direction?: 'horizontal' | 'vertical'
  options?: RadioOption[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  disabled: false,
  type: 'radio',
  direction: 'horizontal',
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

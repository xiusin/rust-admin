<template>
  <a-input
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :max-length="maxLength"
    :allow-clear="allowClear"
    v-bind="$attrs"
  >
    <template v-if="prefix" #prefix>
      <component :is="prefix" />
    </template>
    <template v-if="suffix" #suffix>
      <component :is="suffix" />
    </template>
  </a-input>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string
  placeholder?: string
  disabled?: boolean
  maxLength?: number
  allowClear?: boolean
  prefix?: string
  suffix?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '请输入',
  disabled: false,
  allowClear: true
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})
</script>

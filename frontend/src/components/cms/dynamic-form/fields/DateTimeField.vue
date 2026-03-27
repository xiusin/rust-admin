<template>
  <a-date-picker
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :format="format"
    :value-format="valueFormat"
    :show-time="showTime"
    :allow-clear="allowClear"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface Props {
  modelValue: string | undefined
  placeholder?: string
  disabled?: boolean
  format?: string
  valueFormat?: string
  showTime?: boolean
  allowClear?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  placeholder: '请选择日期时间',
  disabled: false,
  format: 'YYYY-MM-DD HH:mm:ss',
  valueFormat: 'YYYY-MM-DD HH:mm:ss',
  showTime: true,
  allowClear: true
})

const emit = defineEmits<{
  'update:modelValue': [value: string | undefined]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})
</script>

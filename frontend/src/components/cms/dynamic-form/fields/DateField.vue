<template>
  <a-date-picker
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :format="format"
    :value-format="valueFormat"
    :allow-clear="allowClear"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface Props {
  modelValue: string | undefined;
  placeholder?: string;
  disabled?: boolean;
  format?: string;
  valueFormat?: string;
  allowClear?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: undefined,
  placeholder: "请选择日期",
  disabled: false,
  format: "YYYY-MM-DD",
  valueFormat: "YYYY-MM-DD",
  allowClear: true
});

const emit = defineEmits<{
  "update:modelValue": [value: string | undefined];
}>();

const localValue = computed({
  get: () => props.modelValue,
  set: val => emit("update:modelValue", val)
});
</script>

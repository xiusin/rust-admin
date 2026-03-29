<template>
  <a-textarea
    v-model="localValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :max-length="maxLength"
    :auto-size="autoSize"
    :allow-clear="allowClear"
    v-bind="$attrs"
  />
</template>

<script setup lang="ts">
interface Props {
  modelValue: string;
  placeholder?: string;
  disabled?: boolean;
  maxLength?: number;
  autoSize?: { minRows?: number; maxRows?: number } | boolean;
  allowClear?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "请输入",
  disabled: false,
  autoSize: () => ({ minRows: 3, maxRows: 6 }),
  allowClear: true
});

const emit = defineEmits<{
  "update:modelValue": [value: string];
}>();

const localValue = computed({
  get: () => props.modelValue,
  set: val => emit("update:modelValue", val)
});
</script>

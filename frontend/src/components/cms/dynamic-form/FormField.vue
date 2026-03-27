<template>
  <a-form-item
    :label="field.label"
    :field="field.name"
    :required="field.required"
    :rules="field.rules"
    :tooltip="field.config?.tooltip"
  >
    <component
      :is="fieldComponent"
      v-model="localValue"
      :placeholder="field.placeholder || `请输入${field.label}`"
      :disabled="disabled"
      v-bind="field.config"
    />
  </a-form-item>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import TextField from './fields/TextField.vue'
import TextareaField from './fields/TextareaField.vue'
import NumberField from './fields/NumberField.vue'
import SelectField from './fields/SelectField.vue'
import RadioField from './fields/RadioField.vue'
import CheckboxField from './fields/CheckboxField.vue'
import DateField from './fields/DateField.vue'
import DateTimeField from './fields/DateTimeField.vue'
import ImageField from './fields/ImageField.vue'
import FileField from './fields/FileField.vue'
import EditorField from './fields/EditorField.vue'
import JsonField from './fields/JsonField.vue'

interface FormFieldSchema {
  id: string
  name: string
  label: string
  type: string
  required?: boolean
  placeholder?: string
  rules?: any[]
  config?: Record<string, any>
}

interface Props {
  field: FormFieldSchema
  modelValue: any
  disabled?: boolean
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: any]
}>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const fieldComponentMap: Record<string, any> = {
  text: TextField,
  textarea: TextareaField,
  number: NumberField,
  select: SelectField,
  radio: RadioField,
  checkbox: CheckboxField,
  date: DateField,
  datetime: DateTimeField,
  image: ImageField,
  file: FileField,
  editor: EditorField,
  json: JsonField
}

const fieldComponent = computed(() => {
  return fieldComponentMap[props.field.type] || TextField
})
</script>

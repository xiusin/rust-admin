<template>
  <div class="dynamic-form">
    <a-form
      ref="formRef"
      :model="formData"
      :rules="formRules"
      :layout="schema.layout || 'horizontal'"
      :label-align="schema.labelAlign || 'right'"
      :size="schema.size || 'medium'"
      :disabled="disabled"
      @submit="handleSubmit"
    >
      <template v-if="schema.groups && schema.groups.length > 0">
        <div v-for="group in visibleGroups" :key="group.id" class="form-group">
          <div v-if="group.title" class="group-header">
            <h4>{{ group.title }}</h4>
            <span v-if="group.description" class="group-desc">{{ group.description }}</span>
          </div>
          <a-row :gutter="16">
            <a-col
              v-for="field in getVisibleFields(group.fields)"
              :key="field.id"
              :span="getFieldSpan(field)"
            >
              <FormField
                :field="field"
                :model-value="formData[field.name]"
                :disabled="isFieldDisabled(field)"
                @update:model-value="handleFieldChange(field, $event)"
              />
            </a-col>
          </a-row>
        </div>
      </template>
      
      <template v-else>
        <a-row :gutter="16">
          <a-col
            v-for="field in visibleFields"
            :key="field.id"
            :span="getFieldSpan(field)"
          >
            <FormField
              :field="field"
              :model-value="formData[field.name]"
              :disabled="isFieldDisabled(field)"
              @update:model-value="handleFieldChange(field, $event)"
            />
          </a-col>
        </a-row>
      </template>
      
      <div v-if="showActions" class="form-actions">
        <a-space>
          <a-button @click="handleReset">重置</a-button>
          <a-button type="primary" html-type="submit" :loading="loading">
            {{ submitText }}
          </a-button>
        </a-space>
      </div>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { Form } from '@arco-design/web-vue'
import FormField from './FormField.vue'

interface FormFieldSchema {
  id: string
  name: string
  label: string
  type: string
  required?: boolean
  placeholder?: string
  defaultValue?: any
  span?: number
  visible?: boolean
  disabled?: boolean
  rules?: any[]
  config?: Record<string, any>
  condition?: {
    field: string
    operator: string
    value: any
  }
}

interface FormGroupSchema {
  id: string
  title?: string
  description?: string
  fields: FormFieldSchema[]
}

interface FormSchema {
  layout?: 'horizontal' | 'vertical' | 'inline'
  labelAlign?: 'left' | 'right'
  size?: 'small' | 'medium' | 'large'
  columns?: number
  groups?: FormGroupSchema[]
  fields?: FormFieldSchema[]
}

interface Props {
  schema: FormSchema
  modelValue?: Record<string, any>
  disabled?: boolean
  showActions?: boolean
  submitText?: string
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  schema: () => ({ fields: [] }),
  modelValue: () => ({}),
  disabled: false,
  showActions: true,
  submitText: '提交',
  loading: false
})

const emit = defineEmits<{
  'update:modelValue': [value: Record<string, any>]
  'submit': [value: Record<string, any>]
  'reset': []
  'field-change': [field: FormFieldSchema, value: any]
}>()

const formRef = ref<InstanceType<typeof Form>>()
const formData = ref<Record<string, any>>({ ...props.modelValue })

const allFields = computed(() => {
  if (props.schema.groups && props.schema.groups.length > 0) {
    return props.schema.groups.flatMap(g => g.fields)
  }
  return props.schema.fields || []
})

const visibleFields = computed(() => {
  return getVisibleFields(allFields.value)
})

const visibleGroups = computed(() => {
  if (!props.schema.groups) return []
  return props.schema.groups.filter(group => {
    return getVisibleFields(group.fields).length > 0
  })
})

const formRules = computed(() => {
  const rules: Record<string, any> = {}
  allFields.value.forEach(field => {
    if (field.required) {
      rules[field.name] = [{
        required: true,
        message: `请输入${field.label}`
      }]
    }
    if (field.rules && field.rules.length > 0) {
      rules[field.name] = [...(rules[field.name] || []), ...field.rules]
    }
  })
  return rules
})

watch(
  () => props.modelValue,
  (val) => {
    formData.value = { ...val }
  },
  { deep: true }
)

watch(
  formData,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)

onMounted(() => {
  initFormData()
})

const initFormData = () => {
  allFields.value.forEach(field => {
    if (formData.value[field.name] === undefined) {
      formData.value[field.name] = field.defaultValue ?? null
    }
  })
}

const getVisibleFields = (fields: FormFieldSchema[]) => {
  return fields.filter(field => {
    if (field.visible === false) return false
    if (field.condition) {
      return checkCondition(field.condition)
    }
    return true
  })
}

const checkCondition = (condition: { field: string; operator: string; value: any }) => {
  const fieldValue = formData.value[condition.field]
  switch (condition.operator) {
    case '==':
      return fieldValue == condition.value
    case '!=':
      return fieldValue != condition.value
    case 'in':
      return Array.isArray(condition.value) && condition.value.includes(fieldValue)
    case 'not_in':
      return Array.isArray(condition.value) && !condition.value.includes(fieldValue)
    default:
      return true
  }
}

const getFieldSpan = (field: FormFieldSchema) => {
  if (field.span) return field.span
  const columns = props.schema.columns || 1
  return 24 / columns
}

const isFieldDisabled = (field: FormFieldSchema) => {
  return props.disabled || field.disabled
}

const handleFieldChange = (field: FormFieldSchema, value: any) => {
  formData.value[field.name] = value
  emit('field-change', field, value)
}

const handleSubmit = async () => {
  try {
    const valid = await formRef.value?.validate()
    if (!valid) {
      emit('submit', formData.value)
    }
  } catch (errors) {
    console.error('Form validation errors:', errors)
  }
}

const handleReset = () => {
  formRef.value?.resetFields()
  initFormData()
  emit('reset')
}

const validate = () => {
  return formRef.value?.validate()
}

const resetFields = () => {
  formRef.value?.resetFields()
}

defineExpose({
  validate,
  resetFields
})
</script>

<style lang="scss" scoped>
.dynamic-form {
  .form-group {
    margin-bottom: 24px;

    .group-header {
      margin-bottom: 16px;
      padding-bottom: 12px;
      border-bottom: 1px solid var(--color-border);

      h4 {
        margin: 0 0 4px;
        font-size: 15px;
        font-weight: 600;
      }

      .group-desc {
        font-size: 12px;
        color: var(--color-text-3);
      }
    }
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    margin-top: 24px;
    padding-top: 16px;
    border-top: 1px solid var(--color-border);
  }
}
</style>

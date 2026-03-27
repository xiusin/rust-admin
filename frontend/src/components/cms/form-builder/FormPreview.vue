<template>
  <a-modal
    v-model:visible="modalVisible"
    title="表单预览"
    :width="800"
    :footer="false"
  >
    <div class="form-preview">
      <a-form
        :model="formData"
        :label-align="schema.labelPosition"
        :label-col-props="{ style: { width: `${schema.labelWidth}px` } }"
        :size="schema.size"
      >
        <div v-for="group in schema.groups" :key="group.id" class="preview-group">
          <h4 v-if="group.title" class="group-title">{{ group.title }}</h4>
          <a-row :gutter="16">
            <a-col
              v-for="field in group.fields"
              :key="field.id"
              :span="getFieldSpan(field)"
            >
              <a-form-item :label="field.label">
                <component
                  :is="getFieldComponent(field.type)"
                  v-model="formData[field.name]"
                  :placeholder="field.config?.placeholder"
                  v-bind="field.config"
                />
              </a-form-item>
            </a-col>
          </a-row>
        </div>
      </a-form>
      
      <a-divider />
      
      <div class="preview-data">
        <h4>表单数据</h4>
        <pre>{{ JSON.stringify(formData, null, 2) }}</pre>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
interface FormField {
  id: string
  name: string
  label: string
  type: string
  config: Record<string, any>
  span?: number
}

interface FormGroup {
  id: string
  title: string
  fields: FormField[]
}

interface FormSchema {
  name: string
  labelPosition: 'left' | 'right' | 'top'
  labelWidth: number
  columns: number
  size: 'small' | 'medium' | 'large'
  groups: FormGroup[]
}

interface Props {
  visible: boolean
  schema: FormSchema
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:visible': [value: boolean]
}>()

const modalVisible = computed({
  get: () => props.visible,
  set: (val) => emit('update:visible', val)
})

const formData = ref<Record<string, any>>({})

const getFieldSpan = (field: FormField) => {
  if (field.span) return field.span
  return 24 / props.schema.columns
}

const getFieldComponent = (type: string) => {
  const componentMap: Record<string, string> = {
    text: 'a-input',
    textarea: 'a-textarea',
    number: 'a-input-number',
    select: 'a-select',
    radio: 'a-radio-group',
    checkbox: 'a-checkbox-group',
    date: 'a-date-picker',
    datetime: 'a-date-picker',
    image: 'a-upload',
    file: 'a-upload',
    editor: 'a-textarea',
    json: 'a-textarea'
  }
  return componentMap[type] || 'a-input'
}

watch(
  () => props.visible,
  (val) => {
    if (val) {
      formData.value = {}
    }
  }
)
</script>

<style lang="scss" scoped>
.form-preview {
  .preview-group {
    margin-bottom: 24px;

    .group-title {
      margin-bottom: 16px;
      padding-bottom: 8px;
      border-bottom: 1px solid var(--color-border);
    }
  }

  .preview-data {
    h4 {
      margin-bottom: 12px;
    }

    pre {
      padding: 12px;
      overflow: auto;
      font-size: 12px;
      background: var(--color-fill-1);
      border-radius: 4px;
    }
  }
}
</style>

<template>
  <div class="field-layout">
    <div v-if="fields.length === 0" class="empty-layout">
      <a-empty description="拖拽字段到此处" />
    </div>
    
    <draggable
      v-else
      v-model="localFields"
      item-key="id"
      group="formFields"
      animation="200"
      @end="handleDragEnd"
    >
      <template #item="{ element, index }">
        <div
          class="field-item"
          :class="{ 'is-selected': selectedFieldId === element.id }"
          :style="{ width: getFieldWidth(element) }"
          @click="handleSelect(element)"
        >
          <div class="field-header">
            <span class="field-label">{{ element.label }}</span>
            <a-space>
              <a-dropdown @select="handleAction($event, element, index)">
                <a-button type="text" size="small">
                  <icon-more />
                </a-button>
                <template #content>
                  <a-doption value="edit">编辑</a-doption>
                  <a-doption value="copy">复制</a-doption>
                  <a-doption value="delete" class="danger">删除</a-doption>
                </template>
              </a-dropdown>
            </a-space>
          </div>
          
          <div class="field-preview">
            <component
              :is="getFieldComponent(element.type)"
              disabled
              :placeholder="element.config?.placeholder || '请输入'"
              size="small"
            />
          </div>
          
          <div class="field-actions">
            <a-button-group size="mini">
              <a-button @click.stop="handleSpanChange(element, -1)">
                <icon-minus />
              </a-button>
              <a-button disabled>{{ element.span || defaultSpan }}</a-button>
              <a-button @click.stop="handleSpanChange(element, 1)">
                <icon-plus />
              </a-button>
            </a-button-group>
          </div>
        </div>
      </template>
    </draggable>
  </div>
</template>

<script setup lang="ts">
import draggable from 'vuedraggable'
import { Message } from '@arco-design/web-vue'

interface FormField {
  id: string
  name: string
  label: string
  type: string
  config: Record<string, any>
  span?: number
}

interface Props {
  modelValue: FormField[]
  columns?: number
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
  columns: 2
})

const emit = defineEmits<{
  'update:modelValue': [value: FormField[]]
  'select': [field: FormField]
}>()

const localFields = ref<FormField[]>([...props.modelValue])
const selectedFieldId = ref<string>('')

const defaultSpan = computed(() => 24 / props.columns)

watch(
  () => props.modelValue,
  (val) => {
    localFields.value = [...val]
  },
  { deep: true }
)

watch(
  localFields,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)

const getFieldWidth = (field: FormField) => {
  const span = field.span || defaultSpan.value
  return `${(span / 24) * 100}%`
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

const handleSelect = (field: FormField) => {
  selectedFieldId.value = field.id
  emit('select', field)
}

const handleDragEnd = () => {
  Message.success('排序已更新')
}

const handleSpanChange = (field: FormField, delta: number) => {
  const currentSpan = field.span || defaultSpan.value
  const newSpan = Math.max(1, Math.min(24, currentSpan + delta))
  field.span = newSpan
}

const handleAction = (action: string, field: FormField, index: number) => {
  switch (action) {
    case 'edit':
      handleSelect(field)
      break
    case 'copy':
      const newField = {
        ...field,
        id: `${field.id}_copy_${Date.now()}`,
        name: `${field.name}_copy`
      }
      localFields.value.splice(index + 1, 0, newField)
      Message.success('复制成功')
      break
    case 'delete':
      localFields.value.splice(index, 1)
      Message.success('删除成功')
      break
  }
}
</script>

<style lang="scss" scoped>
.field-layout {
  .empty-layout {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 150px;
    border: 1px dashed var(--color-border);
    border-radius: 4px;
  }

  .field-item {
    display: inline-block;
    padding: 8px;
    margin: 4px;
    vertical-align: top;
    background: var(--color-bg-2);
    border: 1px solid var(--color-border);
    border-radius: 4px;
    cursor: move;
    transition: all 0.2s;

    &:hover {
      border-color: rgb(var(--primary-6));
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);

      .field-actions {
        opacity: 1;
      }
    }

    &.is-selected {
      border-color: rgb(var(--primary-6));
      box-shadow: 0 0 0 2px rgba(var(--primary-6), 0.2);
    }

    .field-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 8px;

      .field-label {
        font-size: 13px;
        font-weight: 500;
      }
    }

    .field-preview {
      margin-bottom: 8px;
      pointer-events: none;
    }

    .field-actions {
      display: flex;
      justify-content: flex-end;
      opacity: 0;
      transition: opacity 0.2s;
    }
  }
}

.danger {
  color: rgb(var(--danger-6));
}
</style>

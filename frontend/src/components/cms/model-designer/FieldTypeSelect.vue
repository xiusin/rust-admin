<template>
  <a-select v-model="selectedType" placeholder="请选择字段类型" @change="handleChange">
    <a-option-group v-for="group in fieldGroups" :key="group.label" :label="group.label">
      <a-option v-for="field in group.fields" :key="field.value" :value="field.value">
        <div class="field-option">
          <component :is="field.icon" class="field-icon" />
          <span class="field-label">{{ field.label }}</span>
          <span class="field-desc">{{ field.description }}</span>
        </div>
      </a-option>
    </a-option-group>
  </a-select>
</template>

<script setup lang="ts">
interface Props {
  modelValue: string
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

const selectedType = ref(props.modelValue)

watch(
  () => props.modelValue,
  (val) => {
    selectedType.value = val
  }
)

const handleChange = (val: string) => {
  emit('update:modelValue', val)
}

const fieldGroups = [
  {
    label: '基础字段',
    fields: [
      { value: 'text', label: '文本', description: '短文本输入', icon: 'icon-font-colors' },
      { value: 'textarea', label: '多行文本', description: '长文本输入', icon: 'icon-align-left' },
      { value: 'number', label: '数字', description: '数字输入', icon: 'icon-calculator' }
    ]
  },
  {
    label: '选择字段',
    fields: [
      { value: 'select', label: '下拉选择', description: '下拉选择框', icon: 'icon-down' },
      { value: 'radio', label: '单选', description: '单选按钮', icon: 'icon-check-circle' },
      { value: 'checkbox', label: '多选', description: '复选框', icon: 'icon-check-square' }
    ]
  },
  {
    label: '日期字段',
    fields: [
      { value: 'date', label: '日期', description: '日期选择', icon: 'icon-calendar' },
      { value: 'datetime', label: '日期时间', description: '日期时间选择', icon: 'icon-clock-circle' }
    ]
  },
  {
    label: '媒体字段',
    fields: [
      { value: 'image', label: '图片', description: '图片上传', icon: 'icon-image' },
      { value: 'file', label: '文件', description: '文件上传', icon: 'icon-file' }
    ]
  },
  {
    label: '高级字段',
    fields: [
      { value: 'editor', label: '富文本', description: '富文本编辑器', icon: 'icon-edit' },
      { value: 'json', label: 'JSON', description: 'JSON编辑器', icon: 'icon-code' }
    ]
  }
]
</script>

<style lang="scss" scoped>
.field-option {
  display: flex;
  gap: 8px;
  align-items: center;

  .field-icon {
    color: var(--color-text-2);
  }

  .field-label {
    font-weight: 500;
  }

  .field-desc {
    font-size: 12px;
    color: var(--color-text-3);
  }
}
</style>

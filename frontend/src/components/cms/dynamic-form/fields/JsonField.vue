<template>
  <div class="json-field">
    <div class="json-toolbar">
      <a-space>
        <a-button size="small" @click="handleFormat">
          <icon-code />
          格式化
        </a-button>
        <a-button size="small" @click="handleCompress">
          <icon-mind-mapping />
          压缩
        </a-button>
        <a-button size="small" @click="handleClear">
          <icon-delete />
          清空
        </a-button>
      </a-space>
      <span v-if="error" class="error-text">{{ error }}</span>
    </div>
    <a-textarea
      v-model="localValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :auto-size="{ minRows: 6, maxRows: 20 }"
      @blur="handleValidate"
    />
  </div>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue'

interface Props {
  modelValue: any
  placeholder?: string
  disabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '请输入JSON数据',
  disabled: false
})

const emit = defineEmits<{
  'update:modelValue': [value: any]
}>()

const error = ref('')

const localValue = computed({
  get: () => {
    if (typeof props.modelValue === 'object') {
      return JSON.stringify(props.modelValue, null, 2)
    }
    return props.modelValue || ''
  },
  set: (val) => {
    emit('update:modelValue', val)
  }
})

const handleValidate = () => {
  error.value = ''
  if (!localValue.value) return
  
  try {
    JSON.parse(localValue.value)
  } catch (e: any) {
    error.value = 'JSON格式错误'
  }
}

const handleFormat = () => {
  try {
    const obj = JSON.parse(localValue.value)
    localValue.value = JSON.stringify(obj, null, 2)
    error.value = ''
    Message.success('格式化成功')
  } catch {
    error.value = 'JSON格式错误，无法格式化'
    Message.error('JSON格式错误')
  }
}

const handleCompress = () => {
  try {
    const obj = JSON.parse(localValue.value)
    localValue.value = JSON.stringify(obj)
    error.value = ''
    Message.success('压缩成功')
  } catch {
    error.value = 'JSON格式错误，无法压缩'
    Message.error('JSON格式错误')
  }
}

const handleClear = () => {
  localValue.value = ''
  error.value = ''
}
</script>

<style lang="scss" scoped>
.json-field {
  .json-toolbar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;

    .error-text {
      font-size: 12px;
      color: rgb(var(--danger-6));
    }
  }

  :deep(.arco-textarea) {
    font-family: 'Consolas', 'Monaco', monospace;
    font-size: 13px;
  }
}
</style>

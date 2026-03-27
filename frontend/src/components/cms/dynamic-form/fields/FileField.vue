<template>
  <a-upload
    v-model:file-list="fileList"
    :list-type="listType"
    :accept="accept"
    :multiple="multiple"
    :limit="limit"
    :disabled="disabled"
    :auto-upload="autoUpload"
    :action="action"
    :headers="headers"
    :data="data"
    draggable
    @success="handleSuccess"
    @error="handleError"
    @exceed-limit="handleExceed"
    v-bind="$attrs"
  >
    <template #upload-button>
      <div v-if="fileList.length < (limit || 1)" class="upload-area">
        <icon-cloud-upload class="upload-icon" />
        <div class="upload-text">点击或拖拽文件到此区域上传</div>
        <div class="upload-hint">{{ hint || '支持任意格式文件' }}</div>
      </div>
    </template>
  </a-upload>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue'

interface UploadFile {
  uid: string
  name: string
  url?: string
  status?: string
  response?: any
}

interface Props {
  modelValue: string | string[]
  placeholder?: string
  disabled?: boolean
  accept?: string
  multiple?: boolean
  limit?: number
  listType?: 'text' | 'picture' | 'picture-card'
  autoUpload?: boolean
  action?: string
  headers?: Record<string, string>
  data?: Record<string, any>
  hint?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '请上传文件',
  disabled: false,
  accept: '*',
  multiple: false,
  limit: 1,
  listType: 'text',
  autoUpload: true,
  action: '/api/upload'
})

const emit = defineEmits<{
  'update:modelValue': [value: string | string[]]
  'success': [file: any]
  'error': [error: any]
}>()

const fileList = ref<UploadFile[]>([])

watch(
  () => props.modelValue,
  (val) => {
    if (!val) {
      fileList.value = []
      return
    }
    
    const urls = Array.isArray(val) ? val : [val]
    fileList.value = urls.map((url, index) => ({
      uid: `-${index}`,
      name: url.split('/').pop() || '',
      url,
      status: 'done'
    }))
  },
  { immediate: true }
)

const handleSuccess = (response: any) => {
  if (response.code === 0 || response.success) {
    const url = response.data?.url || response.url
    if (props.multiple) {
      const urls = fileList.value
        .filter(f => f.status === 'done' && f.url)
        .map(f => f.url)
      emit('update:modelValue', urls)
    } else {
      emit('update:modelValue', url)
    }
    emit('success', response)
    Message.success('上传成功')
  }
}

const handleError = (error: any) => {
  Message.error('上传失败')
  emit('error', error)
}

const handleExceed = () => {
  Message.warning(`最多只能上传 ${props.limit} 个文件`)
}
</script>

<style lang="scss" scoped>
.upload-area {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 150px;
  background: var(--color-fill-1);
  border: 1px dashed var(--color-border);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    border-color: rgb(var(--primary-6));
  }

  .upload-icon {
    font-size: 32px;
    color: var(--color-text-3);
  }

  .upload-text {
    margin-top: 8px;
    font-size: 14px;
    color: var(--color-text-2);
  }

  .upload-hint {
    margin-top: 4px;
    font-size: 12px;
    color: var(--color-text-3);
  }
}
</style>

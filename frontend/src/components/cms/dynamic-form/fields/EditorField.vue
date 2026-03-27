<template>
  <div class="editor-field">
    <Toolbar
      v-if="showToolbar"
      class="editor-toolbar"
      :editor="editorRef"
      :default-config="toolbarConfig"
      mode="default"
    />
    <Editor
      v-model="localValue"
      class="editor-content"
      :style="{ height: height }"
      :default-config="editorConfig"
      mode="default"
      @on-created="handleCreated"
      @on-change="handleChange"
    />
  </div>
</template>

<script setup lang="ts">
import { Editor, Toolbar } from '@wangeditor/editor-for-vue'
import type { IDomEditor, IEditorConfig, IToolbarConfig } from '@wangeditor/editor'

interface Props {
  modelValue: string
  placeholder?: string
  disabled?: boolean
  height?: string
  showToolbar?: boolean
  uploadImageServer?: string
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  placeholder: '请输入内容...',
  disabled: false,
  height: '300px',
  showToolbar: true,
  uploadImageServer: '/api/upload'
})

const emit = defineEmits<{
  'update:modelValue': [value: string]
  'change': [value: string]
}>()

const editorRef = shallowRef<IDomEditor>()

const localValue = computed({
  get: () => props.modelValue,
  set: (val) => emit('update:modelValue', val)
})

const toolbarConfig: Partial<IToolbarConfig> = {
  excludeKeys: []
}

const editorConfig: Partial<IEditorConfig> = {
  placeholder: props.placeholder,
  readOnly: props.disabled,
  MENU_CONF: {
    uploadImage: {
      server: props.uploadImageServer,
      fieldName: 'file',
      maxFileSize: 5 * 1024 * 1024,
      allowedFileTypes: ['image/*'],
      customInsert(res: any, insertFn: any) {
        if (res.code === 0 || res.success) {
          const url = res.data?.url || res.url
          insertFn(url)
        }
      }
    }
  }
}

const handleCreated = (editor: IDomEditor) => {
  editorRef.value = editor
}

const handleChange = (editor: IDomEditor) => {
  emit('change', editor.getHtml())
}

onBeforeUnmount(() => {
  const editor = editorRef.value
  if (editor) {
    editor.destroy()
  }
})
</script>

<style lang="scss" scoped>
@import '@wangeditor/editor/dist/css/style.css';

.editor-field {
  border: 1px solid var(--color-border);
  border-radius: 4px;
  overflow: hidden;

  .editor-toolbar {
    border-bottom: 1px solid var(--color-border);
  }

  .editor-content {
    overflow-y: auto;
  }
}
</style>

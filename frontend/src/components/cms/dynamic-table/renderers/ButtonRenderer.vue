<template>
  <a-space>
    <a-button
      v-for="btn in buttons"
      :key="btn.label"
      :type="btn.type || 'text'"
      :status="btn.status"
      :size="size"
      :disabled="isDisabled(btn)"
      @click="handleClick(btn)"
    >
      <template v-if="btn.icon" #icon>
        <component :is="btn.icon" />
      </template>
      {{ btn.label }}
    </a-button>
  </a-space>
</template>

<script setup lang="ts">
interface ButtonConfig {
  label: string
  type?: 'primary' | 'secondary' | 'dashed' | 'outline' | 'text'
  status?: 'normal' | 'success' | 'warning' | 'danger'
  icon?: string
  action?: string
  condition?: string
  confirm?: string
}

interface Props {
  buttons: ButtonConfig[]
  record: any
  size?: 'mini' | 'small' | 'medium' | 'large'
}

const props = withDefaults(defineProps<Props>(), {
  buttons: () => [],
  size: 'small'
})

const emit = defineEmits<{
  'click': [button: ButtonConfig, record: any]
}>()

const isDisabled = (btn: ButtonConfig) => {
  if (!btn.condition) return false
  
  try {
    const fn = new Function('record', `return ${btn.condition}`)
    return !fn(props.record)
  } catch {
    return false
  }
}

const handleClick = async (btn: ButtonConfig) => {
  if (btn.confirm) {
    const confirmed = await new Promise(resolve => {
      Modal.confirm({
        title: '确认操作',
        content: btn.confirm,
        onOk: () => resolve(true),
        onCancel: () => resolve(false)
      })
    })
    if (!confirmed) return
  }
  
  emit('click', btn, props.record)
}
</script>

<script lang="ts">
import { Modal } from '@arco-design/web-vue'

export default {
  name: 'ButtonRenderer'
}
</script>

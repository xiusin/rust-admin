<template>
  <div class="code-viewer">
    <div class="viewer-header">
      <div class="file-info">
        <icon-file class="file-icon" />
        <span class="file-name">{{ filename }}</span>
        <a-tag v-if="language" size="small">{{ language }}</a-tag>
      </div>
      <div class="viewer-actions">
        <a-button size="small" @click="handleCopy">
          <template #icon><icon-copy /></template>
          复制
        </a-button>
        <a-button size="small" @click="handleSelectAll">
          <template #icon><icon-select-all /></template>
          全选
        </a-button>
      </div>
    </div>
    
    <div class="viewer-content">
      <div class="line-numbers">
        <span v-for="line in lineCount" :key="line" class="line-number">{{ line }}</span>
      </div>
      <pre class="code-content"><code :class="`language-${language}`">{{ code }}</code></pre>
    </div>
    
    <div class="viewer-footer">
      <span class="code-stats">
        共 {{ lineCount }} 行 | {{ codeLength }} 字符
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from '@arco-design/web-vue'

interface Props {
  code: string
  language?: string
  filename?: string
}

const props = defineProps<Props>()

const lineCount = computed(() => {
  if (!props.code) return 0
  return props.code.split('\n').length
})

const codeLength = computed(() => {
  return props.code?.length || 0
})

const handleCopy = async () => {
  try {
    await navigator.clipboard.writeText(props.code)
    Message.success('复制成功')
  } catch {
    Message.error('复制失败')
  }
}

const handleSelectAll = () => {
  const selection = window.getSelection()
  const range = document.createRange()
  const codeElement = document.querySelector('.code-content')
  if (codeElement && selection) {
    range.selectNodeContents(codeElement)
    selection.removeAllRanges()
    selection.addRange(range)
  }
}
</script>

<style lang="scss" scoped>
.code-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #1e1e1e;

  .viewer-header {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: space-between;
    padding: 12px 16px;
    background: #252526;
    border-bottom: 1px solid #3c3c3c;

    .file-info {
      display: flex;
      gap: 8px;
      align-items: center;
      color: #cccccc;

      .file-icon {
        color: #519aba;
      }

      .file-name {
        font-size: 13px;
      }
    }

    .viewer-actions {
      display: flex;
      gap: 8px;
    }
  }

  .viewer-content {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: auto;

    .line-numbers {
      flex-shrink: 0;
      padding: 16px 12px;
      font-family: 'Consolas', 'Monaco', monospace;
      font-size: 13px;
      line-height: 1.6;
      color: #858585;
      text-align: right;
      background: #1e1e1e;
      border-right: 1px solid #3c3c3c;
      user-select: none;

      .line-number {
        display: block;
      }
    }

    .code-content {
      flex: 1;
      margin: 0;
      padding: 16px;
      overflow-x: auto;
      font-family: 'Consolas', 'Monaco', monospace;
      font-size: 13px;
      line-height: 1.6;
      color: #d4d4d4;
      white-space: pre;

      code {
        display: block;
      }
    }
  }

  .viewer-footer {
    flex-shrink: 0;
    padding: 8px 16px;
    background: #007acc;
    border-top: 1px solid #3c3c3c;

    .code-stats {
      font-size: 12px;
      color: #ffffff;
    }
  }
}
</style>

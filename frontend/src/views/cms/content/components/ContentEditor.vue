<template>
  <div class="content-editor">
    <a-textarea
      v-if="!showEditor"
      v-model="modelValue"
      placeholder="请输入内容"
      :auto-size="{ minRows: 10, maxRows: 20 }"
      allow-clear
    />
    <div v-else class="editor-wrapper">
      <div class="editor-toolbar">
        <a-space>
          <a-button size="small" @click="insertText">
            <template #icon><icon-font-colors /></template>
          </a-button>
          <a-button size="small" @click="insertImage">
            <template #icon><icon-image /></template>
          </a-button>
          <a-button size="small" @click="insertLink">
            <template #icon><icon-link /></template>
          </a-button>
          <a-divider direction="vertical" />
          <a-button size="small" @click="togglePreview">
            <template #icon><icon-eye /></template>
            预览
          </a-button>
        </a-space>
      </div>
      <div class="editor-body">
        <a-textarea
          v-model="modelValue"
          placeholder="请输入内容（支持HTML）"
          :auto-size="{ minRows: 15, maxRows: 30 }"
          allow-clear
        />
      </div>
    </div>
    <div class="editor-footer">
      <a-switch v-model="showEditor" size="small">
        <template #checked>富文本</template>
        <template #unchecked>纯文本</template>
      </a-switch>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const modelValue = defineModel<string>({ default: '' });
const showEditor = ref(false);
const previewVisible = ref(false);

const insertText = () => {
  modelValue.value += '<p>示例文本</p>';
};

const insertImage = () => {
  modelValue.value += '<img src="图片地址" alt="图片描述" />';
};

const insertLink = () => {
  modelValue.value += '<a href="链接地址">链接文本</a>';
};

const togglePreview = () => {
  previewVisible.value = !previewVisible.value;
};
</script>

<style scoped lang="scss">
.content-editor {
  .editor-wrapper {
    border: 1px solid var(--color-border-2);
    border-radius: 4px;

    .editor-toolbar {
      padding: 8px;
      border-bottom: 1px solid var(--color-border-2);
      background: var(--color-fill-1);
    }

    .editor-body {
      min-height: 300px;
    }
  }

  .editor-footer {
    margin-top: 8px;
    display: flex;
    justify-content: flex-end;
  }
}
</style>

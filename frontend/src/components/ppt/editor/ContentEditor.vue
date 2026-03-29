<template>
  <div class="content-editor">
    <div class="editor-header">
      <h3>内容编辑</h3>
      <div class="ai-tools">
        <a-button size="small" @click="handlePolish">
          <template #icon><icon-magic /></template>
          润色
        </a-button>
        <a-button size="small" @click="handleContinue">
          <template #icon><icon-plus /></template>
          续写
        </a-button>
        <a-button size="small" @click="handleSummarize">
          <template #icon><icon-file /></template>
          总结
        </a-button>
        <a-button size="small" @click="handleTranslate">
          <template #icon><icon-language /></template>
          翻译
        </a-button>
      </div>
    </div>
    
    <div class="editor-content">
      <a-textarea
        v-model="content"
        :auto-size="{ minRows: 6, maxRows: 15 }"
        placeholder="输入或粘贴内容..."
        @change="handleContentChange"
      />
    </div>
    
    <div class="editor-footer">
      <div class="word-count">
        字数: {{ wordCount }}
      </div>
      <div class="actions">
        <a-button size="small" @click="handleReset">重置</a-button>
        <a-button type="primary" size="small" @click="handleApply">应用</a-button>
      </div>
    </div>
    
    <a-modal v-model:visible="showTranslateModal" title="翻译" @ok="confirmTranslate">
      <a-form :model="translateForm" layout="vertical">
        <a-form-item label="目标语言">
          <a-select v-model="translateForm.targetLanguage">
            <a-option value="en">英语</a-option>
            <a-option value="ja">日语</a-option>
            <a-option value="ko">韩语</a-option>
            <a-option value="fr">法语</a-option>
            <a-option value="de">德语</a-option>
            <a-option value="es">西班牙语</a-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
    
    <a-modal v-model:visible="showResultModal" :title="resultTitle" @ok="applyResult">
      <div class="result-content">
        <div class="original">
          <div class="label">原文:</div>
          <div class="text">{{ originalContent }}</div>
        </div>
        <div class="result">
          <div class="label">结果:</div>
          <div class="text">{{ resultContent }}</div>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { Message } from '@arco-design/web-vue';
import { IconMagic, IconPlus, IconFile, IconLanguage } from '@arco-design/web-vue/es/icon';
import { polishText, continueContent, summarizeContent, translateContent } from '@/api/modules/ppt/editor';

const props = defineProps<{
  modelValue: string;
  elementType?: string;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
  (e: 'change', value: string): void;
  (e: 'ai-action', type: string, result: string): void;
}>();

const content = ref(props.modelValue || '');
const originalContent = ref('');
const resultContent = ref('');
const resultTitle = ref('');
const showTranslateModal = ref(false);
const showResultModal = ref(false);

const translateForm = reactive({
  targetLanguage: 'en',
});

const wordCount = computed(() => {
  return content.value.replace(/\s/g, '').length;
});

watch(() => props.modelValue, (val) => {
  content.value = val || '';
});

function handleContentChange() {
  emit('update:modelValue', content.value);
  emit('change', content.value);
}

async function handlePolish() {
  if (!content.value.trim()) {
    Message.warning('请先输入内容');
    return;
  }
  
  try {
    const result = await polishText({ text: content.value });
    originalContent.value = content.value;
    resultContent.value = result.polished;
    resultTitle.value = '润色结果';
    showResultModal.value = true;
  } catch (error: any) {
    Message.error(error.message || '润色失败');
  }
}

async function handleContinue() {
  if (!content.value.trim()) {
    Message.warning('请先输入内容');
    return;
  }
  
  try {
    const result = await continueContent({ content: content.value });
    originalContent.value = content.value;
    resultContent.value = content.value + '\n' + result.continued;
    resultTitle.value = '续写结果';
    showResultModal.value = true;
  } catch (error: any) {
    Message.error(error.message || '续写失败');
  }
}

async function handleSummarize() {
  if (!content.value.trim()) {
    Message.warning('请先输入内容');
    return;
  }
  
  try {
    const result = await summarizeContent({ content: content.value });
    originalContent.value = content.value;
    resultContent.value = result.summary;
    resultTitle.value = '总结结果';
    showResultModal.value = true;
  } catch (error: any) {
    Message.error(error.message || '总结失败');
  }
}

function handleTranslate() {
  if (!content.value.trim()) {
    Message.warning('请先输入内容');
    return;
  }
  showTranslateModal.value = true;
}

async function confirmTranslate() {
  try {
    const result = await translateContent({
      content: content.value,
      target_language: translateForm.targetLanguage,
    });
    originalContent.value = content.value;
    resultContent.value = result.translated;
    resultTitle.value = '翻译结果';
    showTranslateModal.value = false;
    showResultModal.value = true;
  } catch (error: any) {
    Message.error(error.message || '翻译失败');
  }
}

function applyResult() {
  content.value = resultContent.value;
  emit('update:modelValue', content.value);
  emit('change', content.value);
  showResultModal.value = false;
}

function handleReset() {
  content.value = props.modelValue || '';
}

function handleApply() {
  emit('update:modelValue', content.value);
  emit('change', content.value);
  Message.success('内容已更新');
}
</script>

<style scoped>
.content-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg-2);
}

.editor-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.editor-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
}

.ai-tools {
  display: flex;
  gap: 8px;
}

.editor-content {
  flex: 1;
  padding: 16px;
  overflow: auto;
}

.editor-footer {
  padding: 12px 16px;
  border-top: 1px solid var(--color-border);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.word-count {
  font-size: 12px;
  color: var(--color-text-3);
}

.actions {
  display: flex;
  gap: 8px;
}

.result-content {
  padding: 12px 0;
}

.result-content .label {
  font-weight: 500;
  margin-bottom: 8px;
  color: var(--color-text-1);
}

.result-content .text {
  padding: 12px;
  background: var(--color-fill-2);
  border-radius: 6px;
  margin-bottom: 16px;
  white-space: pre-wrap;
  word-break: break-all;
}
</style>

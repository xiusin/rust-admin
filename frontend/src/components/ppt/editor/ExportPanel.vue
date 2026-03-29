<template>
  <div class="export-panel">
    <div class="panel-header">
      <h3>导出设置</h3>
    </div>
    
    <div class="export-content">
      <a-form :model="exportForm" layout="vertical">
        <a-form-item label="导出格式">
          <a-radio-group v-model="exportForm.format">
            <a-radio value="pptx">PPTX</a-radio>
            <a-radio value="pdf">PDF</a-radio>
            <a-radio value="images">图片</a-radio>
          </a-radio-group>
        </a-form-item>
        
        <a-form-item v-if="exportForm.format === 'images'" label="图片格式">
          <a-select v-model="exportForm.imageFormat">
            <a-option value="png">PNG</a-option>
            <a-option value="jpg">JPG</a-option>
            <a-option value="webp">WebP</a-option>
          </a-select>
        </a-form-item>
        
        <a-form-item label="页面范围">
          <a-radio-group v-model="exportForm.pageRange">
            <a-radio value="all">全部页面</a-radio>
            <a-radio value="current">当前页面</a-radio>
            <a-radio value="custom">自定义</a-radio>
          </a-radio-group>
        </a-form-item>
        
        <a-form-item v-if="exportForm.pageRange === 'custom'" label="页码范围">
          <a-input v-model="exportForm.customPages" placeholder="如: 1-5, 8, 10-12" />
        </a-form-item>
        
        <a-form-item label="质量设置">
          <a-slider v-model="exportForm.quality" :min="1" :max="100" show-ticks />
          <div class="quality-label">质量: {{ exportForm.quality }}%</div>
        </a-form-item>
        
        <a-form-item v-if="exportForm.format === 'pdf'" label="页面尺寸">
          <a-select v-model="exportForm.pageSize">
            <a-option value="16:9">16:9 (宽屏)</a-option>
            <a-option value="4:3">4:3 (标准)</a-option>
            <a-option value="A4">A4</a-option>
          </a-select>
        </a-form-item>
        
        <a-form-item v-if="exportForm.format === 'pptx'" label="包含备注">
          <a-switch v-model="exportForm.includeNotes" />
        </a-form-item>
        
        <a-form-item v-if="exportForm.format === 'images'" label="包含背景">
          <a-switch v-model="exportForm.includeBackground" />
        </a-form-item>
      </a-form>
    </div>
    
    <div class="panel-footer">
      <a-button type="primary" long :loading="exporting" @click="handleExport">
        <template #icon><icon-download /></template>
        导出
      </a-button>
    </div>
    
    <a-modal v-model:visible="showProgress" :footer="false" :closable="false">
      <div class="export-progress">
        <a-progress :percent="exportProgress" />
        <div class="progress-text">{{ progressText }}</div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { Message } from '@arco-design/web-vue';
import { IconDownload } from '@arco-design/web-vue/es/icon';
import { exportPPT } from '@/api/modules/ppt/editor';

const props = defineProps<{
  projectId: number;
}>();

const emit = defineEmits<{
  (e: 'export', result: any): void;
}>();

const exportForm = reactive({
  format: 'pptx',
  imageFormat: 'png',
  pageRange: 'all',
  customPages: '',
  quality: 90,
  pageSize: '16:9',
  includeNotes: true,
  includeBackground: true,
});

const exporting = ref(false);
const showProgress = ref(false);
const exportProgress = ref(0);
const progressText = ref('');

async function handleExport() {
  exporting.value = true;
  showProgress.value = true;
  exportProgress.value = 0;
  progressText.value = '正在准备导出...';
  
  try {
    const interval = setInterval(() => {
      if (exportProgress.value < 90) {
        exportProgress.value += 10;
        progressText.value = `正在导出... ${exportProgress.value}%`;
      }
    }, 500);
    
    const result = await exportPPT({
      project_id: props.projectId,
      format: exportForm.format,
    });
    
    clearInterval(interval);
    exportProgress.value = 100;
    progressText.value = '导出完成!';
    
    setTimeout(() => {
      showProgress.value = false;
      Message.success('导出成功!');
      emit('export', result);
      
      if (result.download_url) {
        const link = document.createElement('a');
        link.href = result.download_url;
        link.download = `presentation.${exportForm.format}`;
        link.click();
      }
    }, 500);
  } catch (error: any) {
    showProgress.value = false;
    Message.error(error.message || '导出失败');
  } finally {
    exporting.value = false;
  }
}
</script>

<style scoped>
.export-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-2);
  border-left: 1px solid var(--color-border);
}

.panel-header {
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.panel-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
}

.export-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
}

.quality-label {
  font-size: 12px;
  color: var(--color-text-3);
  margin-top: 4px;
}

.panel-footer {
  padding: 16px;
  border-top: 1px solid var(--color-border);
}

.export-progress {
  padding: 24px;
  text-align: center;
}

.progress-text {
  margin-top: 12px;
  color: var(--color-text-2);
}
</style>

<template>
  <a-modal v-model:visible="modelValue" :width="500" :footer="false" title="下载代码">
    <div class="download-content">
      <a-form :model="form" layout="vertical">
        <a-form-item label="下载格式">
          <a-radio-group v-model="form.format">
            <a-radio value="zip">ZIP压缩包</a-radio>
            <a-radio value="tar">TAR压缩包</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item label="输出路径">
          <a-input v-model="form.outputPath" placeholder="请输入输出路径" />
        </a-form-item>
      </a-form>
      <div class="download-actions">
        <a-space>
          <a-button @click="modelValue = false">取消</a-button>
          <a-button type="primary" @click="handleDownload" :loading="downloading">下载</a-button>
        </a-space>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from "vue";
import { codeGenApi } from "@/api/modules/cms/codeGen";

interface Props {
  modelId: number | null;
}

const props = defineProps<Props>();

const modelValue = defineModel<boolean>({ default: false });
const downloading = ref(false);

const form = reactive({
  format: "zip",
  outputPath: ""
});

const handleDownload = async () => {
  if (!props.modelId) return;
  downloading.value = true;
  try {
    const blob = await codeGenApi.download(props.modelId);
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `code_${props.modelId}.${form.format}`;
    a.click();
    window.URL.revokeObjectURL(url);
    arcoMessage("success", "下载成功");
    modelValue.value = false;
  } catch (error) {
    console.error(error);
  } finally {
    downloading.value = false;
  }
};
</script>

<style scoped lang="scss">
.download-content {
  .download-actions {
    margin-top: 16px;
    text-align: right;
  }
}
</style>

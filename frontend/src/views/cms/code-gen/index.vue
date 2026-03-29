<template>
  <div class="snow-page">
    <div class="snow-inner code-gen">
      <div class="gen-header">
        <div class="header-left">
          <a-button type="text" @click="goBack">
            <template #icon><icon-left /></template>
            返回
          </a-button>
          <a-divider direction="vertical" />
          <span class="page-title">代码生成</span>
        </div>
      </div>

      <div class="gen-content">
        <a-row :gutter="24">
          <a-col :span="8">
            <a-card title="选择模型" class="model-select-card">
              <ModelSelect v-model="selectedModelId" @change="onModelChange" />
            </a-card>
          </a-col>
          <a-col :span="16">
            <a-card title="生成选项" class="options-card">
              <GenOptions v-model="genOptions" :disabled="!selectedModelId" />
              <template #actions>
                <a-space>
                  <a-button @click="handlePreview" :disabled="!selectedModelId" :loading="previewing">
                    <template #icon><icon-eye /></template>
                    预览代码
                  </a-button>
                  <a-button type="primary" @click="handleGenerate" :disabled="!selectedModelId" :loading="generating">
                    <template #icon><icon-code /></template>
                    生成代码
                  </a-button>
                  <a-button
                    type="primary"
                    status="success"
                    @click="handleDownload"
                    :disabled="!selectedModelId"
                    :loading="downloading"
                  >
                    <template #icon><icon-download /></template>
                    下载代码
                  </a-button>
                </a-space>
              </template>
            </a-card>
          </a-col>
        </a-row>
      </div>

      <a-modal v-model:visible="previewVisible" :width="1200" :footer="false" title="代码预览">
        <div class="preview-content">
          <a-row :gutter="16">
            <a-col :span="6">
              <div class="file-tree">
                <a-tree
                  :data="fileTree"
                  :field-names="{ key: 'path', title: 'name', children: 'children' }"
                  :selected-keys="selectedFile"
                  @select="onFileSelect"
                />
              </div>
            </a-col>
            <a-col :span="18">
              <div class="code-preview">
                <div class="code-header">
                  <span class="file-path">{{ currentFile?.filePath }}</span>
                  <a-button type="text" size="small" @click="copyCode">
                    <template #icon><icon-copy /></template>
                    复制
                  </a-button>
                </div>
                <div class="code-body">
                  <pre><code>{{ currentFile?.content }}</code></pre>
                </div>
              </div>
            </a-col>
          </a-row>
        </div>
      </a-modal>

      <DownloadModal v-model:visible="downloadVisible" :model-id="selectedModelId" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { codeGenApi, type GeneratedFile, type FileTreeNode, type CodeGenOptions } from "@/api/modules/cms/codeGen";
import ModelSelect from "./components/ModelSelect.vue";
import GenOptions from "./components/GenOptions.vue";
import DownloadModal from "./components/DownloadModal.vue";

const router = useRouter();
const selectedModelId = ref<number | null>(null);
const previewVisible = ref(false);
const downloadVisible = ref(false);
const previewing = ref(false);
const generating = ref(false);
const downloading = ref(false);
const fileTree = ref<FileTreeNode[]>([]);
const generatedFiles = ref<GeneratedFile[]>([]);
const currentFile = ref<GeneratedFile | null>(null);
const selectedFile = ref<string[]>([]);

const genOptions = reactive<CodeGenOptions>({
  useTypeScript: true,
  useCompositionApi: true,
  generateApi: true,
  generateViews: true,
  generateComponents: true,
  generateRoutes: true,
  generateStore: false,
  author: "",
  version: "1.0.0"
});

const goBack = () => {
  router.push("/cms/model/list");
};

const onModelChange = async () => {
  if (selectedModelId.value) {
    previewing.value = true;
    try {
      fileTree.value = await codeGenApi.getFileTree(selectedModelId.value);
      generatedFiles.value = await codeGenApi.preview(selectedModelId.value);
      if (generatedFiles.value.length > 0) {
        currentFile.value = generatedFiles.value[0];
        selectedFile.value = [currentFile.value.filePath];
      }
    } catch (error) {
      console.error(error);
    } finally {
      previewing.value = false;
    }
  }
};

const onFileSelect = (keys: string[]) => {
  if (keys.length > 0) {
    const file = generatedFiles.value.find(f => f.filePath === keys[0]);
    currentFile.value = file || null;
    selectedFile.value = keys;
  }
};

const copyCode = () => {
  if (currentFile.value?.content) {
    navigator.clipboard.writeText(currentFile.value.content);
    arcoMessage("success", "复制成功");
  }
};

const handlePreview = async () => {
  if (!selectedModelId.value) return;
  previewing.value = true;
  try {
    fileTree.value = await codeGenApi.getFileTree(selectedModelId.value);
    generatedFiles.value = await codeGenApi.preview(selectedModelId.value);
    if (generatedFiles.value.length > 0) {
      currentFile.value = generatedFiles.value[0];
      selectedFile.value = [currentFile.value.filePath];
    }
    previewVisible.value = true;
  } catch (error) {
    console.error(error);
  } finally {
    previewing.value = false;
  }
};

const handleGenerate = async () => {
  if (!selectedModelId.value) return;
  generating.value = true;
  try {
    const result = await codeGenApi.generate({
      modelId: selectedModelId.value,
      options: genOptions
    });
    if (result.success) {
      arcoMessage("success", result.message);
    } else {
      arcoMessage("error", result.message);
    }
  } catch (error) {
    console.error(error);
  } finally {
    generating.value = false;
  }
};

const handleDownload = () => {
  downloadVisible.value = true;
};
</script>

<style scoped lang="scss">
.code-gen {
  display: flex;
  flex-direction: column;
  height: 100%;

  .gen-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background: var(--color-bg-2);
    border-bottom: 1px solid var(--color-border-2);

    .header-left {
      display: flex;
      align-items: center;
      gap: 8px;

      .page-title {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .gen-content {
    flex: 1;
    padding: 16px;
    overflow-y: auto;

    .model-select-card {
      height: 100%;
    }

    .options-card {
      height: 100%;
    }
  }

  .preview-content {
    height: 600px;

    .file-tree {
      height: 100%;
      overflow-y: auto;
      border: 1px solid var(--color-border-2);
      border-radius: 4px;
      padding: 8px;
    }

    .code-preview {
      height: 100%;
      display: flex;
      flex-direction: column;

      .code-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px 12px;
        background: var(--color-fill-1);
        border-bottom: 1px solid var(--color-border-2);

        .file-path {
          font-size: 12px;
          color: var(--color-text-2);
        }
      }

      .code-body {
        flex: 1;
        overflow: auto;
        padding: 12px;
        background: var(--color-bg-1);
        border-radius: 4px;

        pre {
          margin: 0;
          font-size: 12px;
          line-height: 1.6;
        }

        code {
          font-family: "Fira Code", monospace;
        }
      }
    }
  }
}
</style>

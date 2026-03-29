<template>
  <div class="snow-page">
    <div class="snow-inner code-preview">
      <div class="preview-header">
        <div class="header-left">
          <a-button type="text" @click="goBack">
            <template #icon><icon-left /></template>
            返回
          </a-button>
          <a-divider direction="vertical" />
          <span class="preview-title">代码预览</span>
        </div>
        <div class="header-right">
          <a-space>
            <a-button @click="copyAllCode">
              <template #icon><icon-copy /></template>
              复制全部
            </a-button>
            <a-button type="primary" status="success" @click="downloadZip">
              <template #icon><icon-download /></template>
              下载ZIP
            </a-button>
          </a-space>
        </div>
      </div>

      <div class="preview-content">
        <a-row :gutter="16" class="preview-layout">
          <a-col :span="5">
            <div class="file-tree">
              <a-tree
                :data="fileTree"
                :field-names="{ key: 'path', title: 'name', children: 'children' }"
                :selected-keys="selectedFile"
                @select="onFileSelect"
              />
            </div>
          </a-col>
          <a-col :span="19">
            <div class="code-viewer">
              <div class="code-header">
                <span class="file-path">{{ currentFile?.filePath }}</span>
                <a-button type="text" size="small" @click="copyCurrentCode">
                  <template #icon><icon-copy /></template>
                  复制
                </a-button>
              </div>
              <div class="code-body">
                <pre><code :class="`language-${currentFile?.language}`">{{ currentFile?.content }}</code></pre>
              </div>
            </div>
          </a-col>
        </a-row>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { codeGenApi, type GeneratedFile, type FileTreeNode } from "@/api/modules/cms/codeGen";

const router = useRouter();
const route = useRoute();
const fileTree = ref<FileTreeNode[]>([]);
const generatedFiles = ref<GeneratedFile[]>([]);
const currentFile = ref<GeneratedFile | null>(null);
const selectedFile = ref<string[]>([]);

const modelId = computed(() => Number(route.query.modelId));

const goBack = () => {
  router.push("/cms/code-gen/index");
};

const onFileSelect = (keys: string[]) => {
  if (keys.length > 0) {
    const file = generatedFiles.value.find(f => f.filePath === keys[0]);
    currentFile.value = file || null;
    selectedFile.value = keys;
  }
};

const copyCurrentCode = async () => {
  if (!currentFile.value?.content) return;
  try {
    await navigator.clipboard.writeText(currentFile.value.content);
    arcoMessage("success", "复制成功");
  } catch (error) {
    console.error(error);
  }
};

const copyAllCode = async () => {
  const allCode = generatedFiles.value.map(f => `// ${f.filePath}\n${f.content}`).join("\n\n");
  try {
    await navigator.clipboard.writeText(allCode);
    arcoMessage("success", "复制成功");
  } catch (error) {
    console.error(error);
  }
};

const downloadZip = async () => {
  if (!modelId.value) return;
  try {
    const blob = await codeGenApi.download(modelId.value);
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = `code_${modelId.value}.zip`;
    a.click();
    window.URL.revokeObjectURL(url);
  } catch (error) {
    console.error(error);
  }
};

const loadData = async () => {
  if (!modelId.value) return;
  try {
    fileTree.value = await codeGenApi.getFileTree(modelId.value);
    generatedFiles.value = await codeGenApi.preview(modelId.value);
    if (generatedFiles.value.length > 0) {
      currentFile.value = generatedFiles.value[0];
      selectedFile.value = [currentFile.value.filePath];
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped lang="scss">
.code-preview {
  display: flex;
  flex-direction: column;
  height: 100%;

  .preview-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--color-bg-2);
    border-bottom: 1px solid var(--color-border-2);

    .header-left {
      display: flex;
      align-items: center;
      gap: 8px;

      .preview-title {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .preview-content {
    flex: 1;
    overflow: hidden;

    .preview-layout {
      height: 100%;
    }

    .file-tree {
      height: 100%;
      overflow-y: auto;
      border: 1px solid var(--color-border-2);
      border-radius: 4px;
      padding: 12px;
      background: var(--color-bg-2);
    }

    .code-viewer {
      height: 100%;
      display: flex;
      flex-direction: column;
      border: 1px solid var(--color-border-2);
      border-radius: 4px;
      overflow: hidden;

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

        pre {
          margin: 0;
          font-size: 13px;
          line-height: 1.6;

          code {
            font-family: "Fira Code", monospace;
          }
        }
      }
    }
  }
}
</style>

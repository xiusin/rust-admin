<template>
  <div class="code-preview">
    <div class="preview-header">
      <h3>代码预览</h3>
      <a-space>
        <a-button @click="handleCopyAll">
          <template #icon><icon-copy /></template>
          复制全部
        </a-button>
        <a-button type="primary" @click="handleDownload">
          <template #icon><icon-download /></template>
          下载ZIP
        </a-button>
      </a-space>
    </div>

    <div class="preview-content">
      <div class="file-tree-panel">
        <FileTree :files="fileTree" :active-file="activeFile" @select="handleFileSelect" />
      </div>

      <div class="code-viewer-panel">
        <CodeViewer
          v-if="currentCode"
          :code="currentCode.content"
          :language="currentCode.language"
          :filename="currentFile?.name"
        />
        <a-empty v-else description="请选择文件查看代码" />
      </div>
    </div>

    <div class="preview-footer">
      <GenConfig v-model="genConfig" @generate="handleGenerate" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import FileTree from "./FileTree.vue";
import CodeViewer from "./CodeViewer.vue";
import GenConfig from "./GenConfig.vue";

interface CodeFile {
  id: string;
  name: string;
  path: string;
  language: string;
  content: string;
  children?: CodeFile[];
}

interface Props {
  modelValue?: CodeFile[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: CodeFile[]];
  download: [files: CodeFile[]];
}>();

const fileTree = ref<CodeFile[]>(props.modelValue);
const activeFile = ref<string>("");
const currentFile = ref<CodeFile | null>(null);
const genConfig = ref({
  outputDir: "/src",
  overwrite: false,
  generateTests: false,
  generateDocs: false
});

const currentCode = computed(() => currentFile.value);

watch(
  () => props.modelValue,
  val => {
    fileTree.value = val;
  },
  { deep: true }
);

const handleFileSelect = (file: CodeFile) => {
  activeFile.value = file.id;
  currentFile.value = file;
};

const handleCopyAll = async () => {
  try {
    const allCode = fileTree.value.map(f => `// ${f.path}\n${f.content}`).join("\n\n");
    await navigator.clipboard.writeText(allCode);
    Message.success("复制成功");
  } catch {
    Message.error("复制失败");
  }
};

const handleDownload = () => {
  emit("download", fileTree.value);
  Message.success("开始下载");
};

const handleGenerate = (config: any) => {
  console.log("Generate with config:", config);
  Message.success("代码生成成功");
};
</script>

<style lang="scss" scoped>
.code-preview {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg-2);

  .preview-header {
    display: flex;
    flex-shrink: 0;
    align-items: center;
    justify-content: space-between;
    padding: 16px 20px;
    border-bottom: 1px solid var(--color-border);

    h3 {
      margin: 0;
      font-size: 16px;
      font-weight: 600;
    }
  }

  .preview-content {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: hidden;

    .file-tree-panel {
      width: 260px;
      padding: 16px;
      overflow-y: auto;
      border-right: 1px solid var(--color-border);
    }

    .code-viewer-panel {
      flex: 1;
      overflow: auto;
    }
  }

  .preview-footer {
    flex-shrink: 0;
    padding: 16px 20px;
    border-top: 1px solid var(--color-border);
  }
}
</style>

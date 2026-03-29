<template>
  <div class="document-uploader">
    <a-upload
      v-model:file-list="fileList"
      draggable
      :limit="1"
      :accept="acceptTypes"
      :custom-request="handleCustomRequest"
      :before-upload="handleBeforeUpload"
      @change="handleChange"
    >
      <div class="upload-trigger">
        <div class="upload-icon">
          <icon-upload />
        </div>
        <div class="upload-text">
          <span>点击或拖拽文件到此处上传</span>
          <span class="upload-hint">支持 PDF、Word、PPT、TXT 格式，单个文件不超过 10MB</span>
        </div>
      </div>
    </a-upload>

    <div v-if="modelValue" class="file-info">
      <div class="file-icon">
        <icon-file />
      </div>
      <div class="file-detail">
        <div class="file-name">{{ modelValue.name }}</div>
        <div class="file-size">{{ formatFileSize(modelValue.size) }}</div>
      </div>
      <a-button type="text" status="danger" size="small" @click="handleRemove">
        <template #icon><icon-delete /></template>
      </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { Message } from "@arco-design/web-vue";

const props = defineProps<{
  modelValue?: File | null;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: File | null): void;
}>();

const fileList = ref<any[]>([]);
const acceptTypes = ".pdf,.doc,.docx,.ppt,.pptx,.txt";

const formatFileSize = (size: number): string => {
  if (size < 1024) return `${size} B`;
  if (size < 1024 * 1024) return `${(size / 1024).toFixed(1)} KB`;
  return `${(size / (1024 * 1024)).toFixed(1)} MB`;
};

const handleBeforeUpload = (file: File): boolean | Promise<void> => {
  const maxSize = 10 * 1024 * 1024;
  if (file.size > maxSize) {
    Message.error("文件大小不能超过 10MB");
    return Promise.reject(new Error("文件过大"));
  }
  return true;
};

const handleCustomRequest = (options: any) => {
  const { fileItem } = options;
  emit("update:modelValue", fileItem.file);
  return {
    abort() {
      console.log("upload aborted");
    }
  };
};

const handleChange = (fileList: any[], fileItem: any) => {
  if (fileItem.status === "error") {
    Message.error("文件上传失败");
  }
};

const handleRemove = () => {
  emit("update:modelValue", null);
  fileList.value = [];
};

watch(
  () => props.modelValue,
  val => {
    if (!val) {
      fileList.value = [];
    }
  },
  { immediate: true }
);
</script>

<style scoped lang="scss">
.document-uploader {
  width: 100%;
}

.upload-trigger {
  padding: 24px;
  border: 1px dashed var(--color-border);
  border-radius: 4px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    border-color: rgb(var(--primary-6));
    background: var(--color-fill-1);
  }

  .upload-icon {
    font-size: 32px;
    color: var(--color-text-3);
    margin-bottom: 8px;
  }

  .upload-text {
    text-align: center;
    color: var(--color-text-2);

    .upload-hint {
      display: block;
      font-size: 12px;
      color: var(--color-text-3);
      margin-top: 4px;
    }
  }
}

.file-info {
  display: flex;
  align-items: center;
  padding: 12px;
  margin-top: 12px;
  border: 1px solid var(--color-border);
  border-radius: 4px;
  background: var(--color-fill-1);

  .file-icon {
    font-size: 24px;
    margin-right: 12px;
    color: var(--color-text-3);
  }

  .file-detail {
    flex: 1;

    .file-name {
      font-weight: 500;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .file-size {
      font-size: 12px;
      color: var(--color-text-3);
    }
  }
}
</style>

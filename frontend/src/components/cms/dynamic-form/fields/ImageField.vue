<template>
  <a-upload
    v-model:file-list="fileList"
    :list-type="listType"
    :accept="accept"
    :multiple="multiple"
    :limit="limit"
    :disabled="disabled"
    :auto-upload="autoUpload"
    :action="action"
    :headers="headers"
    :data="data"
    @success="handleSuccess"
    @error="handleError"
    @exceed-limit="handleExceed"
    v-bind="$attrs"
  >
    <template v-if="listType === 'picture-card'">
      <template v-if="fileList.length < (limit || 1)">
        <icon-plus />
        <div class="upload-text">上传图片</div>
      </template>
    </template>
    <template v-else>
      <template v-if="fileList.length < (limit || 1)">
        <a-button :disabled="disabled">
          <icon-upload />
          上传图片
        </a-button>
      </template>
    </template>
  </a-upload>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";

interface UploadFile {
  uid: string;
  name: string;
  url?: string;
  status?: string;
  response?: any;
}

interface Props {
  modelValue: string | string[];
  placeholder?: string;
  disabled?: boolean;
  accept?: string;
  multiple?: boolean;
  limit?: number;
  listType?: "text" | "picture" | "picture-card";
  autoUpload?: boolean;
  action?: string;
  headers?: Record<string, string>;
  data?: Record<string, any>;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: "",
  placeholder: "请上传图片",
  disabled: false,
  accept: "image/*",
  multiple: false,
  limit: 1,
  listType: "picture-card",
  autoUpload: true,
  action: "/api/upload"
});

const emit = defineEmits<{
  "update:modelValue": [value: string | string[]];
  success: [file: any];
  error: [error: any];
}>();

const fileList = ref<UploadFile[]>([]);

watch(
  () => props.modelValue,
  val => {
    if (!val) {
      fileList.value = [];
      return;
    }

    const urls = Array.isArray(val) ? val : [val];
    fileList.value = urls.map((url, index) => ({
      uid: `-${index}`,
      name: url.split("/").pop() || "",
      url,
      status: "done"
    }));
  },
  { immediate: true }
);

const handleSuccess = (response: any) => {
  if (response.code === 0 || response.success) {
    const url = response.data?.url || response.url;
    if (props.multiple) {
      const urls = fileList.value.filter(f => f.status === "done" && f.url).map(f => f.url);
      emit("update:modelValue", urls);
    } else {
      emit("update:modelValue", url);
    }
    emit("success", response);
    Message.success("上传成功");
  }
};

const handleError = (error: any) => {
  Message.error("上传失败");
  emit("error", error);
};

const handleExceed = () => {
  Message.warning(`最多只能上传 ${props.limit} 张图片`);
};
</script>

<style lang="scss" scoped>
.upload-text {
  margin-top: 8px;
  font-size: 12px;
}
</style>

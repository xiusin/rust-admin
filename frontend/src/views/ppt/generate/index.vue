<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-row :gutter="16">
        <a-col :span="16">
          <a-card title="PPT生成" :bordered="false">
            <a-form ref="formRef" :model="form" auto-label-width :rules="rules">
              <a-form-item field="title" label="PPT标题" validate-trigger="blur">
                <a-input v-model="form.title" placeholder="请输入PPT标题" allow-clear />
              </a-form-item>
              <a-form-item field="description" label="描述">
                <a-textarea
                  v-model="form.description"
                  placeholder="请输入PPT描述"
                  allow-clear
                  :auto-size="{ minRows: 3, maxRows: 6 }"
                />
              </a-form-item>
              <a-form-item field="sourceType" label="来源类型">
                <a-radio-group v-model="form.sourceType">
                  <a-radio value="document">文档上传</a-radio>
                  <a-radio value="topic">主题输入</a-radio>
                  <a-radio value="outline">大纲输入</a-radio>
                </a-radio-group>
              </a-form-item>
              <a-form-item v-if="form.sourceType === 'document'" field="documentFile" label="上传文档">
                <DocumentUploader v-model="form.documentFile" />
              </a-form-item>
              <a-form-item v-if="form.sourceType === 'topic'" field="topic" label="主题内容">
                <a-textarea
                  v-model="form.topic"
                  placeholder="请输入PPT主题内容"
                  allow-clear
                  :auto-size="{ minRows: 4, maxRows: 8 }"
                />
              </a-form-item>
              <a-form-item v-if="form.sourceType === 'outline'" field="outline" label="大纲内容">
                <a-textarea
                  v-model="form.outline"
                  placeholder="请输入PPT大纲内容"
                  allow-clear
                  :auto-size="{ minRows: 6, maxRows: 12 }"
                />
              </a-form-item>
              <a-form-item field="industry" label="行业类型">
                <a-select v-model="form.industry" placeholder="请选择行业类型" allow-clear>
                  <a-option value="technology">科技</a-option>
                  <a-option value="education">教育</a-option>
                  <a-option value="finance">金融</a-option>
                  <a-option value="medical">医疗</a-option>
                  <a-option value="marketing">营销</a-option>
                  <a-option value="other">其他</a-option>
                </a-select>
              </a-form-item>
              <a-form-item field="styleId" label="选择风格">
                <StyleSelector v-model="form.styleId" />
              </a-form-item>
              <a-form-item>
                <a-space>
                  <a-button type="primary" :loading="generating" @click="handleGenerate">
                    <template #icon><icon-magic /></template>
                    生成PPT
                  </a-button>
                  <a-button @click="handleReset">重置</a-button>
                </a-space>
              </a-form-item>
            </a-form>
          </a-card>
        </a-col>
        <a-col :span="8">
          <a-card title="生成预览" :bordered="false">
            <PreviewPanel :loading="generating" :preview-data="previewData" />
          </a-card>
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from "vue";
import { useRouter } from "vue-router";
import { Message } from "@arco-design/web-vue";
import DocumentUploader from "@/components/ppt/DocumentUploader.vue";
import StyleSelector from "@/components/ppt/StyleSelector.vue";
import PreviewPanel from "@/components/ppt/PreviewPanel.vue";
import { generateApi } from "@/api/modules/ppt";

const router = useRouter();
const formRef = ref();
const generating = ref(false);
const previewData = ref<any>(null);

const form = reactive({
  title: "",
  description: "",
  sourceType: "topic",
  documentFile: null as File | null,
  topic: "",
  outline: "",
  industry: "",
  styleId: null as number | null
});

const rules = {
  title: [{ required: true, message: "请输入PPT标题" }],
  sourceType: [{ required: true, message: "请选择来源类型" }]
};

const handleGenerate = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  generating.value = true;
  try {
    const result = await generateApi.create({
      title: form.title,
      description: form.description,
      sourceType: form.sourceType,
      topic: form.topic,
      outline: form.outline,
      industry: form.industry,
      styleId: form.styleId
    });

    Message.success("PPT生成任务已提交");
    previewData.value = result;

    if (result.projectId) {
      router.push(`/ppt/editor/${result.projectId}`);
    }
  } catch (error: any) {
    Message.error(error?.message || "生成失败");
  } finally {
    generating.value = false;
  }
};

const handleReset = () => {
  formRef.value?.resetFields();
  previewData.value = null;
};
</script>

<style scoped lang="scss">
.snow-page {
  padding: 16px;
}

.snow-inner {
  background: var(--color-bg-2);
  padding: 16px;
  border-radius: 4px;
}
</style>

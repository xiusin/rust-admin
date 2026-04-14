<template>
  <div class="ppt-generate-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-icon">
          <icon-magic />
        </div>
        <div class="header-text">
          <h1>智能PPT生成</h1>
          <p>基于AI技术，一键生成专业级演示文稿</p>
        </div>
      </div>
    </div>

    <div class="page-content">
      <a-row :gutter="24">
        <a-col :xs="24" :lg="16">
          <div class="content-card">
            <div class="card-header">
              <div class="card-title">
                <icon-edit />
                <span>基本信息</span>
              </div>
            </div>
            <div class="card-body">
              <a-form ref="formRef" :model="form" auto-label-width :rules="rules" layout="vertical">
                <a-form-item field="title" label="PPT标题">
                  <a-input 
                    v-model="form.title" 
                    placeholder="例如：2024年度产品发布会" 
                    allow-clear
                    size="large"
                  />
                </a-form-item>
                <a-form-item field="description" label="描述">
                  <a-textarea
                    v-model="form.description"
                    placeholder="请简要描述PPT的主要内容和目的..."
                    allow-clear
                    :auto-size="{ minRows: 2, maxRows: 4 }"
                  />
                </a-form-item>

                <a-divider style="margin: 24px 0" />

                <a-form-item field="sourceType" label="内容来源">
                  <a-radio-group v-model="form.sourceType" type="button" class="source-type-group">
                    <a-radio value="topic">
                      <icon-star />
                      <span>主题输入</span>
                    </a-radio>
                    <a-radio value="document">
                      <icon-upload />
                      <span>文档上传</span>
                    </a-radio>
                    <a-radio value="outline">
                      <icon-list />
                      <span>大纲输入</span>
                    </a-radio>
                  </a-radio-group>
                </a-form-item>

                <div class="source-content">
                  <a-form-item v-if="form.sourceType === 'document'" field="documentFile" label="上传文档">
                    <DocumentUploader v-model="form.documentFile" />
                  </a-form-item>
                  <a-form-item v-if="form.sourceType === 'topic'" field="topic" label="主题内容">
                    <a-textarea
                      v-model="form.topic"
                      placeholder="请详细描述您的PPT主题内容，AI将根据您的描述生成结构化的演示文稿..."
                      allow-clear
                      :auto-size="{ minRows: 5, maxRows: 10 }"
                    />
                  </a-form-item>
                  <a-form-item v-if="form.sourceType === 'outline'" field="outline" label="大纲内容">
                    <a-textarea
                      v-model="form.outline"
                      placeholder="请输入PPT大纲，每行一个要点，例如：&#10;1. 项目背景&#10;2. 市场分析&#10;3. 解决方案"
                      allow-clear
                      :auto-size="{ minRows: 6, maxRows: 12 }"
                    />
                  </a-form-item>
                </div>

                <a-divider style="margin: 24px 0" />

                <a-row :gutter="16">
                  <a-col :span="12">
                    <a-form-item field="industry" label="行业类型">
                      <a-select v-model="form.industry" placeholder="请选择行业类型" allow-clear size="large">
                        <a-option value="technology">
                          <icon-robot /> 科技
                        </a-option>
                        <a-option value="education">
                          <icon-education /> 教育
                        </a-option>
                        <a-option value="finance">
                          <icon-coin-yen /> 金融
                        </a-option>
                        <a-option value="medical">
                          <icon-heartbeat /> 医疗
                        </a-option>
                        <a-option value="marketing">
                          <icon-sound /> 营销
                        </a-option>
                        <a-option value="other">
                          <icon-more /> 其他
                        </a-option>
                      </a-select>
                    </a-form-item>
                  </a-col>
                  <a-col :span="12">
                    <a-form-item field="styleId" label="选择风格">
                      <StyleSelector v-model="form.styleId" />
                    </a-form-item>
                  </a-col>
                </a-row>

                <a-form-item style="margin-top: 32px">
                  <a-space size="large">
                    <a-button type="primary" size="large" :loading="generating" @click="handleGenerate" class="generate-btn">
                      <template #icon><icon-magic /></template>
                      {{ generating ? '生成中...' : '开始生成' }}
                    </a-button>
                    <a-button size="large" @click="handleReset">
                      <template #icon><icon-refresh /></template>
                      重置
                    </a-button>
                  </a-space>
                </a-form-item>
              </a-form>
            </div>
          </div>
        </a-col>
        <a-col :xs="24" :lg="8">
          <div class="preview-card">
            <div class="card-header">
              <div class="card-title">
                <icon-eye />
                <span>实时预览</span>
              </div>
            </div>
            <div class="card-body">
              <PreviewPanel :loading="generating" :preview-data="previewData" />
            </div>
          </div>
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
.ppt-generate-page {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
  padding: 24px;

  .page-header {
    background: white;
    border-radius: 16px;
    padding: 32px;
    margin-bottom: 24px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);

    .header-content {
      display: flex;
      align-items: center;
      gap: 20px;

      .header-icon {
        width: 72px;
        height: 72px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border-radius: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-size: 36px;
        box-shadow: 0 8px 24px rgba(102, 126, 234, 0.3);
      }

      .header-text {
        h1 {
          font-size: 28px;
          font-weight: 700;
          margin: 0 0 8px 0;
          color: #1d2129;
        }

        p {
          margin: 0;
          color: #86909c;
          font-size: 15px;
        }
      }
    }
  }

  .page-content {
    .content-card,
    .preview-card {
      background: white;
      border-radius: 16px;
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
      overflow: hidden;
    }

    .card-header {
      padding: 20px 24px;
      border-bottom: 1px solid #f2f3f5;
      background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);

      .card-title {
        display: flex;
        align-items: center;
        gap: 10px;
        font-size: 16px;
        font-weight: 600;
        color: #1d2129;
      }
    }

    .card-body {
      padding: 24px;
    }
  }

  .source-type-group {
    width: 100%;
    display: flex;

    :deep(.arco-radio-button) {
      flex: 1;
      text-align: center;
      padding: 12px 16px;
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
    }
  }

  .source-content {
    animation: fadeIn 0.3s ease;
  }

  .generate-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    padding: 0 32px;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 6px 20px rgba(102, 126, 234, 0.5);
    }

    &:active {
      transform: translateY(0);
    }
  }
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>

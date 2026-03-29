<template>
  <div class="snow-page model-design">
    <div class="design-header">
      <div class="header-left">
        <a-button type="text" @click="goBack">
          <template #icon><icon-left /></template>
          返回列表
        </a-button>
        <a-divider direction="vertical" />
        <span class="model-name">{{ modelDetail?.name || "模型设计器" }}</span>
      </div>
      <div class="header-right">
        <a-space>
          <a-button @click="handlePreview">
            <template #icon><icon-eye /></template>
            预览
          </a-button>
          <a-button type="primary" @click="handleSave" :loading="saving">
            <template #icon><icon-save /></template>
            保存
          </a-button>
          <a-button type="primary" status="success" @click="handleGenerateCode">
            <template #icon><icon-code /></template>
            生成代码
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="design-content">
      <a-spin :loading="loading" class="design-spin">
        <div class="design-layout">
          <div class="design-sider">
            <a-tabs v-model:active-key="activeTab" type="rounded">
              <a-tab-pane key="fields" title="字段配置">
                <div class="tab-content">
                  <div class="field-list">
                    <div
                      v-for="field in fields"
                      :key="field.id"
                      class="field-item"
                      :class="{ active: currentField?.id === field.id }"
                      @click="selectField(field)"
                    >
                      <div class="field-icon">
                        <icon-font-colors v-if="field.fieldType === 'text'" />
                        <icon-image v-else-if="field.fieldType === 'image'" />
                        <icon-file v-else-if="field.fieldType === 'file'" />
                        <icon-number v-else-if="field.fieldType === 'number'" />
                        <icon-calendar v-else-if="field.fieldType === 'date'" />
                        <icon-select-all v-else />
                      </div>
                      <div class="field-info">
                        <div class="field-name">{{ field.name }}</div>
                        <div class="field-code">{{ field.code }}</div>
                      </div>
                    </div>
                  </div>
                  <a-button long @click="addField">
                    <template #icon><icon-plus /></template>
                    添加字段
                  </a-button>
                </div>
              </a-tab-pane>
              <a-tab-pane key="config" title="模型配置">
                <div class="tab-content">
                  <a-form :model="modelConfig" layout="vertical">
                    <a-form-item label="启用分类">
                      <a-switch v-model="modelConfig.enableCategory" />
                    </a-form-item>
                    <a-form-item label="启用标签">
                      <a-switch v-model="modelConfig.enableTag" />
                    </a-form-item>
                    <a-form-item label="启用评论">
                      <a-switch v-model="modelConfig.enableComment" />
                    </a-form-item>
                    <a-form-item label="启用审核">
                      <a-switch v-model="modelConfig.enableAudit" />
                    </a-form-item>
                    <a-form-item label="启用版本">
                      <a-switch v-model="modelConfig.enableVersion" />
                    </a-form-item>
                    <a-form-item label="启用回收站">
                      <a-switch v-model="modelConfig.enableRecycle" />
                    </a-form-item>
                    <a-form-item label="列表页大小">
                      <a-input-number v-model="modelConfig.listPageSize" :min="10" :max="100" />
                    </a-form-item>
                    <a-form-item label="默认排序">
                      <a-select v-model="modelConfig.defaultSort">
                        <a-option value="createdAt desc">创建时间降序</a-option>
                        <a-option value="createdAt asc">创建时间升序</a-option>
                        <a-option value="sort asc">排序升序</a-option>
                        <a-option value="sort desc">排序降序</a-option>
                      </a-select>
                    </a-form-item>
                  </a-form>
                </div>
              </a-tab-pane>
            </a-tabs>
          </div>

          <div class="design-main">
            <div v-if="currentField" class="field-config">
              <div class="config-header">
                <span class="config-title">字段配置 - {{ currentField.name }}</span>
              </div>
              <div class="config-body">
                <a-form :model="currentField" layout="vertical">
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="字段名称">
                        <a-input v-model="currentField.name" placeholder="请输入字段名称" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="字段编码">
                        <a-input v-model="currentField.code" placeholder="请输入字段编码" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="字段类型">
                        <a-select v-model="currentField.fieldType">
                          <a-option value="text">单行文本</a-option>
                          <a-option value="textarea">多行文本</a-option>
                          <a-option value="editor">富文本</a-option>
                          <a-option value="number">数字</a-option>
                          <a-option value="date">日期</a-option>
                          <a-option value="datetime">日期时间</a-option>
                          <a-option value="select">下拉选择</a-option>
                          <a-option value="radio">单选</a-option>
                          <a-option value="checkbox">多选</a-option>
                          <a-option value="image">图片</a-option>
                          <a-option value="file">文件</a-option>
                          <a-option value="switch">开关</a-option>
                        </a-select>
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="默认值">
                        <a-input v-model="currentField.defaultValue" placeholder="请输入默认值" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-row :gutter="16">
                    <a-col :span="12">
                      <a-form-item label="是否必填">
                        <a-switch v-model="currentField.required" />
                      </a-form-item>
                    </a-col>
                    <a-col :span="12">
                      <a-form-item label="是否唯一">
                        <a-switch v-model="currentField.unique" />
                      </a-form-item>
                    </a-col>
                  </a-row>
                  <a-form-item label="验证规则">
                    <a-textarea
                      v-model="currentField.validation"
                      placeholder="请输入验证规则（JSON格式）"
                      :auto-size="{ minRows: 2, maxRows: 4 }"
                    />
                  </a-form-item>
                  <a-form-item label="字段提示">
                    <a-input v-model="currentField.placeholder" placeholder="请输入字段提示" />
                  </a-form-item>
                  <a-form-item label="帮助文本">
                    <a-input v-model="currentField.helpText" placeholder="请输入帮助文本" />
                  </a-form-item>
                </a-form>
              </div>
            </div>
            <div v-else class="empty-field">
              <a-empty description="请选择或添加字段" />
            </div>
          </div>
        </div>
      </a-spin>
    </div>

    <a-modal v-model:visible="previewVisible" :width="800" :footer="false" title="预览">
      <div class="preview-content">
        <a-descriptions :column="2" bordered>
          <a-descriptions-item label="模型名称">{{ modelDetail?.name }}</a-descriptions-item>
          <a-descriptions-item label="模型编码">{{ modelDetail?.code }}</a-descriptions-item>
          <a-descriptions-item label="表名称">{{ modelDetail?.tableName }}</a-descriptions-item>
          <a-descriptions-item label="状态">{{ modelDetail?.isEnabled ? "启用" : "禁用" }}</a-descriptions-item>
          <a-descriptions-item label="字段数量" :span="2">{{ fields.length }}</a-descriptions-item>
        </a-descriptions>
        <a-divider>字段列表</a-divider>
        <a-table :data="fields" :pagination="false" size="small">
          <template #columns>
            <a-table-column title="字段名称" data-index="name" />
            <a-table-column title="字段编码" data-index="code" />
            <a-table-column title="字段类型" data-index="fieldType" />
            <a-table-column title="必填" data-index="required">
              <template #cell="{ record }">
                <a-tag :color="record.required ? 'green' : 'gray'" size="small">{{ record.required ? "是" : "否" }}</a-tag>
              </template>
            </a-table-column>
          </template>
        </a-table>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { modelApi, type CmsModelDetail, type CmsFieldInfo, type ModelConfig } from "@/api/modules/cms/model";

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const saving = ref(false);
const activeTab = ref("fields");
const previewVisible = ref(false);

const modelId = computed(() => Number(route.query.id));
const modelDetail = ref<CmsModelDetail>();
const fields = ref<CmsFieldInfo[]>([]);
const currentField = ref<CmsFieldInfo | null>(null);

const modelConfig = reactive<ModelConfig>({
  enableCategory: true,
  enableTag: true,
  enableComment: false,
  enableAudit: false,
  enableVersion: true,
  enableRecycle: true,
  listPageSize: 20,
  defaultSort: "createdAt desc"
});

const goBack = () => {
  router.push("/cms/model/list");
};

const loadModel = async () => {
  if (!modelId.value) return;
  loading.value = true;
  try {
    const data = await modelApi.detail(modelId.value);
    modelDetail.value = data;
    fields.value = data.fields || [];
    if (data.config) {
      Object.assign(modelConfig, data.config);
    }
    if (fields.value.length > 0) {
      currentField.value = fields.value[0];
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const selectField = (field: CmsFieldInfo) => {
  currentField.value = field;
};

const addField = () => {
  const newField: CmsFieldInfo = {
    id: Date.now(),
    name: "新字段",
    code: `field_${fields.value.length + 1}`,
    fieldType: "text"
  } as any;
  fields.value.push(newField);
  currentField.value = newField;
};

const handleSave = async () => {
  saving.value = true;
  try {
    await modelApi.edit({
      id: modelId.value,
      name: modelDetail.value!.name,
      code: modelDetail.value!.code,
      tableName: modelDetail.value!.tableName,
      config: modelConfig
    });
    arcoMessage("success", "保存成功");
  } catch (error) {
    console.error(error);
  } finally {
    saving.value = false;
  }
};

const handlePreview = () => {
  previewVisible.value = true;
};

const handleGenerateCode = () => {
  router.push({
    path: "/cms/code-gen/index",
    query: { modelId: modelId.value }
  });
};

onMounted(() => {
  loadModel();
});
</script>

<style scoped lang="scss">
.model-design {
  display: flex;
  flex-direction: column;
  height: 100%;

  .design-header {
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

      .model-name {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .design-content {
    flex: 1;
    overflow: hidden;

    .design-spin {
      height: 100%;
    }

    .design-layout {
      display: flex;
      height: 100%;

      .design-sider {
        width: 320px;
        border-right: 1px solid var(--color-border-2);
        background: var(--color-bg-2);

        .tab-content {
          padding: 12px;
        }

        .field-list {
          max-height: calc(100vh - 300px);
          overflow-y: auto;
          margin-bottom: 12px;

          .field-item {
            display: flex;
            align-items: center;
            padding: 8px 12px;
            margin-bottom: 8px;
            border-radius: 4px;
            cursor: pointer;
            transition: all 0.2s;

            &:hover {
              background: var(--color-fill-2);
            }

            &.active {
              background: rgb(var(--primary-1));
            }

            .field-icon {
              width: 32px;
              height: 32px;
              display: flex;
              align-items: center;
              justify-content: center;
              background: var(--color-fill-2);
              border-radius: 4px;
              margin-right: 12px;
              font-size: 16px;
              color: rgb(var(--primary-6));
            }

            .field-info {
              flex: 1;

              .field-name {
                font-size: 14px;
                color: var(--color-text-1);
              }

              .field-code {
                font-size: 12px;
                color: var(--color-text-3);
              }
            }
          }
        }
      }

      .design-main {
        flex: 1;
        padding: 16px;
        overflow-y: auto;

        .field-config {
          background: var(--color-bg-2);
          border-radius: 4px;

          .config-header {
            padding: 12px 16px;
            border-bottom: 1px solid var(--color-border-2);

            .config-title {
              font-size: 14px;
              font-weight: 500;
            }
          }

          .config-body {
            padding: 16px;
          }
        }

        .empty-field {
          display: flex;
          align-items: center;
          justify-content: center;
          height: 100%;
        }
      }
    }
  }
}
</style>

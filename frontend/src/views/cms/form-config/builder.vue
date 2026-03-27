<template>
  <div class="snow-page">
    <div class="snow-inner form-builder">
      <div class="builder-header">
        <div class="header-left">
          <a-button type="text" @click="goBack">
            <template #icon><icon-left /></template>
            返回列表
          </a-button>
          <a-divider direction="vertical" />
          <span class="builder-title">{{ formTitle }}</span>
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
            <a-button type="primary" status="success" @click="handleExport">
              <template #icon><icon-download /></template>
              导出配置
            </a-button>
          </a-space>
        </div>
      </div>

      <div class="builder-content">
        <a-spin :loading="loading" class="builder-spin">
          <a-row :gutter="24">
            <a-col :span="6">
              <a-card title="字段列表" class="field-list-card">
                <template #extra>
                  <a-button type="text" size="small" @click="addField">
                    <template #icon><icon-plus /></template>
                  </a-button>
                </template>
                <div class="field-list">
                  <div
                    v-for="field in fields"
                    :key="field.id"
                    class="field-item"
                    :class="{ active: currentField?.id === field.id }"
                    draggable="true"
                    @click="selectField(field)"
                    @dragstart="onDragStart($event, field)"
                    @dragover.prevent
                    @drop="onDrop($event, field)"
                  >
                    <icon-drag-dot-vertical class="drag-handle" />
                    <span class="field-name">{{ field.name }}</span>
                    <span class="field-type">{{ getFieldTypeText(field.type) }}</span>
                  </div>
                </div>
              </a-card>
            </a-col>
            <a-col :span="12">
              <a-card title="表单配置" class="form-config-card">
                <div v-if="currentField" class="field-config">
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
                          <a-select v-model="currentField.type">
                            <a-option value="text">单行文本</a-option>
                            <a-option value="textarea">多行文本</a-option>
                            <a-option value="number">数字</a-option>
                            <a-option value="select">下拉选择</a-option>
                            <a-option value="radio">单选</a-option>
                            <a-option value="checkbox">多选</a-option>
                            <a-option value="date">日期</a-option>
                            <a-option value="datetime">日期时间</a-option>
                            <a-option value="image">图片上传</a-option>
                            <a-option value="file">文件上传</a-option>
                            <a-option value="editor">富文本</a-option>
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
                      <a-col :span="8">
                        <a-form-item label="是否必填">
                          <a-switch v-model="currentField.required" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="是否禁用">
                          <a-switch v-model="currentField.disabled" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="是否显示">
                          <a-switch v-model="currentField.visible" />
                        </a-form-item>
                      </a-col>
                    </a-row>
                    <a-form-item label="占位文本">
                      <a-input v-model="currentField.placeholder" placeholder="请输入占位文本" />
                    </a-form-item>
                    <a-form-item label="帮助文本">
                      <a-input v-model="currentField.helpText" placeholder="请输入帮助文本" />
                    </a-form-item>
                  </a-form>
                </div>
                <div v-else class="empty-field">
                  <a-empty description="请选择字段" />
                </div>
              </a-card>
            </a-col>
            <a-col :span="6">
              <a-card title="表单属性" class="form-props-card">
                <a-form :model="formConfig" layout="vertical">
                  <a-form-item label="表单名称">
                    <a-input v-model="formConfig.name" placeholder="请输入表单名称" />
                  </a-form-item>
                  <a-form-item label="表单编码">
                    <a-input v-model="formConfig.code" placeholder="请输入表单编码" />
                  </a-form-item>
                  <a-form-item label="表单类型">
                    <a-select v-model="formConfig.formType">
                      <a-option value="create">新增表单</a-option>
                      <a-option value="edit">编辑表单</a-option>
                      <a-option value="detail">详情表单</a-option>
                      <a-option value="search">搜索表单</a-option>
                    </a-select>
                  </a-form-item>
                  <a-form-item label="标签宽度">
                    <a-input-number v-model="formConfig.labelWidth" :min="60" :max="200" style="width: 100%" />
                  </a-form-item>
                  <a-form-item label="标签位置">
                    <a-select v-model="formConfig.labelPosition">
                      <a-option value="left">左对齐</a-option>
                      <a-option value="right">右对齐</a-option>
                      <a-option value="top">顶部对齐</a-option>
                    </a-select>
                  </a-form-item>
                </a-form>
              </a-card>
            </a-col>
          </a-row>
        </a-spin>
      </div>

      <a-modal v-model:visible="previewVisible" :width="800" :footer="false" title="表单预览">
        <div class="preview-form">
          <a-form :model="previewData" :label-width="formConfig.labelWidth" :layout="formConfig.labelPosition === 'top' ? 'vertical' : 'horizontal'">
            <a-row :gutter="16">
              <a-col :span="12" v-for="field in fields" :key="field.id">
                <a-form-item :label="field.name" :required="field.required">
                  <component
                    :is="getFieldComponent(field.type)"
                    v-model="previewData[field.code]"
                    :placeholder="field.placeholder"
                    :disabled="field.disabled"
                  />
                </a-form-item>
              </a-col>
            </a-row>
          </a-form>
        </div>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { formApi, type FormFieldConfig, type FormConfigDetail } from '@/api/modules/cms/form';

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const saving = ref(false);
const previewVisible = ref(false);
const currentField = ref<FormFieldConfig | null>(null);
const draggedField = ref<FormFieldConfig | null>(null);

const formId = computed(() => Number(route.query.id));
const modelId = computed(() => Number(route.query.modelId));
const formTitle = computed(() => (formId.value ? '编辑表单配置' : '新增表单配置'));

const fields = ref<FormFieldConfig[]>([]);
const previewData = ref<Record<string, any>>({});

const formConfig = reactive({
  name: '',
  code: '',
  formType: 'create' as 'create' | 'edit' | 'detail' | 'search',
  labelWidth: 100,
  labelPosition: 'right' as 'left' | 'right' | 'top',
});

const goBack = () => {
  router.push('/cms/form-config/list');
};

const getFieldTypeText = (type: string) => {
  const texts: Record<string, string> = {
    text: '文本',
    textarea: '多行文本',
    number: '数字',
    select: '下拉',
    radio: '单选',
    checkbox: '多选',
    date: '日期',
    datetime: '日期时间',
    image: '图片',
    file: '文件',
    editor: '富文本',
    switch: '开关',
  };
  return texts[type] || type;
};

const getFieldComponent = (type: string) => {
  const components: Record<string, string> = {
    text: 'a-input',
    textarea: 'a-textarea',
    number: 'a-input-number',
    select: 'a-select',
    radio: 'a-radio-group',
    checkbox: 'a-checkbox-group',
    date: 'a-date-picker',
    datetime: 'a-date-picker',
    image: 'a-upload',
    file: 'a-upload',
    editor: 'a-textarea',
    switch: 'a-switch',
  };
  return components[type] || 'a-input';
};

const selectField = (field: FormFieldConfig) => {
  currentField.value = field;
};

const addField = () => {
  const newField: FormFieldConfig = {
    id: Date.now(),
    name: '新字段',
    code: `field_${fields.value.length + 1}`,
    type: 'text',
    required: false,
    disabled: false,
    visible: true,
  };
  fields.value.push(newField);
  currentField.value = newField;
};

const onDragStart = (e: DragEvent, field: FormFieldConfig) => {
  draggedField.value = field;
};

const onDrop = (e: DragEvent, targetField: FormFieldConfig) => {
  if (!draggedField.value || draggedField.value.id === targetField.id) return;
  const draggedIndex = fields.value.findIndex((f) => f.id === draggedField.value!.id);
  const targetIndex = fields.value.findIndex((f) => f.id === targetField.id);
  fields.value.splice(draggedIndex, 1);
  fields.value.splice(targetIndex, 0, draggedField.value);
};

const handlePreview = () => {
  previewData.value = {};
  fields.value.forEach((f) => {
    previewData.value[f.code] = f.defaultValue || '';
  });
  previewVisible.value = true;
};

const handleSave = async () => {
  saving.value = true;
  try {
    const params = {
      id: formId.value || undefined,
      modelId: modelId.value,
      name: formConfig.name,
      code: formConfig.code,
      formType: formConfig.formType,
      config: {
        labelWidth: formConfig.labelWidth,
        labelPosition: formConfig.labelPosition,
        fields: fields.value,
      },
    };
    if (formId.value) {
      await formApi.edit(params);
      arcoMessage('success', '保存成功');
    } else {
      await formApi.add(params);
      arcoMessage('success', '创建成功');
    }
    goBack();
  } catch (error) {
    console.error(error);
  } finally {
    saving.value = false;
  }
};

const handleExport = () => {
  const config = {
    ...formConfig,
    fields: fields.value,
  };
  const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `form-config-${Date.now()}.json`;
  a.click();
  window.URL.revokeObjectURL(url);
};

const loadForm = async () => {
  if (!formId.value) return;
  loading.value = true;
  try {
    const data = await formApi.detail(formId.value);
    formConfig.name = data.name;
    formConfig.code = data.code;
    formConfig.formType = data.formType;
    formConfig.labelWidth = data.config?.labelWidth || 100;
    formConfig.labelPosition = data.config?.labelPosition || 'right';
    fields.value = data.config?.fields || [];
    if (fields.value.length > 0) {
      currentField.value = fields.value[0];
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  if (formId.value) {
    loadForm();
  }
});
</script>

<style scoped lang="scss">
.form-builder {
  display: flex;
  flex-direction: column;
  height: 100%;

  .builder-header {
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

      .builder-title {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .builder-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;

    .builder-spin {
      height: 100%;
    }

    .field-list-card,
    .form-config-card,
    .form-props-card {
      height: calc(100vh - 200px);
      overflow-y: auto;
    }

    .field-list {
      .field-item {
        display: flex;
        align-items: center;
        padding: 8px 12px;
        margin-bottom: 8px;
        border: 1px solid var(--color-border-2);
        border-radius: 4px;
        cursor: pointer;
        transition: all 0.2s;

        &:hover {
          border-color: rgb(var(--primary-6));
        }

        &.active {
          background: rgb(var(--primary-1));
          border-color: rgb(var(--primary-6));
        }

        .drag-handle {
          cursor: move;
          margin-right: 8px;
          color: var(--color-text-3);
        }

        .field-name {
          flex: 1;
          font-size: 13px;
        }

        .field-type {
          font-size: 12px;
          color: var(--color-text-3);
        }
      }
    }

    .empty-field {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 300px;
    }
  }

  .preview-form {
    padding: 16px;
  }
}
</style>

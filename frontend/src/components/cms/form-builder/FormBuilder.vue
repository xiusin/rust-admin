<template>
  <div class="form-builder">
    <div class="builder-header">
      <h3>表单构建器</h3>
      <a-space>
        <a-button @click="handlePreview">
          <template #icon><icon-eye /></template>
          预览
        </a-button>
        <a-button type="primary" @click="handleSave">
          <template #icon><icon-save /></template>
          保存
        </a-button>
      </a-space>
    </div>

    <div class="builder-content">
      <div class="left-panel">
        <a-card title="可用字段" class="panel-card">
          <div class="field-list">
            <div
              v-for="field in availableFields"
              :key="field.id"
              class="field-item"
              draggable="true"
              @dragstart="handleDragStart($event, field)"
            >
              <icon-drag-dot-vertical class="drag-icon" />
              <span class="field-name">{{ field.label }}</span>
              <a-tag size="small">{{ getFieldTypeLabel(field.type) }}</a-tag>
            </div>
          </div>
        </a-card>
      </div>

      <div class="center-panel">
        <a-card title="表单布局" class="panel-card">
          <template #extra>
            <a-button-group size="small">
              <a-button @click="handleAddGroup">
                <template #icon><icon-plus /></template>
                添加分组
              </a-button>
            </a-button-group>
          </template>

          <div class="layout-container" @dragover.prevent @drop="handleDrop">
            <div v-if="formSchema.groups.length === 0" class="empty-placeholder">
              <a-empty description="拖拽左侧字段到此处开始构建表单" />
            </div>

            <div v-else class="groups-container">
              <div v-for="(group, groupIndex) in formSchema.groups" :key="group.id" class="form-group">
                <div class="group-header">
                  <a-input v-model="group.title" placeholder="分组标题" style="width: 200px" />
                  <a-space>
                    <a-button type="text" size="small" @click="handleToggleGroup(group)">
                      <icon-down v-if="!group.collapsed" />
                      <icon-right v-else />
                    </a-button>
                    <a-button type="text" size="small" status="danger" @click="handleDeleteGroup(groupIndex)">
                      <icon-delete />
                    </a-button>
                  </a-space>
                </div>

                <div v-show="!group.collapsed" class="group-content">
                  <FieldLayout v-model="group.fields" :columns="formSchema.columns" @select="handleSelectField" />
                </div>
              </div>
            </div>
          </div>
        </a-card>
      </div>

      <div class="right-panel">
        <a-card title="配置面板" class="panel-card">
          <a-tabs v-model:active-key="configTab">
            <a-tab-pane key="form" title="表单配置">
              <a-form layout="vertical">
                <a-form-item label="表单名称">
                  <a-input v-model="formSchema.name" placeholder="请输入表单名称" />
                </a-form-item>

                <a-form-item label="标签位置">
                  <a-radio-group v-model="formSchema.labelPosition">
                    <a-radio value="left">左对齐</a-radio>
                    <a-radio value="right">右对齐</a-radio>
                    <a-radio value="top">顶部对齐</a-radio>
                  </a-radio-group>
                </a-form-item>

                <a-form-item label="标签宽度">
                  <a-input-number v-model="formSchema.labelWidth" :min="60" :max="200" />
                </a-form-item>

                <a-form-item label="每行字段数">
                  <a-select v-model="formSchema.columns">
                    <a-option :value="1">1列</a-option>
                    <a-option :value="2">2列</a-option>
                    <a-option :value="3">3列</a-option>
                    <a-option :value="4">4列</a-option>
                  </a-select>
                </a-form-item>

                <a-form-item label="表单尺寸">
                  <a-radio-group v-model="formSchema.size">
                    <a-radio value="small">小</a-radio>
                    <a-radio value="medium">中</a-radio>
                    <a-radio value="large">大</a-radio>
                  </a-radio-group>
                </a-form-item>
              </a-form>
            </a-tab-pane>

            <a-tab-pane key="field" title="字段配置">
              <div v-if="selectedField" class="field-config">
                <FormGroupConfig v-model="selectedField.config" />
              </div>
              <a-empty v-else description="请选择字段进行配置" />
            </a-tab-pane>

            <a-tab-pane key="rules" title="表单规则">
              <FormRuleConfig v-model="formSchema.rules" />
            </a-tab-pane>
          </a-tabs>
        </a-card>
      </div>
    </div>

    <FormPreview v-model:visible="previewVisible" :schema="formSchema" />
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import FieldLayout from "./FieldLayout.vue";
import FormGroupConfig from "./FormGroupConfig.vue";
import FormRuleConfig from "./FormRuleConfig.vue";
import FormPreview from "./FormPreview.vue";

interface FormField {
  id: string;
  name: string;
  label: string;
  type: string;
  config: Record<string, any>;
  span?: number;
}

interface FormGroup {
  id: string;
  title: string;
  collapsed: boolean;
  fields: FormField[];
}

interface FormSchema {
  name: string;
  labelPosition: "left" | "right" | "top";
  labelWidth: number;
  columns: number;
  size: "small" | "medium" | "large";
  groups: FormGroup[];
  rules: any[];
}

interface Props {
  modelValue?: FormSchema;
  fields?: FormField[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({
    name: "",
    labelPosition: "right",
    labelWidth: 100,
    columns: 2,
    size: "medium",
    groups: [],
    rules: []
  }),
  fields: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: FormSchema];
  save: [value: FormSchema];
}>();

const formSchema = ref<FormSchema>(props.modelValue);
const availableFields = ref<FormField[]>(props.fields);
const selectedField = ref<FormField | null>(null);
const configTab = ref("form");
const previewVisible = ref(false);

watch(
  () => props.modelValue,
  val => {
    formSchema.value = val;
  },
  { deep: true }
);

watch(
  formSchema,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const fieldTypeMap: Record<string, string> = {
  text: "文本",
  textarea: "多行文本",
  number: "数字",
  select: "下拉选择",
  radio: "单选",
  checkbox: "多选",
  date: "日期",
  datetime: "日期时间",
  image: "图片",
  file: "文件",
  editor: "富文本",
  json: "JSON"
};

const getFieldTypeLabel = (type: string) => fieldTypeMap[type] || type;

const handleDragStart = (event: DragEvent, field: FormField) => {
  event.dataTransfer?.setData("field", JSON.stringify(field));
};

const handleDrop = (event: DragEvent) => {
  event.preventDefault();
  const fieldData = event.dataTransfer?.getData("field");
  if (fieldData) {
    const field: FormField = JSON.parse(fieldData);
    if (formSchema.value.groups.length === 0) {
      handleAddGroup();
    }
    const lastGroup = formSchema.value.groups[formSchema.value.groups.length - 1];
    lastGroup.fields.push({
      ...field,
      id: `${field.id}_${Date.now()}`,
      config: { ...field.config }
    });
    Message.success("字段添加成功");
  }
};

const handleAddGroup = () => {
  formSchema.value.groups.push({
    id: `group_${Date.now()}`,
    title: `分组 ${formSchema.value.groups.length + 1}`,
    collapsed: false,
    fields: []
  });
};

const handleToggleGroup = (group: FormGroup) => {
  group.collapsed = !group.collapsed;
};

const handleDeleteGroup = (index: number) => {
  formSchema.value.groups.splice(index, 1);
  Message.success("分组删除成功");
};

const handleSelectField = (field: FormField) => {
  selectedField.value = field;
  configTab.value = "field";
};

const handlePreview = () => {
  previewVisible.value = true;
};

const handleSave = () => {
  emit("save", formSchema.value);
  Message.success("保存成功");
};
</script>

<style lang="scss" scoped>
.form-builder {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg-2);

  .builder-header {
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

  .builder-content {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: hidden;

    .left-panel {
      width: 260px;
      padding: 16px;
      overflow-y: auto;
      border-right: 1px solid var(--color-border);

      .field-list {
        .field-item {
          display: flex;
          gap: 8px;
          align-items: center;
          padding: 8px 12px;
          margin-bottom: 8px;
          background: var(--color-fill-1);
          border-radius: 4px;
          cursor: grab;
          transition: all 0.2s;

          &:hover {
            background: var(--color-fill-2);
          }

          .drag-icon {
            color: var(--color-text-3);
          }

          .field-name {
            flex: 1;
            font-size: 13px;
          }
        }
      }
    }

    .center-panel {
      flex: 1;
      padding: 16px;
      overflow-y: auto;

      .layout-container {
        min-height: 400px;
        padding: 16px;
        background: var(--color-fill-1);
        border: 1px dashed var(--color-border);
        border-radius: 4px;

        .empty-placeholder {
          display: flex;
          align-items: center;
          justify-content: center;
          min-height: 300px;
        }

        .groups-container {
          .form-group {
            margin-bottom: 16px;
            background: var(--color-bg-2);
            border: 1px solid var(--color-border);
            border-radius: 4px;

            .group-header {
              display: flex;
              align-items: center;
              justify-content: space-between;
              padding: 12px;
              background: var(--color-fill-1);
              border-bottom: 1px solid var(--color-border);
              border-radius: 4px 4px 0 0;
            }

            .group-content {
              padding: 12px;
            }
          }
        }
      }
    }

    .right-panel {
      width: 320px;
      padding: 16px;
      overflow-y: auto;
      border-left: 1px solid var(--color-border);
    }
  }
}
</style>

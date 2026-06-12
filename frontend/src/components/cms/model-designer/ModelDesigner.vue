<template>
  <div class="model-designer">
    <div class="designer-header">
      <h3>模型设计器</h3>
      <a-space>
        <a-button @click="handlePreview">
          <template #icon><icon-eye /></template>
          预览
        </a-button>
        <a-button type="primary" @click="handleSave">
          <template #icon><icon-save /></template>
          保存
        </a-button>
        <a-button type="outline" @click="handleGenerateCode">
          <template #icon><icon-code /></template>
          生成代码
        </a-button>
      </a-space>
    </div>

    <div class="designer-content">
      <div class="left-panel">
        <a-card title="基本信息" class="panel-card">
          <ModelForm v-model="modelData" />
        </a-card>

        <a-card title="模型配置" class="panel-card">
          <ModelConfig v-model="modelData.config" />
        </a-card>
      </div>

      <div class="right-panel">
        <a-card title="字段列表" class="panel-card">
          <template #extra>
            <a-button type="primary" size="small" @click="handleAddField">
              <template #icon><icon-plus /></template>
              添加字段
            </a-button>
          </template>
          <FieldList v-model="modelData.fields" @edit="handleEditField" @delete="handleDeleteField" />
        </a-card>
      </div>
    </div>

    <FieldEditor v-model:visible="fieldEditorVisible" :field="currentField" :mode="fieldEditorMode" @save="handleFieldSave" />
  </div>
</template>

<script setup lang="ts">
import { ref, watch, defineAsyncComponent } from "vue";
import { Message } from "@arco-design/web-vue";

const ModelForm = defineAsyncComponent(() => import("./ModelForm.vue"));
const ModelConfig = defineAsyncComponent(() => import("./ModelConfig.vue"));
const FieldList = defineAsyncComponent(() => import("./FieldList.vue"));
const FieldEditor = defineAsyncComponent(() => import("./FieldEditor.vue"));

interface ModelField {
  id: string;
  name: string;
  label: string;
  type: string;
  required: boolean;
  defaultValue?: any;
  placeholder?: string;
  formConfig?: Record<string, any>;
  tableConfig?: Record<string, any>;
  validationRules?: any[];
  sort: number;
}

interface ModelConfig {
  tableName?: string;
  description?: string;
  enableCache?: boolean;
  enableSoftDelete?: boolean;
  enableTimestamp?: boolean;
  [key: string]: any;
}

interface ModelData {
  id?: string;
  name: string;
  label: string;
  description?: string;
  config: ModelConfig;
  fields: ModelField[];
}

interface Props {
  modelValue?: ModelData;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({
    name: "",
    label: "",
    config: {
      tableName: "",
      enableCache: false,
      enableSoftDelete: true,
      enableTimestamp: true
    },
    fields: []
  })
});

const emit = defineEmits<{
  "update:modelValue": [value: ModelData];
  save: [value: ModelData];
  preview: [value: ModelData];
  "generate-code": [value: ModelData];
}>();

const modelData = ref<ModelData>(props.modelValue);

watch(
  () => props.modelValue,
  val => {
    modelData.value = val;
  },
  { deep: true }
);

watch(
  modelData,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const fieldEditorVisible = ref(false);
const currentField = ref<ModelField | null>(null);
const fieldEditorMode = ref<"add" | "edit">("add");

const handleAddField = () => {
  currentField.value = {
    id: "",
    name: "",
    label: "",
    type: "text",
    required: false,
    sort: modelData.value.fields.length
  };
  fieldEditorMode.value = "add";
  fieldEditorVisible.value = true;
};

const handleEditField = (field: ModelField) => {
  currentField.value = { ...field };
  fieldEditorMode.value = "edit";
  fieldEditorVisible.value = true;
};

const handleDeleteField = (fieldId: string) => {
  const index = modelData.value.fields.findIndex(f => f.id === fieldId);
  if (index > -1) {
    modelData.value.fields.splice(index, 1);
    Message.success("删除成功");
  }
};

const handleFieldSave = (field: ModelField) => {
  if (fieldEditorMode.value === "add") {
    field.id = `field_${Date.now()}`;
    modelData.value.fields.push(field);
    Message.success("添加成功");
  } else {
    const index = modelData.value.fields.findIndex(f => f.id === field.id);
    if (index > -1) {
      modelData.value.fields[index] = field;
      Message.success("更新成功");
    }
  }
};

const handleSave = () => {
  emit("save", modelData.value);
  Message.success("保存成功");
};

const handlePreview = () => {
  emit("preview", modelData.value);
};

const handleGenerateCode = () => {
  emit("generate-code", modelData.value);
};
</script>

<style lang="scss" scoped>
.model-designer {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--color-bg-2);

  .designer-header {
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

  .designer-content {
    display: flex;
    flex: 1;
    min-height: 0;
    overflow: hidden;

    .left-panel {
      display: flex;
      flex-direction: column;
      width: 360px;
      padding: 16px;
      overflow-y: auto;
      border-right: 1px solid var(--color-border);

      .panel-card {
        margin-bottom: 16px;

        &:last-child {
          margin-bottom: 0;
        }
      }
    }

    .right-panel {
      flex: 1;
      padding: 16px;
      overflow-y: auto;

      .panel-card {
        height: 100%;
      }
    }
  }
}
</style>

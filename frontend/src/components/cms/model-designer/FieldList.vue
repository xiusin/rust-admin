<template>
  <div class="field-list">
    <div v-if="fields.length === 0" class="empty-placeholder">
      <a-empty description="暂无字段，请点击上方按钮添加">
        <template #image>
          <icon-file />
        </template>
      </a-empty>
    </div>

    <draggable v-else v-model="localFields" item-key="id" handle=".drag-handle" animation="200" @end="handleDragEnd">
      <template #item="{ element, index }">
        <div class="field-item" :class="{ 'is-required': element.required }">
          <div class="drag-handle">
            <icon-drag-dot-vertical />
          </div>

          <div class="field-info">
            <div class="field-header">
              <span class="field-name">{{ element.name }}</span>
              <a-tag :color="getFieldTypeColor(element.type)" size="small">
                {{ getFieldTypeLabel(element.type) }}
              </a-tag>
              <a-tag v-if="element.required" color="red" size="small">必填</a-tag>
            </div>
            <div class="field-label">{{ element.label }}</div>
          </div>

          <div class="field-actions">
            <a-button type="text" size="small" @click="handleEdit(element)">
              <template #icon><icon-edit /></template>
            </a-button>
            <a-popconfirm content="确定要删除此字段吗？" @ok="handleDelete(element)">
              <a-button type="text" size="small" status="danger">
                <template #icon><icon-delete /></template>
              </a-button>
            </a-popconfirm>
          </div>
        </div>
      </template>
    </draggable>
  </div>
</template>

<script setup lang="ts">
import draggable from "vuedraggable";
import { Message } from "@arco-design/web-vue";

interface ModelField {
  id: string;
  name: string;
  label: string;
  type: string;
  required: boolean;
  sort: number;
}

interface Props {
  modelValue: ModelField[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:modelValue": [value: ModelField[]];
  edit: [field: ModelField];
  delete: [fieldId: string];
}>();

const localFields = ref<ModelField[]>([...props.modelValue]);

watch(
  () => props.modelValue,
  val => {
    localFields.value = [...val];
  },
  { deep: true }
);

watch(
  localFields,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleDragEnd = () => {
  localFields.value.forEach((field, index) => {
    field.sort = index;
  });
  Message.success("排序已更新");
};

const handleEdit = (field: ModelField) => {
  emit("edit", field);
};

const handleDelete = (field: ModelField) => {
  emit("delete", field.id);
};

const fieldTypeMap: Record<string, { label: string; color: string }> = {
  text: { label: "文本", color: "arcoblue" },
  textarea: { label: "多行文本", color: "blue" },
  number: { label: "数字", color: "green" },
  select: { label: "下拉选择", color: "orange" },
  radio: { label: "单选", color: "purple" },
  checkbox: { label: "多选", color: "magenta" },
  date: { label: "日期", color: "cyan" },
  datetime: { label: "日期时间", color: "teal" },
  image: { label: "图片", color: "gold" },
  file: { label: "文件", color: "lime" },
  editor: { label: "富文本", color: "pink" },
  json: { label: "JSON", color: "gray" }
};

const getFieldTypeLabel = (type: string) => {
  return fieldTypeMap[type]?.label || type;
};

const getFieldTypeColor = (type: string) => {
  return fieldTypeMap[type]?.color || "gray";
};
</script>

<style lang="scss" scoped>
.field-list {
  .empty-placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  .field-item {
    display: flex;
    align-items: center;
    padding: 12px;
    margin-bottom: 8px;
    background: var(--color-fill-1);
    border-radius: 4px;
    transition: all 0.2s;

    &:hover {
      background: var(--color-fill-2);

      .field-actions {
        opacity: 1;
      }
    }

    &.is-required {
      border-left: 3px solid rgb(var(--danger-6));
    }

    .drag-handle {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 24px;
      height: 24px;
      margin-right: 12px;
      color: var(--color-text-3);
      cursor: move;

      &:hover {
        color: var(--color-text-1);
      }
    }

    .field-info {
      flex: 1;
      min-width: 0;

      .field-header {
        display: flex;
        gap: 8px;
        align-items: center;
        margin-bottom: 4px;

        .field-name {
          font-family: monospace;
          font-size: 14px;
          font-weight: 500;
        }
      }

      .field-label {
        font-size: 12px;
        color: var(--color-text-3);
      }
    }

    .field-actions {
      display: flex;
      gap: 4px;
      opacity: 0;
      transition: opacity 0.2s;
    }
  }
}
</style>

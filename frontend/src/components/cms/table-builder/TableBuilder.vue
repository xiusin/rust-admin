<template>
  <div class="table-builder">
    <div class="builder-header">
      <h3>表格构建器</h3>
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
              <a-checkbox v-model="field.selected" @change="handleFieldToggle(field)" />
            </div>
          </div>
        </a-card>
      </div>

      <div class="center-panel">
        <a-tabs v-model:active-key="activeTab">
          <a-tab-pane key="columns" title="列配置">
            <a-card class="panel-card">
              <template #extra>
                <a-button type="primary" size="small" @click="handleAddColumn">
                  <template #icon><icon-plus /></template>
                  添加列
                </a-button>
              </template>
              <ColumnConfig v-model="tableSchema.columns" @select="handleSelectColumn" />
            </a-card>
          </a-tab-pane>

          <a-tab-pane key="search" title="搜索配置">
            <SearchConfig v-model="tableSchema.searchFields" />
          </a-tab-pane>

          <a-tab-pane key="filter" title="筛选配置">
            <FilterConfig v-model="tableSchema.filters" />
          </a-tab-pane>

          <a-tab-pane key="actions" title="操作配置">
            <ActionConfig v-model="tableSchema.actions" />
          </a-tab-pane>
        </a-tabs>
      </div>

      <div class="right-panel">
        <a-card title="表格预览" class="panel-card">
          <div class="preview-container">
            <a-table
              :columns="previewColumns"
              :data="previewData"
              :pagination="tableSchema.pagination"
              :stripe="tableSchema.stripe"
              :bordered="tableSchema.bordered"
              size="small"
            />
          </div>
        </a-card>

        <a-card title="表格配置" class="panel-card">
          <a-form layout="vertical">
            <a-form-item label="表格尺寸">
              <a-radio-group v-model="tableSchema.size">
                <a-radio value="small">小</a-radio>
                <a-radio value="medium">中</a-radio>
                <a-radio value="large">大</a-radio>
              </a-radio-group>
            </a-form-item>

            <a-form-item label="显示边框">
              <a-switch v-model="tableSchema.bordered" />
            </a-form-item>

            <a-form-item label="斑马纹">
              <a-switch v-model="tableSchema.stripe" />
            </a-form-item>

            <a-form-item label="显示序号">
              <a-switch v-model="tableSchema.showIndex" />
            </a-form-item>

            <a-form-item label="显示选择框">
              <a-switch v-model="tableSchema.showSelection" />
            </a-form-item>

            <a-form-item label="分页配置">
              <a-switch v-model="tableSchema.pagination" />
            </a-form-item>

            <a-form-item label="每页条数">
              <a-select v-model="tableSchema.pageSize" :disabled="!tableSchema.pagination">
                <a-option :value="10">10条</a-option>
                <a-option :value="20">20条</a-option>
                <a-option :value="50">50条</a-option>
                <a-option :value="100">100条</a-option>
              </a-select>
            </a-form-item>
          </a-form>
        </a-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import ColumnConfig from "./ColumnConfig.vue";
import SearchConfig from "./SearchConfig.vue";
import FilterConfig from "./FilterConfig.vue";
import ActionConfig from "./ActionConfig.vue";

interface TableColumn {
  id: string;
  field: string;
  title: string;
  width?: number;
  fixed?: "left" | "right";
  sortable?: boolean;
  visible?: boolean;
  render?: string;
}

interface SearchField {
  id: string;
  field: string;
  label: string;
  type: string;
  placeholder?: string;
}

interface Filter {
  id: string;
  field: string;
  label: string;
  type: string;
  options?: any[];
}

interface TableAction {
  id: string;
  label: string;
  type: string;
  icon?: string;
  handler?: string;
}

interface TableSchema {
  name: string;
  columns: TableColumn[];
  searchFields: SearchField[];
  filters: Filter[];
  actions: TableAction[];
  size: "small" | "medium" | "large";
  bordered: boolean;
  stripe: boolean;
  showIndex: boolean;
  showSelection: boolean;
  pagination: boolean;
  pageSize: number;
}

interface Props {
  modelValue?: TableSchema;
  fields?: { id: string; name: string; label: string; type: string; selected?: boolean }[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({
    name: "",
    columns: [],
    searchFields: [],
    filters: [],
    actions: [],
    size: "medium",
    bordered: true,
    stripe: true,
    showIndex: true,
    showSelection: true,
    pagination: true,
    pageSize: 20
  }),
  fields: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: TableSchema];
  save: [value: TableSchema];
}>();

const tableSchema = ref<TableSchema>(props.modelValue);
const availableFields = ref(props.fields.map(f => ({ ...f, selected: false })));
const activeTab = ref("columns");

watch(
  () => props.modelValue,
  val => {
    tableSchema.value = val;
  },
  { deep: true }
);

watch(
  tableSchema,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const previewColumns = computed(() => {
  const cols: any[] = [];

  if (tableSchema.value.showSelection) {
    cols.push({ type: "checkbox", fixed: "left" });
  }

  if (tableSchema.value.showIndex) {
    cols.push({ title: "序号", width: 60, render: (row: any, index: number) => index + 1 });
  }

  tableSchema.value.columns
    .filter(col => col.visible !== false)
    .forEach(col => {
      cols.push({
        title: col.title,
        dataIndex: col.field,
        width: col.width,
        fixed: col.fixed,
        sortable: col.sortable ? { sortDirections: ["ascend", "descend"] } : undefined
      });
    });

  if (tableSchema.value.actions.length > 0) {
    cols.push({
      title: "操作",
      width: 150,
      fixed: "right",
      render: () => "操作按钮"
    });
  }

  return cols;
});

const previewData = ref([
  { id: 1, name: "示例数据1", status: "正常", created_at: "2024-01-01" },
  { id: 2, name: "示例数据2", status: "禁用", created_at: "2024-01-02" },
  { id: 3, name: "示例数据3", status: "正常", created_at: "2024-01-03" }
]);

const handleDragStart = (event: DragEvent, field: any) => {
  event.dataTransfer?.setData("field", JSON.stringify(field));
};

const handleFieldToggle = (field: any) => {
  if (field.selected) {
    const exists = tableSchema.value.columns.find(c => c.field === field.name);
    if (!exists) {
      tableSchema.value.columns.push({
        id: `col_${Date.now()}`,
        field: field.name,
        title: field.label,
        visible: true
      });
    }
  } else {
    const index = tableSchema.value.columns.findIndex(c => c.field === field.name);
    if (index > -1) {
      tableSchema.value.columns.splice(index, 1);
    }
  }
};

const handleAddColumn = () => {
  tableSchema.value.columns.push({
    id: `col_${Date.now()}`,
    field: "",
    title: "新列",
    visible: true
  });
};

const handleSelectColumn = (column: TableColumn) => {
  console.log("Selected column:", column);
};

const handlePreview = () => {
  Message.info("预览功能已启用");
};

const handleSave = () => {
  emit("save", tableSchema.value);
  Message.success("保存成功");
};
</script>

<style lang="scss" scoped>
.table-builder {
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
      width: 240px;
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
    }

    .right-panel {
      width: 320px;
      padding: 16px;
      overflow-y: auto;
      border-left: 1px solid var(--color-border);

      .preview-container {
        margin-bottom: 16px;
        overflow-x: auto;
      }
    }
  }
}
</style>

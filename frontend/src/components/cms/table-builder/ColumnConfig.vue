<template>
  <div class="column-config">
    <div v-if="columns.length === 0" class="empty-columns">
      <a-empty description="暂无列配置，请添加或拖拽字段" />
    </div>

    <draggable v-else v-model="localColumns" item-key="id" handle=".drag-handle" animation="200">
      <template #item="{ element, index }">
        <div class="column-item" :class="{ 'is-selected': selectedId === element.id }" @click="handleSelect(element)">
          <div class="drag-handle">
            <icon-drag-dot-vertical />
          </div>

          <div class="column-info">
            <div class="column-header">
              <a-input v-model="element.title" placeholder="列标题" style="width: 120px" size="small" />
              <a-input v-model="element.field" placeholder="字段名" style="width: 100px" size="small" />
            </div>

            <div class="column-options">
              <a-space>
                <a-checkbox v-model="element.visible">显示</a-checkbox>
                <a-checkbox v-model="element.sortable">排序</a-checkbox>
                <a-select v-model="element.fixed" placeholder="固定" style="width: 80px" size="small" allow-clear>
                  <a-option value="left">左侧</a-option>
                  <a-option value="right">右侧</a-option>
                </a-select>
                <a-input-number v-model="element.width" placeholder="宽度" style="width: 80px" size="small" :min="50" />
              </a-space>
            </div>
          </div>

          <div class="column-actions">
            <a-button type="text" size="small" @click.stop="handleEditRender(element)">
              <icon-code />
            </a-button>
            <a-popconfirm content="确定删除此列吗？" @ok="handleDelete(index)">
              <a-button type="text" size="small" status="danger" @click.stop>
                <icon-delete />
              </a-button>
            </a-popconfirm>
          </div>
        </div>
      </template>
    </draggable>

    <a-modal v-model:visible="renderModalVisible" title="自定义渲染" :width="600" @ok="handleRenderSave">
      <a-form layout="vertical">
        <a-form-item label="渲染类型">
          <a-select v-model="currentColumn.renderType">
            <a-option value="text">文本</a-option>
            <a-option value="image">图片</a-option>
            <a-option value="tag">标签</a-option>
            <a-option value="link">链接</a-option>
            <a-option value="button">按钮</a-option>
            <a-option value="date">日期</a-option>
            <a-option value="custom">自定义</a-option>
          </a-select>
        </a-form-item>

        <a-form-item v-if="currentColumn.renderType === 'custom'" label="渲染函数">
          <a-textarea v-model="currentColumn.render" placeholder="请输入渲染函数代码" :auto-size="{ minRows: 5, maxRows: 15 }" />
          <template #extra>
            <span>参数: record (行数据), column (列配置), index (行索引)</span>
          </template>
        </a-form-item>

        <template v-if="currentColumn.renderType === 'tag'">
          <a-form-item label="标签颜色映射">
            <a-textarea
              v-model="currentColumn.tagColors"
              placeholder='{"success": "正常", "danger": "禁用"}'
              :auto-size="{ minRows: 2 }"
            />
          </a-form-item>
        </template>

        <template v-if="currentColumn.renderType === 'image'">
          <a-form-item label="图片宽度">
            <a-input-number v-model="currentColumn.imageWidth" :min="20" :max="200" />
          </a-form-item>
          <a-form-item label="图片高度">
            <a-input-number v-model="currentColumn.imageHeight" :min="20" :max="200" />
          </a-form-item>
        </template>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import draggable from "vuedraggable";
import { Message } from "@arco-design/web-vue";

interface TableColumn {
  id: string;
  field: string;
  title: string;
  width?: number;
  fixed?: "left" | "right";
  sortable?: boolean;
  visible?: boolean;
  render?: string;
  renderType?: string;
  tagColors?: string;
  imageWidth?: number;
  imageHeight?: number;
}

interface Props {
  modelValue: TableColumn[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:modelValue": [value: TableColumn[]];
  select: [column: TableColumn];
}>();

const localColumns = ref<TableColumn[]>([...props.modelValue]);
const selectedId = ref("");
const renderModalVisible = ref(false);
const currentColumn = ref<TableColumn>({} as TableColumn);

watch(
  () => props.modelValue,
  val => {
    localColumns.value = [...val];
  },
  { deep: true }
);

watch(
  localColumns,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleSelect = (column: TableColumn) => {
  selectedId.value = column.id;
  emit("select", column);
};

const handleDelete = (index: number) => {
  localColumns.value.splice(index, 1);
  Message.success("删除成功");
};

const handleEditRender = (column: TableColumn) => {
  currentColumn.value = { ...column };
  renderModalVisible.value = true;
};

const handleRenderSave = () => {
  const index = localColumns.value.findIndex(c => c.id === currentColumn.value.id);
  if (index > -1) {
    localColumns.value[index] = { ...currentColumn.value };
  }
  renderModalVisible.value = false;
  Message.success("保存成功");
};
</script>

<style lang="scss" scoped>
.column-config {
  .empty-columns {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  .column-item {
    display: flex;
    gap: 12px;
    align-items: flex-start;
    padding: 12px;
    margin-bottom: 8px;
    background: var(--color-fill-1);
    border-radius: 4px;
    transition: all 0.2s;

    &:hover {
      background: var(--color-fill-2);

      .column-actions {
        opacity: 1;
      }
    }

    &.is-selected {
      background: var(--color-fill-2);
      border-left: 3px solid rgb(var(--primary-6));
    }

    .drag-handle {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 20px;
      height: 20px;
      color: var(--color-text-3);
      cursor: move;

      &:hover {
        color: var(--color-text-1);
      }
    }

    .column-info {
      flex: 1;

      .column-header {
        display: flex;
        gap: 8px;
        margin-bottom: 8px;
      }

      .column-options {
        :deep(.arco-checkbox) {
          font-size: 12px;
        }
      }
    }

    .column-actions {
      display: flex;
      gap: 4px;
      opacity: 0;
      transition: opacity 0.2s;
    }
  }
}
</style>

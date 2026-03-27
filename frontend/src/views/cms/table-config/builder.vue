<template>
  <div class="snow-page">
    <div class="snow-inner table-builder">
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
              <a-card title="列配置列表" class="column-list-card">
                <template #extra>
                  <a-button type="text" size="small" @click="addColumn">
                    <template #icon><icon-plus /></template>
                  </a-button>
                </template>
                <div class="column-list">
                  <div
                    v-for="column in columns"
                    :key="column.id"
                    class="column-item"
                    :class="{ active: currentColumn?.id === column.id }"
                    draggable="true"
                    @click="selectColumn(column)"
                    @dragstart="onDragStart($event, column)"
                    @dragover.prevent
                    @drop="onDrop($event, column)"
                  >
                    <icon-drag-dot-vertical class="drag-handle" />
                    <span class="column-name">{{ column.title }}</span>
                    <span class="column-field">{{ column.dataIndex }}</span>
                  </div>
                </div>
              </a-card>
            </a-col>
            <a-col :span="12">
              <a-card title="列配置" class="column-config-card">
                <div v-if="currentColumn" class="column-config">
                  <a-form :model="currentColumn" layout="vertical">
                    <a-row :gutter="16">
                      <a-col :span="12">
                        <a-form-item label="列标题">
                          <a-input v-model="currentColumn.title" placeholder="请输入列标题" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="12">
                        <a-form-item label="字段名">
                          <a-input v-model="currentColumn.dataIndex" placeholder="请输入字段名" />
                        </a-form-item>
                      </a-col>
                    </a-row>
                    <a-row :gutter="16">
                      <a-col :span="8">
                        <a-form-item label="列宽度">
                          <a-input-number v-model="currentColumn.width" :min="50" :max="500" style="width: 100%" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="固定位置">
                          <a-select v-model="currentColumn.fixed">
                            <a-option value="">不固定</a-option>
                            <a-option value="left">左侧</a-option>
                            <a-option value="right">右侧</a-option>
                          </a-select>
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="对齐方式">
                          <a-select v-model="currentColumn.align">
                            <a-option value="left">左对齐</a-option>
                            <a-option value="center">居中</a-option>
                            <a-option value="right">右对齐</a-option>
                          </a-select>
                        </a-form-item>
                      </a-col>
                    </a-row>
                    <a-row :gutter="16">
                      <a-col :span="8">
                        <a-form-item label="是否显示">
                          <a-switch v-model="currentColumn.visible" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="允许排序">
                          <a-switch v-model="currentColumn.sortable" />
                        </a-form-item>
                      </a-col>
                      <a-col :span="8">
                        <a-form-item label="允许筛选">
                          <a-switch v-model="currentColumn.filterable" />
                        </a-form-item>
                      </a-col>
                    </a-row>
                    <a-form-item label="渲染类型">
                      <a-select v-model="currentColumn.renderType">
                        <a-option value="text">文本</a-option>
                        <a-option value="image">图片</a-option>
                        <a-option value="link">链接</a-option>
                        <a-option value="tag">标签</a-option>
                        <a-option value="date">日期</a-option>
                        <a-option value="datetime">日期时间</a-option>
                        <a-option value="number">数字</a-option>
                        <a-option value="money">金额</a-option>
                        <a-option value="status">状态</a-option>
                        <a-option value="action">操作</a-option>
                      </a-select>
                    </a-form-item>
                  </a-form>
                </div>
                <div v-else class="empty-column">
                  <a-empty description="请选择列" />
                </div>
              </a-card>
            </a-col>
            <a-col :span="6">
              <a-card title="表格属性" class="table-props-card">
                <a-form :model="tableConfig" layout="vertical">
                  <a-form-item label="表格名称">
                    <a-input v-model="tableConfig.name" placeholder="请输入表格名称" />
                  </a-form-item>
                  <a-form-item label="表格编码">
                    <a-input v-model="tableConfig.code" placeholder="请输入表格编码" />
                  </a-form-item>
                  <a-form-item label="行选择">
                    <a-select v-model="tableConfig.selectionType">
                      <a-option value="">无</a-option>
                      <a-option value="checkbox">多选</a-option>
                      <a-option value="radio">单选</a-option>
                    </a-select>
                  </a-form-item>
                  <a-form-item label="边框">
                    <a-switch v-model="tableConfig.bordered" />
                  </a-form-item>
                  <a-form-item label="斑马纹">
                    <a-switch v-model="tableConfig.striped" />
                  </a-form-item>
                  <a-form-item label="分页">
                    <a-switch v-model="tableConfig.pagination" />
                  </a-form-item>
                  <a-form-item v-if="tableConfig.pagination" label="每页条数">
                    <a-input-number v-model="tableConfig.pageSize" :min="10" :max="100" :step="10" style="width: 100%" />
                  </a-form-item>
                </a-form>
              </a-card>
            </a-col>
          </a-row>
        </a-spin>
      </div>

      <a-modal v-model:visible="previewVisible" :width="1000" :footer="false" title="表格预览">
        <div class="preview-table">
          <a-table
            :data="previewData"
            :columns="previewColumns"
            :pagination="tableConfig.pagination ? { pageSize: tableConfig.pageSize } : false"
            :row-selection="tableConfig.selectionType ? { type: tableConfig.selectionType } : undefined"
            :bordered="{ cell: tableConfig.bordered }"
            :stripe="tableConfig.striped"
          />
        </div>
      </a-modal>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { tableApi, type TableColumnConfig, type TableConfigDetail } from '@/api/modules/cms/table';

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const saving = ref(false);
const previewVisible = ref(false);
const currentColumn = ref<TableColumnConfig | null>(null);
const draggedColumn = ref<TableColumnConfig | null>(null);

const tableId = computed(() => Number(route.query.id));
const modelId = computed(() => Number(route.query.modelId));
const formTitle = computed(() => (tableId.value ? '编辑表格配置' : '新增表格配置'));

const columns = ref<TableColumnConfig[]>([]);
const previewData = ref<any[]>([]);

const tableConfig = reactive({
  name: '',
  code: '',
  selectionType: '' as '' | 'checkbox' | 'radio',
  bordered: true,
  striped: false,
  pagination: true,
  pageSize: 20,
});

const previewColumns = computed(() => {
  return columns.value
    .filter((c) => c.visible)
    .map((c) => ({
      title: c.title,
      dataIndex: c.dataIndex,
      width: c.width,
      fixed: c.fixed || undefined,
      align: c.align,
      sortable: c.sortable ? { sortDirections: ['ascend', 'descend'] } : undefined,
    }));
});

const goBack = () => {
  router.push('/cms/table-config/list');
};

const selectColumn = (column: TableColumnConfig) => {
  currentColumn.value = column;
};

const addColumn = () => {
  const newColumn: TableColumnConfig = {
    id: Date.now(),
    title: '新列',
    dataIndex: `column_${columns.value.length + 1}`,
    width: 100,
    visible: true,
    sortable: false,
    filterable: false,
    renderType: 'text',
    align: 'left',
  };
  columns.value.push(newColumn);
  currentColumn.value = newColumn;
};

const onDragStart = (e: DragEvent, column: TableColumnConfig) => {
  draggedColumn.value = column;
};

const onDrop = (e: DragEvent, targetColumn: TableColumnConfig) => {
  if (!draggedColumn.value || draggedColumn.value.id === targetColumn.id) return;
  const draggedIndex = columns.value.findIndex((c) => c.id === draggedColumn.value!.id);
  const targetIndex = columns.value.findIndex((c) => c.id === targetColumn.id);
  columns.value.splice(draggedIndex, 1);
  columns.value.splice(targetIndex, 0, draggedColumn.value);
};

const handlePreview = () => {
  previewData.value = Array.from({ length: 5 }, (_, i) => {
    const row: any = { id: i + 1 };
    columns.value.forEach((c) => {
      row[c.dataIndex] = `示例数据 ${i + 1}`;
    });
    return row;
  });
  previewVisible.value = true;
};

const handleSave = async () => {
  saving.value = true;
  try {
    const params = {
      id: tableId.value || undefined,
      modelId: modelId.value,
      name: tableConfig.name,
      code: tableConfig.code,
      config: {
        selectionType: tableConfig.selectionType,
        bordered: tableConfig.bordered,
        striped: tableConfig.striped,
        pagination: tableConfig.pagination,
        pageSize: tableConfig.pageSize,
        columns: columns.value,
      },
    };
    if (tableId.value) {
      await tableApi.edit(params);
      arcoMessage('success', '保存成功');
    } else {
      await tableApi.add(params);
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
    ...tableConfig,
    columns: columns.value,
  };
  const blob = new Blob([JSON.stringify(config, null, 2)], { type: 'application/json' });
  const url = window.URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `table-config-${Date.now()}.json`;
  a.click();
  window.URL.revokeObjectURL(url);
};

const loadTable = async () => {
  if (!tableId.value) return;
  loading.value = true;
  try {
    const data = await tableApi.detail(tableId.value);
    tableConfig.name = data.name;
    tableConfig.code = data.code;
    tableConfig.selectionType = data.config?.selectionType || '';
    tableConfig.bordered = data.config?.bordered ?? true;
    tableConfig.striped = data.config?.striped ?? false;
    tableConfig.pagination = data.config?.pagination ?? true;
    tableConfig.pageSize = data.config?.pageSize || 20;
    columns.value = data.config?.columns || [];
    if (columns.value.length > 0) {
      currentColumn.value = columns.value[0];
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  if (tableId.value) {
    loadTable();
  }
});
</script>

<style scoped lang="scss">
.table-builder {
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

    .column-list-card,
    .column-config-card,
    .table-props-card {
      height: calc(100vh - 200px);
      overflow-y: auto;
    }

    .column-list {
      .column-item {
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

        .column-name {
          flex: 1;
          font-size: 13px;
        }

        .column-field {
          font-size: 12px;
          color: var(--color-text-3);
        }
      }
    }

    .empty-column {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 300px;
    }
  }

  .preview-table {
    padding: 16px;
  }
}
</style>

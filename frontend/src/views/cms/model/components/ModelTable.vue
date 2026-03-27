<template>
  <a-table
    row-key="id"
    :loading="loading"
    :bordered="{ cell: true }"
    :scroll="{ x: '100%', y: '100%' }"
    :columns="columns"
    :data="data"
    :row-selection="{ type: 'checkbox', showCheckedAll: true }"
    :selected-keys="selectedKeys"
    :pagination="pagination"
    @page-change="(page: number) => emit('page-change', page)"
    @page-size-change="(size: number) => emit('page-size-change', size)"
    @selection-change="(keys: (string | number)[]) => emit('selection-change', keys)"
  >
    <template #icon="{ record }">
      <div class="model-icon">
        <component :is="record.icon || 'icon-apps'" />
      </div>
    </template>
    <template #isEnabled="{ record }">
      <a-tag :color="record.isEnabled ? 'green' : 'red'" size="small">
        {{ record.isEnabled ? '启用' : '禁用' }}
      </a-tag>
    </template>
    <template #isSystem="{ record }">
      <a-tag :color="record.isSystem ? 'arcoblue' : 'gray'" size="small">
        {{ record.isSystem ? '系统' : '自定义' }}
      </a-tag>
    </template>
    <template #optional="{ record }">
      <a-space>
        <a-button type="text" size="mini" @click="emit('design', record)">
          <template #icon><icon-settings /></template>
          设计
        </a-button>
        <a-button type="text" size="mini" @click="emit('content', record)">
          <template #icon><icon-file /></template>
          内容
        </a-button>
        <a-dropdown trigger="click">
          <a-button type="text" size="mini">
            <template #icon><icon-more /></template>
          </a-button>
          <template #content>
            <a-doption @click="emit('edit', record)">
              <icon-edit /> 编辑
            </a-doption>
            <a-doption @click="emit('copy', record)">
              <icon-copy /> 复制
            </a-doption>
            <a-doption @click="emit('toggle-status', record)">
              <icon-swap /> {{ record.isEnabled ? '禁用' : '启用' }}
            </a-doption>
            <a-doption @click="handleDelete(record)">
              <icon-delete /> 删除
            </a-doption>
          </template>
        </a-dropdown>
      </a-space>
    </template>
  </a-table>
</template>

<script setup lang="ts">
import type { CmsModelList } from '@/api/modules/cms/model';

interface Props {
  loading: boolean;
  data: CmsModelList[];
  pagination: {
    current: number;
    pageSize: number;
    total: number;
    showPageSize?: boolean;
    showTotal?: boolean;
  };
  selectedKeys: number[];
}

defineProps<Props>();

const emit = defineEmits<{
  'page-change': [page: number];
  'page-size-change': [size: number];
  'selection-change': [keys: (string | number)[]];
  edit: [record: CmsModelList];
  delete: [record: CmsModelList];
  design: [record: CmsModelList];
  content: [record: CmsModelList];
  copy: [record: CmsModelList];
  'toggle-status': [record: CmsModelList];
}>();

const columns = [
  { title: '图标', dataIndex: 'icon', slotName: 'icon', width: 60, align: 'center' },
  { title: '模型名称', dataIndex: 'name', width: 150 },
  { title: '模型编码', dataIndex: 'code', width: 120 },
  { title: '表名称', dataIndex: 'tableName', width: 150 },
  { title: '类型', dataIndex: 'isSystem', slotName: 'isSystem', width: 80, align: 'center' },
  { title: '字段数', dataIndex: 'fieldCount', width: 80, align: 'center' },
  { title: '内容数', dataIndex: 'contentCount', width: 80, align: 'center' },
  { title: '状态', dataIndex: 'isEnabled', slotName: 'isEnabled', width: 80, align: 'center' },
  { title: '排序', dataIndex: 'sort', width: 80, align: 'center' },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 200, fixed: 'right', align: 'center' },
];

const handleDelete = (record: CmsModelList) => {
  Modal.warning({
    title: '确认删除',
    content: `确定要删除模型"${record.name}"吗？`,
    hideCancel: false,
    onOk: () => {
      emit('delete', record);
    },
  });
};
</script>

<style scoped lang="scss">
.model-icon {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, rgb(var(--primary-6)), rgb(var(--primary-5)));
  border-radius: 6px;
  color: #fff;
  font-size: 16px;
  margin: 0 auto;
}
</style>

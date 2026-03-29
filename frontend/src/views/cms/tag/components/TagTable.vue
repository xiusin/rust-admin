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
    <template #color="{ record }">
      <div class="tag-color">
        <span class="color-dot" :style="{ background: record.color || '#1890ff' }"></span>
        <span>{{ record.color || "-" }}</span>
      </div>
    </template>
    <template #status="{ record }">
      <a-tag :color="record.status ? 'green' : 'red'" size="small">
        {{ record.status ? "启用" : "禁用" }}
      </a-tag>
    </template>
    <template #contentCount="{ record }">
      <a-badge :count="record.contentCount" :max-count="999" />
    </template>
    <template #optional="{ record }">
      <a-space>
        <a-button type="text" size="mini" @click="emit('edit', record)">编辑</a-button>
        <a-button type="text" size="mini" @click="emit('toggle-status', record)">
          {{ record.status ? "禁用" : "启用" }}
        </a-button>
        <a-popconfirm content="确定删除该标签吗？" type="warning" @ok="emit('delete', record)">
          <a-button type="text" status="danger" size="mini">删除</a-button>
        </a-popconfirm>
      </a-space>
    </template>
  </a-table>
</template>

<script setup lang="ts">
import type { CmsTagItem } from "@/api/modules/cms/tag";

interface Props {
  loading: boolean;
  data: CmsTagItem[];
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
  "page-change": [page: number];
  "page-size-change": [size: number];
  "selection-change": [keys: (string | number)[]];
  edit: [record: CmsTagItem];
  delete: [record: CmsTagItem];
  "toggle-status": [record: CmsTagItem];
}>();

const columns = [
  { title: "标签名称", dataIndex: "name", width: 150 },
  { title: "标签编码", dataIndex: "code", width: 120 },
  { title: "颜色", dataIndex: "color", slotName: "color", width: 120 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 80, align: "center" },
  { title: "内容数", dataIndex: "contentCount", slotName: "contentCount", width: 100, align: "center" },
  { title: "排序", dataIndex: "sort", width: 80, align: "center" },
  { title: "创建时间", dataIndex: "createdAt", width: 180 },
  { title: "操作", slotName: "optional", width: 180, fixed: "right", align: "center" }
];
</script>

<style scoped lang="scss">
.tag-color {
  display: flex;
  align-items: center;
  gap: 8px;

  .color-dot {
    width: 16px;
    height: 16px;
    border-radius: 4px;
  }
}
</style>

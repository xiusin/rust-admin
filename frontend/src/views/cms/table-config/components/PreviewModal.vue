<template>
  <div class="preview-modal">
    <a-table
      :data="previewData"
      :columns="previewColumns"
      :pagination="config.config?.pagination ? { pageSize: config.config.pageSize } : false"
      :row-selection="config.config?.selectionType ? { type: config.config.selectionType } : undefined"
      :bordered="{ cell: config.config?.bordered }"
      :stripe="config.config?.striped"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import type { TableConfigItem } from "@/api/modules/cms/table";

interface Props {
  config: TableConfigItem;
}

const props = defineProps<Props>();

const previewColumns = computed(() => {
  const columns = props.config.config?.columns || [];
  return columns
    .filter(c => c.visible)
    .map(c => ({
      title: c.title,
      dataIndex: c.dataIndex,
      width: c.width,
      fixed: c.fixed || undefined,
      align: c.align,
      sortable: c.sortable ? { sortDirections: ["ascend", "descend"] } : undefined
    }));
});

const previewData = ref<any[]>([]);

watch(
  () => props.config,
  () => {
    previewData.value = Array.from({ length: 5 }, (_, i) => {
      const row: any = { id: i + 1 };
      const columns = props.config.config?.columns || [];
      columns.forEach(c => {
        row[c.dataIndex] = `示例数据 ${i + 1}`;
      });
      return row;
    });
  },
  { immediate: true }
);
</script>

<style scoped lang="scss">
.preview-modal {
  padding: 16px;
}
</style>

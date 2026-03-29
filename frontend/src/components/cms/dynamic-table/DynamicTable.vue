<template>
  <div class="dynamic-table">
    <TableToolbar
      v-if="showToolbar"
      :search-config="schema.search"
      :filter-config="schema.filters"
      :actions="schema.toolbarActions"
      :selected-keys="selectedKeys"
      @search="handleSearch"
      @filter="handleFilter"
      @action="handleToolbarAction"
    />

    <a-table
      ref="tableRef"
      :columns="tableColumns"
      :data="tableData"
      :loading="loading"
      :pagination="paginationConfig"
      :row-selection="rowSelection"
      :stripe="schema.stripe"
      :bordered="schema.bordered"
      :size="schema.size || 'medium'"
      :scroll="scrollConfig"
      @page-change="handlePageChange"
      @page-size-change="handlePageSizeChange"
      @sorter-change="handleSortChange"
      @selection-change="handleSelectionChange"
    >
      <template v-for="slot in Object.keys($slots)" #[slot]="slotData">
        <slot :name="slot" v-bind="slotData" />
      </template>
    </a-table>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import TableToolbar from "./TableToolbar.vue";

interface TableColumn {
  field: string;
  title: string;
  width?: number;
  fixed?: "left" | "right";
  sortable?: boolean;
  visible?: boolean;
  render?: string;
  renderType?: string;
  renderConfig?: Record<string, any>;
}

interface SearchConfig {
  fields: any[];
  collapsed?: boolean;
}

interface FilterConfig {
  fields: any[];
}

interface TableAction {
  label: string;
  type: string;
  icon?: string;
  handler?: string;
  condition?: string;
}

interface TableSchema {
  columns: TableColumn[];
  search?: SearchConfig;
  filters?: FilterConfig;
  toolbarActions?: TableAction[];
  rowActions?: TableAction[];
  showIndex?: boolean;
  showSelection?: boolean;
  stripe?: boolean;
  bordered?: boolean;
  size?: "small" | "medium" | "large";
  pagination?: boolean;
  pageSize?: number;
}

interface Props {
  schema: TableSchema;
  data: any[];
  loading?: boolean;
  total?: number;
  showToolbar?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  schema: () => ({ columns: [] }),
  data: () => [],
  loading: false,
  total: 0,
  showToolbar: true
});

const emit = defineEmits<{
  search: [params: any];
  filter: [filters: any];
  "page-change": [page: number];
  "page-size-change": [pageSize: number];
  "sort-change": [sorter: any];
  "selection-change": [keys: string[], rows: any[]];
  action: [action: string, rows: any[]];
}>();

const tableRef = ref();
const selectedKeys = ref<string[]>([]);
const currentPage = ref(1);
const currentPageSize = ref(props.schema.pageSize || 20);
const searchParams = ref<Record<string, any>>({});
const filterParams = ref<Record<string, any>>({});

const tableColumns = computed(() => {
  const cols: any[] = [];

  if (props.schema.showSelection) {
    cols.push({
      type: "checkbox",
      fixed: "left"
    });
  }

  if (props.schema.showIndex) {
    cols.push({
      title: "序号",
      width: 60,
      fixed: "left",
      render: ({ rowIndex }: any) => {
        const start = (currentPage.value - 1) * currentPageSize.value;
        return start + rowIndex + 1;
      }
    });
  }

  props.schema.columns
    .filter(col => col.visible !== false)
    .forEach(col => {
      const column: any = {
        title: col.title,
        dataIndex: col.field,
        width: col.width,
        fixed: col.fixed,
        ellipsis: true,
        tooltip: true
      };

      if (col.sortable) {
        column.sortable = {
          sortDirections: ["ascend", "descend"]
        };
      }

      if (col.renderType) {
        column.render = getRenderFunction(col);
      }

      cols.push(column);
    });

  if (props.schema.rowActions && props.schema.rowActions.length > 0) {
    cols.push({
      title: "操作",
      width: 150,
      fixed: "right",
      render: ({ record }: any) => {
        return h(
          "div",
          { class: "row-actions" },
          props.schema.rowActions?.map(action =>
            h(
              "a-button",
              {
                type: "text",
                size: "small",
                onClick: () => handleRowAction(action, record)
              },
              () => action.label
            )
          )
        );
      }
    });
  }

  return cols;
});

const tableData = computed(() => props.data);

const paginationConfig = computed(() => {
  if (!props.schema.pagination) return false;
  return {
    current: currentPage.value,
    pageSize: currentPageSize.value,
    total: props.total,
    showTotal: true,
    showJumper: true,
    showPageSize: true,
    pageSizeOptions: [10, 20, 50, 100]
  };
});

const rowSelection = computed(() => {
  if (!props.schema.showSelection) return undefined;
  return {
    type: "checkbox",
    showCheckedAll: true
  };
});

const scrollConfig = computed(() => {
  const hasFixed = props.schema.columns.some(col => col.fixed);
  if (hasFixed) {
    return { x: 1200 };
  }
  return undefined;
});

const getRenderFunction = (column: TableColumn) => {
  const renderers: Record<string, any> = {
    text: ({ record }: any) => record[column.field],
    image: ({ record }: any) => {
      const url = record[column.field];
      if (!url) return "-";
      return h("img", {
        src: url,
        style: { width: "40px", height: "40px", objectFit: "cover", borderRadius: "4px" }
      });
    },
    tag: ({ record }: any) => {
      const value = record[column.field];
      const config = column.renderConfig || {};
      const colorMap: Record<string, string> = config.colors || {};
      return h("a-tag", { color: colorMap[value] || "gray" }, () => value);
    },
    link: ({ record }: any) => {
      const value = record[column.field];
      return h("a-link", { href: value, target: "_blank" }, () => value);
    },
    date: ({ record }: any) => {
      const value = record[column.field];
      if (!value) return "-";
      return new Date(value).toLocaleString();
    }
  };

  return renderers[column.renderType || "text"] || renderers.text;
};

const handleSearch = (params: any) => {
  searchParams.value = params;
  currentPage.value = 1;
  emit("search", { ...searchParams.value, ...filterParams.value });
};

const handleFilter = (filters: any) => {
  filterParams.value = filters;
  currentPage.value = 1;
  emit("filter", filters);
  emit("search", { ...searchParams.value, ...filterParams.value });
};

const handlePageChange = (page: number) => {
  currentPage.value = page;
  emit("page-change", page);
  emit("search", { ...searchParams.value, ...filterParams.value, page });
};

const handlePageSizeChange = (pageSize: number) => {
  currentPageSize.value = pageSize;
  currentPage.value = 1;
  emit("page-size-change", pageSize);
  emit("search", { ...searchParams.value, ...filterParams.value, pageSize });
};

const handleSortChange = (sorter: any) => {
  emit("sort-change", sorter);
};

const handleSelectionChange = (keys: string[], rows: any[]) => {
  selectedKeys.value = keys;
  emit("selection-change", keys, rows);
};

const handleToolbarAction = (action: string) => {
  emit("action", action, []);
};

const handleRowAction = (action: TableAction, record: any) => {
  emit("action", action.label, [record]);
};

const refresh = () => {
  emit("search", { ...searchParams.value, ...filterParams.value, page: currentPage.value });
};

const clearSelection = () => {
  selectedKeys.value = [];
};

defineExpose({
  refresh,
  clearSelection
});
</script>

<style lang="scss" scoped>
.dynamic-table {
  .row-actions {
    display: flex;
    gap: 4px;
  }
}
</style>

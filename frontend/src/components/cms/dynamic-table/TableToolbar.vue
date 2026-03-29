<template>
  <div class="table-toolbar">
    <div class="toolbar-left">
      <a-input-search
        v-if="showQuickSearch"
        v-model="quickSearchValue"
        placeholder="快速搜索..."
        style="width: 240px"
        allow-clear
        @search="handleQuickSearch"
        @clear="handleQuickSearch"
      />

      <a-button
        v-for="action in leftActions"
        :key="action.label"
        :type="action.type || 'secondary'"
        :status="action.status"
        @click="handleAction(action)"
      >
        <template v-if="action.icon" #icon>
          <component :is="action.icon" />
        </template>
        {{ action.label }}
      </a-button>
    </div>

    <div class="toolbar-right">
      <a-space>
        <a-button @click="handleRefresh">
          <template #icon><icon-refresh /></template>
        </a-button>

        <a-popover v-if="filterConfig && filterConfig.fields?.length" trigger="click" position="br">
          <a-button>
            <template #icon><icon-filter /></template>
            筛选
          </a-button>
          <template #content>
            <div class="filter-panel">
              <div v-for="filter in filterConfig.fields" :key="filter.field" class="filter-item">
                <div class="filter-label">{{ filter.label }}</div>
                <a-select
                  v-if="filter.type === 'select'"
                  v-model="filterValues[filter.field]"
                  :placeholder="filter.placeholder"
                  :options="filter.options"
                  allow-clear
                  style="width: 150px"
                  @change="handleFilterChange"
                />
                <a-date-picker
                  v-else-if="filter.type === 'date'"
                  v-model="filterValues[filter.field]"
                  :placeholder="filter.placeholder"
                  style="width: 150px"
                  @change="handleFilterChange"
                />
              </div>
            </div>
          </template>
        </a-popover>

        <a-dropdown v-if="showMoreActions" trigger="click">
          <a-button>
            <template #icon><icon-more /></template>
          </a-button>
          <template #content>
            <a-doption v-for="action in moreActions" :key="action.label" @click="handleAction(action)">
              <template v-if="action.icon" #icon>
                <component :is="action.icon" />
              </template>
              {{ action.label }}
            </a-doption>
          </template>
        </a-dropdown>
      </a-space>
    </div>

    <div v-if="selectedKeys.length > 0" class="selected-info">
      <a-tag color="arcoblue"> 已选择 {{ selectedKeys.length }} 项 </a-tag>
      <a-button type="text" size="small" @click="handleClearSelection"> 取消选择 </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface FilterField {
  field: string;
  label: string;
  type: string;
  placeholder?: string;
  options?: any[];
}

interface FilterConfig {
  fields: FilterField[];
}

interface ToolbarAction {
  label: string;
  type?: string;
  status?: string;
  icon?: string;
  position?: "left" | "right" | "more";
}

interface Props {
  searchConfig?: any;
  filterConfig?: FilterConfig;
  actions?: ToolbarAction[];
  selectedKeys?: string[];
  showQuickSearch?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  selectedKeys: () => [],
  showQuickSearch: true
});

const emit = defineEmits<{
  search: [params: any];
  filter: [filters: any];
  action: [action: string];
  "clear-selection": [];
}>();

const quickSearchValue = ref("");
const filterValues = ref<Record<string, any>>({});

const leftActions = computed(() => {
  return props.actions?.filter(a => a.position === "left" || !a.position) || [];
});

const moreActions = computed(() => {
  return props.actions?.filter(a => a.position === "more") || [];
});

const showMoreActions = computed(() => {
  return moreActions.value.length > 0;
});

const handleQuickSearch = () => {
  emit("search", { keyword: quickSearchValue.value });
};

const handleFilterChange = () => {
  emit("filter", filterValues.value);
};

const handleAction = (action: ToolbarAction) => {
  emit("action", action.label);
};

const handleRefresh = () => {
  emit("search", { keyword: quickSearchValue.value });
};

const handleClearSelection = () => {
  emit("clear-selection");
};
</script>

<style lang="scss" scoped>
.table-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 16px;

  .toolbar-left {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }

  .toolbar-right {
    display: flex;
    gap: 8px;
    align-items: center;
  }

  .selected-info {
    display: flex;
    gap: 8px;
    align-items: center;
    width: 100%;
  }

  .filter-panel {
    .filter-item {
      margin-bottom: 12px;

      .filter-label {
        margin-bottom: 4px;
        font-size: 12px;
        color: var(--color-text-3);
      }
    }
  }
}
</style>

<template>
  <div class="snow-page plugin-market-list">
    <div class="page-header">
      <div class="header-content">
        <h1 class="page-title">插件市场</h1>
        <p class="page-subtitle">发现和选择适合您业务的优质插件</p>
      </div>
    </div>

    <div class="snow-inner">
      <a-row :gutter="24">
        <a-col :xs="24" :sm="24" :md="6" :lg="5" class="filter-sidebar">
          <div class="filter-panel">
            <div class="filter-section">
              <div class="filter-header">
                <icon-apps class="filter-icon" />
                <span class="filter-title">分类筛选</span>
              </div>
              <div class="category-filter">
                <div 
                  class="category-item" 
                  :class="{ active: selectedCategoryId === 0 }" 
                  @click="handleCategorySelect(0)"
                >
                  <icon-check-circle-fill v-if="selectedCategoryId === 0" class="check-icon" />
                  <span>全部插件</span>
                  <span class="category-count">{{ totalPluginCount }}</span>
                </div>
                <div
                  v-for="category in categoryList"
                  :key="category.id"
                  class="category-item"
                  :class="{ active: selectedCategoryId === category.id }"
                  @click="handleCategorySelect(category.id)"
                >
                  <icon-check-circle-fill v-if="selectedCategoryId === category.id" class="check-icon" />
                  <span>{{ category.name }}</span>
                  <span class="category-count">{{ category.pluginCount }}</span>
                </div>
              </div>
            </div>

            <div class="filter-divider"></div>

            <div class="filter-section">
              <div class="filter-header">
                <icon-money class="filter-icon" />
                <span class="filter-title">价格筛选</span>
              </div>
              <div class="price-filter">
                <a-radio-group v-model="priceType" direction="vertical" @change="handleFilterChange">
                  <a-radio :value="0" class="filter-radio">
                    <span class="radio-label">全部</span>
                  </a-radio>
                  <a-radio :value="1" class="filter-radio">
                    <span class="radio-label">免费</span>
                    <a-tag color="green" size="small">推荐</a-tag>
                  </a-radio>
                  <a-radio :value="2" class="filter-radio">
                    <span class="radio-label">付费</span>
                  </a-radio>
                </a-radio-group>
              </div>
            </div>

            <div class="filter-divider"></div>

            <div class="filter-section">
              <div class="filter-header">
                <icon-sliders class="filter-icon" />
                <span class="filter-title">快捷筛选</span>
              </div>
              <div class="quick-filter">
                <a-checkbox-group v-model="quickFilters" @change="handleFilterChange">
                  <a-checkbox value="official" class="filter-checkbox">
                    <icon-star-fill class="checkbox-icon" />
                    官方插件
                  </a-checkbox>
                  <a-checkbox value="verified" class="filter-checkbox">
                    <icon-shield-check class="checkbox-icon" />
                    已验证
                  </a-checkbox>
                  <a-checkbox value="hot" class="filter-checkbox">
                    <icon-fire class="checkbox-icon" />
                    热门
                  </a-checkbox>
                </a-checkbox-group>
              </div>
            </div>

            <div class="filter-reset" @click="handleResetFilters">
              <icon-refresh />
              重置筛选
            </div>
          </div>
        </a-col>

        <a-col :xs="24" :sm="24" :md="18" :lg="19" class="plugin-content">
          <div class="content-toolbar">
            <div class="result-info">
              <icon-list class="info-icon" />
              <span>共找到 <span class="highlight">{{ pagination.total }}</span> 个插件</span>
            </div>
            <a-space size="16px">
              <a-select 
                v-model="sortType" 
                style="width: 160px" 
                class="sort-select"
                @change="handleSortChange"
              >
                <template #prefix>
                  <icon-sort class="sort-icon" />
                </template>
                <a-option :value="0">综合排序</a-option>
                <a-option :value="1">最新发布</a-option>
                <a-option :value="2">下载最多</a-option>
                <a-option :value="3">评分最高</a-option>
                <a-option :value="4">价格从低到高</a-option>
                <a-option :value="5">价格从高到低</a-option>
              </a-select>
              <div class="view-toggle">
                <div 
                  class="toggle-btn" 
                  :class="{ active: viewMode === 'grid' }"
                  @click="viewMode = 'grid'"
                >
                  <icon-apps />
                </div>
                <div 
                  class="toggle-btn" 
                  :class="{ active: viewMode === 'list' }"
                  @click="viewMode = 'list'"
                >
                  <icon-list />
                </div>
              </div>
            </a-space>
          </div>

          <div :class="['plugin-grid', { 'list-mode': viewMode === 'list' }]" v-if="pluginList.length > 0">
            <a-row :gutter="[24, 24]">
              <a-col 
                v-for="plugin in pluginList" 
                :key="plugin.id" 
                :xs="24" 
                :sm="12" 
                :lg="viewMode === 'list' ? 24 : 8"
              >
                <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
              </a-col>
            </a-row>
          </div>

          <a-empty v-else class="empty-state" description="暂无插件">
            <a-button type="primary" @click="handleResetFilters">
              清除筛选条件
            </a-button>
          </a-empty>

          <div class="pagination-wrapper" v-if="pagination.total > 0">
            <a-pagination
              :total="pagination.total"
              :current="pagination.current"
              :page-size="pagination.pageSize"
              show-page-size
              show-total
              size="large"
              @change="handlePageChange"
              @page-size-change="handlePageSizeChange"
            />
          </div>
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watch, computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { market, category } from "@/api/modules/plugin-market/market";
import PluginCard from "./components/plugin-card.vue";

interface Plugin {
  id: number;
  code: string;
  name: string;
  categoryId: number;
  categoryName: string;
  developerId: number;
  developerName: string;
  summary: string;
  coverImage: string;
  version: string;
  priceType: number;
  priceTypeName: string;
  price: number;
  verifyLevel: number;
  verifyLevelName: string;
  rating: number;
  downloadCount: number;
  status: number;
  statusName: string;
  tags: string[];
  isOfficial: number;
  createdAt: string;
}

interface Category {
  id: number;
  name: string;
  icon: string;
  parentId: number;
  sort: number;
  pluginCount: number;
  status: number;
  children: Category[];
}

const route = useRoute();
const router = useRouter();

const categoryList = ref<Category[]>([]);
const pluginList = ref<Plugin[]>([]);
const selectedCategoryId = ref(0);
const priceType = ref(0);
const sortType = ref(0);
const quickFilters = ref<string[]>([]);
const viewMode = ref<"grid" | "list">("grid");

const pagination = reactive({
  current: 1,
  pageSize: 12,
  total: 0
});

const totalPluginCount = computed(() => {
  return categoryList.value.reduce((sum, cat) => sum + cat.pluginCount, 0);
});

const loadCategories = async () => {
  try {
    const res = await category.tree();
    if (res.code === 200) {
      categoryList.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const loadPluginList = async () => {
  try {
    const res = await market.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      categoryId: selectedCategoryId.value || undefined,
      priceType: priceType.value || undefined,
      sortType: sortType.value || undefined
    });
    if (res.code === 200) {
      pluginList.value = res.data?.list || [];
      pagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  }
};

const handleCategorySelect = (categoryId: number) => {
  selectedCategoryId.value = categoryId;
  pagination.current = 1;
  router.replace({
    query: categoryId ? { categoryId: String(categoryId) } : {}
  });
  loadPluginList();
};

const handleFilterChange = () => {
  pagination.current = 1;
  loadPluginList();
};

const handleSortChange = () => {
  pagination.current = 1;
  loadPluginList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  loadPluginList();
  window.scrollTo({ top: 0, behavior: "smooth" });
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  pagination.current = 1;
  loadPluginList();
};

const handleResetFilters = () => {
  selectedCategoryId.value = 0;
  priceType.value = 0;
  sortType.value = 0;
  quickFilters.value = [];
  pagination.current = 1;
  router.replace({ query: {} });
  loadPluginList();
};

const goToDetail = (id: number) => {
  router.push({ path: "/plugin-market/detail", query: { id: String(id) } });
};

onMounted(() => {
  if (route.query.categoryId) {
    selectedCategoryId.value = Number(route.query.categoryId);
  }
  loadCategories();
  loadPluginList();
});

watch(
  () => route.query.categoryId,
  newVal => {
    if (newVal) {
      selectedCategoryId.value = Number(newVal);
    } else {
      selectedCategoryId.value = 0;
    }
    loadPluginList();
  }
);
</script>

<style lang="scss" scoped>
.plugin-market-list {
  .page-header {
    background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
    padding: 48px 24px;
    margin: -24px -24px 32px;

    .header-content {
      max-width: 1200px;
      margin: 0 auto;
      text-align: center;
    }

    .page-title {
      font-size: 32px;
      font-weight: 700;
      color: #ffffff;
      margin: 0 0 8px;
      letter-spacing: -0.5px;
    }

    .page-subtitle {
      font-size: 15px;
      color: rgba(255, 255, 255, 0.85);
      margin: 0;
    }
  }

  .filter-sidebar {
    .filter-panel {
      position: sticky;
      top: 24px;
      background: #ffffff;
      border-radius: 16px;
      padding: 24px;
      box-shadow: 0 2px 12px rgba(0, 0, 0, 0.04);
    }

    .filter-section {
      .filter-header {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 16px;

        .filter-icon {
          font-size: 18px;
          color: #165dff;
        }

        .filter-title {
          font-size: 15px;
          font-weight: 600;
          color: #1d2129;
        }
      }
    }

    .filter-divider {
      height: 1px;
      background: #f7f8fa;
      margin: 24px 0;
    }

    .category-filter {
      .category-item {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 10px 12px;
        margin: 0 -12px;
        border-radius: 8px;
        cursor: pointer;
        transition: all 0.2s ease;
        color: #4e5969;

        .check-icon {
          color: #165dff;
          font-size: 16px;
        }

        &:hover {
          background: #f7f8fa;
        }

        &.active {
          background: linear-gradient(135deg, #f0f5ff 0%, #e6f4ff 100%);
          color: #165dff;
          font-weight: 600;
        }

        .category-count {
          margin-left: auto;
          font-size: 12px;
          color: #86909c;
          background: #f7f8fa;
          padding: 2px 8px;
          border-radius: 10px;
        }

        &.active .category-count {
          background: #165dff;
          color: #ffffff;
        }
      }
    }

    .price-filter {
      :deep(.arco-radio-group) {
        display: flex;
        flex-direction: column;
        gap: 12px;
      }

      .filter-radio {
        display: flex;
        align-items: center;
        justify-content: space-between;
        width: 100%;
        padding: 8px 12px;
        margin: 0 -12px;
        border-radius: 8px;
        transition: all 0.2s ease;

        &:hover {
          background: #f7f8fa;
        }

        .radio-label {
          font-size: 14px;
        }
      }
    }

    .quick-filter {
      :deep(.arco-checkbox-group) {
        display: flex;
        flex-direction: column;
        gap: 12px;
      }

      .filter-checkbox {
        display: flex;
        align-items: center;
        gap: 8px;
        padding: 8px 0;

        .checkbox-icon {
          font-size: 16px;
          color: #4e5969;
        }
      }
    }

    .filter-reset {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 6px;
      padding: 12px;
      margin-top: 24px;
      border-radius: 8px;
      background: #f7f8fa;
      color: #4e5969;
      font-size: 14px;
      cursor: pointer;
      transition: all 0.2s ease;

      &:hover {
        background: #e5e6eb;
        color: #165dff;
      }
    }
  }

  .plugin-content {
    .content-toolbar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 24px;
      padding: 16px 20px;
      background: #ffffff;
      border-radius: 12px;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.03);

      .result-info {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 14px;
        color: #4e5969;

        .info-icon {
          color: #165dff;
        }

        .highlight {
          color: #165dff;
          font-weight: 700;
          font-size: 18px;
        }
      }

      .sort-select {
        :deep(.arco-select-view) {
          border-radius: 8px;
        }

        .sort-icon {
          margin-right: 4px;
          color: #86909c;
        }
      }

      .view-toggle {
        display: flex;
        gap: 4px;
        padding: 4px;
        background: #f7f8fa;
        border-radius: 8px;

        .toggle-btn {
          display: flex;
          align-items: center;
          justify-content: center;
          width: 36px;
          height: 36px;
          border-radius: 6px;
          cursor: pointer;
          color: #86909c;
          transition: all 0.2s ease;

          &:hover {
            color: #4e5969;
          }

          &.active {
            background: #ffffff;
            color: #165dff;
            box-shadow: 0 2px 6px rgba(0, 0, 0, 0.06);
          }
        }
      }
    }

    .plugin-grid {
      min-height: 400px;
    }

    .empty-state {
      padding: 80px 0;
      background: #ffffff;
      border-radius: 12px;
    }

    .pagination-wrapper {
      display: flex;
      justify-content: center;
      margin-top: 40px;
      padding: 20px;
      background: #ffffff;
      border-radius: 12px;
    }
  }
}
</style>

<template>
  <div class="snow-page plugin-market-list">
    <div class="snow-inner">
      <a-row :gutter="24">
        <a-col :xs="24" :sm="24" :md="6" :lg="5" class="filter-sidebar">
          <a-card title="分类筛选" :bordered="false">
            <div class="category-filter">
              <div
                class="category-item"
                :class="{ active: selectedCategoryId === 0 }"
                @click="handleCategorySelect(0)"
              >
                全部插件
              </div>
              <div
                v-for="category in categoryList"
                :key="category.id"
                class="category-item"
                :class="{ active: selectedCategoryId === category.id }"
                @click="handleCategorySelect(category.id)"
              >
                <span>{{ category.name }}</span>
                <span class="category-count">{{ category.pluginCount }}</span>
              </div>
            </div>
          </a-card>

          <a-card title="价格筛选" :bordered="false" class="filter-card">
            <div class="price-filter">
              <a-radio-group v-model="priceType" @change="handleFilterChange">
                <a-radio :value="0">全部</a-radio>
                <a-radio :value="1">免费</a-radio>
                <a-radio :value="2">付费</a-radio>
              </a-radio-group>
            </div>
          </a-card>
        </a-col>

        <a-col :xs="24" :sm="24" :md="18" :lg="19" class="plugin-content">
          <div class="content-header">
            <div class="result-info">
              共找到 <span class="highlight">{{ pagination.total }}</span> 个插件
            </div>
            <a-space>
              <span class="sort-label">排序：</span>
              <a-select
                v-model="sortType"
                style="width: 140px"
                @change="handleSortChange"
              >
                <a-option :value="0">综合排序</a-option>
                <a-option :value="1">最新发布</a-option>
                <a-option :value="2">下载最多</a-option>
                <a-option :value="3">评分最高</a-option>
                <a-option :value="4">价格从低到高</a-option>
                <a-option :value="5">价格从高到低</a-option>
              </a-select>
            </a-space>
          </div>

          <a-row :gutter="[24, 24]" class="plugin-grid" v-if="pluginList.length > 0">
            <a-col v-for="plugin in pluginList" :key="plugin.id" :xs="24" :sm="12" :lg="8">
              <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
            </a-col>
          </a-row>

          <a-empty v-else class="empty-state" description="暂无插件" />

          <div class="pagination-wrapper" v-if="pagination.total > 0">
            <a-pagination
              :total="pagination.total"
              :current="pagination.current"
              :page-size="pagination.pageSize"
              show-page-size
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
import { ref, reactive, onMounted, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { market, category } from '@/api/modules/plugin-market/market';
import PluginCard from './components/plugin-card.vue';

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

const pagination = reactive({
  current: 1,
  pageSize: 12,
  total: 0,
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
      sortType: sortType.value || undefined,
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
    query: categoryId ? { categoryId: String(categoryId) } : {},
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
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  pagination.current = 1;
  loadPluginList();
};

const goToDetail = (id: number) => {
  router.push({ path: '/plugin-market/detail', query: { id: String(id) } });
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
  (newVal) => {
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
  .filter-sidebar {
    .filter-card {
      margin-top: 16px;
    }

    .category-filter {
      .category-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 10px 12px;
        margin: 0 -12px;
        border-radius: 6px;
        cursor: pointer;
        transition: all 0.2s;
        color: #4e5969;

        &:hover {
          background: #f7f8fa;
        }

        &.active {
          background: #e6f4ff;
          color: #165dff;
          font-weight: 500;
        }

        .category-count {
          font-size: 12px;
          color: #86909c;
        }
      }
    }

    .price-filter {
      :deep(.arco-radio-group) {
        display: flex;
        flex-direction: column;
        gap: 8px;
      }
    }
  }

  .plugin-content {
    .content-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      margin-bottom: 24px;
      padding: 16px 20px;
      background: #f7f8fa;
      border-radius: 8px;

      .result-info {
        font-size: 14px;
        color: #4e5969;

        .highlight {
          color: #165dff;
          font-weight: 600;
        }
      }

      .sort-label {
        font-size: 14px;
        color: #4e5969;
      }
    }

    .plugin-grid {
      min-height: 400px;
    }

    .empty-state {
      padding: 60px 0;
    }

    .pagination-wrapper {
      display: flex;
      justify-content: center;
      margin-top: 32px;
    }
  }
}
</style>

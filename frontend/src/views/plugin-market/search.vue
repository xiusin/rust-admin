<template>
  <div class="snow-page plugin-market-search">
    <div class="search-header">
      <div class="search-container">
        <a-input-search
          v-model="searchKeyword"
          placeholder="搜索插件名称、功能..."
          search-button
          size="large"
          class="search-input"
          @search="handleSearch"
        >
          <template #button-icon>
            <icon-search />
          </template>
        </a-input-search>
      </div>
    </div>

    <div class="snow-inner">
      <div class="search-result-header">
        <div class="result-info">
          找到 <span class="highlight">{{ pagination.total }}</span> 个与 "<span class="keyword">{{ searchKeyword }}</span>" 相关的插件
        </div>
        <a-space>
          <span class="sort-label">排序：</span>
          <a-select v-model="sortType" style="width: 140px" @change="handleSortChange">
            <a-option :value="0">综合排序</a-option>
            <a-option :value="1">最新发布</a-option>
            <a-option :value="2">下载最多</a-option>
            <a-option :value="3">评分最高</a-option>
            <a-option :value="4">价格从低到高</a-option>
            <a-option :value="5">价格从高到低</a-option>
          </a-select>
        </a-space>
      </div>

      <div class="filter-bar">
        <div class="filter-group">
          <span class="filter-label">价格：</span>
          <a-radio-group v-model="priceType" @change="handleFilterChange">
            <a-radio :value="0">全部</a-radio>
            <a-radio :value="1">免费</a-radio>
            <a-radio :value="2">付费</a-radio>
          </a-radio-group>
        </div>
        <div class="filter-group">
          <span class="filter-label">分类：</span>
          <a-select v-model="selectedCategoryId" placeholder="全部分类" style="width: 160px" allow-clear @change="handleFilterChange">
            <a-option v-for="category in categoryList" :key="category.id" :value="category.id">
              {{ category.name }}
            </a-option>
          </a-select>
        </div>
        <div class="filter-group">
          <span class="filter-label">验证级别：</span>
          <a-select v-model="verifyLevel" placeholder="全部" style="width: 140px" allow-clear @change="handleFilterChange">
            <a-option :value="0">无验证</a-option>
            <a-option :value="1">基础验证</a-option>
            <a-option :value="2">高级验证</a-option>
          </a-select>
        </div>
      </div>

      <a-row :gutter="[24, 24]" class="plugin-grid" v-if="searchResults.length > 0">
        <a-col v-for="plugin in searchResults" :key="plugin.id" :xs="24" :sm="12" :lg="8">
          <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
        </a-col>
      </a-row>

      <a-empty v-else class="empty-state" description="未找到相关插件试试其他关键词" />

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

const searchKeyword = ref('');
const searchResults = ref<Plugin[]>([]);
const categoryList = ref<Category[]>([]);
const sortType = ref(0);
const priceType = ref(0);
const selectedCategoryId = ref<number | null>(null);
const verifyLevel = ref<number | null>(null);

const pagination = reactive({
  current: 1,
  pageSize: 12,
  total: 0,
});

const handleSearch = (value: string) => {
  searchKeyword.value = value;
  pagination.current = 1;
  router.replace({ query: { keyword: value } });
  loadSearchResults();
};

const handleSortChange = () => {
  pagination.current = 1;
  loadSearchResults();
};

const handleFilterChange = () => {
  pagination.current = 1;
  loadSearchResults();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  loadSearchResults();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  pagination.current = 1;
  loadSearchResults();
};

const goToDetail = (id: number) => {
  router.push({ path: '/plugin-market/detail', query: { id: String(id) } });
};

const loadCategoryList = async () => {
  try {
    const res = await category.tree();
    if (res.code === 200) {
      categoryList.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const loadSearchResults = async () => {
  if (!searchKeyword.value) {
    searchResults.value = [];
    pagination.total = 0;
    return;
  }

  try {
    const res = await market.search({
      keyword: searchKeyword.value,
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      categoryId: selectedCategoryId.value || undefined,
      priceType: priceType.value || undefined,
      verifyLevel: verifyLevel.value || undefined,
      sortType: sortType.value || undefined,
    });
    if (res.code === 200) {
      searchResults.value = res.data?.list || [];
      pagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  if (route.query.keyword) {
    searchKeyword.value = String(route.query.keyword);
  }
  loadCategoryList();
  loadSearchResults();
});

watch(
  () => route.query.keyword,
  (newVal) => {
    if (newVal) {
      searchKeyword.value = String(newVal);
      pagination.current = 1;
      loadSearchResults();
    }
  }
);
</script>

<style lang="scss" scoped>
.plugin-market-search {
  .search-header {
    background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
    padding: 40px 24px;
    margin: -24px -24px 24px;

    .search-container {
      max-width: 700px;
      margin: 0 auto;
    }

    .search-input {
      width: 100%;

      :deep(.arco-input-wrapper) {
        background: #fff;
        border-radius: 8px;
      }
    }
  }

  .search-result-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
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

      .keyword {
        color: #f53f3f;
        font-weight: 500;
      }
    }

    .sort-label {
      font-size: 14px;
      color: #4e5969;
    }
  }

  .filter-bar {
    display: flex;
    flex-wrap: wrap;
    gap: 24px;
    margin-bottom: 24px;
    padding: 16px 20px;
    background: #fff;
    border-radius: 8px;
    border: 1px solid #e5e6eb;

    .filter-group {
      display: flex;
      align-items: center;

      .filter-label {
        font-size: 14px;
        color: #4e5969;
        margin-right: 8px;
      }
    }
  }

  .plugin-grid {
    min-height: 400px;
  }

  .empty-state {
    padding: 80px 0;
  }

  .pagination-wrapper {
    display: flex;
    justify-content: center;
    margin-top: 32px;
  }
}
</style>

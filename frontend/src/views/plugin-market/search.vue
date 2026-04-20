<template>
  <div class="snow-page plugin-market-search">
    <div class="search-header">
      <div class="header-content">
        <h1 class="header-title">搜索插件</h1>
        <div class="search-input-wrapper">
          <a-input-search
            v-model="searchKeyword"
            placeholder="搜索插件名称、功能、关键词..."
            search-button
            size="large"
            class="search-input"
            @search="handleSearch"
          >
            <template #button>
              <a-button type="primary" :loading="searching">
                <template #icon>
                  <icon-search />
                </template>
                搜索
              </a-button>
            </template>
          </a-input-search>
        </div>
      </div>
    </div>

    <div class="snow-inner">
      <div class="search-result-header" v-if="searchKeyword">
        <div class="result-info">
          <icon-search class="info-icon" />
          <span>找到 <span class="highlight">{{ pagination.total }}</span> 个与 "<span class="keyword">{{ searchKeyword }}</span
          >" 相关的插件</span>
        </div>
        <a-space size="16px">
          <span class="sort-label">排序：</span>
          <a-select v-model="sortType" style="width: 140px" class="sort-select" @change="handleSortChange">
            <a-option :value="0">综合排序</a-option>
            <a-option :value="1">最新发布</a-option>
            <a-option :value="2">下载最多</a-option>
            <a-option :value="3">评分最高</a-option>
            <a-option :value="4">价格从低到高</a-option>
            <a-option :value="5">价格从高到低</a-option>
          </a-select>
        </a-space>
      </div>

      <div class="filter-bar" v-if="searchKeyword">
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
          <a-select
            v-model="selectedCategoryId"
            placeholder="全部分类"
            style="width: 160px"
            allow-clear
            class="filter-select"
            @change="handleFilterChange"
          >
            <a-option v-for="category in categoryList" :key="category.id" :value="category.id">
              {{ category.name }}
            </a-option>
          </a-select>
        </div>
        <div class="filter-group">
          <span class="filter-label">验证级别：</span>
          <a-select 
            v-model="verifyLevel" 
            placeholder="全部" 
            style="width: 140px" 
            allow-clear 
            class="filter-select"
            @change="handleFilterChange"
          >
            <a-option :value="0">无验证</a-option>
            <a-option :value="1">基础验证</a-option>
            <a-option :value="2">高级验证</a-option>
          </a-select>
        </div>
        <div class="filter-group filter-reset" @click="handleResetFilters">
          <icon-refresh />
          重置
        </div>
      </div>

      <div class="search-tips" v-if="!searchKeyword">
        <div class="tips-icon">
          <icon-search />
        </div>
        <div class="tips-text">
          <h3>开始搜索您需要的插件</h3>
          <p>输入关键词，搜索插件名称、功能或描述</p>
        </div>
        <div class="hot-search" v-if="hotKeywords.length > 0">
          <div class="hot-search-title">
            <icon-fire />
            热门搜索
          </div>
          <div class="hot-search-tags">
            <a-tag
              v-for="keyword in hotKeywords"
              :key="keyword"
              class="hot-tag"
              @click="handleHotSearchClick(keyword)"
            >
              {{ keyword }}
            </a-tag>
          </div>
        </div>
      </div>

      <a-row :gutter="[24, 24]" class="plugin-grid" v-if="searchKeyword && searchResults.length > 0">
        <a-col v-for="plugin in searchResults" :key="plugin.id" :xs="24" :sm="12" :lg="8">
          <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
        </a-col>
      </a-row>

      <a-empty 
        v-else-if="searchKeyword" 
        class="empty-state" 
        :description="`未找到与 \"${searchKeyword}\" 相关的插件`"
      >
        <div class="empty-suggestions">
          <p>建议您：</p>
          <ul>
            <li>检查输入的关键词是否正确</li>
            <li>尝试使用更通用的关键词</li>
            <li>浏览分类或使用其他筛选条件</li>
          </ul>
          <a-button type="primary" @click="handleResetFilters">
            清除筛选条件
          </a-button>
        </div>
      </a-empty>

      <div class="pagination-wrapper" v-if="searchKeyword && pagination.total > 0">
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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, watch } from "vue";
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

const searchKeyword = ref("");
const searching = ref(false);
const searchResults = ref<Plugin[]>([]);
const categoryList = ref<Category[]>([]);
const sortType = ref(0);
const priceType = ref(0);
const selectedCategoryId = ref<number | null>(null);
const verifyLevel = ref<number | null>(null);

const hotKeywords = ref<string[]>(["AI营销", "数据分析", "会员管理", "优惠券", "物流追踪", "多店铺"]);

const pagination = reactive({
  current: 1,
  pageSize: 12,
  total: 0
});

const handleSearch = (value: string) => {
  searchKeyword.value = value;
  pagination.current = 1;
  router.replace({ query: { keyword: value } });
  loadSearchResults();
};

const handleHotSearchClick = (keyword: string) => {
  searchKeyword.value = keyword;
  pagination.current = 1;
  router.replace({ query: { keyword } });
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
  window.scrollTo({ top: 0, behavior: "smooth" });
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  pagination.current = 1;
  loadSearchResults();
};

const handleResetFilters = () => {
  priceType.value = 0;
  sortType.value = 0;
  selectedCategoryId.value = null;
  verifyLevel.value = null;
  pagination.current = 1;
  loadSearchResults();
};

const goToDetail = (id: number) => {
  router.push({ path: "/plugin-market/detail", query: { id: String(id) } });
};

const loadCategoryList = async () => {
  try {
    const res = await category.tree();
    categoryList.value = res.data || [];
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

  searching.value = true;
  try {
    const res = await market.search({
      keyword: searchKeyword.value,
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      categoryId: selectedCategoryId.value || undefined,
      priceType: priceType.value || undefined,
      verifyLevel: verifyLevel.value || undefined,
      sortType: sortType.value || undefined
    });
    searchResults.value = res.data?.list || [];
    pagination.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    searching.value = false;
  }
};

onMounted(() => {
  if (route.query.keyword) {
    searchKeyword.value = String(route.query.keyword);
  }
  loadCategoryList();
  if (searchKeyword.value) {
    loadSearchResults();
  }
});

watch(
  () => route.query.keyword,
  newVal => {
    if (newVal) {
      searchKeyword.value = String(newVal);
      pagination.current = 1;
      loadSearchResults();
    } else {
      searchKeyword.value = "";
      searchResults.value = [];
      pagination.total = 0;
    }
  }
);
</script>

<style lang="scss" scoped>
.plugin-market-search {
  .search-header {
    background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
    padding: 60px 24px;
    margin: -24px -24px 32px;

    .header-content {
      max-width: 700px;
      margin: 0 auto;
      text-align: center;
    }

    .header-title {
      font-size: 32px;
      font-weight: 700;
      color: #ffffff;
      margin: 0 0 24px;
      letter-spacing: -0.5px;
    }

    .search-input-wrapper {
      .search-input {
        width: 100%;

        :deep(.arco-input-wrapper) {
          background: #fff;
          border-radius: 12px;
          padding: 4px;
          box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
        }

        :deep(.arco-input-search-btn) {
          border-radius: 8px;
        }
      }
    }
  }

  .search-result-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
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

      .keyword {
        color: #f53f3f;
        font-weight: 600;
      }
    }

    .sort-label {
      font-size: 14px;
      color: #4e5969;
    }

    .sort-select {
      :deep(.arco-select-view) {
        border-radius: 8px;
      }
    }
  }

  .filter-bar {
    display: flex;
    flex-wrap: wrap;
    gap: 24px;
    margin-bottom: 24px;
    padding: 16px 20px;
    background: #ffffff;
    border-radius: 12px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.03);

    .filter-group {
      display: flex;
      align-items: center;

      .filter-label {
        font-size: 14px;
        color: #4e5969;
        margin-right: 8px;
        font-weight: 500;
      }

      &.filter-reset {
        margin-left: auto;
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 6px 12px;
        border-radius: 6px;
        color: #86909c;
        cursor: pointer;
        transition: all 0.2s ease;

        &:hover {
          background: #f7f8fa;
          color: #165dff;
        }
      }
    }

    .filter-select {
      :deep(.arco-select-view) {
        border-radius: 8px;
      }
    }
  }

  .search-tips {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 24px;
    background: #ffffff;
    border-radius: 16px;
    text-align: center;

    .tips-icon {
      width: 80px;
      height: 80px;
      border-radius: 50%;
      background: linear-gradient(135deg, #f0f5ff 0%, #e6f4ff 100%);
      display: flex;
      align-items: center;
      justify-content: center;
      margin-bottom: 24px;

      svg {
        font-size: 40px;
        color: #165dff;
      }
    }

    .tips-text {
      h3 {
        font-size: 20px;
        font-weight: 600;
        color: #1d2129;
        margin: 0 0 8px;
      }

      p {
        font-size: 14px;
        color: #86909c;
        margin: 0;
      }
    }

    .hot-search {
      margin-top: 40px;
      width: 100%;
      max-width: 500px;

      .hot-search-title {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 14px;
        font-weight: 600;
        color: #1d2129;
        margin-bottom: 16px;

        svg {
          color: #ff7d00;
        }
      }

      .hot-search-tags {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        justify-content: center;

        .hot-tag {
          padding: 8px 16px;
          border-radius: 20px;
          font-size: 14px;
          cursor: pointer;
          transition: all 0.2s ease;
          background: #f7f8fa;
          border: 1px solid #e5e6eb;
          color: #4e5969;

          &:hover {
            background: #e6f4ff;
            border-color: #4080ff;
            color: #165dff;
            transform: translateY(-2px);
          }
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
    border-radius: 16px;

    .empty-suggestions {
      text-align: left;
      max-width: 400px;
      margin: 0 auto;

      p {
        font-size: 14px;
        color: #4e5969;
        margin-bottom: 12px;
      }

      ul {
        margin: 0 0 24px;
        padding-left: 20px;

        li {
          font-size: 13px;
          color: #86909c;
          line-height: 1.8;
        }
      }
    }
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
</style>

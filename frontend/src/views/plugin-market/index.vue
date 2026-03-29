<template>
  <div class="snow-page plugin-market-home">
    <div class="search-section">
      <div class="search-container">
        <h1 class="search-title">插件市场</h1>
        <p class="search-subtitle">发现优质插件，提升开发效率</p>
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
      <a-row :gutter="24" class="category-grid">
        <a-col
          v-for="category in categories"
          :key="category.id"
          :xs="24"
          :sm="12"
          :md="6"
          class="category-item"
          @click="handleCategoryClick(category)"
        >
          <a-card hoverable class="category-card">
            <div class="category-icon">
              <icon-book />
            </div>
            <div class="category-info">
              <div class="category-name">{{ category.name }}</div>
              <div class="category-count">{{ category.pluginCount }} 个插件</div>
            </div>
          </a-card>
        </a-col>
      </a-row>

      <a-divider :margin="40" />

      <div class="section-header">
        <h2 class="section-title">为您推荐</h2>
        <a-link class="section-more" @click="goToList">查看更多 <icon-right /></a-link>
      </div>
      <a-row :gutter="[24, 24]" class="plugin-grid">
        <a-col v-for="plugin in recommendList" :key="plugin.id" :xs="24" :sm="12" :md="8" :lg="6">
          <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
        </a-col>
      </a-row>

      <a-divider :margin="40" />

      <div class="section-header">
        <h2 class="section-title">热门插件</h2>
        <a-link class="section-more" @click="goToList">查看更多 <icon-right /></a-link>
      </div>
      <a-row :gutter="[24, 24]" class="plugin-grid">
        <a-col v-for="plugin in hotList" :key="plugin.id" :xs="24" :sm="12" :md="8" :lg="6">
          <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useRouter } from "vue-router";
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

const router = useRouter();
const searchKeyword = ref("");
const categories = ref<Category[]>([]);
const recommendList = ref<Plugin[]>([]);
const hotList = ref<Plugin[]>([]);

const handleSearch = (value: string) => {
  router.push({ path: "/plugin-market/search", query: { keyword: value } });
};

const handleCategoryClick = (category: Category) => {
  router.push({ path: "/plugin-market/list", query: { categoryId: String(category.id) } });
};

const goToList = () => {
  router.push("/plugin-market/list");
};

const goToDetail = (id: number) => {
  router.push({ path: "/plugin-market/detail", query: { id: String(id) } });
};

const loadCategories = async () => {
  try {
    const res = await category.tree();
    if (res.code === 200) {
      categories.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const loadRecommendList = async () => {
  try {
    const res = await market.recommend(8);
    if (res.code === 200) {
      recommendList.value = res.data?.list || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const loadHotList = async () => {
  try {
    const res = await market.hot(8);
    if (res.code === 200) {
      hotList.value = res.data?.list || [];
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadCategories();
  loadRecommendList();
  loadHotList();
});
</script>

<style lang="scss" scoped>
.plugin-market-home {
  .search-section {
    background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
    padding: 60px 24px;
    margin: -24px -24px 24px;

    .search-container {
      max-width: 600px;
      margin: 0 auto;
      text-align: center;
    }

    .search-title {
      font-size: 36px;
      font-weight: 600;
      color: #fff;
      margin-bottom: 8px;
    }

    .search-subtitle {
      font-size: 16px;
      color: rgba(255, 255, 255, 0.8);
      margin-bottom: 24px;
    }

    .search-input {
      width: 100%;

      :deep(.arco-input-wrapper) {
        background: #fff;
        border-radius: 8px;
      }
    }
  }

  .category-grid {
    .category-item {
      margin-bottom: 16px;
    }

    .category-card {
      display: flex;
      align-items: center;
      padding: 8px 0;
      border: none;
      background: #f7f8fa;

      &:hover {
        background: #e5e6eb;
      }

      :deep(.arco-card-body) {
        width: 100%;
        display: flex;
        align-items: center;
      }

      .category-icon {
        width: 48px;
        height: 48px;
        border-radius: 8px;
        background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
        display: flex;
        align-items: center;
        justify-content: center;
        color: #fff;
        font-size: 24px;
        margin-right: 16px;
      }

      .category-info {
        flex: 1;

        .category-name {
          font-size: 16px;
          font-weight: 500;
          color: #1d2129;
          margin-bottom: 4px;
        }

        .category-count {
          font-size: 13px;
          color: #86909c;
        }
      }
    }
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;

    .section-title {
      font-size: 20px;
      font-weight: 600;
      color: #1d2129;
      margin: 0;
    }

    .section-more {
      font-size: 14px;
    }
  }

  .plugin-grid {
    margin-bottom: 24px;
  }
}
</style>

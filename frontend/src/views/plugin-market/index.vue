<template>
  <div class="snow-page plugin-market-home">
    <div class="search-section">
      <div class="search-bg-decoration">
        <div class="bg-circle circle-1"></div>
        <div class="bg-circle circle-2"></div>
        <div class="bg-circle circle-3"></div>
      </div>
      <div class="search-container">
        <div class="search-icon-wrapper">
          <icon-shop />
        </div>
        <h1 class="search-title">插件市场</h1>
        <p class="search-subtitle">发现优质插件，提升您的业务效率</p>
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
              <a-button type="primary" class="search-btn">
                <template #icon>
                  <icon-search />
                </template>
                搜索
              </a-button>
            </template>
          </a-input-search>
        </div>
        <div class="search-hot-words" v-if="hotKeywords.length > 0">
          <span class="hot-label">
            <icon-fire />
            热门搜索：
          </span>
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

    <div class="snow-inner">
      <div class="section-categories">
        <div class="section-header">
          <h2 class="section-title">
            <icon-apps class="title-icon" />
            热门分类
          </h2>
          <a-link class="section-more" @click="goToList">
            查看全部插件
            <icon-right />
          </a-link>
        </div>
        <a-row :gutter="20" class="category-grid">
          <a-col
            v-for="category in categories"
            :key="category.id"
            :xs="12"
            :sm="8"
            :md="6"
            :lg="4"
            class="category-item"
            @click="handleCategoryClick(category)"
          >
            <a-card hoverable class="category-card">
              <div class="category-icon-wrapper">
                <div class="category-icon" :style="{ background: getCategoryColor(category.id) }">
                  <icon-book />
                </div>
              </div>
              <div class="category-info">
                <div class="category-name">{{ category.name }}</div>
                <div class="category-count">{{ category.pluginCount }} 个插件</div>
              </div>
              <div class="category-arrow">
                <icon-right />
              </div>
            </a-card>
          </a-col>
        </a-row>
      </div>

      <a-divider style="margin: 48px 0" />

      <div class="section-recommend">
        <div class="section-header">
          <div class="header-left">
            <h2 class="section-title">
              <icon-star class="title-icon" />
              为您推荐
            </h2>
            <p class="section-desc">精选优质插件，提升您的开发效率</p>
          </div>
          <a-link class="section-more" @click="goToList">
            查看更多
            <icon-right />
          </a-link>
        </div>
        <a-row :gutter="[24, 24]" class="plugin-grid">
          <a-col v-for="plugin in recommendList" :key="plugin.id" :xs="24" :sm="12" :md="8" :lg="6">
            <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
          </a-col>
        </a-row>
      </div>

      <a-divider style="margin: 48px 0" />

      <div class="section-hot">
        <div class="section-header">
          <div class="header-left">
            <h2 class="section-title">
              <icon-fire class="title-icon hot" />
              热门插件
            </h2>
            <p class="section-desc">最受欢迎的插件，大家都在用</p>
          </div>
          <a-link class="section-more" @click="goToList">
            查看更多
            <icon-right />
          </a-link>
        </div>
        <a-row :gutter="[24, 24]" class="plugin-grid">
          <a-col v-for="plugin in hotList" :key="plugin.id" :xs="24" :sm="12" :md="8" :lg="6">
            <PluginCard :plugin="plugin" @click="goToDetail(plugin.id)" />
          </a-col>
        </a-row>
      </div>

      <a-divider style="margin: 48px 0" />

      <div class="section-features">
        <div class="features-header">
          <h2 class="section-title">为什么选择我们的插件市场</h2>
          <p class="section-desc">专业、安全、高效的插件服务</p>
        </div>
        <a-row :gutter="32" class="features-grid">
          <a-col :xs="24" :sm="12" :md="6" class="feature-item">
            <div class="feature-card">
              <div class="feature-icon" style="background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);">
                <icon-shield-check />
              </div>
              <h3 class="feature-title">安全可靠</h3>
              <p class="feature-desc">所有插件都经过严格审核，确保安全稳定</p>
            </div>
          </a-col>
          <a-col :xs="24" :sm="12" :md="6" class="feature-item">
            <div class="feature-card">
              <div class="feature-icon" style="background: linear-gradient(135deg, #23a33c 0%, #49c75e 100%);">
                <icon-thumb-up />
              </div>
              <h3 class="feature-title">优质精选</h3>
              <p class="feature-desc">精选优质插件，只提供最好的体验</p>
            </div>
          </a-col>
          <a-col :xs="24" :sm="12" :md="6" class="feature-item">
            <div class="feature-card">
              <div class="feature-icon" style="background: linear-gradient(135deg, #ff7d00 0%, #ff9a2e 100%);">
                <icon-customer-service />
              </div>
              <h3 class="feature-title">专业支持</h3>
              <p class="feature-desc">7x24小时技术支持，解决您的问题</p>
            </div>
          </a-col>
          <a-col :xs="24" :sm="12" :md="6" class="feature-item">
            <div class="feature-card">
              <div class="feature-icon" style="background: linear-gradient(135deg, #722ed1 0%, #9254de 100%);">
                <icon-sync />
              </div>
              <h3 class="feature-title">持续更新</h3>
              <p class="feature-desc">插件持续更新，保持最新功能</p>
            </div>
          </a-col>
        </a-row>
      </div>
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

const hotKeywords = ref<string[]>(["AI营销", "数据分析", "会员管理", "优惠券", "物流追踪", "多店铺"]);

const categoryColors = [
  "linear-gradient(135deg, #165dff 0%, #4080ff 100%)",
  "linear-gradient(135deg, #23a33c 0%, #49c75e 100%)",
  "linear-gradient(135deg, #ff7d00 0%, #ff9a2e 100%)",
  "linear-gradient(135deg, #722ed1 0%, #9254de 100%)",
  "linear-gradient(135deg, #f53f3f 0%, #ff7875 100%)",
  "linear-gradient(135deg, #14c9c9 0%, #34d1d1 100%)"
];

const getCategoryColor = (id: number) => {
  return categoryColors[id % categoryColors.length];
};

const handleSearch = (value: string) => {
  router.push({ path: "/plugin-market/search", query: { keyword: value } });
};

const handleHotSearchClick = (keyword: string) => {
  router.push({ path: "/plugin-market/search", query: { keyword } });
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
    categories.value = res.data || [];
  } catch (error) {
    console.error(error);
  }
};

const loadRecommendList = async () => {
  try {
    const res = await market.recommend(8);
    recommendList.value = res.data?.list || [];
  } catch (error) {
    console.error(error);
  }
};

const loadHotList = async () => {
  try {
    const res = await market.hot(8);
    hotList.value = res.data?.list || [];
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
    position: relative;
    background: linear-gradient(135deg, #165dff 0%, #4080ff 50%, #722ed1 100%);
    padding: 80px 24px;
    margin: -24px -24px 48px;
    overflow: hidden;

    .search-bg-decoration {
      position: absolute;
      inset: 0;
      overflow: hidden;

      .bg-circle {
        position: absolute;
        border-radius: 50%;
        background: rgba(255, 255, 255, 0.08);
        animation: float 6s ease-in-out infinite;
      }

      .circle-1 {
        width: 300px;
        height: 300px;
        top: -100px;
        right: -50px;
      }

      .circle-2 {
        width: 200px;
        height: 200px;
        bottom: -60px;
        left: 10%;
        animation-delay: -2s;
      }

      .circle-3 {
        width: 150px;
        height: 150px;
        top: 20%;
        left: 5%;
        animation-delay: -4s;
      }
    }

    @keyframes float {
      0%, 100% {
        transform: translateY(0);
      }
      50% {
        transform: translateY(-20px);
      }
    }

    .search-container {
      position: relative;
      max-width: 700px;
      margin: 0 auto;
      text-align: center;
      z-index: 1;
    }

    .search-icon-wrapper {
      width: 72px;
      height: 72px;
      border-radius: 50%;
      background: rgba(255, 255, 255, 0.15);
      backdrop-filter: blur(8px);
      display: flex;
      align-items: center;
      justify-content: center;
      margin: 0 auto 24px;
      color: #fff;
      font-size: 36px;
    }

    .search-title {
      font-size: 42px;
      font-weight: 700;
      color: #fff;
      margin-bottom: 12px;
      letter-spacing: -1px;
    }

    .search-subtitle {
      font-size: 16px;
      color: rgba(255, 255, 255, 0.9);
      margin-bottom: 32px;
    }

    .search-input-wrapper {
      .search-input {
        width: 100%;

        :deep(.arco-input-wrapper) {
          background: #fff;
          border-radius: 16px;
          padding: 6px;
          box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
        }

        :deep(.arco-input-search-btn) {
          border-radius: 12px;
          padding: 0 24px;
          font-weight: 600;
        }

        .search-btn {
          background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
          border: none;
        }
      }
    }

    .search-hot-words {
      display: flex;
      align-items: center;
      justify-content: center;
      gap: 8px;
      margin-top: 20px;
      flex-wrap: wrap;

      .hot-label {
        display: flex;
        align-items: center;
        gap: 4px;
        color: rgba(255, 255, 255, 0.9);
        font-size: 14px;

        svg {
          color: #ff7d00;
        }
      }

      .hot-tag {
        background: rgba(255, 255, 255, 0.15);
        border: none;
        color: #fff;
        cursor: pointer;
        transition: all 0.2s ease;
        font-size: 13px;

        &:hover {
          background: rgba(255, 255, 255, 0.25);
          transform: translateY(-2px);
        }
      }
    }
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 28px;

    .header-left {
      .section-title {
        font-size: 24px;
        font-weight: 700;
        color: #1d2129;
        margin: 0 0 6px;
        display: flex;
        align-items: center;
        gap: 10px;

        .title-icon {
          font-size: 24px;
          color: #165dff;

          &.hot {
            color: #ff7d00;
          }
        }
      }

      .section-desc {
        font-size: 14px;
        color: #86909c;
        margin: 0;
      }
    }

    .section-more {
      font-size: 14px;
      color: #165dff;
      font-weight: 500;
      display: flex;
      align-items: center;
      gap: 4px;

      &:hover {
        color: #0e42d2;
      }
    }
  }

  .section-categories {
    .category-grid {
      .category-item {
        margin-bottom: 20px;
      }

      .category-card {
        display: flex;
        align-items: center;
        padding: 20px;
        border: none;
        background: #ffffff;
        border-radius: 16px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        position: relative;
        overflow: hidden;

        &::before {
          content: '';
          position: absolute;
          top: 0;
          left: 0;
          right: 0;
          height: 3px;
          background: linear-gradient(90deg, #165dff 0%, #4080ff 100%);
          transform: scaleX(0);
          transition: transform 0.3s ease;
        }

        &:hover {
          transform: translateY(-4px);
          box-shadow: 0 12px 32px rgba(22, 93, 255, 0.12);

          &::before {
            transform: scaleX(1);
          }
        }

        :deep(.arco-card-body) {
          width: 100%;
          display: flex;
          align-items: center;
          padding: 0;
        }

        .category-icon-wrapper {
          flex-shrink: 0;
        }

        .category-icon {
          width: 56px;
          height: 56px;
          border-radius: 14px;
          display: flex;
          align-items: center;
          justify-content: center;
          color: #fff;
          font-size: 28px;
        }

        .category-info {
          flex: 1;
          margin-left: 16px;
          text-align: left;

          .category-name {
            font-size: 16px;
            font-weight: 600;
            color: #1d2129;
            margin-bottom: 4px;
          }

          .category-count {
            font-size: 13px;
            color: #86909c;
          }
        }

        .category-arrow {
          color: #c9cdd4;
          font-size: 18px;
          transition: all 0.3s ease;
        }

        &:hover .category-arrow {
          color: #165dff;
          transform: translateX(4px);
        }
      }
    }
  }

  .section-recommend,
  .section-hot {
    .plugin-grid {
      margin-bottom: 0;
    }
  }

  .section-features {
    padding: 48px 0 24px;

    .features-header {
      text-align: center;
      margin-bottom: 40px;

      .section-title {
        font-size: 28px;
        font-weight: 700;
        color: #1d2129;
        margin: 0 0 12px;
      }

      .section-desc {
        font-size: 15px;
        color: #86909c;
        margin: 0;
      }
    }

    .features-grid {
      .feature-item {
        margin-bottom: 24px;
      }

      .feature-card {
        text-align: center;
        padding: 32px 24px;
        background: #ffffff;
        border-radius: 16px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        height: 100%;

        &:hover {
          transform: translateY(-8px);
          box-shadow: 0 16px 40px rgba(0, 0, 0, 0.08);
        }

        .feature-icon {
          width: 72px;
          height: 72px;
          border-radius: 20px;
          display: flex;
          align-items: center;
          justify-content: center;
          color: #fff;
          font-size: 32px;
          margin: 0 auto 20px;
        }

        .feature-title {
          font-size: 18px;
          font-weight: 600;
          color: #1d2129;
          margin: 0 0 10px;
        }

        .feature-desc {
          font-size: 14px;
          color: #86909c;
          margin: 0;
          line-height: 1.6;
        }
      }
    }
  }
}
</style>

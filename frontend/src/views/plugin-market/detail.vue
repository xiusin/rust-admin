<template>
  <div class="snow-page plugin-market-detail">
    <div class="plugin-header" v-if="pluginDetail">
      <div class="header-bg"></div>
      <div class="header-content">
        <div class="header-main">
          <div class="plugin-cover-large">
            <img :src="pluginDetail.coverImage" :alt="pluginDetail.name" />
          </div>
          <div class="plugin-info-main">
            <div class="plugin-badges">
              <a-tag v-if="pluginDetail.isOfficial" color="blue" class="badge-official">
                <icon-star-fill />
                官方
              </a-tag>
              <a-tag v-if="pluginDetail.verifyLevel >= 2" color="green" class="badge-verified">
                <icon-shield-check />
                已验证
              </a-tag>
              <a-tag v-for="tag in pluginDetail.tags?.slice(0, 3)" :key="tag" :color="getTagColor(tag)" class="badge-tag">
                {{ tag }}
              </a-tag>
            </div>
            <h1 class="plugin-name">{{ pluginDetail.name }}</h1>
            <p class="plugin-summary">{{ pluginDetail.summary }}</p>
            <div class="plugin-meta-row">
              <div class="meta-item">
                <icon-star-fill class="star-icon" />
                <span class="meta-value">{{ pluginDetail.rating }}</span>
                <span class="meta-label">评分</span>
              </div>
              <div class="meta-divider"></div>
              <div class="meta-item">
                <icon-download class="download-icon" />
                <span class="meta-value">{{ formatCount(pluginDetail.downloadCount) }}</span>
                <span class="meta-label">下载</span>
              </div>
              <div class="meta-divider"></div>
              <div class="meta-item">
                <icon-user class="user-icon" />
                <span class="meta-value">{{ pluginDetail.developerName }}</span>
                <span class="meta-label">开发者</span>
              </div>
              <div class="meta-divider"></div>
              <div class="meta-item">
                <icon-history class="version-icon" />
                <span class="meta-value">v{{ pluginDetail.version }}</span>
                <span class="meta-label">版本</span>
              </div>
            </div>
          </div>
        </div>
        <div class="plugin-quick-actions">
          <a-button type="outline" class="action-btn">
            <template #icon><icon-heart /></template>
            收藏
          </a-button>
          <a-button type="outline" class="action-btn">
            <template #icon><icon-share-alt /></template>
            分享
          </a-button>
        </div>
      </div>
    </div>

    <div class="snow-inner">
      <a-row :gutter="32">
        <a-col :xs="24" :sm="24" :md="16" :lg="17" class="main-content">
          <a-tabs v-model:active-key="activeTab" class="detail-tabs" type="rounded">
            <a-tab-pane key="basic" title="插件介绍">
              <div class="tab-content" v-if="pluginDetail">
                <div class="content-section">
                  <h3 class="section-title">插件简介</h3>
                  <div class="description" v-html="pluginDetail.description"></div>
                </div>
                
                <div class="content-section" v-if="pluginDetail.screenshots?.length">
                  <h3 class="section-title">
                    <icon-image />
                    截图预览
                  </h3>
                  <div class="screenshots">
                    <a-carousel
                      :arrow-hover="true"
                      :show-tip="true"
                      class="screenshot-carousel"
                      indicator-type="dot"
                      auto-play
                    >
                      <div v-for="(screenshot, index) in pluginDetail.screenshots" :key="index">
                        <div class="screenshot-item">
                          <a-image-preview>
                            <img :src="screenshot" :alt="`截图${index + 1}`" class="screenshot-img" />
                          </a-image-preview>
                        </div>
                      </div>
                    </a-carousel>
                  </div>
                </div>

                <div class="content-section">
                  <h3 class="section-title">
                    <icon-info-circle />
                    基本信息
                  </h3>
                  <a-descriptions :column="2" bordered size="default" class="info-table">
                    <a-descriptions-item label="分类">{{ pluginDetail.categoryName }}</a-descriptions-item>
                    <a-descriptions-item label="最新版本">v{{ pluginDetail.version }}</a-descriptions-item>
                    <a-descriptions-item label="更新时间">{{ pluginDetail.updatedAt }}</a-descriptions-item>
                    <a-descriptions-item label="发布时间">{{ pluginDetail.createdAt }}</a-descriptions-item>
                    <a-descriptions-item label="验证级别">{{ pluginDetail.verifyLevelName }}</a-descriptions-item>
                    <a-descriptions-item label="下载次数">{{ formatCount(pluginDetail.downloadCount) }}</a-descriptions-item>
                  </a-descriptions>
                </div>
              </div>
            </a-tab-pane>

            <a-tab-pane key="plans" title="套餐定价">
              <div class="tab-content">
                <div class="plans-header">
                  <h3 class="section-title">选择适合您的套餐</h3>
                  <p class="section-subtitle">灵活的定价方案，满足不同规模的业务需求</p>
                </div>
                <div class="plan-list" v-if="planList.length > 0">
                  <div
                    v-for="plan in planList"
                    :key="plan.id"
                    class="plan-card"
                    :class="{ 
                      selected: selectedPlanId === plan.id,
                      recommended: plan.recommended
                    }"
                    @click="selectedPlanId = plan.id"
                  >
                    <div class="plan-recommend-badge" v-if="plan.recommended">
                      <icon-crown />
                      推荐
                    </div>
                    <div class="plan-header">
                      <div class="plan-name">{{ plan.name }}</div>
                      <div class="plan-description">{{ plan.description }}</div>
                    </div>
                    <div class="plan-price">
                      <span class="price-currency">¥</span>
                      <span class="price-value">{{ getPlanPrice(plan) }}</span>
                      <span class="price-period">/{{ getPeriodName(plan.periodType) }}</span>
                    </div>
                    <div class="plan-original-price" v-if="plan.originalPrice > plan.price">
                      原价 ¥{{ plan.originalPrice }}
                    </div>
                    <div class="plan-features">
                      <div class="features-title">包含功能</div>
                      <div
                        v-for="feature in plan.features"
                        :key="feature.code"
                        class="feature-item"
                        :class="{ disabled: !feature.included }"
                      >
                        <icon-check-circle-fill v-if="feature.included" class="feature-icon included" />
                        <icon-close-circle-fill v-else class="feature-icon excluded" />
                        <span class="feature-name">{{ feature.name }}</span>
                        <span v-if="feature.limit" class="feature-limit">({{ feature.limit }})</span>
                      </div>
                    </div>
                    <div class="plan-action">
                      <a-button 
                        type="primary" 
                        size="large" 
                        long 
                        :class="{ 'btn-recommended': plan.recommended }"
                        @click.stop="handleSelectPlan(plan)"
                      >
                        选择此套餐
                      </a-button>
                    </div>
                  </div>
                </div>
                <a-empty v-else description="暂无套餐信息" />
              </div>
            </a-tab-pane>

            <a-tab-pane key="tutorial" title="使用教程">
              <div class="tab-content">
                <div class="tutorial-empty">
                  <div class="empty-icon">
                    <icon-book />
                  </div>
                  <h3>使用教程即将上线</h3>
                  <p>我们正在准备详细的使用教程，敬请期待</p>
                </div>
              </div>
            </a-tab-pane>

            <a-tab-pane key="changelog" title="更新日志">
              <div class="tab-content">
                <div class="changelog-header">
                  <h3 class="section-title">版本历史</h3>
                </div>
                <a-timeline class="changelog-timeline">
                  <a-timeline-item v-for="(item, index) in changelogList" :key="index" :color="index === 0 ? 'blue' : 'gray'">
                    <div class="changelog-item">
                      <div class="changelog-version">v{{ item.version }}</div>
                      <div class="changelog-date">{{ item.date }}</div>
                      <div class="changelog-content">{{ item.content }}</div>
                    </div>
                  </a-timeline-item>
                </a-timeline>
              </div>
            </a-tab-pane>

            <a-tab-pane key="reviews" title="用户评价">
              <div class="tab-content">
                <div class="review-summary" v-if="reviewStats">
                  <div class="rating-overview">
                    <div class="rating-score">{{ reviewStats.averageRating }}</div>
                    <div class="rating-stars">
                      <icon-star-fill v-for="i in 5" :key="i" :class="{ active: i <= reviewStats.averageRating }" />
                    </div>
                    <div class="rating-count">{{ reviewStats.totalCount }} 条评价</div>
                  </div>
                  <div class="rating-breakdown">
                    <div v-for="item in reviewStats.breakdown" :key="item.star" class="breakdown-item">
                      <span class="breakdown-label">{{ item.star }}星</span>
                      <a-progress :percent="item.percent" :show-text="false" :stroke-width="8" class="breakdown-progress" />
                      <span class="breakdown-count">{{ item.count }}</span>
                    </div>
                  </div>
                </div>
                <a-divider style="margin: 32px 0" />
                <div class="review-list" v-if="reviewList.length > 0">
                  <div v-for="review in reviewList" :key="review.id" class="review-item">
                    <div class="review-header">
                      <a-avatar :size="48" class="review-avatar">{{ review.userName?.charAt(0) }}</a-avatar>
                      <div class="review-info">
                        <div class="review-user">{{ review.userName }}</div>
                        <div class="review-meta">
                          <div class="review-stars">
                            <icon-star-fill v-for="i in 5" :key="i" :class="{ active: i <= review.rating }" />
                          </div>
                          <span class="review-date">{{ review.createdAt }}</span>
                        </div>
                      </div>
                    </div>
                    <div class="review-content">{{ review.content }}</div>
                  </div>
                </div>
                <a-empty v-else description="暂无评价" />
              </div>
            </a-tab-pane>
          </a-tabs>
        </a-col>

        <a-col :xs="24" :sm="24" :md="8" :lg="7" class="action-sidebar">
          <div class="sidebar-sticky">
            <a-card title="购买插件" :bordered="false" class="action-card">
              <template #extra>
                <a-tooltip content="安全支付保障">
                  <icon-shield-check class="security-icon" />
                </a-tooltip>
              </template>
              
              <div class="selected-plan-info" v-if="selectedPlan">
                <div class="plan-badge" v-if="selectedPlan.recommended">
                  <icon-crown />
                  推荐套餐
                </div>
                <div class="plan-name">{{ selectedPlan.name }}</div>
                <div class="plan-price-display">
                  <span class="current-price">
                    <span class="currency">¥</span>
                    <span class="value">{{ getPlanPrice(selectedPlan) }}</span>
                  </span>
                  <span class="price-period">/{{ getPeriodName(selectedPlan.periodType) }}</span>
                  <span class="original-price" v-if="selectedPlan.originalPrice > selectedPlan.price">
                    ¥{{ selectedPlan.originalPrice }}
                  </span>
                </div>
                <div class="saving-tip" v-if="selectedPlan.originalPrice > selectedPlan.price">
                  <icon-tag />
                  为您节省 ¥{{ selectedPlan.originalPrice - selectedPlan.price }}
                </div>
              </div>

              <div class="period-selector" v-if="planList.length > 0">
                <div class="period-label">选择订阅周期</div>
                <a-radio-group v-model="selectedPeriod" class="period-group" @change="handlePeriodChange">
                  <a-radio value="monthly">月付</a-radio>
                  <a-radio value="quarterly">季付</a-radio>
                  <a-radio value="yearly">年付</a-radio>
                  <a-radio value="permanent">永久</a-radio>
                </a-radio-group>
                <div class="period-discount" v-if="selectedPeriod === 'yearly'">
                  <icon-gift />
                  年付享8折优惠
                </div>
              </div>

              <div class="action-buttons">
                <a-button type="primary" size="large" long class="btn-buy" @click="handleBuyNow">
                  <template #icon><icon-shopping-cart /></template>
                  立即购买
                </a-button>
                <a-button type="outline" size="large" long class="btn-cart" @click="handleAddToCart">
                  <template #icon><icon-plus /></template>
                  加入购物车
                </a-button>
              </div>

              <div class="plugin-stats">
                <div class="stat-item">
                  <div class="stat-icon">
                    <icon-download />
                  </div>
                  <div class="stat-content">
                    <div class="stat-value">{{ formatCount(pluginDetail?.downloadCount) }}</div>
                    <div class="stat-label">下载次数</div>
                  </div>
                </div>
                <div class="stat-item">
                  <div class="stat-icon">
                    <icon-star-fill />
                  </div>
                  <div class="stat-content">
                    <div class="stat-value">{{ pluginDetail?.rating }}</div>
                    <div class="stat-label">用户评分</div>
                  </div>
                </div>
                <div class="stat-item">
                  <div class="stat-icon">
                    <icon-history />
                  </div>
                  <div class="stat-content">
                    <div class="stat-value">v{{ pluginDetail?.version }}</div>
                    <div class="stat-label">当前版本</div>
                  </div>
                </div>
              </div>

              <div class="security-notes">
                <div class="security-note">
                  <icon-safe />
                  <span>资金安全保障</span>
                </div>
                <div class="security-note">
                  <icon-sync />
                  <span>7天无理由退款</span>
                </div>
                <div class="security-note">
                  <icon-customer-service />
                  <span>专业技术支持</span>
                </div>
              </div>
            </a-card>

            <a-card title="开发者信息" :bordered="false" class="developer-card">
              <div class="developer-info">
                <a-avatar :size="64" class="developer-avatar">
                  <img v-if="pluginDetail?.developerLogo" :src="pluginDetail.developerLogo" :alt="pluginDetail.developerName" />
                </a-avatar>
                <div class="developer-name">{{ pluginDetail?.developerName }}</div>
                <div class="developer-bio">专业插件开发者，提供优质插件服务</div>
                <div class="developer-stats">
                  <div class="dev-stat">
                    <span class="dev-stat-value">12</span>
                    <span class="dev-stat-label">插件数</span>
                  </div>
                  <div class="dev-stat-divider"></div>
                  <div class="dev-stat">
                    <span class="dev-stat-value">5.0w</span>
                    <span class="dev-stat-label">总下载</span>
                  </div>
                  <div class="dev-stat-divider"></div>
                  <div class="dev-stat">
                    <span class="dev-stat-value">4.9</span>
                    <span class="dev-stat-label">平均评分</span>
                  </div>
                </div>
              </div>
              <a-space size="12px" style="width: 100%; margin-top: 16px">
                <a-button type="outline" long class="contact-btn">
                  <template #icon><icon-message /></template>
                  联系开发者
                </a-button>
                <a-button type="primary" long class="follow-btn">
                  <template #icon><icon-plus /></template>
                  关注
                </a-button>
              </a-space>
            </a-card>
          </div>
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Message } from "@arco-design/web-vue";
import { market, plan, review } from "@/api/modules/plugin-market/market";

interface PluginDetail {
  id: number;
  code: string;
  name: string;
  categoryId: number;
  categoryName: string;
  developerId: number;
  developerName: string;
  developerLogo?: string;
  summary: string;
  description: string;
  coverImage: string;
  screenshots: string[];
  version: string;
  priceType: number;
  priceTypeName: string;
  verifyLevel: number;
  verifyLevelName: string;
  rating: number;
  downloadCount: number;
  status: number;
  tags: string[];
  isOfficial: number;
  createdAt: string;
  updatedAt: string;
}

interface Plan {
  id: number;
  pluginId: number;
  name: string;
  description: string;
  periodType: number;
  periodTypeName: string;
  periodDays: number;
  price: number;
  originalPrice: number;
  features: Feature[];
  maxDevices: number;
  maxUsers: number;
  storageLimit: number;
  apiCallsLimit: number;
  supportLevel: number;
  supportLevelName: string;
  sort: number;
  status: number;
  recommended?: boolean;
}

interface Feature {
  code: string;
  name: string;
  included: boolean;
  limit?: number;
}

interface Review {
  id: number;
  userId: number;
  userName: string;
  rating: number;
  content: string;
  createdAt: string;
}

interface ReviewStats {
  averageRating: number;
  totalCount: number;
  breakdown: { star: number; count: number; percent: number }[];
}

const route = useRoute();
const router = useRouter();

const activeTab = ref("basic");
const pluginDetail = ref<PluginDetail | null>(null);
const planList = ref<Plan[]>([]);
const reviewList = ref<Review[]>([]);
const reviewStats = ref<ReviewStats | null>(null);
const selectedPlanId = ref<number | null>(null);
const selectedPeriod = ref("monthly");

const changelogList = ref([
  { version: "2.1.0", date: "2024-03-15", content: "新增多种优惠券模板，支持自定义背景图" },
  { version: "2.0.5", date: "2024-02-28", content: "修复已知问题，优化性能" },
  { version: "2.0.0", date: "2024-01-20", content: "全新改版，支持更多营销场景" },
  { version: "1.5.0", date: "2023-12-10", content: "新增满减券功能" }
]);

const selectedPlan = computed(() => {
  return planList.value.find(p => p.id === selectedPlanId.value) || null;
});

const getTagColor = (tag: string) => {
  const colorMap: Record<string, string> = {
    官方: "blue",
    热门: "red",
    稳定: "green",
    AI: "purple",
    智能: "purple",
    免费: "cyan",
    实用: "orange",
    高评分: "pink",
    高性价比: "gold"
  };
  return colorMap[tag] || "default";
};

const getPlanPrice = (plan: Plan) => {
  if (selectedPeriod.value === "permanent") {
    return plan.originalPrice;
  }
  return plan.price;
};

const getPeriodName = (periodType: number) => {
  const periodMap: Record<string, string> = {
    monthly: "月",
    quarterly: "季",
    yearly: "年",
    permanent: "永久"
  };
  return periodMap[selectedPeriod.value] || "月";
};

const formatCount = (count: number) => {
  if (count >= 10000) {
    return `${(count / 10000).toFixed(1)}w`;
  }
  if (count >= 1000) {
    return `${(count / 1000).toFixed(1)}k`;
  }
  return String(count);
};

const handlePeriodChange = () => {};

const handleSelectPlan = (plan: Plan) => {
  selectedPlanId.value = plan.id;
  activeTab.value = "basic";
};

const handleAddToCart = () => {
  if (!selectedPlanId.value) {
    Message.warning("请先选择套餐");
    return;
  }
  Message.success("已加入购物车");
};

const handleBuyNow = () => {
  if (!selectedPlanId.value) {
    Message.warning("请先选择套餐");
    return;
  }
  router.push({ path: "/plugin-market/order", query: { planId: String(selectedPlanId.value) } });
};

const loadPluginDetail = async () => {
  try {
    const id = Number(route.query.id);
    if (!id) return;
    const res = await market.detail(id);
    if (res.code === 200) {
      pluginDetail.value = res.data;
      if (res.data.plans?.length) {
        planList.value = res.data.plans.map((p: Plan, index: number) => ({
          ...p,
          recommended: index === 1
        }));
        selectedPlanId.value = res.data.plans[1]?.id || res.data.plans[0]?.id;
      }
    }
  } catch (error) {
    console.error(error);
  }
};

const loadPlanList = async () => {
  try {
    const id = Number(route.query.id);
    if (!id) return;
    const res = await plan.list(id);
    if (res.code === 200) {
      planList.value = (res.data || []).map((p: Plan, index: number) => ({
        ...p,
        recommended: index === 1
      }));
      if (planList.value.length) {
        selectedPlanId.value = planList.value[1]?.id || planList.value[0]?.id;
      }
    }
  } catch (error) {
    console.error(error);
  }
};

const loadReviewList = async () => {
  try {
    const id = Number(route.query.id);
    if (!id) return;
    const res = await review.list(id);
    if (res.code === 200) {
      reviewList.value = res.data?.list || [];
    }
    const statsRes = await review.stats(id);
    if (statsRes.code === 200) {
      reviewStats.value = statsRes.data;
    }
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadPluginDetail();
  loadPlanList();
  loadReviewList();
});
</script>

<style lang="scss" scoped>
.plugin-market-detail {
  .plugin-header {
    position: relative;
    padding: 48px 24px;
    margin: -24px -24px 32px;
    overflow: hidden;

    .header-bg {
      position: absolute;
      inset: 0;
      background: linear-gradient(135deg, #165dff 0%, #4080ff 50%, #722ed1 100%);
    }

    .header-content {
      position: relative;
      max-width: 1200px;
      margin: 0 auto;
      display: flex;
      align-items: flex-start;
      justify-content: space-between;
      gap: 24px;
    }

    .header-main {
      display: flex;
      gap: 24px;
      flex: 1;
    }

    .plugin-cover-large {
      width: 280px;
      height: 180px;
      border-radius: 16px;
      overflow: hidden;
      flex-shrink: 0;
      box-shadow: 0 12px 32px rgba(0, 0, 0, 0.15);

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
    }

    .plugin-info-main {
      flex: 1;
      color: #fff;
    }

    .plugin-badges {
      display: flex;
      gap: 8px;
      margin-bottom: 12px;
      flex-wrap: wrap;

      .badge-official,
      .badge-verified,
      .badge-tag {
        font-weight: 500;
      }
    }

    .plugin-name {
      font-size: 32px;
      font-weight: 700;
      margin: 0 0 12px;
      letter-spacing: -0.5px;
    }

    .plugin-summary {
      font-size: 15px;
      opacity: 0.9;
      margin: 0 0 20px;
      line-height: 1.6;
      max-width: 600px;
    }

    .plugin-meta-row {
      display: flex;
      align-items: center;
      gap: 16px;
      flex-wrap: wrap;

      .meta-item {
        display: flex;
        align-items: center;
        gap: 6px;
        font-size: 14px;

        .meta-value {
          font-weight: 600;
        }

        .meta-label {
          opacity: 0.8;
        }

        .star-icon {
          color: #ff7d00;
        }

        .download-icon,
        .user-icon,
        .version-icon {
          opacity: 0.8;
        }
      }

      .meta-divider {
        width: 1px;
        height: 16px;
        background: rgba(255, 255, 255, 0.3);
      }
    }

    .plugin-quick-actions {
      display: flex;
      gap: 12px;

      .action-btn {
        background: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.3);
        color: #fff;

        &:hover {
          background: rgba(255, 255, 255, 0.25);
        }
      }
    }
  }

  .main-content {
    .detail-tabs {
      :deep(.arco-tabs-nav) {
        background: #ffffff;
        padding: 0 24px;
        border-radius: 16px 16px 0 0;
        margin-bottom: 0;
      }

      .tab-content {
        background: #ffffff;
        padding: 32px;
        border-radius: 0 0 16px 16px;
        min-height: 400px;
      }
    }

    .content-section {
      margin-bottom: 40px;

      &:last-child {
        margin-bottom: 0;
      }

      .section-title {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 18px;
        font-weight: 600;
        color: #1d2129;
        margin: 0 0 20px;
      }
    }

    .description {
      line-height: 1.8;
      color: #4e5969;
      font-size: 15px;

      :deep(h2) {
        font-size: 18px;
        margin-top: 24px;
        margin-bottom: 12px;
        color: #1d2129;
      }

      :deep(ul) {
        padding-left: 20px;
      }

      :deep(p) {
        margin-bottom: 16px;
      }
    }

    .screenshots {
      .screenshot-carousel {
        border-radius: 12px;
        overflow: hidden;

        :deep(.arco-carousel) {
          border-radius: 12px;
        }
      }

      .screenshot-item {
        display: flex;
        justify-content: center;
        align-items: center;
        background: #f7f8fa;
        padding: 40px;
      }

      .screenshot-img {
        max-width: 100%;
        max-height: 500px;
        border-radius: 8px;
        cursor: pointer;
        box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
      }
    }

    .info-table {
      :deep(.arco-descriptions-view) {
        border-radius: 8px;
      }
    }

    .plans-header {
      text-align: center;
      margin-bottom: 32px;

      .section-title {
        font-size: 24px;
        font-weight: 700;
        color: #1d2129;
        margin: 0 0 8px;
      }

      .section-subtitle {
        font-size: 14px;
        color: #86909c;
        margin: 0;
      }
    }

    .plan-list {
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
      gap: 24px;

      .plan-card {
        position: relative;
        padding: 28px 24px 24px;
        border: 2px solid #e5e6eb;
        border-radius: 16px;
        cursor: pointer;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        background: #ffffff;

        &:hover {
          border-color: #4080ff;
          transform: translateY(-4px);
          box-shadow: 0 12px 32px rgba(22, 93, 255, 0.12);
        }

        &.selected {
          border-color: #165dff;
          background: linear-gradient(135deg, #f0f5ff 0%, #ffffff 100%);
        }

        &.recommended {
          border-color: #ff7d00;
        }
      }

      .plan-recommend-badge {
        position: absolute;
        top: -12px;
        right: 20px;
        display: inline-flex;
        align-items: center;
        gap: 4px;
        padding: 4px 12px;
        background: linear-gradient(135deg, #ff7d00 0%, #ff9a2e 100%);
        color: #fff;
        border-radius: 20px;
        font-size: 12px;
        font-weight: 600;
      }

      .plan-header {
        margin-bottom: 20px;
        text-align: center;

        .plan-name {
          font-size: 20px;
          font-weight: 700;
          color: #1d2129;
          margin-bottom: 6px;
        }

        .plan-description {
          font-size: 13px;
          color: #86909c;
        }
      }

      .plan-price {
        text-align: center;
        margin-bottom: 8px;

        .price-currency {
          font-size: 20px;
          font-weight: 600;
          color: #f53f3f;
        }

        .price-value {
          font-size: 36px;
          font-weight: 700;
          color: #f53f3f;
          letter-spacing: -1px;
        }

        .price-period {
          font-size: 14px;
          color: #86909c;
        }
      }

      .plan-original-price {
        text-align: center;
        font-size: 13px;
        color: #86909c;
        text-decoration: line-through;
        margin-bottom: 20px;
      }

      .plan-features {
        .features-title {
          font-size: 13px;
          font-weight: 600;
          color: #1d2129;
          margin-bottom: 12px;
        }

        .feature-item {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px 0;
          font-size: 14px;
          color: #4e5969;

          &.disabled {
            color: #86909c;
          }

          .feature-icon {
            font-size: 16px;
            flex-shrink: 0;

            &.included {
              color: #23a33c;
            }

            &.excluded {
              color: #c9cdd4;
            }
          }

          .feature-name {
            flex: 1;
          }

          .feature-limit {
            color: #86909c;
            font-size: 13px;
          }
        }
      }

      .plan-action {
        margin-top: 24px;

        .btn-recommended {
          background: linear-gradient(135deg, #ff7d00 0%, #ff9a2e 100%);
          border: none;

          &:hover {
            background: linear-gradient(135deg, #d45d00 0%, #ff7d00 100%);
          }
        }
      }
    }

    .tutorial-empty {
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      padding: 80px 0;

      .empty-icon {
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

    .changelog-header {
      margin-bottom: 24px;
    }

    .changelog-timeline {
      padding-left: 12px;

      .changelog-item {
        .changelog-version {
          font-weight: 600;
          color: #165dff;
          font-size: 15px;
        }

        .changelog-date {
          font-size: 12px;
          color: #86909c;
          margin: 4px 0 8px;
        }

        .changelog-content {
          color: #4e5969;
          font-size: 14px;
        }
      }
    }

    .review-summary {
      display: flex;
      gap: 48px;
      padding: 32px;
      background: linear-gradient(135deg, #f7f8fa 0%, #ffffff 100%);
      border-radius: 16px;

      .rating-overview {
        text-align: center;
        min-width: 160px;

        .rating-score {
          font-size: 56px;
          font-weight: 700;
          color: #ff7d00;
          line-height: 1;
          margin-bottom: 8px;
        }

        .rating-stars {
          color: #d9d9d9;
          font-size: 20px;
          margin-bottom: 8px;

          .active {
            color: #ff7d00;
          }
        }

        .rating-count {
          font-size: 13px;
          color: #86909c;
        }
      }

      .rating-breakdown {
        flex: 1;

        .breakdown-item {
          display: flex;
          align-items: center;
          gap: 12px;
          margin-bottom: 12px;

          &:last-child {
            margin-bottom: 0;
          }

          .breakdown-label {
            font-size: 13px;
            color: #4e5969;
            width: 40px;
          }

          .breakdown-progress {
            flex: 1;
          }

          .breakdown-count {
            font-size: 13px;
            color: #86909c;
            width: 40px;
            text-align: right;
          }
        }
      }
    }

    .review-list {
      .review-item {
        padding: 24px 0;
        border-bottom: 1px solid #e5e6eb;

        &:last-child {
          border-bottom: none;
        }

        .review-header {
          display: flex;
          gap: 16px;
          margin-bottom: 16px;
        }

        .review-avatar {
          flex-shrink: 0;
        }

        .review-info {
          flex: 1;

          .review-user {
            font-weight: 600;
            color: #1d2129;
            font-size: 15px;
            margin-bottom: 6px;
          }

          .review-meta {
            display: flex;
            align-items: center;
            gap: 12px;
          }

          .review-stars {
            color: #d9d9d9;
            font-size: 14px;

            .active {
              color: #ff7d00;
            }
          }

          .review-date {
            font-size: 13px;
            color: #86909c;
          }
        }

        .review-content {
          color: #4e5969;
          line-height: 1.7;
          font-size: 14px;
        }
      }
    }
  }

  .action-sidebar {
    .sidebar-sticky {
      position: sticky;
      top: 24px;
    }

    .action-card {
      margin-bottom: 20px;
      border-radius: 16px;

      :deep(.arco-card-header) {
        padding: 20px 24px;
        border-bottom: 1px solid #f7f8fa;
      }

      .security-icon {
        color: #23a33c;
        font-size: 18px;
      }

      .selected-plan-info {
        text-align: center;
        padding: 20px;
        background: linear-gradient(135deg, #f7f8fa 0%, #ffffff 100%);
        border-radius: 12px;
        margin-bottom: 20px;

        .plan-badge {
          display: inline-flex;
          align-items: center;
          gap: 4px;
          padding: 4px 12px;
          background: linear-gradient(135deg, #ff7d00 0%, #ff9a2e 100%);
          color: #fff;
          border-radius: 20px;
          font-size: 11px;
          font-weight: 600;
          margin-bottom: 12px;
        }

        .plan-name {
          font-size: 16px;
          font-weight: 600;
          color: #1d2129;
          margin-bottom: 12px;
        }

        .plan-price-display {
          margin-bottom: 8px;

          .current-price {
            .currency {
              font-size: 18px;
              font-weight: 600;
              color: #f53f3f;
            }

            .value {
              font-size: 32px;
              font-weight: 700;
              color: #f53f3f;
              letter-spacing: -1px;
            }
          }

          .price-period {
            font-size: 14px;
            color: #86909c;
            margin-left: 4px;
          }

          .original-price {
            font-size: 14px;
            color: #86909c;
            text-decoration: line-through;
            margin-left: 8px;
          }
        }

        .saving-tip {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 4px;
          font-size: 13px;
          color: #f53f3f;
          padding: 6px 12px;
          background: #fff1f0;
          border-radius: 6px;
        }
      }

      .period-selector {
        margin-bottom: 20px;

        .period-label {
          font-size: 14px;
          color: #1d2129;
          font-weight: 500;
          margin-bottom: 12px;
        }

        .period-group {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;

          :deep(.arco-radio) {
            flex: 1;
            min-width: 70px;
            margin-right: 0;
            text-align: center;
            padding: 8px 12px;
            border: 1px solid #e5e6eb;
            border-radius: 8px;
            transition: all 0.2s ease;

            &:hover {
              border-color: #4080ff;
            }

            &.arco-radio-checked {
              border-color: #165dff;
              background: #f0f5ff;
            }
          }
        }

        .period-discount {
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 4px;
          margin-top: 12px;
          padding: 8px 12px;
          background: #fff7e8;
          color: #ff7d00;
          border-radius: 8px;
          font-size: 13px;
          font-weight: 500;
        }
      }

      .action-buttons {
        display: flex;
        flex-direction: column;
        gap: 12px;
        margin-bottom: 24px;

        .btn-buy {
          background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
          border: none;
          font-weight: 600;

          &:hover {
            background: linear-gradient(135deg, #0e42d2 0%, #165dff 100%);
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(22, 93, 255, 0.3);
          }
        }

        .btn-cart {
          font-weight: 500;
        }
      }

      .plugin-stats {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 16px;
        padding-top: 20px;
        border-top: 1px solid #e5e6eb;

        .stat-item {
          display: flex;
          flex-direction: column;
          align-items: center;
          text-align: center;

          .stat-icon {
            width: 40px;
            height: 40px;
            border-radius: 10px;
            background: #f7f8fa;
            display: flex;
            align-items: center;
            justify-content: center;
            margin-bottom: 8px;
            color: #165dff;
          }

          .stat-value {
            font-size: 16px;
            font-weight: 700;
            color: #1d2129;
          }

          .stat-label {
            font-size: 12px;
            color: #86909c;
          }
        }
      }

      .security-notes {
        margin-top: 20px;
        padding-top: 20px;
        border-top: 1px solid #e5e6eb;

        .security-note {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 8px 0;
          font-size: 13px;
          color: #4e5969;

          svg {
            color: #23a33c;
          }
        }
      }
    }

    .developer-card {
      border-radius: 16px;

      :deep(.arco-card-header) {
        padding: 20px 24px;
        border-bottom: 1px solid #f7f8fa;
      }

      .developer-info {
        text-align: center;
        padding: 8px 0;

        .developer-avatar {
          margin-bottom: 16px;
        }

        .developer-name {
          font-size: 18px;
          font-weight: 600;
          color: #1d2129;
          margin-bottom: 6px;
        }

        .developer-bio {
          font-size: 13px;
          color: #86909c;
          margin-bottom: 16px;
        }

        .developer-stats {
          display: flex;
          align-items: center;
          justify-content: center;
          padding: 16px 0;
          background: #f7f8fa;
          border-radius: 12px;

          .dev-stat {
            text-align: center;

            .dev-stat-value {
              display: block;
              font-size: 18px;
              font-weight: 700;
              color: #1d2129;
            }

            .dev-stat-label {
              display: block;
              font-size: 12px;
              color: #86909c;
              margin-top: 2px;
            }
          }

          .dev-stat-divider {
            width: 1px;
            height: 32px;
            background: #e5e6eb;
            margin: 0 20px;
          }
        }
      }

      .contact-btn,
      .follow-btn {
        margin-top: 8px;
      }
    }
  }
}
</style>

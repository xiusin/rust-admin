<template>
  <div class="snow-page plugin-market-detail">
    <div class="plugin-header" v-if="pluginDetail">
      <div class="header-cover">
        <img :src="pluginDetail.coverImage" :alt="pluginDetail.name" />
      </div>
      <div class="header-info">
        <div class="plugin-tags" v-if="pluginDetail.tags?.length">
          <a-tag v-for="tag in pluginDetail.tags" :key="tag" :color="getTagColor(tag)">
            {{ tag }}
          </a-tag>
        </div>
        <h1 class="plugin-name">{{ pluginDetail.name }}</h1>
        <p class="plugin-summary">{{ pluginDetail.summary }}</p>
        <div class="plugin-meta">
          <div class="meta-item">
            <icon-star-fill class="star-icon" />
            <span>{{ pluginDetail.rating }}</span>
          </div>
          <div class="meta-item">
            <icon-download />
            <span>{{ pluginDetail.downloadCount }} 下载</span>
          </div>
          <div class="meta-item">
            <icon-user />
            <span>{{ pluginDetail.developerName }}</span>
          </div>
          <div class="meta-item">
            <icon-history />
            <span>v{{ pluginDetail.version }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="snow-inner">
      <a-row :gutter="24">
        <a-col :xs="24" :sm="24" :md="16" :lg="17">
          <a-tabs v-model:active-key="activeTab" class="detail-tabs">
            <a-tab-pane key="basic" title="基本信息">
              <div class="tab-content" v-if="pluginDetail">
                <div class="description" v-html="pluginDetail.description"></div>
                <div class="screenshots" v-if="pluginDetail.screenshots?.length">
                  <h3>截图预览</h3>
                  <a-row :gutter="16">
                    <a-col
                      v-for="(screenshot, index) in pluginDetail.screenshots"
                      :key="index"
                      :xs="24"
                      :sm="12"
                      :md="8"
                    >
                      <a-image-preview>
                        <img :src="screenshot" :alt="`截图${index + 1}`" class="screenshot-img" />
                      </a-image-preview>
                    </a-col>
                  </a-row>
                </div>
              </div>
            </a-tab-pane>

            <a-tab-pane key="plans" title="套餐定价">
              <div class="tab-content">
                <div class="plan-list" v-if="planList.length > 0">
                  <div
                    v-for="plan in planList"
                    :key="plan.id"
                    class="plan-card"
                    :class="{ selected: selectedPlanId === plan.id }"
                    @click="selectedPlanId = plan.id"
                  >
                    <div class="plan-header">
                      <div class="plan-name">{{ plan.name }}</div>
                      <div class="plan-description">{{ plan.description }}</div>
                    </div>
                    <div class="plan-price">
                      <span class="price-value">¥{{ getPlanPrice(plan) }}</span>
                      <span class="price-period">/{{ getPeriodName(plan.periodType) }}</span>
                    </div>
                    <div class="plan-features">
                      <div
                        v-for="feature in plan.features"
                        :key="feature.code"
                        class="feature-item"
                        :class="{ disabled: !feature.included }"
                      >
                        <icon-check-circle-fill v-if="feature.included" class="feature-icon included" />
                        <icon-close-circle-fill v-else class="feature-icon excluded" />
                        <span>{{ feature.name }}</span>
                        <span v-if="feature.limit" class="feature-limit">({{ feature.limit }})</span>
                      </div>
                    </div>
                    <div class="plan-select-indicator" v-if="selectedPlanId === plan.id">
                      <icon-check-circle-fill />
                    </div>
                  </div>
                </div>
                <a-empty v-else description="暂无套餐信息" />
              </div>
            </a-tab-pane>

            <a-tab-pane key="tutorial" title="使用教程">
              <div class="tab-content">
                <a-empty description="暂无使用教程" />
              </div>
            </a-tab-pane>

            <a-tab-pane key="changelog" title="更新日志">
              <div class="tab-content">
                <a-timeline>
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
                      <span>{{ item.star }}星</span>
                      <a-progress :percent="item.percent" :show-text="false" :stroke-width="6" />
                      <span>{{ item.count }}</span>
                    </div>
                  </div>
                </div>
                <a-divider />
                <div class="review-list" v-if="reviewList.length > 0">
                  <div v-for="review in reviewList" :key="review.id" class="review-item">
                    <div class="review-header">
                      <a-avatar :size="36">{{ review.userName?.charAt(0) }}</a-avatar>
                      <div class="review-info">
                        <div class="review-user">{{ review.userName }}</div>
                        <div class="review-meta">
                          <icon-star-fill class="star-icon" />
                          <span>{{ review.rating }}</span>
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
          <a-card title="立即购买" :bordered="false" class="action-card">
            <div class="selected-plan-info" v-if="selectedPlan">
              <div class="plan-name">{{ selectedPlan.name }}</div>
              <div class="plan-price">
                <span class="current-price">¥{{ getPlanPrice(selectedPlan) }}</span>
                <span class="original-price" v-if="selectedPlan.originalPrice > selectedPlan.price">
                  ¥{{ selectedPlan.originalPrice }}
                </span>
              </div>
            </div>

            <div class="period-selector" v-if="planList.length > 0">
              <div class="period-label">选择订阅周期</div>
              <a-radio-group v-model="selectedPeriod" @change="handlePeriodChange">
                <a-radio value="monthly">月付</a-radio>
                <a-radio value="quarterly">季付</a-radio>
                <a-radio value="yearly">年付</a-radio>
                <a-radio value="permanent">永久</a-radio>
              </a-radio-group>
            </div>

            <div class="action-buttons">
              <a-button type="primary" size="large" long @click="handleAddToCart">
                <template #icon><icon-shopping-cart /></template>
                加入购物车
              </a-button>
              <a-button type="outline" size="large" long @click="handleBuyNow">
                立即购买
              </a-button>
            </div>

            <div class="plugin-stats">
              <div class="stat-item">
                <div class="stat-label">下载次数</div>
                <div class="stat-value">{{ pluginDetail?.downloadCount }}</div>
              </div>
              <div class="stat-item">
                <div class="stat-label">评分</div>
                <div class="stat-value">{{ pluginDetail?.rating }}</div>
              </div>
              <div class="stat-item">
                <div class="stat-label">版本</div>
                <div class="stat-value">v{{ pluginDetail?.version }}</div>
              </div>
            </div>
          </a-card>

          <a-card title="开发者信息" :bordered="false" class="developer-card">
            <div class="developer-info">
              <a-avatar :size="48" class="developer-avatar">
                <img v-if="pluginDetail?.developerLogo" :src="pluginDetail.developerLogo" :alt="pluginDetail.developerName" />
              </a-avatar>
              <div class="developer-name">{{ pluginDetail?.developerName }}</div>
              <div class="developer-bio">专业插件开发者，提供优质插件服务</div>
            </div>
            <a-button type="outline" long class="contact-btn">
              <template #icon><icon-message /></template>
              联系开发者
            </a-button>
          </a-card>
        </a-col>
      </a-row>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { Message } from '@arco-design/web-vue';
import { market, plan, review } from '@/api/modules/plugin-market/market';

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

const activeTab = ref('basic');
const pluginDetail = ref<PluginDetail | null>(null);
const planList = ref<Plan[]>([]);
const reviewList = ref<Review[]>([]);
const reviewStats = ref<ReviewStats | null>(null);
const selectedPlanId = ref<number | null>(null);
const selectedPeriod = ref('monthly');

const changelogList = ref([
  { version: '2.1.0', date: '2024-03-15', content: '新增多种优惠券模板，支持自定义背景图' },
  { version: '2.0.5', date: '2024-02-28', content: '修复已知问题，优化性能' },
  { version: '2.0.0', date: '2024-01-20', content: '全新改版，支持更多营销场景' },
  { version: '1.5.0', date: '2023-12-10', content: '新增满减券功能' },
]);

const selectedPlan = computed(() => {
  return planList.value.find(p => p.id === selectedPlanId.value) || null;
});

const getTagColor = (tag: string) => {
  const colorMap: Record<string, string> = {
    '官方': 'blue',
    '热门': 'red',
    '稳定': 'green',
    'AI': 'purple',
    '智能': 'purple',
    '免费': 'cyan',
    '实用': 'orange',
    '高评分': 'pink',
    '高性价比': 'gold',
  };
  return colorMap[tag] || 'default';
};

const getPlanPrice = (plan: Plan) => {
  if (selectedPeriod.value === 'permanent') {
    return plan.originalPrice;
  }
  return plan.price;
};

const getPeriodName = (periodType: number) => {
  const periodMap: Record<string, string> = {
    'monthly': '月',
    'quarterly': '季',
    'yearly': '年',
    'permanent': '永久',
  };
  return periodMap[selectedPeriod.value] || '月';
};

const handlePeriodChange = () => {
};

const handleAddToCart = () => {
  if (!selectedPlanId.value) {
    Message.warning('请选择套餐');
    return;
  }
  Message.success('已加入购物车');
};

const handleBuyNow = () => {
  if (!selectedPlanId.value) {
    Message.warning('请选择套餐');
    return;
  }
  router.push({ path: '/plugin-market/order', query: { planId: String(selectedPlanId.value) } });
};

const loadPluginDetail = async () => {
  try {
    const id = Number(route.query.id);
    if (!id) return;
    const res = await market.detail(id);
    if (res.code === 200) {
      pluginDetail.value = res.data;
      if (res.data.plans?.length) {
        planList.value = res.data.plans;
        selectedPlanId.value = res.data.plans[0].id;
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
      planList.value = res.data || [];
      if (res.data?.length) {
        selectedPlanId.value = res.data[0].id;
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
    display: flex;
    gap: 24px;
    padding: 24px;
    background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
    margin: -24px -24px 24px;

    .header-cover {
      width: 280px;
      height: 180px;
      border-radius: 12px;
      overflow: hidden;
      flex-shrink: 0;

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }
    }

    .header-info {
      flex: 1;
      color: #fff;

      .plugin-tags {
        margin-bottom: 8px;

        :deep(.arco-tag) {
          background: rgba(255, 255, 255, 0.2);
          border: none;
          color: #fff;
        }
      }

      .plugin-name {
        font-size: 28px;
        font-weight: 600;
        margin: 0 0 8px;
      }

      .plugin-summary {
        font-size: 14px;
        opacity: 0.9;
        margin: 0 0 16px;
      }

      .plugin-meta {
        display: flex;
        gap: 24px;

        .meta-item {
          display: flex;
          align-items: center;
          gap: 6px;
          font-size: 14px;

          .star-icon {
            color: #ff7d00;
          }
        }
      }
    }
  }

  .detail-tabs {
    :deep(.arco-tabs-nav) {
      background: #fff;
      padding: 0 16px;
      border-radius: 8px 8px 0 0;
    }

    .tab-content {
      background: #fff;
      padding: 24px;
      border-radius: 0 0 8px 8px;
      min-height: 400px;

      .description {
        line-height: 1.8;
        color: #4e5969;

        :deep(h2) {
          font-size: 18px;
          margin-top: 24px;
        }

        :deep(ul) {
          padding-left: 20px;
        }
      }

      .screenshots {
        margin-top: 32px;

        h3 {
          font-size: 16px;
          margin-bottom: 16px;
        }

        .screenshot-img {
          width: 100%;
          border-radius: 8px;
          cursor: pointer;
          transition: transform 0.2s;

          &:hover {
            transform: scale(1.02);
          }
        }
      }

      .changelog-item {
        .changelog-version {
          font-weight: 600;
          color: #165dff;
        }

        .changelog-date {
          font-size: 12px;
          color: #86909c;
          margin: 4px 0;
        }

        .changelog-content {
          color: #4e5969;
        }
      }

      .review-summary {
        display: flex;
        gap: 32px;
        padding: 20px;
        background: #f7f8fa;
        border-radius: 8px;

        .rating-overview {
          text-align: center;

          .rating-score {
            font-size: 48px;
            font-weight: 600;
            color: #ff7d00;
          }

          .rating-stars {
            color: #d9d9d9;
            font-size: 16px;

            .active {
              color: #ff7d00;
            }
          }

          .rating-count {
            font-size: 12px;
            color: #86909c;
            margin-top: 4px;
          }
        }

        .rating-breakdown {
          flex: 1;

          .breakdown-item {
            display: flex;
            align-items: center;
            gap: 8px;
            margin-bottom: 8px;
            font-size: 12px;
            color: #86909c;
          }
        }
      }

      .review-item {
        padding: 16px 0;
        border-bottom: 1px solid #e5e6eb;

        &:last-child {
          border-bottom: none;
        }

        .review-header {
          display: flex;
          gap: 12px;
          margin-bottom: 12px;

          .review-info {
            flex: 1;

            .review-user {
              font-weight: 500;
              color: #1d2129;
            }

            .review-meta {
              display: flex;
              align-items: center;
              gap: 8px;
              font-size: 12px;
              color: #86909c;
              margin-top: 4px;

              .star-icon {
                color: #ff7d00;
              }
            }
          }
        }

        .review-content {
          color: #4e5969;
          line-height: 1.6;
        }
      }
    }
  }

  .action-sidebar {
    .action-card {
      margin-bottom: 16px;

      .selected-plan-info {
        text-align: center;
        padding: 16px;
        background: #f7f8fa;
        border-radius: 8px;
        margin-bottom: 16px;

        .plan-name {
          font-size: 16px;
          font-weight: 500;
          color: #1d2129;
          margin-bottom: 8px;
        }

        .plan-price {
          .current-price {
            font-size: 28px;
            font-weight: 600;
            color: #f53f3f;
          }

          .original-price {
            font-size: 14px;
            color: #86909c;
            text-decoration: line-through;
            margin-left: 8px;
          }
        }
      }

      .period-selector {
        margin-bottom: 16px;

        .period-label {
          font-size: 14px;
          color: #4e5969;
          margin-bottom: 12px;
        }

        :deep(.arco-radio-group) {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;

          .arco-radio {
            margin-right: 0;
          }
        }
      }

      .action-buttons {
        display: flex;
        flex-direction: column;
        gap: 12px;
        margin-bottom: 24px;
      }

      .plugin-stats {
        display: flex;
        justify-content: space-between;
        padding-top: 16px;
        border-top: 1px solid #e5e6eb;

        .stat-item {
          text-align: center;

          .stat-label {
            font-size: 12px;
            color: #86909c;
          }

          .stat-value {
            font-size: 16px;
            font-weight: 600;
            color: #1d2129;
          }
        }
      }
    }

    .developer-card {
      .developer-info {
        text-align: center;
        padding: 16px 0;

        .developer-avatar {
          margin-bottom: 12px;
        }

        .developer-name {
          font-size: 16px;
          font-weight: 500;
          color: #1d2129;
          margin-bottom: 4px;
        }

        .developer-bio {
          font-size: 12px;
          color: #86909c;
        }
      }

      .contact-btn {
        margin-top: 12px;
      }
    }
  }

  .plan-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 16px;

    .plan-card {
      position: relative;
      padding: 20px;
      border: 2px solid #e5e6eb;
      border-radius: 12px;
      cursor: pointer;
      transition: all 0.3s;

      &:hover {
        border-color: #165dff;
      }

      &.selected {
        border-color: #165dff;
        background: #f0f5ff;
      }

      .plan-header {
        margin-bottom: 16px;

        .plan-name {
          font-size: 18px;
          font-weight: 600;
          color: #1d2129;
          margin-bottom: 4px;
        }

        .plan-description {
          font-size: 13px;
          color: #86909c;
        }
      }

      .plan-price {
        margin-bottom: 16px;

        .price-value {
          font-size: 24px;
          font-weight: 600;
          color: #f53f3f;
        }

        .price-period {
          font-size: 14px;
          color: #86909c;
        }
      }

      .plan-features {
        .feature-item {
          display: flex;
          align-items: center;
          gap: 8px;
          padding: 6px 0;
          font-size: 13px;
          color: #4e5969;

          &.disabled {
            color: #86909c;
          }

          .feature-icon {
            font-size: 16px;

            &.included {
              color: #00b42a;
            }

            &.excluded {
              color: #d9d9d9;
            }
          }

          .feature-limit {
            color: #86909c;
          }
        }
      }

      .plan-select-indicator {
        position: absolute;
        top: 12px;
        right: 12px;
        font-size: 20px;
        color: #165dff;
      }
    }
  }
}
</style>

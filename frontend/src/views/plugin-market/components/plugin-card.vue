<template>
  <a-card hoverable class="plugin-card" @click="$emit('click')">
    <div class="plugin-card-inner">
      <div class="plugin-cover">
        <div class="cover-overlay"></div>
        <img :src="plugin.coverImage" :alt="plugin.name" />
        <div class="plugin-badges">
          <template v-if="plugin.isOfficial">
            <div class="badge official">
              <icon-star-fill />
              官方
            </div>
          </template>
          <template v-if="plugin.verifyLevel >= 2">
            <div class="badge verified">
              <icon-shield-check />
              已验证
            </div>
          </template>
        </div>
        <div class="plugin-tags" v-if="plugin.tags?.length">
          <a-tag v-for="tag in plugin.tags.slice(0, 2)" :key="tag" size="small" :color="getTagColor(tag)">
            {{ tag }}
          </a-tag>
        </div>
      </div>
      
      <div class="plugin-info">
        <div class="plugin-header">
          <div class="plugin-name">{{ plugin.name }}</div>
          <div class="plugin-category">{{ plugin.categoryName }}</div>
        </div>
        <div class="plugin-summary">{{ plugin.summary }}</div>
        
        <div class="plugin-meta">
          <div class="meta-left">
            <div class="meta-item rating">
              <icon-star-fill />
              <span>{{ plugin.rating }}</span>
            </div>
            <div class="meta-divider"></div>
            <div class="meta-item download">
              <icon-download />
              <span>{{ formatCount(plugin.downloadCount) }}</span>
            </div>
          </div>
          <div class="meta-right">
            <div class="plugin-price" v-if="plugin.priceType !== 0">
              <span class="price-value">¥{{ plugin.price }}</span>
            </div>
            <div class="plugin-price free" v-else>
              <span class="price-value">免费</span>
            </div>
          </div>
        </div>
        
        <div class="plugin-action">
          <a-button type="primary" size="small" class="action-btn">
            <icon-eye />
            查看详情
          </a-button>
        </div>
      </div>
    </div>
  </a-card>
</template>

<script setup lang="ts">
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

defineProps<{
  plugin: Plugin;
}>();

defineEmits<{
  (e: "click"): void;
}>();

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

const formatCount = (count: number) => {
  if (count >= 10000) {
    return `${(count / 10000).toFixed(1)}w`;
  }
  if (count >= 1000) {
    return `${(count / 1000).toFixed(1)}k`;
  }
  return String(count);
};
</script>

<style lang="scss" scoped>
.plugin-card {
  height: 100%;
  overflow: hidden;
  border-radius: 16px;
  border: 1px solid transparent;
  background: #ffffff;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);

  &:hover {
    transform: translateY(-8px);
    box-shadow: 0 20px 40px -12px rgba(22, 93, 255, 0.15), 0 8px 24px -8px rgba(0, 0, 0, 0.08);
    border-color: #d6e4ff;
  }

  :deep(.arco-card-body) {
    padding: 0;
    height: 100%;
  }

  .plugin-card-inner {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .plugin-cover {
    position: relative;
    width: 100%;
    height: 180px;
    overflow: hidden;
    border-radius: 12px 12px 0 0;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      transition: transform 0.5s ease;
    }

    .cover-overlay {
      position: absolute;
      inset: 0;
      background: linear-gradient(to top, rgba(0, 0, 0, 0.3) 0%, transparent 50%;
      z-index: 1;
      opacity: 0;
      transition: opacity 0.3s ease;
    }
  }

  &:hover .plugin-cover {
    img {
      transform: scale(1.08);
    }

    .cover-overlay {
      opacity: 1;
    }
  }

  .plugin-badges {
    position: absolute;
    top: 12px;
    left: 12px;
    display: flex;
    gap: 8px;
    z-index: 2;

    .badge {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      padding: 4px 10px;
      border-radius: 20px;
      font-size: 11px;
      font-weight: 600;
      backdrop-filter: blur(8px);

      &.official {
        background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
        color: #ffffff;
      }

      &.verified {
        background: linear-gradient(135deg, #23a33c 0%, #49c75e 100%);
        color: #ffffff;
      }
    }
  }

  .plugin-tags {
    position: absolute;
    bottom: 12px;
    left: 12px;
    display: flex;
    gap: 6px;
    z-index: 2;

    :deep(.arco-tag) {
      background: rgba(255, 255, 255, 0.95);
      backdrop-filter: blur(8px);
      border: none;
      font-weight: 500;
    }
  }

  .plugin-info {
    padding: 20px;
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .plugin-header {
    margin-bottom: 12px;
  }

  .plugin-name {
    font-size: 17px;
    font-weight: 600;
    color: #1d2129;
    margin-bottom: 6px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    line-height: 1.3;
  }

  .plugin-category {
    font-size: 12px;
    color: #4080ff;
    font-weight: 500;
    display: inline-block;
    padding: 2px 8px;
    background: #f0f5ff;
    border-radius: 4px;
  }

  .plugin-summary {
    font-size: 13px;
    color: #86909c;
    line-height: 1.6;
    height: 42px;
    overflow: hidden;
    text-overflow: ellipsis;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    margin-bottom: 16px;
    flex: 1;
  }

  .plugin-meta {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 0;
    border-top: 1px solid #f7f8fa;
    margin-bottom: 12px;

    .meta-left {
      display: flex;
      align-items: center;
      gap: 8px;
    }

    .meta-item {
      display: flex;
      align-items: center;
      gap: 4px;
      font-size: 13px;
      color: #86909c;

      &.rating {
        color: #ff7d00;
        font-weight: 500;
      }

      &.download {
        color: #4e5969;
      }
    }

    .meta-divider {
      width: 1px;
      height: 12px;
      background: #e5e6eb;
    }

    .meta-right {
      display: flex;
      align-items: center;
    }
  }

  .plugin-price {
    display: flex;
    align-items: baseline;
    gap: 4px;

    .price-value {
      font-size: 18px;
      font-weight: 700;
      color: #f53f3f;
      letter-spacing: -0.5px;
    }
  }

  .plugin-price.free {
    .price-value {
      color: #23a33c;
      font-size: 16px;
    }
  }

  .plugin-action {
    .action-btn {
      width: 100%;
      border-radius: 8px;
      font-weight: 500;
      background: linear-gradient(135deg, #165dff 0%, #4080ff 100%);
      border: none;
      transition: all 0.3s ease;

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 8px 16px rgba(22, 93, 255, 0.3);
      }
    }
  }
}
</style>

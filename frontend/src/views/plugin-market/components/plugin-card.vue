<template>
  <a-card hoverable class="plugin-card" @click="$emit('click')">
    <div class="plugin-cover">
      <img :src="plugin.coverImage" :alt="plugin.name" />
      <div class="plugin-tags" v-if="plugin.tags?.length">
        <a-tag v-for="tag in plugin.tags.slice(0, 2)" :key="tag" size="small" :color="getTagColor(tag)">
          {{ tag }}
        </a-tag>
      </div>
      <div class="plugin-price" v-if="plugin.priceType !== 0">
        <span class="price-value">¥{{ plugin.price }}</span>
        <span class="price-type">{{ plugin.priceTypeName }}</span>
      </div>
      <div class="plugin-price free" v-else>
        <span class="price-value">免费</span>
      </div>
    </div>
    <div class="plugin-info">
      <div class="plugin-name">{{ plugin.name }}</div>
      <div class="plugin-summary">{{ plugin.summary }}</div>
      <div class="plugin-meta">
        <div class="plugin-developer">
          <icon-user />
          <span>{{ plugin.developerName }}</span>
        </div>
        <div class="plugin-rating">
          <icon-star-fill />
          <span>{{ plugin.rating }}</span>
        </div>
        <div class="plugin-download">
          <icon-download />
          <span>{{ formatCount(plugin.downloadCount) }}</span>
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
  (e: 'click'): void;
}>();

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
  border-radius: 8px;
  transition: all 0.3s;

  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }

  :deep(.arco-card-body) {
    padding: 0;
  }

  .plugin-cover {
    position: relative;
    width: 100%;
    height: 160px;
    overflow: hidden;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }

    .plugin-tags {
      position: absolute;
      top: 8px;
      left: 8px;
      display: flex;
      gap: 4px;
    }

    .plugin-price {
      position: absolute;
      bottom: 8px;
      right: 8px;
      background: rgba(0, 0, 0, 0.7);
      padding: 4px 8px;
      border-radius: 4px;
      color: #fff;

      .price-value {
        font-size: 14px;
        font-weight: 600;
      }

      .price-type {
        font-size: 12px;
        margin-left: 4px;
      }

      &.free {
        background: #00b42a;
      }
    }
  }

  .plugin-info {
    padding: 16px;

    .plugin-name {
      font-size: 16px;
      font-weight: 600;
      color: #1d2129;
      margin-bottom: 8px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .plugin-summary {
      font-size: 13px;
      color: #86909c;
      line-height: 1.5;
      height: 39px;
      overflow: hidden;
      text-overflow: ellipsis;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      margin-bottom: 12px;
    }

    .plugin-meta {
      display: flex;
      align-items: center;
      justify-content: space-between;
      font-size: 12px;
      color: #86909c;

      > div {
        display: flex;
        align-items: center;
        gap: 4px;
      }

      .plugin-rating {
        color: #ff7d00;
      }
    }
  }
}
</style>

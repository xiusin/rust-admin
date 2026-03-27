<template>
  <div class="snow-page">
    <div class="snow-inner content-detail">
      <div class="detail-header">
        <div class="header-left">
          <a-button type="text" @click="goBack">
            <template #icon><icon-left /></template>
            返回列表
          </a-button>
          <a-divider direction="vertical" />
          <span class="detail-title">内容详情</span>
        </div>
        <div class="header-right">
          <a-space>
            <a-button @click="onEdit">
              <template #icon><icon-edit /></template>
              编辑
            </a-button>
            <VersionHistory :content-id="contentId" @rollback="onRollback" />
          </a-space>
        </div>
      </div>

      <div class="detail-content">
        <a-spin :loading="loading" class="detail-spin">
          <a-row :gutter="24">
            <a-col :span="16">
              <a-card title="基本信息">
                <a-descriptions :column="2" bordered>
                  <a-descriptions-item label="标题" :span="2">{{ contentDetail?.title }}</a-descriptions-item>
                  <a-descriptions-item label="别名">{{ contentDetail?.slug || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="状态">
                    <a-tag :color="getStatusColor(contentDetail?.status)">
                      {{ getStatusText(contentDetail?.status) }}
                    </a-tag>
                  </a-descriptions-item>
                  <a-descriptions-item label="分类">{{ contentDetail?.categoryName || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="来源">{{ contentDetail?.source || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="关键词" :span="2">{{ contentDetail?.keywords || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="摘要" :span="2">{{ contentDetail?.description || '-' }}</a-descriptions-item>
                </a-descriptions>
              </a-card>

              <a-card title="内容" class="content-card">
                <div class="content-body" v-html="contentDetail?.content || '暂无内容'"></div>
              </a-card>
            </a-col>
            <a-col :span="8">
              <a-card title="发布信息">
                <a-descriptions :column="1" bordered>
                  <a-descriptions-item label="作者">{{ contentDetail?.authorName || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="发布时间">{{ contentDetail?.publishTime || '-' }}</a-descriptions-item>
                  <a-descriptions-item label="创建时间">{{ contentDetail?.createdAt }}</a-descriptions-item>
                  <a-descriptions-item label="更新时间">{{ contentDetail?.updatedAt }}</a-descriptions-item>
                </a-descriptions>
              </a-card>

              <a-card title="统计数据" class="stats-card">
                <a-row :gutter="16">
                  <a-col :span="8">
                    <a-statistic title="浏览量" :value="contentDetail?.viewCount || 0" />
                  </a-col>
                  <a-col :span="8">
                    <a-statistic title="点赞数" :value="contentDetail?.likeCount || 0" />
                  </a-col>
                  <a-col :span="8">
                    <a-statistic title="评论数" :value="contentDetail?.commentCount || 0" />
                  </a-col>
                </a-row>
              </a-card>

              <a-card title="属性设置" class="settings-card">
                <a-descriptions :column="1" bordered>
                  <a-descriptions-item label="置顶">
                    <a-tag :color="contentDetail?.isTop ? 'red' : 'gray'">{{ contentDetail?.isTop ? '是' : '否' }}</a-tag>
                  </a-descriptions-item>
                  <a-descriptions-item label="推荐">
                    <a-tag :color="contentDetail?.isRecommend ? 'orange' : 'gray'">{{ contentDetail?.isRecommend ? '是' : '否' }}</a-tag>
                  </a-descriptions-item>
                  <a-descriptions-item label="热门">
                    <a-tag :color="contentDetail?.isHot ? 'arcoblue' : 'gray'">{{ contentDetail?.isHot ? '是' : '否' }}</a-tag>
                  </a-descriptions-item>
                  <a-descriptions-item label="允许评论">
                    <a-tag :color="contentDetail?.allowComment ? 'green' : 'gray'">{{ contentDetail?.allowComment ? '是' : '否' }}</a-tag>
                  </a-descriptions-item>
                  <a-descriptions-item label="排序">{{ contentDetail?.sort }}</a-descriptions-item>
                </a-descriptions>
              </a-card>

              <a-card title="标签" class="tags-card">
                <a-space wrap>
                  <a-tag v-for="tag in contentDetail?.tags" :key="tag.id" :color="tag.color || 'arcoblue'">
                    {{ tag.name }}
                  </a-tag>
                  <span v-if="!contentDetail?.tags?.length">暂无标签</span>
                </a-space>
              </a-card>

              <a-card v-if="contentDetail?.thumbnail" title="缩略图">
                <a-image :src="contentDetail.thumbnail" fit="cover" />
              </a-card>
            </a-col>
          </a-row>
        </a-spin>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { contentApi, type CmsContentDetail, type ContentStatus } from '@/api/modules/cms/content';
import VersionHistory from './components/VersionHistory.vue';

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const contentDetail = ref<CmsContentDetail>();

const contentId = computed(() => Number(route.query.id));

const getStatusColor = (status?: ContentStatus) => {
  if (!status) return 'gray';
  const colors: Record<ContentStatus, string> = {
    draft: 'gray',
    pending: 'orange',
    published: 'green',
    offline: 'gray',
    rejected: 'red',
    recycled: 'gray',
  };
  return colors[status] || 'gray';
};

const getStatusText = (status?: ContentStatus) => {
  if (!status) return '-';
  const texts: Record<ContentStatus, string> = {
    draft: '草稿',
    pending: '待审核',
    published: '已发布',
    offline: '已下线',
    rejected: '已拒绝',
    recycled: '回收站',
  };
  return texts[status] || status;
};

const goBack = () => {
  router.back();
};

const onEdit = () => {
  router.push({
    path: '/cms/content/form',
    query: { id: contentId.value, modelId: contentDetail.value?.modelId },
  });
};

const onRollback = async (versionId: number) => {
  try {
    await contentApi.rollback({ contentId: contentId.value, versionId });
    arcoMessage('success', '回滚成功');
    loadContent();
  } catch (error) {
    console.error(error);
  }
};

const loadContent = async () => {
  if (!contentId.value) return;
  loading.value = true;
  try {
    contentDetail.value = await contentApi.detail(contentId.value);
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadContent();
});
</script>

<style scoped lang="scss">
.content-detail {
  display: flex;
  flex-direction: column;
  height: 100%;

  .detail-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--color-bg-2);
    border-bottom: 1px solid var(--color-border-2);

    .header-left {
      display: flex;
      align-items: center;
      gap: 8px;

      .detail-title {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .detail-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;

    .detail-spin {
      height: 100%;
    }

    .content-card {
      margin-top: 16px;

      .content-body {
        min-height: 200px;
        line-height: 1.8;
      }
    }

    .stats-card,
    .settings-card,
    .tags-card {
      margin-top: 16px;
    }
  }
}
</style>

<template>
  <div class="preview-panel">
    <div v-if="loading" class="loading-state">
      <a-spin :size="40" />
      <p>正在生成预览...</p>
    </div>

    <div v-else-if="previewData" class="preview-content">
      <div class="preview-header">
        <h4>{{ previewData.title || "PPT预览" }}</h4>
        <a-tag v-if="previewData.status" :color="getStatusColor(previewData.status)">
          {{ getStatusText(previewData.status) }}
        </a-tag>
      </div>

      <div v-if="previewData.slides && previewData.slides.length > 0" class="preview-slides">
        <div v-for="(slide, index) in previewData.slides" :key="index" class="preview-slide-item">
          <div class="slide-thumbnail">
            <img v-if="slide.thumbnail" :src="slide.thumbnail" :alt="`Slide ${index + 1}`" />
            <div v-else class="slide-placeholder">
              <icon-file />
            </div>
          </div>
          <div class="slide-info">
            <span class="slide-number">{{ index + 1 }}</span>
            <span class="slide-title">{{ slide.title || "无标题" }}</span>
          </div>
        </div>
      </div>

      <div v-else class="empty-slides">
        <icon-file />
        <p>暂无幻灯片内容</p>
      </div>

      <div v-if="previewData.progress !== undefined" class="progress-info">
        <a-progress :percent="previewData.progress" :show-text="true" />
      </div>
    </div>

    <div v-else class="empty-state">
      <icon-image />
      <p>填写PPT信息后点击生成</p>
      <p class="hint">预览效果将在生成后展示</p>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  loading?: boolean;
  previewData?: any;
}>();

const getStatusColor = (status: string): string => {
  const colorMap: Record<string, string> = {
    pending: "gray",
    generating: "blue",
    completed: "green",
    failed: "red"
  };
  return colorMap[status] || "gray";
};

const getStatusText = (status: string): string => {
  const textMap: Record<string, string> = {
    pending: "等待中",
    generating: "生成中",
    completed: "已完成",
    failed: "生成失败"
  };
  return textMap[status] || "未知";
};
</script>

<style scoped lang="scss">
.preview-panel {
  min-height: 400px;
  display: flex;
  flex-direction: column;
}

.loading-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--color-text-3);

  p {
    margin-top: 16px;
  }
}

.preview-content {
  flex: 1;

  .preview-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;

    h4 {
      margin: 0;
    }
  }

  .preview-slides {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;
    max-height: 300px;
    overflow-y: auto;
  }

  .preview-slide-item {
    border: 1px solid var(--color-border);
    border-radius: 4px;
    overflow: hidden;
    cursor: pointer;
    transition: all 0.2s;

    &:hover {
      border-color: rgb(var(--primary-6));
    }

    .slide-thumbnail {
      aspect-ratio: 16 / 9;
      background: var(--color-fill-2);
      display: flex;
      align-items: center;
      justify-content: center;

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
      }

      .slide-placeholder {
        font-size: 24px;
        color: var(--color-text-3);
      }
    }

    .slide-info {
      padding: 8px;
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 12px;

      .slide-number {
        width: 20px;
        height: 20px;
        background: var(--color-fill-2);
        border-radius: 50%;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 10px;
        color: var(--color-text-3);
      }

      .slide-title {
        flex: 1;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }
    }
  }

  .progress-info {
    margin-top: 16px;
  }
}

.empty-slides,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--color-text-3);

  p {
    margin: 8px 0;
  }

  .hint {
    font-size: 12px;
  }
}

.empty-state {
  min-height: 300px;

  svg {
    font-size: 48px;
  }
}
</style>

<template>
  <div class="template-market-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-icon">
          <icon-template />
        </div>
        <div class="header-text">
          <h1>模板市场</h1>
          <p>精选上千套专业PPT模板，助您快速创建精彩演示</p>
        </div>
      </div>
    </div>

    <div class="page-content">
      <div class="filter-section">
        <a-form ref="searchFormRef" :model="searchForm" layout="inline">
          <a-form-item field="name">
            <a-input 
              v-model="searchForm.name" 
              placeholder="搜索模板名称..." 
              allow-clear
              class="search-input"
            >
              <template #prefix>
                <icon-search />
              </template>
            </a-input>
          </a-form-item>
          <a-form-item field="category">
            <a-select v-model="searchForm.category" placeholder="全部分类" allow-clear class="filter-select">
              <a-option value="business">
                <icon-briefcase /> 商务
              </a-option>
              <a-option value="education">
                <icon-education /> 教育
              </a-option>
              <a-option value="technology">
                <icon-robot /> 科技
              </a-option>
              <a-option value="creative">
                <icon-palette /> 创意
              </a-option>
            </a-select>
          </a-form-item>
          <a-form-item field="industry">
            <a-select v-model="searchForm.industry" placeholder="全部行业" allow-clear class="filter-select">
              <a-option value="technology">科技</a-option>
              <a-option value="education">教育</a-option>
              <a-option value="finance">金融</a-option>
              <a-option value="medical">医疗</a-option>
            </a-select>
          </a-form-item>
          <a-form-item>
            <a-space>
              <a-button type="primary" @click="search" class="search-btn">
                <template #icon><icon-search /></template>
                搜索
              </a-button>
              <a-button @click="reset">
                <template #icon><icon-refresh /></template>
                重置
              </a-button>
            </a-space>
          </a-form-item>
        </a-form>
      </div>

      <div class="tabs-section">
        <a-tabs v-model:active-key="currentTab" type="rounded">
          <a-tab-pane key="all">
            <template #title>
              <icon-apps /> 全部模板
            </template>
          </a-tab-pane>
          <a-tab-pane key="free">
            <template #title>
              <icon-gift /> 免费模板
            </template>
          </a-tab-pane>
          <a-tab-pane key="premium">
            <template #title>
              <icon-crown /> 付费模板
            </template>
          </a-tab-pane>
          <a-tab-pane key="my">
            <template #title>
              <icon-heart /> 我的收藏
            </template>
          </a-tab-pane>
        </a-tabs>
      </div>

      <div class="template-grid">
        <a-row :gutter="[20, 20]">
          <a-col :xs="24" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in templateList" :key="item.id">
            <div class="template-card" @click="handlePreview(item)">
              <div class="template-cover">
                <img :src="item.coverImage" :alt="item.name" />
                <div class="template-badges">
                  <a-tag v-if="item.isHot" color="red" size="small">
                    <icon-fire /> 热门
                  </a-tag>
                  <a-tag v-if="item.isNew" color="arcoblue" size="small">
                    <icon-plus /> 新品
                  </a-tag>
                </div>
                <div class="template-overlay">
                  <div class="overlay-actions">
                    <a-space size="small">
                      <a-button type="primary" size="small" @click.stop="handleUse(item)" class="use-btn">
                        <template #icon><icon-play-arrow /></template>
                        使用
                      </a-button>
                      <a-button 
                        size="small" 
                        @click.stop="handleFavorite(item)"
                        :class="{ favorite: item.isFavorite }"
                      >
                        <icon-heart v-if="!item.isFavorite" />
                        <icon-heart-fill v-else />
                      </a-button>
                      <a-button size="small" @click.stop="handlePreview(item)">
                        <icon-eye />
                      </a-button>
                    </a-space>
                  </div>
                </div>
              </div>
              <div class="template-info">
                <div class="template-name">{{ item.name }}</div>
                <div class="template-meta">
                  <span class="template-category">
                    <icon-tag />
                    {{ item.categoryName }}
                  </span>
                  <div class="template-stats">
                    <span class="stat-item">
                      <icon-eye /> {{ item.viewCount || 0 }}
                    </span>
                    <span class="stat-item">
                      <icon-download /> {{ item.downloadCount || 0 }}
                    </span>
                  </div>
                </div>
                <div class="template-footer">
                  <span class="template-page-count">
                    <icon-file /> {{ item.pageCount }}页
                  </span>
                  <span class="template-price" :class="{ free: item.isFree }">
                    {{ item.isFree ? '免费' : `¥${item.price}` }}
                  </span>
                </div>
              </div>
            </div>
          </a-col>
        </a-row>
      </div>

      <div class="pagination-wrapper" v-if="pagination.total > 0">
        <a-pagination
          v-model:current="pagination.current"
          v-model:page-size="pagination.pageSize"
          :total="pagination.total"
          show-total
          show-page-size
          size="large"
          @change="handlePageChange"
        />
      </div>
    </div>

    <a-modal 
      v-model:visible="previewVisible" 
      :footer="false" 
      :width="900"
      class="preview-modal"
    >
      <template #title>
        <div class="modal-title">
          <icon-eye />
          <span>模板预览</span>
        </div>
      </template>
      <div class="preview-content" v-if="previewTemplate">
        <div class="preview-main">
          <div class="preview-image">
            <img :src="previewTemplate.coverImage" :alt="previewTemplate.name" />
          </div>
        </div>
        <div class="preview-sidebar">
          <div class="sidebar-header">
            <h3>{{ previewTemplate.name }}</h3>
            <span class="template-price" :class="{ free: previewTemplate.isFree }">
              {{ previewTemplate.isFree ? '免费' : `¥${previewTemplate.price}` }}
            </span>
          </div>
          <p class="template-description">{{ previewTemplate.description }}</p>
          <a-divider style="margin: 16px 0" />
          <a-descriptions :column="1" size="small" class="template-details">
            <a-descriptions-item label="分类">{{ previewTemplate.categoryName }}</a-descriptions-item>
            <a-descriptions-item label="行业">{{ previewTemplate.industryName }}</a-descriptions-item>
            <a-descriptions-item label="页数">{{ previewTemplate.pageCount }}页</a-descriptions-item>
            <a-descriptions-item label="浏览量">{{ previewTemplate.viewCount || 0 }}次</a-descriptions-item>
            <a-descriptions-item label="下载量">{{ previewTemplate.downloadCount || 0 }}次</a-descriptions-item>
          </a-descriptions>
          <a-divider style="margin: 16px 0" />
          <div class="preview-tags">
            <a-tag v-for="tag in previewTemplate.tags" :key="tag" color="arcoblue" size="small">
              {{ tag }}
            </a-tag>
          </div>
          <div class="preview-actions">
            <a-button type="primary" size="large" @click="handleUse(previewTemplate)" class="use-btn">
              <template #icon><icon-magic /></template>
              立即使用
            </a-button>
            <a-button size="large" @click="handleFavorite(previewTemplate)">
              <template #icon>
                <icon-heart v-if="!previewTemplate.isFavorite" />
                <icon-heart-fill v-else />
              </template>
              {{ previewTemplate.isFavorite ? '已收藏' : '收藏' }}
            </a-button>
          </div>
        </div>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Message } from "@arco-design/web-vue";
import { templateApi, type PPTTemplate } from "@/api/modules/ppt";

const router = useRouter();
const loading = ref(false);
const previewVisible = ref(false);
const previewTemplate = ref<PPTTemplate | null>(null);
const currentTab = ref("all");
const templateList = ref<PPTTemplate[]>([]);

const searchForm = reactive({
  name: "",
  category: "",
  industry: ""
});

const pagination = reactive({
  current: 1,
  pageSize: 12,
  total: 0
});

const getList = async () => {
  loading.value = true;
  try {
    const data = await templateApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      category: searchForm.category || undefined,
      industry: searchForm.industry || undefined,
      type: currentTab.value
    });
    templateList.value = (data.list || []).map((item: any) => ({
      ...item,
      tags: item.tags || ["商务", "简约", "现代"],
      isHot: Math.random() > 0.7,
      isNew: Math.random() > 0.8,
      viewCount: item.viewCount || Math.floor(Math.random() * 10000)
    }));
    pagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchForm.name = "";
  searchForm.category = "";
  searchForm.industry = "";
  pagination.current = 1;
  getList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePreview = (template: PPTTemplate) => {
  previewTemplate.value = template;
  previewVisible.value = true;
};

const handleUse = (template: PPTTemplate) => {
  router.push({
    path: "/ppt/generate",
    query: { templateId: template.id }
  });
  Message.success("已选择模板，请完善PPT信息");
  previewVisible.value = false;
};

const handleFavorite = async (template: PPTTemplate) => {
  try {
    await templateApi.favorite(template.id);
    template.isFavorite = !template.isFavorite;
    Message.success(template.isFavorite ? "收藏成功" : "取消收藏");
  } catch (error: any) {
    Message.error(error?.message || "操作失败");
  }
};

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.template-market-page {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e8ec 100%);
  padding: 24px;

  .page-header {
    background: white;
    border-radius: 16px;
    padding: 32px;
    margin-bottom: 24px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);

    .header-content {
      display: flex;
      align-items: center;
      gap: 20px;

      .header-icon {
        width: 72px;
        height: 72px;
        background: linear-gradient(135deg, #11998e 0%, #38ef7d 100%);
        border-radius: 20px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-size: 36px;
        box-shadow: 0 8px 24px rgba(17, 153, 142, 0.3);
      }

      .header-text {
        h1 {
          font-size: 28px;
          font-weight: 700;
          margin: 0 0 8px 0;
          color: #1d2129;
        }

        p {
          margin: 0;
          color: #86909c;
          font-size: 15px;
        }
      }
    }
  }

  .page-content {
    background: white;
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  }

  .filter-section {
    margin-bottom: 24px;
    padding: 20px;
    background: linear-gradient(135deg, #fafbfc 0%, #f5f7fa 100%);
    border-radius: 12px;

    .search-input {
      width: 280px;
    }

    .filter-select {
      width: 150px;
    }

    .search-btn {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border: none;
    }
  }

  .tabs-section {
    margin-bottom: 24px;

    :deep(.arco-tabs-nav) {
      padding: 4px;
      background: #f2f3f5;
      border-radius: 10px;
    }
  }

  .template-grid {
    min-height: 400px;
  }

  .template-card {
    border-radius: 12px;
    overflow: hidden;
    background: white;
    border: 1px solid #e5e6eb;
    cursor: pointer;
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 12px 32px rgba(0, 0, 0, 0.12);

      .template-overlay {
        opacity: 1;
      }
    }

    .template-cover {
      position: relative;
      aspect-ratio: 16 / 9;
      overflow: hidden;
      background: #f2f3f5;

      img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        transition: transform 0.5s ease;
      }

      &:hover img {
        transform: scale(1.05);
      }

      .template-badges {
        position: absolute;
        top: 12px;
        left: 12px;
        display: flex;
        gap: 8px;
        z-index: 2;
      }

      .template-overlay {
        position: absolute;
        inset: 0;
        background: linear-gradient(to top, rgba(0, 0, 0, 0.8) 0%, rgba(0, 0, 0, 0.3) 50%, transparent 100%);
        display: flex;
        align-items: flex-end;
        justify-content: center;
        padding-bottom: 20px;
        opacity: 0;
        transition: opacity 0.3s ease;

        .overlay-actions {
          .use-btn {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            border: none;
          }

          .favorite {
            background: #ff4d4f;
            color: white;
            border: none;
          }
        }
      }
    }

    .template-info {
      padding: 16px;

      .template-name {
        font-weight: 600;
        font-size: 15px;
        margin-bottom: 12px;
        color: #1d2129;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .template-meta {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 12px;

        .template-category {
          display: flex;
          align-items: center;
          gap: 4px;
          color: #86909c;
          font-size: 13px;
        }

        .template-stats {
          display: flex;
          gap: 12px;

          .stat-item {
            display: flex;
            align-items: center;
            gap: 4px;
            color: #86909c;
            font-size: 12px;
          }
        }
      }

      .template-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-top: 12px;
        border-top: 1px solid #f2f3f5;

        .template-page-count {
          display: flex;
          align-items: center;
          gap: 4px;
          color: #86909c;
          font-size: 13px;
        }

        .template-price {
          font-weight: 700;
          font-size: 16px;
          color: #ff4d4f;

          &.free {
            color: #00b42a;
          }
        }
      }
    }
  }

  .pagination-wrapper {
    margin-top: 32px;
    display: flex;
    justify-content: center;
  }
}

.preview-modal {
  :deep(.arco-modal-content) {
    padding: 0;
    overflow: hidden;
  }

  .modal-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 600;
  }

  .preview-content {
    display: flex;
    gap: 24px;
    padding: 24px;

    .preview-main {
      flex: 1;

      .preview-image {
        width: 100%;
        aspect-ratio: 16 / 9;
        background: #f2f3f5;
        border-radius: 12px;
        overflow: hidden;

        img {
          width: 100%;
          height: 100%;
          object-fit: cover;
        }
      }
    }

    .preview-sidebar {
      width: 280px;
      flex-shrink: 0;

      .sidebar-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 12px;

        h3 {
          margin: 0;
          font-size: 18px;
          font-weight: 600;
          color: #1d2129;
        }

        .template-price {
          font-weight: 700;
          font-size: 20px;
          color: #ff4d4f;

          &.free {
            color: #00b42a;
          }
        }
      }

      .template-description {
        color: #86909c;
        font-size: 14px;
        line-height: 1.6;
        margin: 0;
      }

      .template-details {
        :deep(.arco-descriptions-item-label) {
          color: #86909c;
        }
      }

      .preview-tags {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        margin-bottom: 24px;
      }

      .preview-actions {
        display: flex;
        flex-direction: column;
        gap: 12px;

        .use-btn {
          background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
          border: none;
        }
      }
    }
  }
}
</style>
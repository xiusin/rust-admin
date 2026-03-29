<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="name" label="模板名称">
              <a-input v-model="searchForm.name" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="category" label="模板分类">
              <a-select v-model="searchForm.category" placeholder="请选择分类" allow-clear>
                <a-option value="business">商务</a-option>
                <a-option value="education">教育</a-option>
                <a-option value="technology">科技</a-option>
                <a-option value="creative">创意</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="industry" label="行业">
              <a-select v-model="searchForm.industry" placeholder="请选择行业" allow-clear>
                <a-option value="technology">科技</a-option>
                <a-option value="education">教育</a-option>
                <a-option value="finance">金融</a-option>
                <a-option value="medical">医疗</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" size="small" @click="search">
                <template #icon><icon-search /></template>
                查询
              </a-button>
              <a-button size="small" @click="reset">
                <template #icon><icon-refresh /></template>
                重置
              </a-button>
            </a-space>
          </a-col>
        </a-row>
      </a-form>

      <a-divider :margin="0" />

      <div class="template-tabs">
        <a-radio-group v-model="currentTab" type="button">
          <a-radio value="all">全部模板</a-radio>
          <a-radio value="free">免费模板</a-radio>
          <a-radio value="premium">付费模板</a-radio>
          <a-radio value="my">我的收藏</a-radio>
        </a-radio-group>
      </div>

      <div class="template-grid">
        <a-row :gutter="[16, 16]">
          <a-col :xs="24" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in templateList" :key="item.id">
            <div class="template-card" @click="handlePreview(item)">
              <div class="template-cover">
                <img :src="item.coverImage" :alt="item.name" />
                <div class="template-overlay">
                  <a-space>
                    <a-button type="primary" size="small" @click.stop="handleUse(item)"> 使用模板 </a-button>
                    <a-button size="small" @click.stop="handleFavorite(item)">
                      <icon-heart v-if="!item.isFavorite" />
                      <icon-heart-fill v-else />
                    </a-button>
                  </a-space>
                </div>
              </div>
              <div class="template-info">
                <div class="template-name">{{ item.name }}</div>
                <div class="template-meta">
                  <span class="template-category">{{ item.categoryName }}</span>
                  <span class="template-price" :class="{ free: item.isFree }">
                    {{ item.isFree ? "免费" : `¥${item.price}` }}
                  </span>
                </div>
              </div>
            </div>
          </a-col>
        </a-row>
      </div>

      <div class="pagination-wrapper">
        <a-pagination
          v-model:current="pagination.current"
          v-model:page-size="pagination.pageSize"
          :total="pagination.total"
          show-total
          show-page-size
          @change="handlePageChange"
        />
      </div>
    </div>

    <a-modal v-model:visible="previewVisible" :width="800" :footer="false">
      <template #title>模板预览</template>
      <div class="preview-content" v-if="previewTemplate">
        <div class="preview-image">
          <img :src="previewTemplate.coverImage" :alt="previewTemplate.name" />
        </div>
        <div class="preview-info">
          <h3>{{ previewTemplate.name }}</h3>
          <p>{{ previewTemplate.description }}</p>
          <a-descriptions :column="2" size="small">
            <a-descriptions-item label="分类">{{ previewTemplate.categoryName }}</a-descriptions-item>
            <a-descriptions-item label="行业">{{ previewTemplate.industryName }}</a-descriptions-item>
            <a-descriptions-item label="页数">{{ previewTemplate.pageCount }}页</a-descriptions-item>
            <a-descriptions-item label="下载">{{ previewTemplate.downloadCount }}次</a-descriptions-item>
          </a-descriptions>
          <div class="preview-actions">
            <a-button type="primary" @click="handleUse(previewTemplate)"> 使用此模板 </a-button>
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
    templateList.value = data.list || [];
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
.snow-page {
  padding: 16px;
}

.snow-inner {
  background: var(--color-bg-2);
  padding: 16px;
  border-radius: 4px;
}

.search-btn {
  margin-bottom: 20px;
}

.template-tabs {
  margin: 16px 0;
}

.template-grid {
  min-height: 400px;
}

.template-card {
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--color-border);
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);

    .template-overlay {
      opacity: 1;
    }
  }

  .template-cover {
    position: relative;
    aspect-ratio: 16 / 9;
    overflow: hidden;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }

    .template-overlay {
      position: absolute;
      inset: 0;
      background: rgba(0, 0, 0, 0.5);
      display: flex;
      align-items: center;
      justify-content: center;
      opacity: 0;
      transition: opacity 0.2s;
    }
  }

  .template-info {
    padding: 12px;

    .template-name {
      font-weight: 500;
      margin-bottom: 8px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .template-meta {
      display: flex;
      justify-content: space-between;
      align-items: center;
      font-size: 12px;

      .template-category {
        color: var(--color-text-3);
      }

      .template-price {
        color: rgb(var(--danger-6));
        font-weight: 500;

        &.free {
          color: rgb(var(--success-6));
        }
      }
    }
  }
}

.pagination-wrapper {
  margin-top: 24px;
  display: flex;
  justify-content: center;
}

.preview-content {
  .preview-image {
    width: 100%;
    aspect-ratio: 16 / 9;
    background: var(--color-fill-2);
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 16px;

    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
  }

  .preview-info {
    h3 {
      margin-bottom: 8px;
    }

    p {
      color: var(--color-text-2);
      margin-bottom: 16px;
    }

    .preview-actions {
      margin-top: 16px;
    }
  }
}
</style>

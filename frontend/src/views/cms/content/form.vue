<template>
  <div class="snow-page">
    <div class="snow-inner content-form">
      <div class="form-header">
        <div class="header-left">
          <a-button type="text" @click="goBack">
            <template #icon><icon-left /></template>
            返回列表
          </a-button>
          <a-divider direction="vertical" />
          <span class="form-title">{{ formTitle }}</span>
        </div>
        <div class="header-right">
          <a-space>
            <a-button @click="saveDraft" :loading="saving">
              <template #icon><icon-save /></template>
              保存草稿
            </a-button>
            <a-button type="primary" @click="saveAndPublish" :loading="saving">
              <template #icon><icon-upload /></template>
              保存并发布
            </a-button>
          </a-space>
        </div>
      </div>

      <div class="form-content">
        <a-spin :loading="loading" class="form-spin">
          <a-form ref="formRef" :model="form" :rules="rules" layout="vertical">
            <a-row :gutter="24">
              <a-col :span="16">
                <a-card title="基本信息">
                  <a-form-item field="title" label="标题" validate-trigger="blur">
                    <a-input v-model="form.title" placeholder="请输入标题" allow-clear />
                  </a-form-item>
                  <a-form-item field="slug" label="别名">
                    <a-input v-model="form.slug" placeholder="请输入别名（用于URL）" allow-clear />
                  </a-form-item>
                  <a-form-item field="keywords" label="关键词">
                    <a-input-tag v-model="keywords" placeholder="请输入关键词，回车添加" allow-clear />
                  </a-form-item>
                  <a-form-item field="description" label="摘要">
                    <a-textarea v-model="form.description" placeholder="请输入摘要" :auto-size="{ minRows: 3, maxRows: 5 }" allow-clear />
                  </a-form-item>
                  <a-form-item field="content" label="内容">
                    <ContentEditor v-model="form.content" />
                  </a-form-item>
                </a-card>
              </a-col>
              <a-col :span="8">
                <a-card title="发布设置" class="settings-card">
                  <a-form-item field="categoryId" label="所属分类">
                    <a-tree-select
                      v-model="form.categoryId"
                      :data="categories"
                      :field-names="{ key: 'value', title: 'label', children: 'children' }"
                      placeholder="请选择分类"
                      allow-clear
                    />
                  </a-form-item>
                  <a-form-item field="source" label="来源">
                    <a-input v-model="form.source" placeholder="请输入来源" allow-clear />
                  </a-form-item>
                  <a-form-item field="thumbnail" label="缩略图">
                    <a-input v-model="form.thumbnail" placeholder="请输入图片URL或上传" allow-clear />
                  </a-form-item>
                  <a-form-item field="sort" label="排序">
                    <a-input-number v-model="form.sort" :min="0" :max="9999" style="width: 100%" />
                  </a-form-item>
                  <a-form-item field="publishTime" label="发布时间">
                    <a-date-picker v-model="form.publishTime" show-time style="width: 100%" />
                  </a-form-item>
                </a-card>

                <a-card title="属性设置" class="settings-card">
                  <a-form-item label="置顶">
                    <a-switch v-model="form.isTop" />
                  </a-form-item>
                  <a-form-item label="推荐">
                    <a-switch v-model="form.isRecommend" />
                  </a-form-item>
                  <a-form-item label="热门">
                    <a-switch v-model="form.isHot" />
                  </a-form-item>
                  <a-form-item label="允许评论">
                    <a-switch v-model="form.allowComment" />
                  </a-form-item>
                </a-card>

                <a-card title="标签" class="settings-card">
                  <a-form-item>
                    <a-select v-model="tagIds" multiple placeholder="请选择标签" allow-clear>
                      <a-option v-for="tag in tags" :key="tag.id" :value="tag.id">{{ tag.name }}</a-option>
                    </a-select>
                  </a-form-item>
                </a-card>
              </a-col>
            </a-row>
          </a-form>
        </a-spin>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { useRouter, useRoute } from 'vue-router';
import { contentApi, type ContentAddParams, type ContentEditParams } from '@/api/modules/cms/content';
import { categoryApi, type CmsCategoryTree } from '@/api/modules/cms/category';
import { tagApi, type CmsTagItem } from '@/api/modules/cms/tag';
import ContentEditor from './components/ContentEditor.vue';

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const saving = ref(false);
const formRef = ref();
const categories = ref<CmsCategoryTree[]>([]);
const tags = ref<CmsTagItem[]>([]);

const contentId = computed(() => Number(route.query.id));
const modelId = computed(() => Number(route.query.modelId));
const formTitle = computed(() => (contentId.value ? '编辑内容' : '新增内容'));

const form = reactive({
  id: 0,
  modelId: 0,
  categoryId: null as number | null,
  title: '',
  slug: '',
  keywords: '',
  description: '',
  content: '',
  source: '',
  thumbnail: '',
  sort: 0,
  publishTime: '',
  isTop: false,
  isRecommend: false,
  isHot: false,
  allowComment: true,
});

const keywords = computed({
  get: () => form.keywords.split(',').filter(Boolean),
  set: (val: string[]) => {
    form.keywords = val.join(',');
  },
});

const tagIds = ref<number[]>([]);

const rules = {
  title: [{ required: true, message: '请输入标题' }],
};

const goBack = () => {
  router.back();
};

const loadCategories = async () => {
  if (!modelId.value) return;
  try {
    const data = await categoryApi.tree({ modelId: modelId.value });
    categories.value = data || [];
  } catch (error) {
    console.error(error);
  }
};

const loadTags = async () => {
  try {
    const data = await tagApi.simpleList({ status: true });
    tags.value = data || [];
  } catch (error) {
    console.error(error);
  }
};

const loadContent = async () => {
  if (!contentId.value) return;
  loading.value = true;
  try {
    const data = await contentApi.detail(contentId.value);
    form.id = data.id;
    form.modelId = data.modelId;
    form.categoryId = data.categoryId || null;
    form.title = data.title;
    form.slug = data.slug || '';
    form.keywords = data.keywords || '';
    form.description = data.description || '';
    form.content = data.content || '';
    form.source = data.source || '';
    form.thumbnail = data.thumbnail || '';
    form.sort = data.sort;
    form.publishTime = data.publishTime || '';
    form.isTop = data.isTop;
    form.isRecommend = data.isRecommend;
    form.isHot = data.isHot;
    form.allowComment = data.allowComment;
    tagIds.value = data.tags?.map((t) => t.id) || [];
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const saveDraft = async () => {
  await save('draft');
};

const saveAndPublish = async () => {
  await save('published');
};

const save = async (status: 'draft' | 'published') => {
  const state = await formRef.value?.validate();
  if (state) return;

  saving.value = true;
  try {
    const params = {
      modelId: modelId.value,
      categoryId: form.categoryId || undefined,
      title: form.title,
      slug: form.slug || undefined,
      keywords: form.keywords || undefined,
      description: form.description || undefined,
      content: form.content || undefined,
      source: form.source || undefined,
      thumbnail: form.thumbnail || undefined,
      sort: form.sort,
      status,
      publishTime: form.publishTime || undefined,
      isTop: form.isTop,
      isRecommend: form.isRecommend,
      isHot: form.isHot,
      allowComment: form.allowComment,
      tagIds: tagIds.value,
    };

    if (contentId.value) {
      await contentApi.edit({ ...params, id: contentId.value } as ContentEditParams);
      arcoMessage('success', '保存成功');
    } else {
      await contentApi.add(params as ContentAddParams);
      arcoMessage('success', '创建成功');
    }
    goBack();
  } catch (error) {
    console.error(error);
  } finally {
    saving.value = false;
  }
};

onMounted(() => {
  form.modelId = modelId.value;
  loadCategories();
  loadTags();
  if (contentId.value) {
    loadContent();
  }
});
</script>

<style scoped lang="scss">
.content-form {
  display: flex;
  flex-direction: column;
  height: 100%;

  .form-header {
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

      .form-title {
        font-size: 16px;
        font-weight: 500;
      }
    }
  }

  .form-content {
    flex: 1;
    overflow-y: auto;
    padding: 16px;

    .form-spin {
      height: 100%;
    }

    .settings-card {
      margin-top: 16px;

      &:first-child {
        margin-top: 0;
      }
    }
  }
}
</style>

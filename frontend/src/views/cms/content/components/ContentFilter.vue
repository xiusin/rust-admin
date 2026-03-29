<template>
  <a-form :model="modelValue" auto-label-width>
    <a-row :gutter="16">
      <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
        <a-form-item field="title" label="内容标题">
          <a-input v-model="modelValue.title" placeholder="请输入内容标题" allow-clear />
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
        <a-form-item field="categoryId" label="所属分类">
          <a-tree-select
            v-model="modelValue.categoryId"
            :data="categories"
            :field-names="{ key: 'value', title: 'label', children: 'children' }"
            placeholder="请选择分类"
            allow-clear
          />
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
        <a-form-item field="contentType" label="内容类型">
          <a-select v-model="modelValue.contentType" placeholder="请选择类型" allow-clear>
            <a-option value="article">文章</a-option>
            <a-option value="image">图片</a-option>
            <a-option value="video">视频</a-option>
            <a-option value="audio">音频</a-option>
            <a-option value="file">文件</a-option>
            <a-option value="link">链接</a-option>
          </a-select>
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
        <a-space class="search-btn">
          <a-button type="primary" size="small" @click="emit('search')">
            <template #icon><icon-search /></template>
            <template #default>查询</template>
          </a-button>
          <a-button size="small" @click="emit('reset')">
            <template #icon><icon-refresh /></template>
            <template #default>重置</template>
          </a-button>
        </a-space>
      </a-col>
    </a-row>
    <a-row :gutter="16">
      <a-col :xs="24" :sm="12" :md="8" :lg="6">
        <a-form-item label="置顶">
          <a-select v-model="modelValue.isTop" placeholder="全部" allow-clear style="width: 100%">
            <a-option :value="true">是</a-option>
            <a-option :value="false">否</a-option>
          </a-select>
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="12" :md="8" :lg="6">
        <a-form-item label="推荐">
          <a-select v-model="modelValue.isRecommend" placeholder="全部" allow-clear style="width: 100%">
            <a-option :value="true">是</a-option>
            <a-option :value="false">否</a-option>
          </a-select>
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="12" :md="8" :lg="6">
        <a-form-item label="热门">
          <a-select v-model="modelValue.isHot" placeholder="全部" allow-clear style="width: 100%">
            <a-option :value="true">是</a-option>
            <a-option :value="false">否</a-option>
          </a-select>
        </a-form-item>
      </a-col>
      <a-col :xs="24" :sm="12" :md="8" :lg="6">
        <a-form-item label="创建时间">
          <a-range-picker v-model="dateRange" style="width: 100%" />
        </a-form-item>
      </a-col>
    </a-row>
  </a-form>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { CmsCategoryTree } from "@/api/modules/cms/category";

interface SearchForm {
  title: string;
  categoryId: number | null;
  status: ContentStatus | null;
  contentType: string | null;
  isTop: boolean | null;
  isRecommend: boolean | null;
  isHot: boolean | null;
  startTime: string;
  endTime: string;
}

interface Props {
  modelValue: SearchForm;
  modelId: number;
  categories: CmsCategoryTree[];
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:modelValue": [value: SearchForm];
  search: [];
  reset: [];
}>();

const modelValue = computed({
  get: () => props.modelValue,
  set: val => emit("update:modelValue", val)
});

const dateRange = computed({
  get: () => {
    if (props.modelValue.startTime && props.modelValue.endTime) {
      return [props.modelValue.startTime, props.modelValue.endTime];
    }
    return [];
  },
  set: (val: string[] | undefined) => {
    if (val && val.length === 2) {
      props.modelValue.startTime = val[0];
      props.modelValue.endTime = val[1];
    } else {
      props.modelValue.startTime = "";
      props.modelValue.endTime = "";
    }
  }
});
</script>

<style scoped lang="scss">
.search-btn {
  margin-bottom: 20px;
}
</style>

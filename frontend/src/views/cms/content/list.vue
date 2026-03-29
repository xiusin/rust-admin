<template>
  <div class="snow-page">
    <div class="snow-inner">
      <ContentFilter v-model="searchForm" :model-id="modelId" :categories="categories" @search="search" @reset="reset" />

      <a-divider :margin="0" />

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
            <a-button type="primary" status="success" size="small" :disabled="selectedIds.length === 0" @click="onBatchPublish">
              <template #icon><icon-upload /></template>
              发布
            </a-button>
            <a-button type="primary" status="warning" size="small" :disabled="selectedIds.length === 0" @click="onBatchOffline">
              <template #icon><icon-download /></template>
              下线
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedIds.length === 0" @click="onBatchDelete">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-radio-group v-model="currentStatus" type="button" size="small" @change="onStatusChange">
              <a-radio value="all">全部</a-radio>
              <a-radio value="published">已发布</a-radio>
              <a-radio value="draft">草稿</a-radio>
              <a-radio value="pending">待审核</a-radio>
              <a-radio value="recycled">回收站</a-radio>
            </a-radio-group>
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1200 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #thumbnail="{ record }">
          <a-image v-if="record.thumbnail" :src="record.thumbnail" width="60" height="40" fit="cover" />
          <span v-else>-</span>
        </template>
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)" size="small">
            {{ getStatusText(record.status) }}
          </a-tag>
        </template>
        <template #flags="{ record }">
          <a-space>
            <a-tag v-if="record.isTop" color="red" size="small">置顶</a-tag>
            <a-tag v-if="record.isRecommend" color="orange" size="small">推荐</a-tag>
            <a-tag v-if="record.isHot" color="arcoblue" size="small">热门</a-tag>
          </a-space>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onDetail(record)">详情</a-button>
            <a-dropdown trigger="click">
              <a-button type="text" size="mini">
                <template #icon><icon-more /></template>
              </a-button>
              <template #content>
                <a-doption v-if="record.status === 'draft'" @click="onPublish(record)"> <icon-upload /> 发布 </a-doption>
                <a-doption v-if="record.status === 'published'" @click="onOffline(record)"> <icon-download /> 下线 </a-doption>
                <a-doption @click="onToggleTop(record)"> <icon-to-top /> {{ record.isTop ? "取消置顶" : "置顶" }} </a-doption>
                <a-doption @click="onToggleRecommend(record)">
                  <icon-star /> {{ record.isRecommend ? "取消推荐" : "推荐" }}
                </a-doption>
                <a-doption @click="onToggleHot(record)"> <icon-fire /> {{ record.isHot ? "取消热门" : "热门" }} </a-doption>
                <a-doption v-if="record.status === 'recycled'" @click="onRestore(record)"> <icon-undo /> 恢复 </a-doption>
                <a-doption @click="onDelete(record)"> <icon-delete /> 删除 </a-doption>
              </template>
            </a-dropdown>
          </a-space>
        </template>
      </a-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed, watch } from "vue";
import { useRouter, useRoute } from "vue-router";
import { contentApi, type CmsContentList, type ContentStatus } from "@/api/modules/cms/content";
import { modelApi } from "@/api/modules/cms/model";
import { categoryApi, type CmsCategoryTree } from "@/api/modules/cms/category";
import ContentFilter from "./components/ContentFilter.vue";

const router = useRouter();
const route = useRoute();
const loading = ref(false);
const tableData = ref<CmsContentList[]>([]);
const selectedIds = ref<number[]>([]);
const categories = ref<CmsCategoryTree[]>([]);
const currentStatus = ref<string>("all");

const modelId = computed(() => Number(route.query.modelId));

const searchForm = reactive({
  title: "",
  categoryId: null as number | null,
  status: null as ContentStatus | null,
  contentType: null as string | null,
  isTop: null as boolean | null,
  isRecommend: null as boolean | null,
  isHot: null as boolean | null,
  startTime: "",
  endTime: ""
});

const pagination = reactive({
  current: 1,
  pageSize: 20,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = computed(() => [
  { title: "缩略图", dataIndex: "thumbnail", slotName: "thumbnail", width: 80 },
  { title: "标题", dataIndex: "title", width: 200, ellipsis: true, tooltip: true },
  { title: "分类", dataIndex: "categoryName", width: 100 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 80, align: "center" },
  { title: "标记", slotName: "flags", width: 150 },
  { title: "浏览", dataIndex: "viewCount", width: 80, align: "center" },
  { title: "点赞", dataIndex: "likeCount", width: 80, align: "center" },
  { title: "评论", dataIndex: "commentCount", width: 80, align: "center" },
  { title: "排序", dataIndex: "sort", width: 80, align: "center" },
  { title: "发布时间", dataIndex: "publishTime", width: 160 },
  { title: "创建时间", dataIndex: "createdAt", width: 160 },
  { title: "操作", slotName: "optional", width: 160, fixed: "right", align: "center" }
]);

const getStatusColor = (status: ContentStatus) => {
  const colors: Record<ContentStatus, string> = {
    draft: "gray",
    pending: "orange",
    published: "green",
    offline: "gray",
    rejected: "red",
    recycled: "gray"
  };
  return colors[status] || "gray";
};

const getStatusText = (status: ContentStatus) => {
  const texts: Record<ContentStatus, string> = {
    draft: "草稿",
    pending: "待审核",
    published: "已发布",
    offline: "已下线",
    rejected: "已拒绝",
    recycled: "回收站"
  };
  return texts[status] || status;
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

const getList = async () => {
  if (!modelId.value) return;
  loading.value = true;
  try {
    const params: any = {
      modelId: modelId.value,
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      title: searchForm.title || undefined,
      categoryId: searchForm.categoryId || undefined,
      status: searchForm.status || undefined,
      contentType: searchForm.contentType || undefined,
      isTop: searchForm.isTop ?? undefined,
      isRecommend: searchForm.isRecommend ?? undefined,
      isHot: searchForm.isHot ?? undefined,
      startTime: searchForm.startTime || undefined,
      endTime: searchForm.endTime || undefined
    };

    if (currentStatus.value === "recycled") {
      const data = await contentApi.recycleList({
        modelId: modelId.value,
        pageNum: pagination.current,
        pageSize: pagination.pageSize
      });
      tableData.value = data.list || [];
      pagination.total = data.total || 0;
    } else {
      if (currentStatus.value !== "all") {
        params.status = currentStatus.value as ContentStatus;
      }
      const data = await contentApi.list(params);
      tableData.value = data.list || [];
      pagination.total = data.total || 0;
    }
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
  searchForm.title = "";
  searchForm.categoryId = null;
  searchForm.status = null;
  searchForm.contentType = null;
  searchForm.isTop = null;
  searchForm.isRecommend = null;
  searchForm.isHot = null;
  searchForm.startTime = "";
  searchForm.endTime = "";
  pagination.current = 1;
  getList();
};

const refresh = () => {
  getList();
};

const onStatusChange = () => {
  pagination.current = 1;
  getList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  getList();
};

const onAdd = () => {
  router.push({
    path: "/cms/content/form",
    query: { modelId: modelId.value }
  });
};

const onEdit = (record: CmsContentList) => {
  router.push({
    path: "/cms/content/form",
    query: { id: record.id, modelId: modelId.value }
  });
};

const onDetail = (record: CmsContentList) => {
  router.push({
    path: "/cms/content/detail",
    query: { id: record.id }
  });
};

const onPublish = async (record: CmsContentList) => {
  try {
    await contentApi.publish(record.id);
    arcoMessage("success", "发布成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onOffline = async (record: CmsContentList) => {
  try {
    await contentApi.offline(record.id);
    arcoMessage("success", "下线成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onToggleTop = async (record: CmsContentList) => {
  try {
    await contentApi.updateTop(record.id, !record.isTop);
    arcoMessage("success", record.isTop ? "取消置顶成功" : "置顶成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onToggleRecommend = async (record: CmsContentList) => {
  try {
    await contentApi.updateRecommend(record.id, !record.isRecommend);
    arcoMessage("success", record.isRecommend ? "取消推荐成功" : "推荐成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onToggleHot = async (record: CmsContentList) => {
  try {
    await contentApi.updateHot(record.id, !record.isHot);
    arcoMessage("success", record.isHot ? "取消热门成功" : "热门成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onRestore = async (record: CmsContentList) => {
  try {
    await contentApi.restore(record.id);
    arcoMessage("success", "恢复成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: CmsContentList) => {
  Modal.warning({
    title: "确认删除",
    content: `确定要删除内容"${record.title}"吗？`,
    hideCancel: false,
    onOk: async () => {
      try {
        await contentApi.delete(record.id);
        arcoMessage("success", "删除成功");
        getList();
      } catch (error) {
        console.error(error);
      }
    }
  });
};

const onBatchPublish = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await contentApi.batchPublish(selectedIds.value);
    arcoMessage("success", "批量发布成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchOffline = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await contentApi.batchOffline(selectedIds.value);
    arcoMessage("success", "批量下线成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  Modal.warning({
    title: "确认删除",
    content: `确定要删除选中的${selectedIds.value.length}条内容吗？`,
    hideCancel: false,
    onOk: async () => {
      try {
        await contentApi.batchDelete(selectedIds.value);
        arcoMessage("success", "批量删除成功");
        selectedIds.value = [];
        getList();
      } catch (error) {
        console.error(error);
      }
    }
  });
};

watch(modelId, () => {
  if (modelId.value) {
    loadCategories();
    getList();
  }
});

onMounted(() => {
  if (modelId.value) {
    loadCategories();
    getList();
  }
});
</script>

<style scoped lang="scss">
.action-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;
  &:hover {
    background-color: var(--color-fill-2);
  }
}
</style>

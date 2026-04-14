<template>
  <div class="content-list-page">
    <div class="page-wrapper">
      <div class="page-header">
        <ContentFilter v-model="searchForm" :model-id="modelId" :categories="categories" @search="search" @reset="reset" />
      </div>

      <div class="page-toolbar">
        <div class="toolbar-left">
          <a-space size="medium">
            <a-button type="primary" size="middle" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增内容
            </a-button>
            <div v-if="selectedIds.length > 0" class="batch-actions">
              <a-tag color="arcoblue" : closable="false">已选 {{ selectedIds.length }} 项</a-tag>
              <a-button status="success" size="small" @click="onBatchPublish">
                <template #icon><icon-upload /></template>
                批量发布
              </a-button>
              <a-button status="warning" size="small" @click="onBatchOffline">
                <template #icon><icon-download /></template>
                批量下线
              </a-button>
              <a-button status="danger" size="small" @click="onBatchDelete">
                <template #icon><icon-delete /></template>
                批量删除
              </a-button>
            </div>
          </a-space>
        </div>
        <div class="toolbar-right">
          <a-space size="medium">
            <a-radio-group v-model="currentStatus" type="button" @change="onStatusChange">
              <a-radio value="all">全部</a-radio>
              <a-radio value="published">已发布</a-radio>
              <a-radio value="draft">草稿</a-radio>
              <a-radio value="pending">待审核</a-radio>
              <a-radio value="recycled">回收站</a-radio>
            </a-radio-group>
            <a-tooltip content="刷新数据">
              <a-button shape="circle" @click="refresh">
                <icon-refresh />
              </a-button>
            </a-tooltip>
          </a-space>
        </div>
      </div>

      <div class="table-container">
        <a-table
          row-key="id"
          :loading="loading"
          :scroll="{ x: 1400 }"
          :columns="columns"
          :data="tableData"
          :row-selection="{ type: 'checkbox', showCheckedAll: true }"
          v-model:selectedKeys="selectedIds"
          :pagination="pagination"
          :sticky="{ header: true }"
          @page-change="handlePageChange"
          @page-size-change="handlePageSizeChange"
          row-class-name="table-row"
        >
          <template #thumbnail="{ record }">
            <div class="thumbnail-cell">
              <a-image v-if="record.thumbnail" :src="record.thumbnail" width="56" height="42" fit="cover" class="content-thumbnail" />
              <div v-else class="thumbnail-placeholder">
                <icon-image />
              </div>
            </div>
          </template>
          <template #status="{ record }">
            <a-tag :color="getStatusColor(record.status)" class="status-tag">
              <template #icon>
                <component :is="getStatusIcon(record.status)" />
              </template>
              {{ getStatusText(record.status) }}
            </a-tag>
          </template>
          <template #flags="{ record }">
            <div class="flags-container">
              <a-tag v-if="record.isTop" color="red" class="flag-tag">
                <template #icon><icon-to-top /></template>
                置顶
              </a-tag>
              <a-tag v-if="record.isRecommend" color="orange" class="flag-tag">
                <template #icon><icon-star /></template>
                推荐
              </a-tag>
              <a-tag v-if="record.isHot" color="arcoblue" class="flag-tag">
                <template #icon><icon-fire /></template>
                热门
              </a-tag>
            </div>
          </template>
          <template #stats="{ record }">
            <div class="stats-cell">
              <div class="stat-item">
                <icon-eye />
                <span>{{ record.viewCount || 0 }}</span>
              </div>
              <div class="stat-item">
                <icon-heart-fill />
                <span>{{ record.likeCount || 0 }}</span>
              </div>
              <div class="stat-item">
                <icon-chat-round />
                <span>{{ record.commentCount || 0 }}</span>
              </div>
            </div>
          </template>
          <template #time="{ record }">
            <div class="time-cell">
              <div class="time-item">
                <span class="time-label">发布：</span>
                <span class="time-value">{{ record.publishTime || '-' }}</span>
              </div>
              <div class="time-item">
                <span class="time-label">创建：</span>
                <span class="time-value">{{ record.createdAt || '-' }}</span>
              </div>
            </div>
          </template>
          <template #optional="{ record }">
            <div class="action-cell">
              <a-button type="text" size="small" @click="onEdit(record)">
                <template #icon><icon-edit /></template>
                编辑
              </a-button>
              <a-button type="text" size="small" @click="onDetail(record)">
                <template #icon><icon-eye /></template>
                详情
              </a-button>
              <a-dropdown trigger="click" trigger-props={{ hideOnClick: true }}>
                <a-button type="text" size="small">
                  <template #icon><icon-more /></template>
                  更多
                </a-button>
                <template #content>
                  <a-doption v-if="record.status === 'draft'" @click="onPublish(record)">
                    <icon-upload /> 发布
                  </a-doption>
                  <a-doption v-if="record.status === 'published'" @click="onOffline(record)">
                    <icon-download /> 下线
                  </a-doption>
                  <a-doption @click="onToggleTop(record)">
                    <icon-to-top /> {{ record.isTop ? '取消置顶' : '置顶' }}
                  </a-doption>
                  <a-doption @click="onToggleRecommend(record)">
                    <icon-star /> {{ record.isRecommend ? '取消推荐' : '推荐' }}
                  </a-doption>
                  <a-doption @click="onToggleHot(record)">
                    <icon-fire /> {{ record.isHot ? '取消热门' : '热门' }}
                  </a-doption>
                  <a-doption v-if="record.status === 'recycled'" @click="onRestore(record)">
                    <icon-undo /> 恢复
                  </a-doption>
                  <a-doption @click="onDelete(record)" class="danger-option">
                    <icon-delete /> 删除
                  </a-doption>
                </template>
              </a-dropdown>
            </div>
          </template>
        </a-table>
      </div>
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
  pageSize: 15,
  showPageSize: true,
  showTotal: true,
  total: 0,
  pageSizeOptions: [10, 15, 20, 30, 50]
});

const columns = computed(() => [
  { title: "缩略图", dataIndex: "thumbnail", slotName: "thumbnail", width: 90, fixed: "left" },
  { title: "标题", dataIndex: "title", width: 220, ellipsis: true, tooltip: true },
  { title: "分类", dataIndex: "categoryName", width: 110 },
  { title: "状态", dataIndex: "status", slotName: "status", width: 100, align: "center" },
  { title: "属性标记", slotName: "flags", width: 160 },
  { title: "数据统计", slotName: "stats", width: 140, align: "center" },
  { title: "排序", dataIndex: "sort", width: 70, align: "center" },
  { title: "时间", slotName: "time", width: 180 },
  { title: "操作", slotName: "optional", width: 200, fixed: "right", align: "center" }
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

const getStatusIcon = (status: ContentStatus) => {
  const icons: Record<ContentStatus, string> = {
    draft: "icon-edit",
    pending: "icon-clock-circle",
    published: "icon-check-circle-fill",
    offline: "icon-pause-circle-fill",
    rejected: "icon-close-circle-fill",
    recycled: "icon-delete"
  };
  return icons[status] || "icon-file";
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
.content-list-page {
  height: 100%;
  background: var(--color-bg-1);

  .page-wrapper {
    height: 100%;
    padding: 20px;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .page-header {
    background: var(--color-bg-2);
    border-radius: 8px;
    padding: 16px;
    border: 1px solid var(--color-border-1);
  }

  .page-toolbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);

    .toolbar-left {
      display: flex;
      align-items: center;

      .batch-actions {
        display: flex;
        align-items: center;
        gap: 8px;
        padding-left: 12px;
        border-left: 1px solid var(--color-border-1);
      }
    }

    .toolbar-right {
      display: flex;
      align-items: center;
    }
  }

  .table-container {
    flex: 1;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);
    overflow: hidden;

    :deep(.arco-table) {
      border-radius: 0;
      border: none;
    }

    :deep(.arco-table-th) {
      background: var(--color-bg-3);
    }

    .table-row {
      transition: background-color 0.2s;

      &:hover {
        background: var(--color-bg-3);
      }
    }
  }

  .thumbnail-cell {
    display: flex;
    justify-content: center;
    padding: 4px 0;

    .content-thumbnail {
      border-radius: 6px;
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
    }

    .thumbnail-placeholder {
      width: 56px;
      height: 42px;
      background: var(--color-bg-3);
      border-radius: 6px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: var(--color-text-3);
      font-size: 18px;
    }
  }

  .status-tag {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-weight: 500;
  }

  .flags-container {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;

    .flag-tag {
      display: inline-flex;
      align-items: center;
      gap: 2px;
    }
  }

  .stats-cell {
    display: flex;
    justify-content: center;
    gap: 12px;

    .stat-item {
      display: inline-flex;
      align-items: center;
      gap: 4px;
      color: var(--color-text-2);
      font-size: 13px;

      svg {
        color: var(--color-text-3);
      }
    }
  }

  .time-cell {
    display: flex;
    flex-direction: column;
    gap: 4px;
    font-size: 12px;

    .time-item {
      display: flex;
      align-items: center;
      gap: 4px;

      .time-label {
        color: var(--color-text-3);
      }

      .time-value {
        color: var(--color-text-2);
      }
    }
  }

  .action-cell {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 4px;
  }
}

:deep(.danger-option .arco-dropdown-option-content) {
  color: var(--status-danger-color) !important;
}
</style>

<template>
  <div class="history-page">
    <div class="page-header">
      <div class="header-content">
        <div class="header-icon">
          <icon-history />
        </div>
        <div class="header-text">
          <h1>历史记录</h1>
          <p>管理您的所有PPT项目，随时编辑和导出</p>
        </div>
      </div>
      <div class="header-actions">
        <a-button type="primary" size="large" @click="handleCreate" class="create-btn">
          <template #icon><icon-plus /></template>
          新建项目
        </a-button>
      </div>
    </div>

    <div class="page-content">
      <div class="filter-section">
        <a-form ref="searchFormRef" :model="searchForm" layout="inline">
          <a-form-item field="title">
            <a-input 
              v-model="searchForm.title" 
              placeholder="搜索项目名称..." 
              allow-clear
              class="search-input"
            >
              <template #prefix>
                <icon-search />
              </template>
            </a-input>
          </a-form-item>
          <a-form-item field="status">
            <a-select v-model="searchForm.status" placeholder="全部状态" allow-clear class="filter-select">
              <a-option value="draft">草稿</a-option>
              <a-option value="generating">生成中</a-option>
              <a-option value="completed">已完成</a-option>
              <a-option value="failed">失败</a-option>
            </a-select>
          </a-form-item>
          <a-form-item field="dateRange">
            <a-range-picker v-model="searchForm.dateRange" style="width: 260px" placeholder="选择日期范围" />
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

      <div class="view-toggle">
        <a-radio-group v-model="viewMode" type="button">
          <a-radio value="card">
            <icon-apps /> 卡片视图
          </a-radio>
          <a-radio value="list">
            <icon-list /> 列表视图
          </a-radio>
        </a-radio-group>
        <div class="batch-actions" v-if="selectedIds.length > 0">
          <a-space size="medium">
            <span class="selected-count">已选 {{ selectedIds.length }} 项</span>
            <a-button type="outline" status="danger" size="small" @click="handleBatchDelete">
              <template #icon><icon-delete /></template>
              批量删除
            </a-button>
          </a-space>
        </div>
      </div>

      <div class="content-section">
        <a-empty v-if="!loading && tableData.length === 0" description="暂无数据">
          <a-button type="primary" @click="handleCreate">
            <template #icon><icon-plus /></template>
            立即创建
          </a-button>
        </a-empty>

        <div v-else-if="viewMode === 'card'" class="card-view">
          <a-row :gutter="[20, 20]">
            <a-col :xs="24" :sm="12" :md="8" :lg="6" :xl="4" v-for="item in tableData" :key="item.id">
              <div class="project-card" :class="{ selected: selectedIds.includes(item.id) }">
                <div class="card-checkbox">
                  <a-checkbox v-model:checked="item._selected" @change="handleSelectItem(item)" />
                </div>
                <div class="card-cover" @click="handleEdit(item)">
                  <div class="cover-placeholder">
                    <icon-file />
                  </div>
                  <div class="cover-badge">
                    <a-tag :color="getStatusColor(item.status)" size="small">
                      {{ getStatusText(item.status) }}
                    </a-tag>
                  </div>
                </div>
                <div class="card-content">
                  <div class="card-title" @click="handleEdit(item)">
                    {{ item.title }}
                  </div>
                  <div class="card-desc">
                    {{ item.description || '暂无描述' }}
                  </div>
                  <div class="card-meta">
                    <span class="meta-item">
                      <icon-tag />
                      {{ getSourceTypeText(item.sourceType) }}
                    </span>
                    <span class="meta-item">
                      <icon-calendar />
                      {{ formatDate(item.createdAt) }}
                    </span>
                  </div>
                </div>
                <div class="card-actions">
                  <a-space size="small">
                    <a-tooltip content="编辑">
                      <a-button type="text" size="small" @click="handleEdit(item)">
                        <icon-edit />
                      </a-button>
                    </a-tooltip>
                    <a-tooltip content="复制">
                      <a-button type="text" size="small" @click="handleCopy(item)">
                        <icon-copy />
                      </a-button>
                    </a-tooltip>
                    <a-dropdown @select="(v) => handleMoreAction(v, item)">
                      <a-button type="text" size="small">
                        <icon-more />
                      </a-button>
                      <template #content>
                        <a-doption value="download">
                          <icon-download /> 下载
                        </a-doption>
                        <a-doption value="share">
                          <icon-share-alt /> 分享
                        </a-doption>
                        <a-doption value="delete" style="color: #ff4d4f">
                          <icon-delete /> 删除
                        </a-doption>
                      </template>
                    </a-dropdown>
                  </a-space>
                </div>
              </div>
            </a-col>
          </a-row>
        </div>

        <div v-else class="table-view">
          <a-table
            :data="tableData"
            :loading="loading"
            :pagination="false"
            :row-selection="rowSelection"
            row-key="id"
            class="history-table"
          >
            <template #columns>
              <a-table-column title="项目名称" data-index="title" :width="240">
                <template #cell="{ record }">
                  <div class="table-title-cell" @click="handleEdit(record)">
                    <div class="title-icon">
                      <icon-file />
                    </div>
                    <div class="title-text">
                      <div class="title-name">{{ record.title }}</div>
                      <div class="title-desc">{{ record.description || '暂无描述' }}</div>
                    </div>
                  </div>
                </template>
              </a-table-column>
              <a-table-column title="来源类型" data-index="sourceType" :width="120">
                <template #cell="{ record }">
                  <a-tag :color="getSourceTypeColor(record.sourceType)" size="small">
                    {{ getSourceTypeText(record.sourceType) }}
                  </a-tag>
                </template>
              </a-table-column>
              <a-table-column title="行业" data-index="industryName" :width="100" />
              <a-table-column title="状态" data-index="status" :width="100">
                <template #cell="{ record }">
                  <a-tag :color="getStatusColor(record.status)" size="small">
                    {{ getStatusText(record.status) }}
                  </a-tag>
                </template>
              </a-table-column>
              <a-table-column title="创建时间" data-index="createdAt" :width="180">
                <template #cell="{ record }">
                  <div class="date-cell">
                    <icon-calendar />
                    {{ formatDate(record.createdAt) }}
                  </div>
                </template>
              </a-table-column>
              <a-table-column title="更新时间" data-index="updatedAt" :width="180">
                <template #cell="{ record }">
                  <div class="date-cell">
                    <icon-edit />
                    {{ formatDate(record.updatedAt) }}
                  </div>
                </template>
              </a-table-column>
              <a-table-column title="操作" :width="200" fixed="right">
                <template #cell="{ record }">
                  <a-space size="small">
                    <a-button type="text" size="small" @click="handleEdit(record)">
                      <template #icon><icon-edit /></template>
                      编辑
                    </a-button>
                    <a-button type="text" size="small" @click="handleCopy(record)">
                      <template #icon><icon-copy /></template>
                      复制
                    </a-button>
                    <a-dropdown @select="(v) => handleMoreAction(v, record)">
                      <a-button type="text" size="small">
                        <template #icon><icon-more /></template>
                        更多
                      </a-button>
                      <template #content>
                        <a-doption value="download">
                          <icon-download /> 下载
                        </a-doption>
                        <a-doption value="share">
                          <icon-share-alt /> 分享
                        </a-doption>
                        <a-doption value="delete" style="color: #ff4d4f">
                          <icon-delete /> 删除
                        </a-doption>
                      </template>
                    </a-dropdown>
                  </a-space>
                </template>
              </a-table-column>
            </template>
          </a-table>
        </div>
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
          @page-size-change="handlePageSizeChange"
        />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import { useRouter } from "vue-router";
import { Message, Modal } from "@arco-design/web-vue";
import { projectApi, type PPTProject } from "@/api/modules/ppt";

const router = useRouter();
const loading = ref(false);
const viewMode = ref("card");
const tableData = ref<(PPTProject & { _selected?: boolean })[]>([]);
const selectedIds = ref<number[]>([]);

const searchForm = reactive({
  title: "",
  status: "",
  dateRange: [] as string[]
});

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const rowSelection = computed(() => ({
  type: "checkbox" as const,
  selectedRowKeys: selectedIds.value,
  onlyCurrent: true,
  onChange: (selectedRowKeys: number[]) => {
    selectedIds.value = selectedRowKeys;
    tableData.value.forEach(item => {
      item._selected = selectedRowKeys.includes(item.id);
    });
  }
}));

const getList = async () => {
  loading.value = true;
  try {
    const data = await projectApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      title: searchForm.title || undefined,
      status: searchForm.status || undefined
    });
    tableData.value = (data.list || []).map(item => ({
      ...item,
      _selected: false
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
  searchForm.title = "";
  searchForm.status = "";
  searchForm.dateRange = [];
  pagination.current = 1;
  selectedIds.value = [];
  getList();
};

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  pagination.current = 1;
  getList();
};

const handleCreate = () => {
  router.push("/ppt/generate");
};

const handleEdit = (record: PPTProject) => {
  router.push(`/ppt/editor/${record.id}`);
};

const handleCopy = async (record: PPTProject) => {
  try {
    await projectApi.copy(record.id);
    Message.success("复制成功");
    getList();
  } catch (error: any) {
    Message.error(error?.message || "复制失败");
  }
};

const handleDelete = (record: PPTProject) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除项目"${record.title}"吗？`,
    okButtonProps: { status: "danger" },
    onOk: async () => {
      try {
        await projectApi.delete(record.id);
        Message.success("删除成功");
        getList();
      } catch (error: any) {
        Message.error(error?.message || "删除失败");
      }
    }
  });
};

const handleBatchDelete = () => {
  if (selectedIds.value.length === 0) return;

  Modal.confirm({
    title: "确认删除",
    content: `确定要删除选中的 ${selectedIds.value.length} 个项目吗？`,
    okButtonProps: { status: "danger" },
    onOk: async () => {
      try {
        await Promise.all(selectedIds.value.map(id => projectApi.delete(id)));
        Message.success("批量删除成功");
        selectedIds.value = [];
        getList();
      } catch (error: any) {
        Message.error(error?.message || "删除失败");
      }
    }
  });
};

const handleSelectItem = (item: any) => {
  if (item._selected) {
    if (!selectedIds.value.includes(item.id)) {
      selectedIds.value.push(item.id);
    }
  } else {
    selectedIds.value = selectedIds.value.filter(id => id !== item.id);
  }
};

const handleMoreAction = (action: string, record: PPTProject) => {
  switch (action) {
    case "download":
      Message.info("下载功能开发中...");
      break;
    case "share":
      Message.info("分享功能开发中...");
      break;
    case "delete":
      handleDelete(record);
      break;
  }
};

const getStatusText = (status: string) => {
  const map: Record<string, string> = {
    draft: "草稿",
    generating: "生成中",
    completed: "已完成",
    failed: "失败"
  };
  return map[status] || status;
};

const getStatusColor = (status: string) => {
  const map: Record<string, string> = {
    draft: "gray",
    generating: "arcoblue",
    completed: "green",
    failed: "red"
  };
  return map[status] || "default";
};

const getSourceTypeText = (type: string) => {
  const map: Record<string, string> = {
    document: "文档上传",
    topic: "主题输入",
    outline: "大纲输入"
  };
  return map[type] || type;
};

const getSourceTypeColor = (type: string) => {
  const map: Record<string, string> = {
    document: "blue",
    topic: "green",
    outline: "orange"
  };
  return map[type] || "default";
};

const formatDate = (dateStr: string) => {
  if (!dateStr) return "-";
  const date = new Date(dateStr);
  const year = date.getFullYear();
  const month = (date.getMonth() + 1).toString().padStart(2, "0");
  const day = date.getDate().toString().padStart(2, "0");
  const hours = date.getHours().toString().padStart(2, "0");
  const minutes = date.getMinutes().toString().padStart(2, "0");
  return `${year}-${month}-${day} ${hours}:${minutes}`;
};

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.history-page {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e8ec 100%);
  padding: 24px;

  .page-header {
    background: white;
    border-radius: 16px;
    padding: 24px 32px;
    margin-bottom: 24px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
    display: flex;
    align-items: center;
    justify-content: space-between;

    .header-content {
      display: flex;
      align-items: center;
      gap: 20px;

      .header-icon {
        width: 64px;
        height: 64px;
        background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
        border-radius: 18px;
        display: flex;
        align-items: center;
        justify-content: center;
        color: white;
        font-size: 32px;
        box-shadow: 0 8px 24px rgba(240, 147, 251, 0.3);
      }

      .header-text {
        h1 {
          font-size: 24px;
          font-weight: 700;
          margin: 0 0 6px 0;
          color: #1d2129;
        }

        p {
          margin: 0;
          color: #86909c;
          font-size: 14px;
        }
      }
    }

    .create-btn {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border: none;
      box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
      transition: all 0.3s ease;

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
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
      width: 240px;
    }

    .filter-select {
      width: 140px;
    }

    .search-btn {
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border: none;
    }
  }

  .view-toggle {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;

    .batch-actions {
      .selected-count {
        color: #165dff;
        font-weight: 500;
      }
    }
  }

  .content-section {
    min-height: 400px;
  }

  .card-view {
    .project-card {
      border-radius: 12px;
      overflow: hidden;
      background: white;
      border: 2px solid transparent;
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
      transition: all 0.3s ease;
      position: relative;

      &:hover {
        transform: translateY(-4px);
        box-shadow: 0 12px 32px rgba(0, 0, 0, 0.12);
      }

      &.selected {
        border-color: #165dff;
        background: #f0f3ff;
      }

      .card-checkbox {
        position: absolute;
        top: 12px;
        left: 12px;
        z-index: 10;
      }

      .card-cover {
        aspect-ratio: 16 / 9;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        display: flex;
        align-items: center;
        justify-content: center;
        cursor: pointer;
        position: relative;

        .cover-placeholder {
          color: white;
          font-size: 48px;
          opacity: 0.8;
        }

        .cover-badge {
          position: absolute;
          top: 12px;
          right: 12px;
        }
      }

      .card-content {
        padding: 16px;

        .card-title {
          font-weight: 600;
          font-size: 15px;
          color: #1d2129;
          margin-bottom: 8px;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
          cursor: pointer;

          &:hover {
            color: #165dff;
          }
        }

        .card-desc {
          color: #86909c;
          font-size: 13px;
          margin-bottom: 12px;
          overflow: hidden;
          text-overflow: ellipsis;
          display: -webkit-box;
          -webkit-line-clamp: 2;
          -webkit-box-orient: vertical;
        }

        .card-meta {
          display: flex;
          gap: 16px;

          .meta-item {
            display: flex;
            align-items: center;
            gap: 4px;
            color: #86909c;
            font-size: 12px;
          }
        }
      }

      .card-actions {
        padding: 0 16px 16px;
        display: flex;
        justify-content: flex-end;
      }
    }
  }

  .table-view {
    .history-table {
      :deep(.arco-table-th) {
        background: #f7f8fa;
      }

      .table-title-cell {
        display: flex;
        align-items: center;
        gap: 12px;
        cursor: pointer;

        &:hover {
          .title-name {
            color: #165dff;
          }
        }

        .title-icon {
          width: 40px;
          height: 40px;
          background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
          border-radius: 10px;
          display: flex;
          align-items: center;
          justify-content: center;
          color: white;
          flex-shrink: 0;
        }

        .title-text {
          flex: 1;
          min-width: 0;

          .title-name {
            font-weight: 500;
            color: #1d2129;
            margin-bottom: 4px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
          }

          .title-desc {
            color: #86909c;
            font-size: 12px;
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
          }
        }
      }

      .date-cell {
        display: flex;
        align-items: center;
        gap: 6px;
        color: #86909c;
      }
    }
  }

  .pagination-wrapper {
    margin-top: 32px;
    display: flex;
    justify-content: center;
  }
}
</style>
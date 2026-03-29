<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="title" label="项目名称">
              <a-input v-model="searchForm.title" placeholder="请输入项目名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="status" label="状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option value="draft">草稿</a-option>
                <a-option value="generating">生成中</a-option>
                <a-option value="completed">已完成</a-option>
                <a-option value="failed">失败</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="8" :xl="6" :xxl="6">
            <a-form-item field="dateRange" label="创建时间">
              <a-range-picker v-model="searchForm.dateRange" style="width: 100%" />
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

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="handleCreate">
              <template #icon><icon-plus /></template>
              新建项目
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedIds.length === 0" @click="handleBatchDelete">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        :data="tableData"
        :loading="loading"
        :pagination="pagination"
        :row-selection="rowSelection"
        row-key="id"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #columns>
          <a-table-column title="项目名称" data-index="title" :width="200">
            <template #cell="{ record }">
              <a-link @click="handleEdit(record)">{{ record.title }}</a-link>
            </template>
          </a-table-column>
          <a-table-column title="描述" data-index="description" :width="250" ellipsis />
          <a-table-column title="来源类型" data-index="sourceType" :width="100">
            <template #cell="{ record }">
              <a-tag v-if="record.sourceType === 'document'" color="blue">文档</a-tag>
              <a-tag v-else-if="record.sourceType === 'topic'" color="green">主题</a-tag>
              <a-tag v-else-if="record.sourceType === 'outline'" color="orange">大纲</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="行业" data-index="industryName" :width="100" />
          <a-table-column title="状态" data-index="status" :width="100">
            <template #cell="{ record }">
              <a-tag v-if="record.status === 'draft'" color="gray">草稿</a-tag>
              <a-tag v-else-if="record.status === 'generating'" color="blue">生成中</a-tag>
              <a-tag v-else-if="record.status === 'completed'" color="green">已完成</a-tag>
              <a-tag v-else-if="record.status === 'failed'" color="red">失败</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="创建时间" data-index="createdAt" :width="180" />
          <a-table-column title="更新时间" data-index="updatedAt" :width="180" />
          <a-table-column title="操作" :width="200" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleEdit(record)">
                  <template #icon><icon-edit /></template>
                  编辑
                </a-button>
                <a-button type="text" size="small" @click="handleCopy(record)">
                  <template #icon><icon-copy /></template>
                  复制
                </a-button>
                <a-button type="text" size="small" status="danger" @click="handleDelete(record)">
                  <template #icon><icon-delete /></template>
                  删除
                </a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
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
const tableData = ref<PPTProject[]>([]);
const selectedIds = ref<number[]>([]);

const searchForm = reactive({
  title: "",
  status: "",
  dateRange: [] as string[]
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const rowSelection = computed(() => ({
  type: "checkbox" as const,
  selectedRowKeys: selectedIds.value,
  onlyCurrent: true
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
    tableData.value = data.list || [];
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
  getList();
};

const refresh = () => {
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

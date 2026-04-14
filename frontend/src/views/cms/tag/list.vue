<template>
  <div class="tag-list-page">
    <div class="page-wrapper">
      <div class="page-header">
        <div class="header-left">
          <a-form ref="searchFormRef" :model="searchForm" class="search-form">
            <a-space size="medium">
              <a-input-search
                v-model="searchForm.name"
                placeholder="搜索标签名称"
                style="width: 220px"
                @search="search"
                allow-clear
              />
              <a-input
                v-model="searchForm.code"
                placeholder="标签编码"
                style="width: 160px"
                allow-clear
              />
              <a-select
                v-model="searchForm.status"
                placeholder="状态"
                style="width: 120px"
                allow-clear
              >
                <a-option :value="true">启用</a-option>
                <a-option :value="false">禁用</a-option>
              </a-select>
              <a-button type="primary" @click="search">
                <template #icon><icon-search /></template>
                查询
              </a-button>
              <a-button @click="reset">
                <template #icon><icon-refresh /></template>
                重置
              </a-button>
            </a-space>
          </a-form>
        </div>
        <div class="header-right">
          <a-space size="medium">
            <a-tooltip content="标签云">
              <a-button shape="circle" @click="showTagCloud">
                <icon-apps />
              </a-button>
            </a-tooltip>
            <a-tooltip content="刷新数据">
              <a-button shape="circle" @click="refresh">
                <icon-refresh />
              </a-button>
            </a-tooltip>
          </a-space>
        </div>
      </div>

      <div class="page-toolbar">
        <div class="toolbar-left">
          <a-space size="medium">
            <a-button type="primary" size="middle" @click="onAdd">
              <template #icon><icon-plus /></template>
              新建标签
            </a-button>
            <a-button size="middle" @click="onBatchAdd">
              <template #icon><icon-plus-multiple /></template>
              批量添加
            </a-button>
            <div v-if="selectedIds.length > 0" class="batch-actions">
              <a-tag color="arcoblue" :closable="false">已选 {{ selectedIds.length }} 项</a-tag>
              <a-button status="danger" size="small" @click="onBatchDelete">
                <template #icon><icon-delete /></template>
                批量删除
              </a-button>
            </div>
          </a-space>
        </div>
      </div>

      <div class="table-container">
        <TagTable
          :loading="loading"
          :data="tableData"
          :pagination="pagination"
          :selected-keys="selectedIds"
          @page-change="handlePageChange"
          @page-size-change="handlePageSizeChange"
          @selection-change="onSelectionChange"
          @edit="onEdit"
          @delete="onDelete"
          @toggle-status="onToggleStatus"
        />
      </div>
    </div>

    <a-modal v-model:visible="modalVisible" :width="500" @ok="onSubmit" @cancel="onCancel">
      <template #title>
        <div class="modal-title">
          <icon-plus v-if="!form.id" />
          <icon-edit v-else />
          <span>{{ modalTitle }}</span>
        </div>
      </template>
      <div class="modal-content">
        <a-form ref="formRef" auto-label-width :rules="rules" :model="form" layout="vertical">
          <a-form-item field="name" label="标签名称" validate-trigger="blur">
            <a-input v-model="form.name" placeholder="请输入标签名称" allow-clear />
          </a-form-item>
          <a-form-item field="code" label="标签编码" validate-trigger="blur">
            <a-input v-model="form.code" placeholder="请输入标签编码" allow-clear :disabled="!!form.id" />
          </a-form-item>
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="color" label="标签颜色">
                <a-input v-model="form.color" placeholder="#1890ff" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="sort" label="排序">
                <a-input-number v-model="form.sort" :min="0" :max="9999" style="width: 100%" mode="button" />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="status" label="标签状态">
            <a-switch type="round" v-model="form.status">
              <template #checked>启用</template>
              <template #unchecked>禁用</template>
            </a-switch>
          </a-form-item>
          <a-form-item field="description" label="标签描述">
            <a-textarea v-model="form.description" placeholder="请输入标签描述" allow-clear :auto-size="{ minRows: 3, maxRows: 5 }" />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>

    <a-modal
      v-model:visible="batchModalVisible"
      :width="600"
      @ok="onBatchSubmit"
      @cancel="batchModalVisible = false"
    >
      <template #title>
        <div class="modal-title">
          <icon-plus-multiple />
          <span>批量添加标签</span>
        </div>
      </template>
      <div class="modal-content">
        <a-form ref="batchFormRef" :model="batchForm" layout="vertical">
          <a-form-item label="标签列表">
            <a-textarea
              v-model="batchForm.tagsText"
              placeholder="请输入标签，每行一个标签，格式：标签名称|标签编码|颜色（可选）"
              :auto-size="{ minRows: 10, maxRows: 15 }"
            />
          </a-form-item>
        </a-form>
        <div class="batch-tips">
          <p class="tips-title">格式说明：</p>
          <pre>技术|tech|#1890ff
生活|life|#52c41a
旅行|travel|#fa8c16</pre>
        </div>
      </div>
    </a-modal>

    <a-modal v-model:visible="tagCloudVisible" :width="900" :footer="false">
      <template #title>
        <div class="modal-title">
          <icon-apps />
          <span>标签云</span>
        </div>
      </template>
      <div class="tag-cloud-container">
        <TagCloud :tags="tagCloudData" @select="onTagSelect" />
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { tagApi, type CmsTagItem, type TagAddParams, type TagEditParams } from "@/api/modules/cms/tag";
import TagTable from "./components/TagTable.vue";
import TagCloud from "./components/TagCloud.vue";

const loading = ref(false);
const tableData = ref<CmsTagItem[]>([]);
const selectedIds = ref<number[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增标签");
const formRef = ref();
const searchFormRef = ref();
const batchModalVisible = ref(false);
const batchFormRef = ref();
const tagCloudVisible = ref(false);
const tagCloudData = ref<CmsTagItem[]>([]);

const searchForm = reactive({
  name: "",
  code: "",
  status: null as boolean | null
});

const form = reactive({
  id: 0,
  name: "",
  code: "",
  color: "",
  sort: 0,
  status: true,
  description: ""
});

const batchForm = reactive({
  tagsText: ""
});

const rules = {
  name: [{ required: true, message: "请输入标签名称" }],
  code: [{ required: true, message: "请输入标签编码" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 20,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const getList = async () => {
  loading.value = true;
  try {
    const data = await tagApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      code: searchForm.code || undefined,
      status: searchForm.status ?? undefined
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
  searchFormRef.value?.resetFields();
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

const onSelectionChange = (keys: (string | number)[]) => {
  selectedIds.value = keys as number[];
};

const onAdd = () => {
  modalTitle.value = "新增标签";
  form.id = 0;
  form.name = "";
  form.code = "";
  form.color = "";
  form.sort = 0;
  form.status = true;
  form.description = "";
  modalVisible.value = true;
};

const onEdit = (record: CmsTagItem) => {
  modalTitle.value = "编辑标签";
  form.id = record.id;
  form.name = record.name;
  form.code = record.code;
  form.color = record.color || "";
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || "";
  modalVisible.value = true;
};

const onDelete = async (record: CmsTagItem) => {
  try {
    await tagApi.delete(record.id);
    arcoMessage("success", "删除成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  Modal.warning({
    title: "确认删除",
    content: `确定要删除选中的${selectedIds.value.length}个标签吗？`,
    hideCancel: false,
    onOk: async () => {
      try {
        await tagApi.batchDelete(selectedIds.value);
        arcoMessage("success", "批量删除成功");
        selectedIds.value = [];
        getList();
      } catch (error) {
        console.error(error);
      }
    }
  });
};

const onToggleStatus = async (record: CmsTagItem) => {
  try {
    await tagApi.updateStatus(record.id, !record.status);
    arcoMessage("success", record.status ? "禁用成功" : "启用成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  const state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await tagApi.edit(form as TagEditParams);
      arcoMessage("success", "修改成功");
    } else {
      await tagApi.add(form as TagAddParams);
      arcoMessage("success", "新增成功");
    }
    modalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onCancel = () => {
  modalVisible.value = false;
  formRef.value?.resetFields();
};

const onBatchAdd = () => {
  batchForm.tagsText = "";
  batchModalVisible.value = true;
};

const onBatchSubmit = async () => {
  if (!batchForm.tagsText.trim()) {
    arcoMessage("warning", "请输入标签");
    return;
  }
  const lines = batchForm.tagsText.split("\n").filter(Boolean);
  const tags = lines
    .map(line => {
      const parts = line.split("|");
      return {
        name: parts[0]?.trim() || "",
        code: parts[1]?.trim() || parts[0]?.trim() || "",
        color: parts[2]?.trim() || undefined
      };
    })
    .filter(t => t.name && t.code);

  if (tags.length === 0) {
    arcoMessage("warning", "请输入有效的标签");
    return;
  }

  try {
    await tagApi.batchAdd({ tags });
    arcoMessage("success", "批量添加成功");
    batchModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const showTagCloud = async () => {
  try {
    tagCloudData.value = await tagApi.cloud(100);
    tagCloudVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onTagSelect = (tag: CmsTagItem) => {
  searchForm.name = tag.name;
  tagCloudVisible.value = false;
  search();
};

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.tag-list-page {
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
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--color-bg-2);
    border-radius: 8px;
    padding: 16px;
    border: 1px solid var(--color-border-1);

    .header-left {
      flex: 1;
      .search-form {
        display: flex;
        align-items: center;
      }
    }

    .header-right {
      display: flex;
      align-items: center;
    }
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
  }

  .table-container {
    flex: 1;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);
    overflow: hidden;
  }
}

.modal-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
}

.modal-content {
  padding: 8px 0;
}

.batch-tips {
  margin-top: 16px;
  padding: 16px;
  background: var(--color-bg-3);
  border-radius: 6px;

  .tips-title {
    margin: 0 0 8px;
    font-weight: 500;
    color: var(--color-text-2);
  }

  pre {
    margin: 0;
    padding: 12px;
    background: var(--color-bg-2);
    border-radius: 4px;
    font-size: 13px;
    color: var(--color-text-2);
    line-height: 1.8;
  }
}

.tag-cloud-container {
  padding: 20px 0;
}
</style>

<template>
  <div class="brand-page">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">品牌管理</h2>
        <div class="page-stats">
          <a-statistic title="全部品牌" :value="stats.total" :value-style="{ color: '#165dff' }" />
          <a-divider direction="vertical" />
          <a-statistic title="已启用" :value="stats.active" :value-style="{ color: '#00b42a' }" />
          <a-divider direction="vertical" />
          <a-statistic title="已禁用" :value="stats.inactive" :value-style="{ color: '#f53f3f' }" />
        </div>
      </div>
      <div class="header-right">
        <a-space size="medium">
          <a-button-group>
            <a-button 
              :type="viewMode === 'grid' ? 'primary' : 'secondary'" 
              @click="viewMode = 'grid'"
            >
              <template #icon><icon-apps /></template>
              卡片视图
            </a-button>
            <a-button 
              :type="viewMode === 'list' ? 'primary' : 'secondary'" 
              @click="viewMode = 'list'"
            >
              <template #icon><icon-list /></template>
              列表视图
            </a-button>
          </a-button-group>
          <a-button type="primary" size="large" @click="onAdd">
            <template #icon><icon-plus /></template>
            添加品牌
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="filter-card">
      <a-form ref="searchFormRef" :model="searchForm" layout="inline">
        <a-form-item field="name" label="品牌名称">
          <a-input 
            v-model="searchForm.name" 
            placeholder="搜索品牌名称" 
            allow-clear 
            class="search-input"
          >
            <template #prefix><icon-search /></template>
          </a-input>
        </a-form-item>
        <a-form-item field="status" label="品牌状态">
          <a-select v-model="searchForm.status" placeholder="选择状态" allow-clear style="width: 140px">
            <a-option value="0">已启用</a-option>
            <a-option value="1">已禁用</a-option>
          </a-select>
        </a-form-item>
        <a-form-item>
          <a-space>
            <a-button type="primary" @click="search">
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

    <div class="action-bar">
      <div class="action-left">
        <a-space size="medium">
          <a-button 
            type="primary" 
            status="success" 
            :disabled="selectedIds.length === 0" 
            @click="onBatchUpdateStatus('0')"
          >
            <template #icon><icon-check-circle /></template>
            批量启用
          </a-button>
          <a-button 
            type="primary" 
            status="warning" 
            :disabled="selectedIds.length === 0" 
            @click="onBatchUpdateStatus('1')"
          >
            <template #icon><icon-pause-circle /></template>
            批量禁用
          </a-button>
          <a-button 
            type="primary" 
            status="danger" 
            :disabled="selectedIds.length === 0" 
            @click="onBatchDelete"
          >
            <template #icon><icon-delete /></template>
            批量删除
          </a-button>
        </a-space>
      </div>
      <div class="action-right">
        <a-space size="medium">
          <span class="selected-count" v-if="selectedIds.length > 0">
            已选择 {{ selectedIds.length }} 项
          </span>
          <a-tooltip content="刷新">
            <div class="action-icon" @click="refresh">
              <icon-refresh size="18" />
            </div>
          </a-tooltip>
        </a-space>
      </div>
    </div>

    <div class="content-area">
      <a-row :gutter="20" v-if="viewMode === 'grid'">
        <a-col 
          :xs="24" 
          :sm="12" 
          :md="8" 
          :lg="6" 
          :xl="6" 
          v-for="item in tableData" 
          :key="item.id"
        >
          <a-card 
            hoverable 
            class="brand-card"
            :class="{ 'card-selected': selectedIds.includes(String(item.id)) }"
          >
            <div class="card-checkbox" @click.stop>
              <a-checkbox v-model:checked="selectedIds.includes(String(item.id))" />
            </div>
            <div class="brand-image">
              <img :src="item.logo || defaultBrandLogo" :alt="item.name" />
            </div>
            <div class="brand-info">
              <div class="brand-name">{{ item.name }}</div>
              <div class="brand-status">
                <a-tag :color="item.status === '0' ? 'green' : 'red'" size="small">
                  {{ item.status === '0' ? '已启用' : '已禁用' }}
                </a-tag>
              </div>
              <div class="brand-website" v-if="item.website">
                <icon-global />
                <a href :href="item.website" target="_blank">{{ item.website }}</a>
              </div>
            </div>
            <div class="card-actions">
              <a-button type="text" size="small" @click.stop="onEdit(item)">
                <template #icon><icon-edit /></template>
                编辑
              </a-button>
              <a-dropdown trigger="click">
                <a-button type="text" size="small">
                  更多
                  <template #icon><icon-down /></template>
                </a-button>
                <template #content>
                  <a-doption v-if="item.status === '0'" @click.stop="onUpdateStatus(item, '1')">
                    <icon-pause-circle /> 禁用
                  </a-doption>
                  <a-doption v-if="item.status === '1'" @click.stop="onUpdateStatus(item, '0')">
                    <icon-play-circle /> 启用
                  </a-doption>
                  <a-doption @click.stop="onCopy(item)">
                    <icon-copy /> 复制
                  </a-doption>
                  <a-divider style="margin: 4px 0" />
                  <a-doption @click.stop="onDelete(item)" style="color: #f53f3f">
                    <icon-delete /> 删除
                  </a-doption>
                </template>
              </a-dropdown>
            </div>
          </a-card>
        </a-col>
      </a-row>

      <a-table
        v-else
        row-key="id"
        :loading="loading"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        class="brand-table"
      >
        <template #logo="{ record }">
          <a-image-preview>
            <div class="brand-logo">
              <img :src="record.logo || defaultBrandLogo" :alt="record.name" />
            </div>
          </a-image-preview>
        </template>
        <template #name="{ record }">
          <div class="brand-name-cell">
            <div class="name">{{ record.name }}</div>
            <div class="website" v-if="record.website">
              <icon-global />
              <a href :href="record.website" target="_blank">{{ record.website }}</a>
            </div>
          </div>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" size="large">
            {{ record.status === '0' ? '已启用' : '已禁用' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <div class="action-cell">
            <a-space size="small">
              <a-button type="text" size="small" @click="onEdit(record)">
                <template #icon><icon-edit /></template>
                编辑
              </a-button>
              <a-dropdown trigger="click">
                <a-button type="text" size="small">
                  更多
                  <template #icon><icon-down /></template>
                </a-button>
                <template #content>
                  <a-doption v-if="record.status === '0'" @click="onUpdateStatus(record, '1')">
                    <icon-pause-circle /> 禁用
                  </a-doption>
                  <a-doption v-if="record.status === '1'" @click="onUpdateStatus(record, '0')">
                    <icon-play-circle /> 启用
                  </a-doption>
                  <a-doption @click="onCopy(record)">
                    <icon-copy /> 复制
                  </a-doption>
                  <a-divider style="margin: 4px 0" />
                  <a-doption @click="onDelete(record)" style="color: #f53f3f">
                    <icon-delete /> 删除
                  </a-doption>
                </template>
              </a-dropdown>
            </a-space>
          </div>
        </template>
      </a-table>

      <div class="pagination-wrapper" v-if="viewMode === 'grid'">
        <a-pagination
          :current="pagination.current"
          :page-size="pagination.pageSize"
          :total="pagination.total"
          :show-total
          :show-page-size
          @page-change="handlePageChange"
          @page-size-change="handlePageSizeChange"
        />
      </div>
    </div>

    <a-modal
      v-model:visible="modalVisible"
      :title="modalTitle"
      :width="600"
      @ok="onSubmit"
      @cancel="onCancel"
      class="brand-modal"
    >
      <a-form ref="formRef" :rules="rules" :model="form" layout="vertical">
        <div class="form-header">
          <div class="logo-upload">
            <div class="logo-preview">
              <img :src="form.logo || defaultBrandLogo" :alt="form.name" />
            </div>
            <div class="logo-actions">
              <a-button size="small">
                <template #icon><icon-upload /></template>
                上传Logo
              </a-button>
            </div>
          </div>
          <div class="form-basic">
            <a-form-item field="name" label="品牌名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入品牌名称" allow-clear />
            </a-form-item>
            <a-form-item field="website" label="官方网站" validate-trigger="blur">
              <a-input v-model="form.website" placeholder="请输入官方网站地址" allow-clear />
            </a-form-item>
          </div>
        </div>
        <a-form-item field="logo" label="Logo URL">
          <a-input v-model="form.logo" placeholder="请输入Logo图片URL" allow-clear />
        </a-form-item>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="sort" label="排序" validate-trigger="blur">
              <a-input-number
                v-model="form.sort"
                :min="0"
                :max="9999"
                style="width: 100%"
                placeholder="排序"
                mode="button"
              />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="status" label="品牌状态">
              <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
                <template #checked>
                  <icon-check /> 启用
                </template>
                <template #unchecked>
                  <icon-close /> 禁用
                </template>
              </a-switch>
            </a-form-item>
          </a-col>
        </a-row>
        <a-form-item field="description" label="品牌描述">
          <a-textarea v-model="form.description" placeholder="请输入品牌描述（选填）" :rows="4" :max-length="500" show-word-limit />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { brandApi, BrandListItem } from "@/api/modules/product/brand";

const loading = ref(false);
const tableData = ref<BrandListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增品牌");
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const viewMode = ref<"grid" | "list">("grid");

const defaultBrandLogo = "https://via.placeholder.com/200x200?text=Brand";

const stats = reactive({
  total: 0,
  active: 0,
  inactive: 0
});

const searchForm = reactive({
  name: "",
  status: null as string | null
});

const form = reactive({
  id: 0,
  name: "",
  logo: "",
  website: "",
  sort: 0,
  status: "0",
  description: ""
});

const rules = {
  name: [{ required: true, message: "请输入品牌名称" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { type: "selection", width: 60, fixed: "left" },
  { title: "品牌Logo", dataIndex: "logo", slotName: "logo", width: 120 },
  { title: "品牌信息", dataIndex: "name", slotName: "name", width: 280 },
  { title: "排序", dataIndex: "sort", width: 100, align: "center" },
  { title: "状态", slotName: "status", width: 120, align: "center" },
  { title: "创建时间", dataIndex: "createdAt", width: 180 },
  { title: "操作", slotName: "optional", width: 180, fixed: "right" }
];

const getList = async () => {
  loading.value = true;
  try {
    const data = await brandApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status || undefined
    });
    tableData.value = data.list || [];
    pagination.total = data.total || 0;
    calculateStats(data.list || []);
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const calculateStats = (list: BrandListItem[]) => {
  stats.total = list.length;
  stats.active = list.filter(item => item.status === "0").length;
  stats.inactive = list.filter(item => item.status === "1").length;
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

const onAdd = () => {
  modalTitle.value = "新增品牌";
  form.id = 0;
  form.name = "";
  form.logo = "";
  form.website = "";
  form.sort = 0;
  form.status = "0";
  form.description = "";
  modalVisible.value = true;
};

const onEdit = (record: BrandListItem) => {
  modalTitle.value = "编辑品牌";
  form.id = record.id;
  form.name = record.name;
  form.logo = record.logo || "";
  form.website = record.website || "";
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || "";
  modalVisible.value = true;
};

const onUpdateStatus = async (record: BrandListItem, status: string) => {
  try {
    await brandApi.edit({ ...record, status });
    arcoMessage("success", status === "0" ? "启用成功" : "禁用成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onCopy = (record: BrandListItem) => {
  modalTitle.value = "新增品牌";
  form.id = 0;
  form.name = record.name + "(副本)";
  form.logo = record.logo || "";
  form.website = record.website || "";
  form.sort = record.sort;
  form.status = "0";
  form.description = record.description || "";
  modalVisible.value = true;
};

const onDelete = (record: BrandListItem) => {
  arcoMessage("warning", "确定删除该品牌?");
};

const onBatchUpdateStatus = (status: string) => {
  arcoMessage("info", `批量${status === "0" ? "启用" : "禁用"} ${selectedIds.value.length} 个品牌`);
};

const onBatchDelete = () => {
  if (selectedIds.value.length === 0) return;
  arcoMessage("warning", `确定删除选中的 ${selectedIds.value.length} 个品牌?`);
};

const onSubmit = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    if (form.id) {
      await brandApi.edit(form);
      arcoMessage("success", "修改成功");
    } else {
      await brandApi.add(form);
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

onMounted(() => {
  getList();
});
</script>

<style scoped lang="scss">
.brand-page {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}

.header-left {
  .page-title {
    font-size: 24px;
    font-weight: 600;
    color: #1d2129;
    margin: 0 0 12px 0;
  }
  
  .page-stats {
    display: flex;
    align-items: center;
  }
}

.filter-card {
  background: #ffffff;
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 16px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  
  .search-input {
    width: 280px;
  }
}

.action-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 0;
  margin-bottom: 16px;
  
  .action-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 36px;
    height: 36px;
    border-radius: 50%;
    cursor: pointer;
    transition: all 0.2s;
    &:hover {
      background-color: var(--color-fill-2);
    }
  }
  
  .selected-count {
    color: #165dff;
    font-weight: 500;
  }
}

.content-area {
  flex: 1;
  overflow: auto;
  background: #ffffff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.brand-table {
  :deep(.arco-table-th) {
    background-color: #f7f8fa;
  }
}

.brand-logo {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e5e6eb;
  background: #f7f8fa;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.brand-name-cell {
  .name {
    font-weight: 500;
    color: #1d2129;
    margin-bottom: 4px;
  }
  
  .website {
    font-size: 12px;
    color: #86909c;
    display: flex;
    align-items: center;
    gap: 4px;
    
    a {
      color: #165dff;
      text-decoration: none;
      
      &:hover {
        text-decoration: underline;
      }
    }
  }
}

.action-cell {
  display: flex;
  justify-content: flex-start;
}

.brand-card {
  margin-bottom: 20px;
  position: relative;
  transition: all 0.3s;
  
  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }
  
  &.card-selected {
    border: 2px solid #165dff;
  }
}

.card-checkbox {
  position: absolute;
  top: 12px;
  left: 12px;
  z-index: 10;
}

.brand-image {
  width: 100%;
  height: 160px;
  overflow: hidden;
  border-radius: 8px;
  background: #f7f8fa;
  margin-bottom: 16px;
  
  img {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }
}

.brand-info {
  .brand-name {
    font-size: 16px;
    font-weight: 600;
    color: #1d2129;
    margin-bottom: 8px;
  }
  
  .brand-status {
    margin-bottom: 8px;
  }
  
  .brand-website {
    font-size: 12px;
    color: #86909c;
    display: flex;
    align-items: center;
    gap: 4px;
    
    a {
      color: #165dff;
      text-decoration: none;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
      
      &:hover {
        text-decoration: underline;
      }
    }
  }
}

.card-actions {
  display: flex;
  justify-content: flex-end;
  padding-top: 12px;
  border-top: 1px solid #e5e6eb;
  margin-top: 12px;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 24px;
}

.form-header {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
}

.logo-upload {
  flex-shrink: 0;
}

.logo-preview {
  width: 120px;
  height: 120px;
  border-radius: 8px;
  overflow: hidden;
  border: 2px dashed #e5e6eb;
  background: #f7f8fa;
  margin-bottom: 12px;
  
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.logo-actions {
  text-align: center;
}

.form-basic {
  flex: 1;
}

.brand-modal {
  :deep(.arco-modal-body) {
    padding-top: 24px;
  }
}
</style>

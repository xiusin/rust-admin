<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="分组名称">
              <a-input v-model="searchForm.name" placeholder="请输入分组名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="分组状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option value="0">启用</a-option>
                <a-option value="1">禁用</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" size="small" @click="search">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button size="small" @click="reset">
                <template #icon><icon-refresh /></template>
                <template #default>重置</template>
              </a-button>
            </a-space>
          </a-col>
        </a-row>
      </a-form>

      <a-divider :margin="0" />

      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
            <a-button type="primary" status="danger" size="small" :disabled="selectedIds.length === 0" @click="onBatchDelete">
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
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" bordered size="small">
            {{ record.status === "0" ? "启用" : "禁用" }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onAssignProduct(record)">商品</a-button>
            <a-popconfirm type="warning" content="确定删除该分组吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="500" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
        <a-form-item field="name" label="分组名称" validate-trigger="blur">
          <a-input v-model="form.name" placeholder="请输入分组名称" allow-clear />
        </a-form-item>
        <a-form-item field="description" label="分组描述" validate-trigger="blur">
          <a-textarea v-model="form.description" placeholder="请输入分组描述" :rows="3" allow-clear />
        </a-form-item>
        <a-form-item field="sort" label="排序" validate-trigger="blur">
          <a-input-number
            v-model="form.sort"
            :min="0"
            :max="9999"
            :style="{ width: '150px' }"
            placeholder="请输入排序"
            mode="button"
          />
        </a-form-item>
        <a-form-item field="status" label="状态" validate-trigger="blur">
          <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
            <template #checked> 启用 </template>
            <template #unchecked> 禁用 </template>
          </a-switch>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal
      v-model:visible="productModalVisible"
      title="分配商品"
      :width="800"
      @ok="onSubmitAssign"
      @cancel="productModalVisible = false"
    >
      <a-transfer
        :data="transferData"
        :target-keys="selectedProductIds"
        :titles="['未关联商品', '已关联商品']"
        :pagination="{ pageSize: 10 }"
        @change="handleTransferChange"
      />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { productGroupApi, ProductGroupListItem } from "@/api/modules/product/productGroup";
import { productApi, ProductListItem } from "@/api/modules/product/product";

const loading = ref(false);
const tableData = ref<ProductGroupListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增分组");
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const productModalVisible = ref(false);
const currentGroupId = ref(0);
const productList = ref<ProductListItem[]>([]);
const transferData = ref<{ key: string; label: string }[]>([]);
const selectedProductIds = ref<string[]>([]);

const searchForm = reactive({
  name: "",
  status: null as string | null
});

const form = reactive({
  id: 0,
  name: "",
  description: "",
  sort: 0,
  status: "0"
});

const rules = {
  name: [{ required: true, message: "请输入分组名称" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { type: "selection", width: 60, fixed: "left" },
  { title: "分组名称", dataIndex: "name", width: 200 },
  { title: "分组描述", dataIndex: "description", ellipsis: true },
  { title: "商品数量", dataIndex: "productCount", width: 100, align: "center" },
  { title: "排序", dataIndex: "sort", width: 80, align: "center" },
  { title: "状态", slotName: "status", width: 100, align: "center" },
  { title: "创建时间", dataIndex: "createdAt", width: 180 },
  { title: "操作", slotName: "optional", width: 180, fixed: "right" }
];

const getList = async () => {
  loading.value = true;
  try {
    const data = await productGroupApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
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

const getProductList = async () => {
  try {
    const data = await productApi.list({ pageSize: 100 });
    productList.value = data.list || [];
    transferData.value = productList.value.map(item => ({
      key: String(item.id),
      label: item.name
    }));
  } catch (error) {
    console.error(error);
  }
};

const getGroupDetail = async (id: number) => {
  try {
    const data = await productGroupApi.detail(id);
    const groupIds = (data as any).groupIds || [];
    selectedProductIds.value = groupIds.map(String);
  } catch (error) {
    console.error(error);
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

const onAdd = () => {
  modalTitle.value = "新增分组";
  form.id = 0;
  form.name = "";
  form.description = "";
  form.sort = 0;
  form.status = "0";
  modalVisible.value = true;
};

const onEdit = (record: ProductGroupListItem) => {
  modalTitle.value = "编辑分组";
  form.id = record.id;
  form.name = record.name;
  form.description = record.description || "";
  form.sort = record.sort;
  form.status = record.status;
  modalVisible.value = true;
};

const onDelete = async (record: ProductGroupListItem) => {
  try {
    await productGroupApi.delete([record.id]);
    arcoMessage("success", "删除成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await productGroupApi.delete(selectedIds.value.map(id => Number(id)));
    arcoMessage("success", "批量删除成功");
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onAssignProduct = async (record: ProductGroupListItem) => {
  currentGroupId.value = record.id;
  selectedProductIds.value = [];
  await getProductList();
  await getGroupDetail(record.id);
  productModalVisible.value = true;
};

const handleTransferChange = (keys: string[]) => {
  selectedProductIds.value = keys;
};

const onSubmit = async () => {
  let state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await productGroupApi.edit(form);
      arcoMessage("success", "修改成功");
    } else {
      await productGroupApi.add(form);
      arcoMessage("success", "新增成功");
    }
    modalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmitAssign = async () => {
  try {
    await productGroupApi.assignProducts(
      currentGroupId.value,
      selectedProductIds.value.map(id => Number(id))
    );
    arcoMessage("success", "分配成功");
    productModalVisible.value = false;
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

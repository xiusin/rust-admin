<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="门店名称">
              <a-input v-model="searchForm.name" placeholder="请输入门店名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="门店状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option value="0">营业中</a-option>
                <a-option value="1">已停业</a-option>
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
        :scroll="{ x: '100%', y: '100%', minWidth: 1400 }"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
      >
        <template #logo="{ record }">
          <a-image-preview>
            <div class="store-logo" v-if="record.logo">
              <img :src="record.logo" :alt="record.name" />
            </div>
            <div class="store-logo empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" bordered size="small">
            {{ record.status === '0' ? '营业中' : '已停业' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onViewStock(record)">库存</a-button>
            <a-popconfirm type="warning" content="确定删除该门店吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="800" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="name" label="门店名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入门店名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="contactName" label="联系人" validate-trigger="blur">
              <a-input v-model="form.contactName" placeholder="请输入联系人" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="contactPhone" label="联系电话" validate-trigger="blur">
              <a-input v-model="form.contactPhone" placeholder="请输入联系电话" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="businessHours" label="营业时间" validate-trigger="blur">
              <a-input v-model="form.businessHours" placeholder="如: 09:00-21:00" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="sort" label="排序" validate-trigger="blur">
              <a-input-number v-model="form.sort" :min="0" :max="9999" :style="{ width: '150px' }" placeholder="请输入排序" mode="button" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="status" label="状态" validate-trigger="blur">
              <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
                <template #checked> 营业中 </template>
                <template #unchecked> 已停业 </template>
              </a-switch>
            </a-form-item>
          </a-col>
          <a-col :span="24">
            <a-form-item field="logo" label="门店Logo" validate-trigger="blur">
              <a-input v-model="form.logo" placeholder="请输入Logo URL" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="province" label="省份" validate-trigger="blur">
              <a-input v-model="form.province" placeholder="请输入省份" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="city" label="城市" validate-trigger="blur">
              <a-input v-model="form.city" placeholder="请输入城市" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="district" label="区/县" validate-trigger="blur">
              <a-input v-model="form.district" placeholder="请输入区/县" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="24">
            <a-form-item field="address" label="详细地址" validate-trigger="blur">
              <a-input v-model="form.address" placeholder="请输入详细地址" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="24">
            <a-form-item field="description" label="门店描述" validate-trigger="blur">
              <a-textarea v-model="form.description" placeholder="请输入门店描述" :rows="3" allow-clear />
            </a-form-item>
          </a-col>
        </a-row>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="stockModalVisible" title="门店库存" :width="900" :footer="null">
      <a-table
        :loading="stockLoading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: 400 }"
        :columns="stockColumns"
        :data="stockTableData"
        :pagination="stockPagination"
        @page-change="handleStockPageChange"
        @page-size-change="handleStockPageSizeChange"
      >
        <template #productImage="{ record }">
          <div class="product-image" v-if="record.productImage">
            <img :src="record.productImage" :alt="record.productName" />
          </div>
          <div class="product-image empty" v-else>
            <icon-image />
          </div>
        </template>
        <template #optional="{ record }">
          <a-button type="text" size="mini" @click="onAdjustStock(record)">调整</a-button>
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="adjustStockModalVisible" title="库存调整" :width="500" @ok="onSubmitAdjustStock" @cancel="adjustStockModalVisible = false">
      <a-form :model="adjustStockForm" auto-label-width layout="vertical">
        <a-form-item label="商品">
          <a-input :value="adjustStockForm.productName" disabled />
        </a-form-item>
        <a-form-item label="SKU" v-if="adjustStockForm.skuCode">
          <a-input :value="adjustStockForm.skuCode" disabled />
        </a-form-item>
        <a-form-item label="当前库存">
          <a-input-number v-model="adjustStockForm.currentStock" disabled :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item field="changeNum" label="调整数量">
          <a-input-number v-model="adjustStockForm.changeNum" placeholder="正数增加，负数减少" style="width: 100%" />
        </a-form-item>
        <a-form-item field="remark" label="备注">
          <a-textarea v-model="adjustStockForm.remark" placeholder="请输入备注" :rows="3" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { storeApi, StoreListItem, StoreStockItem } from '@/api/modules/product/store';

const loading = ref(false);
const stockLoading = ref(false);
const tableData = ref<StoreListItem[]>([]);
const stockTableData = ref<StoreStockItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增门店');
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const stockModalVisible = ref(false);
const adjustStockModalVisible = ref(false);
const currentStoreId = ref(0);

const searchForm = reactive({
  name: '',
  status: null as string | null,
});

const form = reactive({
  id: 0,
  name: '',
  logo: '',
  contactName: '',
  contactPhone: '',
  province: '',
  city: '',
  district: '',
  address: '',
  businessHours: '',
  description: '',
  sort: 0,
  status: '0',
});

const adjustStockForm = reactive({
  storeId: 0,
  productId: 0,
  skuId: undefined as number | undefined,
  productName: '',
  skuCode: '',
  currentStock: 0,
  changeNum: 0,
  remark: '',
});

const rules = {
  name: [{ required: true, message: '请输入门店名称' }],
};

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const stockPagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60, fixed: 'left' },
  { title: '门店Logo', dataIndex: 'logo', slotName: 'logo', width: 100 },
  { title: '门店名称', dataIndex: 'name', width: 150 },
  { title: '联系人', dataIndex: 'contactName', width: 100 },
  { title: '联系电话', dataIndex: 'contactPhone', width: 130 },
  { title: '地址', dataIndex: 'address', width: 250 },
  { title: '排序', dataIndex: 'sort', width: 80, align: 'center' },
  { title: '状态', dataIndex: 'status', slotName: 'status', width: 100, align: 'center' },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 180, fixed: 'right' },
];

const stockColumns = [
  { title: '商品图片', slotName: 'productImage', width: 100 },
  { title: '商品名称', dataIndex: 'productName', width: 180 },
  { title: 'SKU编码', dataIndex: 'skuCode', width: 130 },
  { title: '规格', dataIndex: 'specText', width: 120 },
  { title: '库存', dataIndex: 'stock', width: 80 },
  { title: '预警值', dataIndex: 'alertStock', width: 80 },
  { title: '操作', slotName: 'optional', width: 80 },
];

const getList = async () => {
  loading.value = true;
  try {
    const data = await storeApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status || undefined,
    });
    tableData.value = data.list || [];
    pagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const getStoreStockList = async () => {
  stockLoading.value = true;
  try {
    const data = await storeApi.stockList(currentStoreId.value, {
      pageNum: stockPagination.current,
      pageSize: stockPagination.pageSize,
    });
    stockTableData.value = data.list || [];
    stockPagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    stockLoading.value = false;
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

const handleStockPageChange = (page: number) => {
  stockPagination.current = page;
  getStoreStockList();
};

const handleStockPageSizeChange = (pageSize: number) => {
  stockPagination.pageSize = pageSize;
  getStoreStockList();
};

const onAdd = () => {
  modalTitle.value = '新增门店';
  form.id = 0;
  form.name = '';
  form.logo = '';
  form.contactName = '';
  form.contactPhone = '';
  form.province = '';
  form.city = '';
  form.district = '';
  form.address = '';
  form.businessHours = '';
  form.description = '';
  form.sort = 0;
  form.status = '0';
  modalVisible.value = true;
};

const onEdit = async (record: StoreListItem) => {
  modalTitle.value = '编辑门店';
  try {
    const data = await storeApi.detail(record.id);
    form.id = data.id;
    form.name = data.name;
    form.logo = data.logo || '';
    form.contactName = data.contactName || '';
    form.contactPhone = data.contactPhone || '';
    form.province = data.province || '';
    form.city = data.city || '';
    form.district = data.district || '';
    form.address = data.address || '';
    form.businessHours = data.businessHours || '';
    form.description = data.description || '';
    form.sort = data.sort;
    form.status = data.status;
    modalVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: StoreListItem) => {
  try {
    await storeApi.delete([record.id]);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await storeApi.delete(selectedIds.value.map((id) => Number(id)));
    arcoMessage('success', '批量删除成功');
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    if (form.id) {
      await storeApi.edit(form);
      arcoMessage('success', '修改成功');
    } else {
      await storeApi.add(form);
      arcoMessage('success', '新增成功');
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

const onViewStock = (record: StoreListItem) => {
  currentStoreId.value = record.id;
  stockPagination.current = 1;
  getStoreStockList();
  stockModalVisible.value = true;
};

const onAdjustStock = (record: StoreStockItem) => {
  adjustStockForm.storeId = currentStoreId.value;
  adjustStockForm.productId = record.productId;
  adjustStockForm.skuId = record.skuId;
  adjustStockForm.productName = record.productName;
  adjustStockForm.skuCode = record.skuCode || '';
  adjustStockForm.currentStock = record.stock;
  adjustStockForm.changeNum = 0;
  adjustStockForm.remark = '';
  adjustStockModalVisible.value = true;
};

const onSubmitAdjustStock = async () => {
  try {
    await storeApi.stockAdjust({
      storeId: adjustStockForm.storeId,
      productId: adjustStockForm.productId,
      skuId: adjustStockForm.skuId,
      changeNum: adjustStockForm.changeNum,
      remark: adjustStockForm.remark,
    });
    arcoMessage('success', '库存调整成功');
    adjustStockModalVisible.value = false;
    getStoreStockList();
  } catch (error) {
    console.error(error);
  }
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

.store-logo {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  &.empty {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f2f3f5;
    color: #86909c;
    font-size: 24px;
  }
}

.product-image {
  width: 48px;
  height: 48px;
  border-radius: 4px;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  &.empty {
    display: flex;
    align-items: center;
    justify-content: center;
    background: #f2f3f5;
    color: #86909c;
    font-size: 20px;
  }
}
</style>

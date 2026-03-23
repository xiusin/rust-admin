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
            <a-form-item field="city" label="所在城市">
              <a-input v-model="searchForm.city" placeholder="请输入城市" allow-clear />
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
              新增门店
            </a-button>
            <a-button type="primary" size="small" status="danger" @click="onBatchDelete" :disabled="selectedIds.length === 0">
              <template #icon><icon-delete /></template>
              批量删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-statistic title="门店总数" :value="statistics.totalStores" />
            <a-statistic title="营业中" :value="statistics.activeStores" status="success" />
            <a-statistic title="已停业" :value="statistics.inactiveStores" status="error" />
          </a-space>
        </a-col>
      </a-row>

      <a-table
        ref="tableRef"
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1200 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="handleSelectionChange"
      >
        <template #logo="{ record }">
          <a-avatar v-if="record.logo" :size="40" :image-url="record.logo" />
          <a-avatar v-else :size="40"><icon-shop /></a-avatar>
        </template>
        <template #address="{ record }">
          <span>{{ record.province }}{{ record.city }}{{ record.district }}{{ record.address }}</span>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'">{{ record.status === '0' ? '营业中' : '已停业' }}</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onViewStock(record)">库存</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-if="record.status === '0'">停业</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-else>营业</a-button>
            <a-popconfirm type="warning" content="确定删除该门店吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="800" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-tabs v-model:active-key="activeTab">
          <a-tab-pane key="basic" title="基本信息">
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item field="name" label="门店名称" :rules="[{ required: true, message: '请输入门店名称' }]">
                  <a-input v-model="form.name" placeholder="请输入门店名称" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="contactName" label="联系人">
                  <a-input v-model="form.contactName" placeholder="请输入联系人" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="contactPhone" label="联系电话">
                  <a-input v-model="form.contactPhone" placeholder="请输入联系电话" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="businessHours" label="营业时间">
                  <a-input v-model="form.businessHours" placeholder="如: 09:00-21:00" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="logo" label="门店Logo">
                  <a-input v-model="form.logo" placeholder="请输入Logo URL" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="coverImage" label="门店封面">
                  <a-input v-model="form.coverImage" placeholder="请输入封面图片URL" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="address" title="地址信息">
            <a-row :gutter="16">
              <a-col :span="8">
                <a-form-item field="province" label="省份">
                  <a-input v-model="form.province" placeholder="请输入省份" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="city" label="城市">
                  <a-input v-model="form.city" placeholder="请输入城市" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="district" label="区/县">
                  <a-input v-model="form.district" placeholder="请输入区/县" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="address" label="详细地址">
                  <a-input v-model="form.address" placeholder="请输入详细地址" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="longitude" label="经度">
                  <a-input-number v-model="form.longitude" placeholder="请输入经度" :precision="6" style="width: 100%" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="latitude" label="纬度">
                  <a-input-number v-model="form.latitude" placeholder="请输入纬度" :precision="6" style="width: 100%" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="other" title="其他设置">
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item field="sort" label="排序">
                  <a-input-number v-model="form.sort" placeholder="请输入排序" :min="0" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="status" label="状态">
                  <a-radio-group v-model="form.status">
                    <a-radio value="0">营业中</a-radio>
                    <a-radio value="1">已停业</a-radio>
                  </a-radio-group>
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="description" label="门店描述">
                  <a-textarea v-model="form.description" placeholder="请输入门店描述" :rows="4" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
        </a-tabs>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="stockModalVisible" title="门店库存" :width="1000" :footer="false">
      <div class="stock-header">
        <a-form :model="stockSearchForm" auto-label-width>
          <a-row :gutter="16">
            <a-col :span="8">
              <a-form-item field="productName" label="商品名称">
                <a-input v-model="stockSearchForm.productName" placeholder="请输入商品名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-button type="primary" size="small" @click="getStoreStockList">查询</a-button>
            </a-col>
          </a-row>
        </a-form>
      </div>
      <a-table
        :data="stockTableData"
        :pagination="stockPagination"
        @page-change="handleStockPageChange"
        @page-size-change="handleStockPageSizeChange"
      >
        <template #columns>
          <a-table-column title="商品图片" :width="80">
            <template #cell="{ record }">
              <a-avatar v-if="record.productImage" :size="40" :image-url="record.productImage" />
              <a-avatar v-else :size="40"><icon-image /></a-avatar>
            </template>
          </a-table-column>
          <a-table-column title="商品名称" data-index="productName" :width="150" />
          <a-table-column title="SKU编码" data-index="skuCode" :width="120" />
          <a-table-column title="规格" data-index="specText" :width="120" />
          <a-table-column title="库存" data-index="stock" :width="80" />
          <a-table-column title="预警值" data-index="alertStock" :width="80" />
          <a-table-column title="状态" :width="80">
            <template #cell="{ record }">
              <a-tag :color="record.isAlert ? 'red' : 'green'">{{ record.isAlert ? '预警' : '正常' }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="操作" :width="100">
            <template #cell="{ record }">
              <a-button type="text" size="mini" @click="onAdjustStock(record)">调整</a-button>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="adjustStockModalVisible" title="库存调整" :width="500" @ok="onSubmitAdjustStock" @cancel="adjustStockModalVisible = false">
      <a-form :model="adjustStockForm" auto-label-width>
        <a-form-item label="商品">
          <a-input :value="adjustStockForm.productName" disabled />
        </a-form-item>
        <a-form-item label="SKU" v-if="adjustStockForm.skuCode">
          <a-input :value="adjustStockForm.skuCode" disabled />
        </a-form-item>
        <a-form-item label="当前库存">
          <a-input-number v-model="adjustStockForm.currentStock" disabled :min="0" style="width: 100%" />
        </a-form-item>
        <a-form-item field="changeNum" label="调整数量" :rules="[{ required: true, message: '请输入调整数量' }]">
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
import { Message } from '@arco-design/web-vue';
import { storeApi, StoreListItem, StoreDetail, StoreStockItem, StoreStatistics } from '@/api/modules/product/store';

const loading = ref(false);
const tableData = ref<StoreListItem[]>([]);
const stockTableData = ref<StoreStockItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增门店');
const stockModalVisible = ref(false);
const adjustStockModalVisible = ref(false);
const formRef = ref();
const selectedIds = ref<number[]>([]);
const activeTab = ref('basic');
const currentStoreId = ref(0);

const statistics = reactive<StoreStatistics>({
  totalStores: 0,
  activeStores: 0,
  inactiveStores: 0,
});

const searchForm = reactive({
  name: '',
  city: '',
  status: '',
});

const form = reactive({
  id: 0,
  name: '',
  logo: '',
  coverImage: '',
  contactName: '',
  contactPhone: '',
  province: '',
  city: '',
  district: '',
  address: '',
  longitude: undefined as number | undefined,
  latitude: undefined as number | undefined,
  businessHours: '',
  description: '',
  sort: 0,
  status: '0',
});

const stockSearchForm = reactive({
  productName: '',
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

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const stockPagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60 },
  {
    title: '门店Logo',
    dataIndex: 'logo',
    slotName: 'logo',
    width: 80,
  },
  {
    title: '门店名称',
    dataIndex: 'name',
    width: 150,
  },
  {
    title: '联系人',
    dataIndex: 'contactName',
    width: 100,
  },
  {
    title: '联系电话',
    dataIndex: 'contactPhone',
    width: 130,
  },
  {
    title: '地址',
    slotName: 'address',
    width: 250,
    ellipsis: true,
  },
  {
    title: '营业时间',
    dataIndex: 'businessHours',
    width: 120,
  },
  {
    title: '排序',
    dataIndex: 'sort',
    width: 80,
  },
  {
    title: '状态',
    dataIndex: 'status',
    slotName: 'status',
    width: 100,
  },
  {
    title: '创建时间',
    dataIndex: 'createdAt',
    width: 180,
  },
  {
    title: '操作',
    slotName: 'optional',
    width: 250,
    fixed: 'right',
  },
];

const getList = async () => {
  loading.value = true;
  try {
    const res = await storeApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      city: searchForm.city || undefined,
      status: searchForm.status || undefined,
    });
    if (res.code === 200) {
      tableData.value = res.data?.list || [];
      pagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const getStatistics = async () => {
  try {
    const res = await storeApi.statistics();
    if (res.code === 200 && res.data) {
      statistics.totalStores = res.data.totalStores;
      statistics.activeStores = res.data.activeStores;
      statistics.inactiveStores = res.data.inactiveStores;
    }
  } catch (error) {
    console.error(error);
  }
};

const getStoreStockList = async () => {
  try {
    const res = await storeApi.stockList(currentStoreId.value, {
      pageNum: stockPagination.current,
      pageSize: stockPagination.pageSize,
      productName: stockSearchForm.productName || undefined,
    });
    if (res.code === 200) {
      stockTableData.value = res.data?.list || [];
      stockPagination.total = res.data?.total || 0;
    }
  } catch (error) {
    console.error(error);
  }
};

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchForm.name = '';
  searchForm.city = '';
  searchForm.status = '';
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

const handleSelectionChange = (keys: (string | number)[]) => {
  selectedIds.value = keys as number[];
};

const handleStockPageChange = (page: number) => {
  stockPagination.current = page;
  getStoreStockList();
};

const handleStockPageSizeChange = (pageSize: number) => {
  stockPagination.pageSize = pageSize;
  getStoreStockList();
};

const resetForm = () => {
  form.id = 0;
  form.name = '';
  form.logo = '';
  form.coverImage = '';
  form.contactName = '';
  form.contactPhone = '';
  form.province = '';
  form.city = '';
  form.district = '';
  form.address = '';
  form.longitude = undefined;
  form.latitude = undefined;
  form.businessHours = '';
  form.description = '';
  form.sort = 0;
  form.status = '0';
  activeTab.value = 'basic';
};

const onAdd = () => {
  modalTitle.value = '新增门店';
  resetForm();
  modalVisible.value = true;
};

const onEdit = async (record: StoreListItem) => {
  modalTitle.value = '编辑门店';
  try {
    const res = await storeApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
      form.id = data.id;
      form.name = data.name;
      form.logo = data.logo || '';
      form.coverImage = data.coverImage || '';
      form.contactName = data.contactName || '';
      form.contactPhone = data.contactPhone || '';
      form.province = data.province || '';
      form.city = data.city || '';
      form.district = data.district || '';
      form.address = data.address || '';
      form.longitude = data.longitude;
      form.latitude = data.latitude;
      form.businessHours = data.businessHours || '';
      form.description = data.description || '';
      form.sort = data.sort;
      form.status = data.status;
      modalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onViewStock = (record: StoreListItem) => {
  currentStoreId.value = record.id;
  stockSearchForm.productName = '';
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

const onDelete = async (record: StoreListItem) => {
  try {
    const res = await storeApi.delete([record.id]);
    if (res.code === 200) {
      Message.success('删除成功');
      getList();
      getStatistics();
    } else {
      Message.error(res.message || '删除失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    const res = await storeApi.delete(selectedIds.value);
    if (res.code === 200) {
      Message.success('批量删除成功');
      getList();
      getStatistics();
    } else {
      Message.error(res.message || '删除失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onUpdateStatus = async (record: StoreListItem) => {
  const newStatus = record.status === '0' ? '1' : '0';
  try {
    const res = await storeApi.edit({
      id: record.id,
      name: record.name,
      status: newStatus,
    });
    if (res.code === 200) {
      Message.success(newStatus === '0' ? '营业成功' : '停业成功');
      getList();
      getStatistics();
    } else {
      Message.error(res.message || '操作失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  try {
    let res;
    if (form.id) {
      res = await storeApi.edit({
        id: form.id,
        name: form.name,
        logo: form.logo || undefined,
        coverImage: form.coverImage || undefined,
        contactName: form.contactName || undefined,
        contactPhone: form.contactPhone || undefined,
        province: form.province || undefined,
        city: form.city || undefined,
        district: form.district || undefined,
        address: form.address || undefined,
        longitude: form.longitude,
        latitude: form.latitude,
        businessHours: form.businessHours || undefined,
        description: form.description || undefined,
        sort: form.sort,
        status: form.status,
      });
    } else {
      res = await storeApi.add({
        name: form.name,
        logo: form.logo || undefined,
        coverImage: form.coverImage || undefined,
        contactName: form.contactName || undefined,
        contactPhone: form.contactPhone || undefined,
        province: form.province || undefined,
        city: form.city || undefined,
        district: form.district || undefined,
        address: form.address || undefined,
        longitude: form.longitude,
        latitude: form.latitude,
        businessHours: form.businessHours || undefined,
        description: form.description || undefined,
        sort: form.sort,
        status: form.status,
      });
    }

    if (res.code === 200) {
      Message.success(form.id ? '编辑成功' : '新增成功');
      modalVisible.value = false;
      getList();
      getStatistics();
    } else {
      Message.error(res.message || '操作失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onSubmitAdjustStock = async () => {
  if (adjustStockForm.changeNum === 0) {
    Message.warning('请输入调整数量');
    return;
  }

  try {
    const res = await storeApi.stockAdjust({
      storeId: adjustStockForm.storeId,
      productId: adjustStockForm.productId,
      skuId: adjustStockForm.skuId,
      changeNum: adjustStockForm.changeNum,
      remark: adjustStockForm.remark,
    });
    if (res.code === 200) {
      Message.success('库存调整成功');
      adjustStockModalVisible.value = false;
      getStoreStockList();
    } else {
      Message.error(res.message || '调整失败');
    }
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
  getStatistics();
});
</script>

<style scoped lang="less">
.snow-page {
  padding: 16px;
  height: 100%;
  box-sizing: border-box;
}

.snow-inner {
  background: var(--color-bg-2);
  padding: 16px;
  border-radius: 4px;
  height: 100%;
  display: flex;
  flex-direction: column;
}

.search-btn {
  display: flex;
  align-items: center;
}

.stock-header {
  margin-bottom: 16px;
}

:deep(.arco-statistic) {
  margin-left: 24px;
}
</style>

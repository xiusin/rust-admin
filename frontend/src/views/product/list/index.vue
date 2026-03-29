<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="productName" label="商品名称">
              <a-input v-model="searchForm.productName" placeholder="请输入商品名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="商品状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option value="draft">草稿</a-option>
                <a-option value="pending">待审核</a-option>
                <a-option value="onshelves">已上架</a-option>
                <a-option value="offshelves">已下架</a-option>
                <a-option value="rejected">审核拒绝</a-option>
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
            <a-button type="primary" size="small" @click="onBatchAudit" :disabled="selectedIds.length === 0">
              <template #icon><icon-check-circle /></template>
              批量审核
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
        <template #productImage="{ record }">
          <a-image-preview>
            <div class="product-image" v-if="record.productImage">
              <img :src="record.productImage" :alt="record.name" />
            </div>
            <div class="product-image empty" v-else>
              <icon-image />
            </div>
          </a-image-preview>
        </template>
        <template #status="{ record }">
          <a-tag :color="getStatusColor(record.status)" bordered size="small">{{ record.statusName }}</a-tag>
        </template>
        <template #price="{ record }">
          <span class="price">¥{{ record.price }}</span>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onView(record)">查看</a-button>
            <a-button v-if="record.status === 'pending'" type="text" size="mini" @click="onAudit(record)">审核</a-button>
            <a-button v-if="record.status === 'onshelves'" type="text" size="mini" @click="onShelves(record, 'offshelves')"
              >下架</a-button
            >
            <a-button v-if="record.status === 'offshelves'" type="text" size="mini" @click="onShelves(record, 'onshelves')"
              >上架</a-button
            >
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal
      v-model:visible="auditModalVisible"
      title="商品审核"
      :width="600"
      @ok="onSubmitAudit"
      @cancel="auditModalVisible = false"
    >
      <a-form :model="auditForm" auto-label-width layout="vertical">
        <a-form-item label="商品信息">
          <a-input :value="auditForm.productName" disabled />
        </a-form-item>
        <a-form-item field="auditStatus" label="审核结果">
          <a-radio-group v-model="auditForm.auditStatus">
            <a-radio :value="1">通过</a-radio>
            <a-radio :value="2">拒绝</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item field="auditRemark" label="审核备注">
          <a-textarea v-model="auditForm.auditRemark" placeholder="请输入审核备注" :rows="3" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="detailModalVisible" title="商品详情" :width="900" :footer="null">
      <a-descriptions v-if="currentRecord" :column="2" bordered size="large">
        <a-descriptions-item label="商品名称" :span="2">{{ currentRecord.name }}</a-descriptions-item>
        <a-descriptions-item label="商品图片">
          <a-image-preview>
            <div class="product-image-detail" v-if="currentRecord.productImage">
              <img :src="currentRecord.productImage" :alt="currentRecord.name" />
            </div>
          </a-image-preview>
        </a-descriptions-item>
        <a-descriptions-item label="商品价格">
          <span class="price">¥{{ currentRecord.price }}</span>
        </a-descriptions-item>
        <a-descriptions-item label="商品状态">
          <a-tag :color="getStatusColor(currentRecord.status)" bordered size="small">{{ currentRecord.statusName }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="商品描述" :span="2">{{ currentRecord.description || "-" }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.createdAt }}</a-descriptions-item>
        <a-descriptions-item label="审核时间">{{ currentRecord.auditAt || "-" }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { productApi, ProductListItem } from "@/api/modules/product/product";

const loading = ref(false);
const tableData = ref<ProductListItem[]>([]);
const modalVisible = ref(false);
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const auditModalVisible = ref(false);
const detailModalVisible = ref(false);
const currentRecord = ref<ProductListItem | null>(null);

const searchForm = reactive({
  productName: "",
  status: null as string | null
});

const auditForm = reactive({
  productId: 0,
  productName: "",
  auditStatus: 1,
  auditRemark: ""
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { type: "selection", width: 60, fixed: "left" },
  { title: "商品图片", dataIndex: "productImage", slotName: "productImage", width: 100 },
  { title: "商品名称", dataIndex: "name", width: 200, ellipsis: true },
  { title: "价格", slotName: "price", width: 100 },
  { title: "库存", dataIndex: "stock", width: 80 },
  { title: "销量", dataIndex: "sales", width: 80 },
  { title: "状态", slotName: "status", width: 100 },
  { title: "创建时间", dataIndex: "createdAt", width: 180 },
  { title: "操作", slotName: "optional", width: 180, fixed: "right" }
];

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    draft: "default",
    pending: "orange",
    onshelves: "green",
    offshelves: "gray",
    rejected: "red"
  };
  return colors[status] || "default";
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await productApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      productName: searchForm.productName || undefined,
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

const onView = (record: ProductListItem) => {
  currentRecord.value = record;
  detailModalVisible.value = true;
};

const onAudit = (record: ProductListItem) => {
  auditForm.productId = record.id;
  auditForm.productName = record.name;
  auditForm.auditStatus = 1;
  auditForm.auditRemark = "";
  auditModalVisible.value = true;
};

const onShelves = async (record: ProductListItem, action: string) => {
  try {
    if (action === "onshelves") {
      await productApi.updateStatus(record.id, "onshelves");
      arcoMessage("success", "上架成功");
    } else {
      await productApi.updateStatus(record.id, "offshelves");
      arcoMessage("success", "下架成功");
    }
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchAudit = () => {
  if (selectedIds.value.length === 0) return;
  arcoMessage("warning", "请选择一个商品进行审核");
};

const onSubmitAudit = async () => {
  try {
    await productApi.audit({
      productId: auditForm.productId,
      auditStatus: auditForm.auditStatus,
      auditRemark: auditForm.auditRemark
    });
    arcoMessage("success", "审核成功");
    auditModalVisible.value = false;
    getList();
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

.product-image-detail {
  width: 120px;
  height: 120px;
  border-radius: 4px;
  overflow: hidden;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.price {
  color: #f53f3f;
  font-weight: 600;
}
</style>

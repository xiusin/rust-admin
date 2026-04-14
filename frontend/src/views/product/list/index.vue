<template>
  <div class="product-list-page">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">产品管理</h2>
        <div class="page-stats">
          <a-statistic title="全部" :value="stats.total" :value-style="{ color: '#165dff' }" />
          <a-divider direction="vertical" />
          <a-statistic title="已上架" :value="stats.onShelves" :value-style="{ color: '#00b42a' }" />
          <a-divider direction="vertical" />
          <a-statistic title="待审核" :value="stats.pending" :value-style="{ color: '#ff7d00' }" />
          <a-divider direction="vertical" />
          <a-statistic title="库存预警" :value="stats.lowStock" :value-style="{ color: '#f53f3f' }" />
        </div>
      </div>
      <div class="header-right">
        <a-space size="medium">
          <a-button-group>
            <a-button 
              :type="viewMode === 'table' ? 'primary' : 'secondary'" 
              @click="viewMode = 'table'"
            >
              <template #icon><icon-list /></template>
              列表视图
            </a-button>
            <a-button 
              :type="viewMode === 'card' ? 'primary' : 'secondary'" 
              @click="viewMode = 'card'"
            >
              <template #icon><icon-apps /></template>
              卡片视图
            </a-button>
          </a-button-group>
          <a-button type="primary" size="large" @click="onAdd">
            <template #icon><icon-plus /></template>
            添加产品
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="filter-card">
      <a-form ref="searchFormRef" :model="searchForm" layout="inline">
        <a-form-item field="productName" label="产品名称">
          <a-input 
            v-model="searchForm.productName" 
            placeholder="搜索产品名称、SKU..." 
            allow-clear 
            class="search-input"
          >
            <template #prefix><icon-search /></template>
          </a-input>
        </a-form-item>
        <a-form-item field="categoryId" label="产品分类">
          <a-tree-select
            v-model="searchForm.categoryId"
            :data="categoryTree"
            :field-names="{ key: 'id', title: 'name', children: 'children' }"
            placeholder="选择分类"
            allow-clear
            style="width: 180px"
          />
        </a-form-item>
        <a-form-item field="brandId" label="产品品牌">
          <a-select v-model="searchForm.brandId" placeholder="选择品牌" allow-clear style="width: 150px">
            <a-option v-for="brand in brandList" :key="brand.id" :value="brand.id">
              {{ brand.name }}
            </a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="status" label="产品状态">
          <a-select v-model="searchForm.status" placeholder="选择状态" allow-clear style="width: 120px">
            <a-option value="draft">草稿</a-option>
            <a-option value="pending">待审核</a-option>
            <a-option value="onshelves">已上架</a-option>
            <a-option value="offshelves">已下架</a-option>
            <a-option value="rejected">审核拒绝</a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="priceRange" label="价格区间">
          <a-input-group style="width: 240px">
            <a-input-number v-model="searchForm.minPrice" :min="0" placeholder="最低" style="width: 50%" />
            <span class="input-group-split">-</span>
            <a-input-number v-model="searchForm.maxPrice" :min="0" placeholder="最高" style="width: 50%" />
          </a-input-group>
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
          <template v-if="viewMode === 'table'">
            <a-button 
              type="primary" 
              status="success" 
              :disabled="selectedIds.length === 0" 
              @click="onBatchShelves('onshelves')"
            >
              <template #icon><icon-check-circle /></template>
              批量上架
            </a-button>
            <a-button 
              type="primary" 
              status="warning" 
              :disabled="selectedIds.length === 0" 
              @click="onBatchShelves('offshelves')"
            >
              <template #icon><icon-pause-circle /></template>
              批量下架
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
          </template>
          <template v-else>
            <span class="selected-count" v-if="selectedIds.length > 0">
              已选择 {{ selectedIds.length }} 项
            </span>
          </template>
        </a-space>
      </div>
      <div class="action-right">
        <a-space size="medium">
          <a-dropdown trigger="click">
            <a-button>
              <template #icon><icon-sort /></template>
              排序
              <template #icon><icon-down /></template>
            </a-button>
            <template #content>
              <a-doption @click="sortData('createdAt', 'desc')">最新创建</a-doption>
              <a-doption @click="sortData('sales', 'desc')">销量最高</a-doption>
              <a-doption @click="sortData('price', 'desc')">价格从高到低</a-doption>
              <a-doption @click="sortData('price', 'asc')">价格从低到高</a-doption>
              <a-doption @click="sortData('stock', 'asc')">库存最低</a-doption>
            </template>
          </a-dropdown>
          <a-tooltip content="刷新">
            <div class="action-icon" @click="refresh">
              <icon-refresh size="18" />
            </div>
          </a-tooltip>
        </a-space>
      </div>
    </div>

    <div class="content-area">
      <a-table
        v-if="viewMode === 'table'"
        row-key="id"
        :loading="loading"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        class="product-table"
      >
        <template #productImage="{ record }">
          <a-image-preview>
            <div class="product-image">
              <img :src="record.productImage || defaultProductImage" :alt="record.name" />
            </div>
          </a-image-preview>
        </template>
        <template #name="{ record }">
          <div class="product-name-cell">
            <div class="product-title">{{ record.name }}</div>
            <div class="product-meta">
              <span class="meta-item">SKU: {{ record.skuCode || '-' }}</span>
            </div>
          </div>
        </template>
        <template #price="{ record }">
          <div class="price-cell">
            <span class="price">¥{{ record.price }}</span>
            <span class="original-price" v-if="record.originalPrice">¥{{ record.originalPrice }}</span>
          </div>
        </template>
        <template #stock="{ record }">
          <div class="stock-cell">
            <span :class="['stock-value', getStockClass(record)]">{{ record.stock }}</span>
            <a-tag v-if="record.isAlert" color="red" size="small">预警</a-tag>
          </div>
        </template>
        <template #status="{ record }">
          <div class="status-cell">
            <a-tag :color="getStatusColor(record.status)" size="large">
              {{ record.statusName }}
            </a-tag>
          </div>
        </template>
        <template #optional="{ record }">
          <div class="action-cell">
            <a-space size="small">
              <a-button type="text" size="small" @click="onView(record)">
                <template #icon><icon-eye /></template>
                查看
              </a-button>
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
                  <a-doption v-if="record.status === 'pending'" @click="onAudit(record)">
                    <icon-check-circle /> 审核
                  </a-doption>
                  <a-doption v-if="record.status === 'onshelves'" @click="onShelves(record, 'offshelves')">
                    <icon-pause-circle /> 下架
                  </a-doption>
                  <a-doption v-if="record.status === 'offshelves'" @click="onShelves(record, 'onshelves')">
                    <icon-play-circle /> 上架
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

      <div v-else class="card-view">
        <a-row :gutter="20">
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
              class="product-card"
              :class="{ 'card-selected': selectedIds.includes(String(item.id)) }"
              @click="toggleCardSelection(item.id)"
            >
              <div class="card-checkbox" @click.stop>
                <a-checkbox v-model:checked="selectedIds.includes(String(item.id))" />
              </div>
              <a-image-preview>
                <div class="card-image">
                  <img :src="item.productImage || defaultProductImage" :alt="item.name" />
                  <div class="card-status" v-if="item.status !== 'onshelves'">
                    <a-tag :color="getStatusColor(item.status)" size="small">
                      {{ item.statusName }}
                    </a-tag>
                  </div>
                </div>
              </a-image-preview>
              <div class="card-content">
                <div class="card-title">{{ item.name }}</div>
                <div class="card-meta">
                  <span>SKU: {{ item.skuCode || '-' }}</span>
                </div>
                <div class="card-footer">
                  <div class="card-price">
                    <span class="current-price">¥{{ item.price }}</span>
                    <span class="old-price" v-if="item.originalPrice">¥{{ item.originalPrice }}</span>
                  </div>
                  <div class="card-stock">
                    库存: <span :class="getStockClass(item)">{{ item.stock }}</span>
                  </div>
                </div>
              </div>
              <div class="card-actions">
                <a-button type="text" size="small" @click.stop="onView(item)">
                  <icon-eye />
                </a-button>
                <a-button type="text" size="small" @click.stop="onEdit(item)">
                  <icon-edit />
                </a-button>
                <a-dropdown trigger="click">
                  <a-button type="text" size="small">
                    <icon-more />
                  </a-button>
                  <template #content>
                    <a-doption v-if="item.status === 'pending'" @click.stop="onAudit(item)">审核</a-doption>
                    <a-doption v-if="item.status === 'onshelves'" @click.stop="onShelves(item, 'offshelves')">下架</a-doption>
                    <a-doption v-if="item.status === 'offshelves'" @click.stop="onShelves(item, 'onshelves')">上架</a-doption>
                    <a-doption @click.stop="onCopy(item)">复制</a-doption>
                    <a-doption @click.stop="onDelete(item)" style="color: #f53f3f">删除</a-doption>
                  </template>
                </a-dropdown>
              </div>
            </a-card>
          </a-col>
        </a-row>
        <div class="pagination-wrapper">
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
    </div>

    <a-modal
      v-model:visible="auditModalVisible"
      title="产品审核"
      :width="600"
      @ok="onSubmitAudit"
      @cancel="auditModalVisible = false"
    >
      <a-form :model="auditForm" layout="vertical">
        <a-form-item label="产品信息">
          <a-input :value="auditForm.productName" disabled />
        </a-form-item>
        <a-form-item field="auditStatus" label="审核结果">
          <a-radio-group v-model="auditForm.auditStatus" direction="vertical">
            <a-radio :value="1">
              <div class="radio-option">
                <icon-check-circle-fill style="color: #00b42a; font-size: 20px; margin-right: 8px;" />
                <div>
                  <div class="radio-title">审核通过</div>
                  <div class="radio-desc">产品将直接上架销售</div>
                </div>
              </div>
            </a-radio>
            <a-radio :value="2">
              <div class="radio-option">
                <icon-close-circle-fill style="color: #f53f3f; font-size: 20px; margin-right: 8px;" />
                <div>
                  <div class="radio-title">审核拒绝</div>
                  <div class="radio-desc">产品将返回给商家修改</div>
                </div>
              </div>
            </a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item field="auditRemark" label="审核备注">
          <a-textarea 
            v-model="auditForm.auditRemark" 
            placeholder="请输入审核备注（必填）" 
            :rows="4" 
            :max-length="500"
            show-word-limit
          />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal 
      v-model:visible="detailModalVisible" 
      title="产品详情" 
      :width="1000" 
      :footer="null"
      class="detail-modal"
    >
      <a-descriptions v-if="currentRecord" :column="2" bordered size="large">
        <a-descriptions-item label="产品名称" :span="2">{{ currentRecord.name }}</a-descriptions-item>
        <a-descriptions-item label="产品图片">
          <a-image-preview>
            <div class="product-image-detail">
              <img :src="currentRecord.productImage || defaultProductImage" :alt="currentRecord.name" />
            </div>
          </a-image-preview>
        </a-descriptions-item>
        <a-descriptions-item label="产品价格">
          <div class="detail-price">
            <span class="price">¥{{ currentRecord.price }}</span>
            <span class="original-price" v-if="currentRecord.originalPrice">原价: ¥{{ currentRecord.originalPrice }}</span>
          </div>
        </a-descriptions-item>
        <a-descriptions-item label="产品状态">
          <a-tag :color="getStatusColor(currentRecord.status)" size="large">
            {{ currentRecord.statusName }}
          </a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="库存数量">{{ currentRecord.stock }}</a-descriptions-item>
        <a-descriptions-item label="销量">{{ currentRecord.sales }}</a-descriptions-item>
        <a-descriptions-item label="产品分类">{{ currentRecord.categoryName || '-' }}</a-descriptions-item>
        <a-descriptions-item label="产品品牌">{{ currentRecord.brandName || '-' }}</a-descriptions-item>
        <a-descriptions-item label="产品描述" :span="2">{{ currentRecord.description || '-' }}</a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentRecord.createdAt }}</a-descriptions-item>
        <a-descriptions-item label="更新时间">{{ currentRecord.updatedAt || '-' }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from "vue";
import { useRouter } from "vue-router";
import { productApi, ProductListItem } from "@/api/modules/product/product";
import { categoryApi } from "@/api/modules/product/category";
import { brandApi } from "@/api/modules/product/brand";

const loading = ref(false);
const tableData = ref<ProductListItem[]>([]);
const modalVisible = ref(false);
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const auditModalVisible = ref(false);
const detailModalVisible = ref(false);
const currentRecord = ref<ProductListItem | null>(null);
const viewMode = ref<"table" | "card">("table");
const categoryTree = ref<any[]>([]);
const brandList = ref<any[]>([]);
const router = useRouter();

const defaultProductImage = "https://via.placeholder.com/400x400?text=No+Image";

const stats = reactive({
  total: 0,
  onShelves: 0,
  pending: 0,
  lowStock: 0
});

const searchForm = reactive({
  productName: "",
  categoryId: null as number | null,
  brandId: null as number | null,
  status: null as string | null,
  minPrice: null as number | null,
  maxPrice: null as number | null
});

const auditForm = reactive({
  productId: 0,
  productName: "",
  auditStatus: 1,
  auditRemark: ""
});

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { type: "selection", width: 60, fixed: "left" },
  { title: "产品图片", dataIndex: "productImage", slotName: "productImage", width: 100 },
  { title: "产品信息", dataIndex: "name", slotName: "name", width: 280 },
  { title: "价格", slotName: "price", width: 140 },
  { title: "库存", slotName: "stock", width: 100 },
  { title: "销量", dataIndex: "sales", width: 80 },
  { title: "状态", slotName: "status", width: 100 },
  { title: "创建时间", dataIndex: "createdAt", width: 170 },
  { title: "操作", slotName: "optional", width: 200, fixed: "right" }
];

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    draft: "gray",
    pending: "orange",
    onshelves: "green",
    offshelves: "arcoblue",
    rejected: "red"
  };
  return colors[status] || "default";
};

const getStockClass = (record: ProductListItem) => {
  if (record.stock === 0) return "out-of-stock";
  if (record.isAlert) return "low-stock";
  return "";
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await productApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      productName: searchForm.productName || undefined,
      categoryId: searchForm.categoryId || undefined,
      brandId: searchForm.brandId || undefined,
      status: searchForm.status || undefined,
      minPrice: searchForm.minPrice || undefined,
      maxPrice: searchForm.maxPrice || undefined
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

const calculateStats = (list: ProductListItem[]) => {
  stats.total = list.length;
  stats.onShelves = list.filter(item => item.status === "onshelves").length;
  stats.pending = list.filter(item => item.status === "pending").length;
  stats.lowStock = list.filter(item => item.isAlert || item.stock === 0).length;
};

const getCategoryTree = async () => {
  try {
    const data = await categoryApi.tree();
    categoryTree.value = data || [];
  } catch (error) {
    console.error(error);
  }
};

const getBrandList = async () => {
  try {
    const data = await brandApi.list({ pageNum: 1, pageSize: 100 });
    brandList.value = data.list || [];
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

const sortData = (field: string, order: string) => {
  arcoMessage("info", `按 ${field} ${order === "desc" ? "降序" : "升序"} 排序`);
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
  router.push("/product/form");
};

const onEdit = (record: ProductListItem) => {
  router.push(`/product/form/${record.id}`);
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

const onCopy = (record: ProductListItem) => {
  arcoMessage("info", "复制产品");
};

const onDelete = (record: ProductListItem) => {
  arcoMessage("warning", "确认删除该产品?");
};

const onBatchShelves = (action: string) => {
  arcoMessage("info", `批量${action === "onshelves" ? "上架" : "下架"} ${selectedIds.value.length} 个产品`);
};

const onBatchDelete = () => {
  if (selectedIds.value.length === 0) return;
  arcoMessage("warning", `确认删除选中的 ${selectedIds.value.length} 个产品?`);
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

const toggleCardSelection = (id: number) => {
  const idStr = String(id);
  const index = selectedIds.value.indexOf(idStr);
  if (index > -1) {
    selectedIds.value.splice(index, 1);
  } else {
    selectedIds.value.push(idStr);
  }
};

onMounted(() => {
  getList();
  getCategoryTree();
  getBrandList();
});
</script>

<style scoped lang="scss">
.product-list-page {
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

.header-right {
  display: flex;
  align-items: center;
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
  
  .input-group-split {
    padding: 0 8px;
    color: #86909c;
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

.product-table {
  :deep(.arco-table-th) {
    background-color: #f7f8fa;
  }
}

.product-image {
  width: 64px;
  height: 64px;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e5e6eb;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.product-name-cell {
  .product-title {
    font-weight: 500;
    color: #1d2129;
    margin-bottom: 4px;
  }
  
  .product-meta {
    .meta-item {
      font-size: 12px;
      color: #86909c;
    }
  }
}

.price-cell {
  .price {
    color: #f53f3f;
    font-weight: 600;
    font-size: 16px;
  }
  
  .original-price {
    color: #86909c;
    text-decoration: line-through;
    margin-left: 8px;
    font-size: 12px;
  }
}

.stock-cell {
  display: flex;
  align-items: center;
  gap: 8px;
  
  .stock-value {
    font-weight: 600;
    
    &.out-of-stock {
      color: #f53f3f;
    }
    
    &.low-stock {
      color: #ff7d00;
    }
  }
}

.status-cell {
  display: flex;
  justify-content: center;
}

.action-cell {
  display: flex;
  justify-content: flex-start;
}

.card-view {
  .product-card {
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
  
  .card-image {
    width: 100%;
    height: 200px;
    overflow: hidden;
    border-radius: 8px;
    position: relative;
    
    img {
      width: 100%;
      height: 100%;
      object-fit: cover;
    }
    
    .card-status {
      position: absolute;
      top: 8px;
      right: 8px;
    }
  }
  
  .card-content {
    padding: 12px 0;
    
    .card-title {
      font-size: 14px;
      font-weight: 500;
      color: #1d2129;
      margin-bottom: 8px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    
    .card-meta {
      font-size: 12px;
      color: #86909c;
      margin-bottom: 12px;
    }
    
    .card-footer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      
      .card-price {
        .current-price {
          color: #f53f3f;
          font-size: 18px;
          font-weight: 600;
        }
        
        .old-price {
          color: #86909c;
          text-decoration: line-through;
          font-size: 12px;
          margin-left: 4px;
        }
      }
      
      .card-stock {
        color: #86909c;
        font-size: 12px;
        
        .out-of-stock {
          color: #f53f3f;
          font-weight: 600;
        }
        
        .low-stock {
          color: #ff7d00;
          font-weight: 600;
        }
      }
    }
  }
  
  .card-actions {
    position: absolute;
    top: 8px;
    right: 8px;
    display: flex;
    gap: 4px;
  }
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 24px;
}

.radio-option {
  display: flex;
  align-items: flex-start;
  padding: 12px 0;
  
  .radio-title {
    font-weight: 500;
    color: #1d2129;
  }
  
  .radio-desc {
    font-size: 12px;
    color: #86909c;
    margin-top: 4px;
  }
}

.product-image-detail {
  width: 160px;
  height: 160px;
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid #e5e6eb;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.detail-price {
  .price {
    color: #f53f3f;
    font-size: 24px;
    font-weight: 600;
  }
  
  .original-price {
    color: #86909c;
    text-decoration: line-through;
    margin-left: 12px;
  }
}

.detail-modal {
  :deep(.arco-modal-body) {
    max-height: 70vh;
    overflow-y: auto;
  }
}
</style>

<template>
  <div class="stock-page">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">库存管理</h2>
        <div class="page-stats">
          <a-statistic title="全部商品" :value="stats.total" :value-style="{ color: '#165dff' }" />
          <a-divider direction="vertical" />
          <a-statistic title="库存正常" :value="stats.normal" :value-style="{ color: '#00b42a' }" />
          <a-divider direction="vertical" />
          <a-statistic title="库存不足" :value="stats.low" :value-style="{ color: '#ff7d00' }" />
          <a-divider direction="vertical" />
          <a-statistic title="缺货" :value="stats.out" :value-style="{ color: '#f53f3f' }" />
        </div>
      </div>
      <div class="header-right">
        <a-space size="medium">
          <a-button type="primary" status="warning" size="large" @click="onBatchAdjust" :disabled="selectedIds.length === 0">
            <template #icon><icon-edit /></template>
            批量调整
          </a-button>
          <a-button type="primary" size="large" @click="onQuickAdd">
            <template #icon><icon-plus /></template>
            快速入库
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="filter-card">
      <a-form ref="searchFormRef" :model="searchForm" layout="inline">
        <a-form-item field="productName" label="商品名称">
          <a-input 
            v-model="searchForm.productName" 
            placeholder="搜索商品名称或SKU" 
            allow-clear 
            class="search-input"
          >
            <template #prefix><icon-search /></template>
          </a-input>
        </a-form-item>
        <a-form-item field="stockStatus" label="库存状态">
          <a-select v-model="searchForm.stockStatus" placeholder="选择状态" allow-clear style="width: 140px">
            <a-option value="normal">库存正常</a-option>
            <a-option value="low">库存不足</a-option>
            <a-option value="out">缺货</a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="categoryId" label="商品分类">
          <a-tree-select
            v-model="searchForm.categoryId"
            :data="categoryTree"
            :field-names="{ key: 'id', title: 'name', children: 'children' }"
            placeholder="选择分类"
            allow-clear
            style="width: 180px"
          />
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
          <span class="selected-count" v-if="selectedIds.length > 0">
            已选择 {{ selectedIds.length }} 项
          </span>
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
              <a-doption @click="sortData('stock', 'asc')">库存最低</a-doption>
              <a-doption @click="sortData('stock', 'desc')">库存最高</a-doption>
              <a-doption @click="sortData('sales', 'desc')">销量最高</a-doption>
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
        row-key="skuId"
        :loading="loading"
        :columns="columns"
        :data="tableData"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedIds"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        class="stock-table"
      >
        <template #productImage="{ record }">
          <a-image-preview>
            <div class="product-image">
              <img :src="record.productImage || defaultProductImage" :alt="record.productName" />
            </div>
          </a-image-preview>
        </template>
        <template #productInfo="{ record }">
          <div class="product-info-cell">
            <div class="product-name">{{ record.productName }}</div>
            <div class="product-meta">
              <a-tag color="arcoblue" size="small">SKU: {{ record.skuCode || '-' }}</a-tag>
              <span class="spec-text" v-if="record.specText">{{ record.specText }}</span>
            </div>
          </div>
        </template>
        <template #stock="{ record }">
          <div class="stock-cell">
            <div class="stock-indicator" :class="getStockClass(record)">
              <div class="stock-bar">
                <div class="stock-fill" :style="{ width: getStockPercentage(record) + '%' }"></div>
              </div>
              <span class="stock-value">{{ record.stock }}</span>
            </div>
            <a-tag v-if="record.isAlert" color="red" size="small">
              <icon-exclamation-circle-fill /> 预警
            </a-tag>
            <a-tag v-if="record.stock === 0" color="red" size="small">
              <icon-minus-circle-fill /> 缺货
            </a-tag>
          </div>
        </template>
        <template #optional="{ record }">
          <div class="action-cell">
            <a-space size="small">
              <a-button type="primary" size="small" @click="onAdjust(record)">
                <template #icon><icon-edit /></template>
                调整库存
              </a-button>
              <a-button type="text" size="small" @click="onViewLog(record)">
                <template #icon><icon-history /></template>
                变动记录
              </a-button>
            </a-space>
          </div>
        </template>
      </a-table>
    </div>

    <a-modal
      v-model:visible="adjustModalVisible"
      title="库存调整"
      :width="520"
      @ok="onSubmitAdjust"
      @cancel="adjustModalVisible = false"
      class="adjust-modal"
    >
      <a-form :model="adjustForm" layout="vertical">
        <div class="adjust-product-info">
          <div class="product-thumb">
            <img :src="adjustForm.productImage || defaultProductImage" :alt="adjustForm.productName" />
          </div>
          <div class="product-details">
            <div class="product-title">{{ adjustForm.productName }}</div>
            <div class="product-sku" v-if="adjustForm.skuCode">SKU: {{ adjustForm.skuCode }}</div>
            <div class="current-stock">
              当前库存: <span class="stock-number">{{ adjustForm.currentStock }}</span>
            </div>
          </div>
        </div>
        
        <a-divider style="margin: 16px 0" />
        
        <a-form-item field="changeType" label="调整类型">
          <a-radio-group v-model="adjustForm.changeType" direction="vertical" class="change-type-group">
            <a-radio :value="0">
              <div class="radio-option">
                <icon-plus-circle-fill style="color: #00b42a; font-size: 20px; margin-right: 8px;" />
                <div>
                  <div class="radio-title">增加库存</div>
                  <div class="radio-desc">商品入库、退货等场景</div>
                </div>
              </div>
            </a-radio>
            <a-radio :value="1">
              <div class="radio-option">
                <icon-minus-circle-fill style="color: #f53f3f; font-size: 20px; margin-right: 8px;" />
                <div>
                  <div class="radio-title">减少库存</div>
                  <div class="radio-desc">商品出库、盘点损耗等场景</div>
                </div>
              </div>
            </a-radio>
            <a-radio :value="2">
              <div class="radio-option">
                <icon-edit-circle-fill style="color: #165dff; font-size: 20px; margin-right: 8px;" />
                <div>
                  <div class="radio-title">设置库存</div>
                  <div class="radio-desc">直接设置为指定库存数量</div>
                </div>
              </div>
            </a-radio>
          </a-radio-group>
        </a-form-item>
        
        <a-form-item field="changeNum" label="调整数量">
          <a-input-number 
            v-model="adjustForm.changeNum" 
            :min="0" 
            placeholder="请输入调整数量" 
            style="width: 100%"
            size="large"
          />
        </a-form-item>
        
        <a-form-item field="remark" label="备注">
          <a-textarea 
            v-model="adjustForm.remark" 
            placeholder="请输入调整备注（选填）" 
            :rows="3" 
            :max-length="200"
            show-word-limit
          />
        </a-form-item>
        
        <a-form-item label="调整后库存">
          <div class="result-stock">
            <span class="result-label">预计库存：</span>
            <span class="result-value">{{ calculateAfterStock() }}</span>
          </div>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal 
      v-model:visible="logModalVisible" 
      title="库存变动记录" 
      :width="1000" 
      :footer="null"
      class="log-modal"
    >
      <div class="log-header" v-if="currentSku">
        <div class="log-product-info">
          <div class="product-thumb">
            <img :src="currentSku.productImage || defaultProductImage" :alt="currentSku.productName" />
          </div>
          <div class="product-details">
            <div class="product-title">{{ currentSku.productName }}</div>
            <div class="product-sku" v-if="currentSku.skuCode">SKU: {{ currentSku.skuCode }}</div>
          </div>
        </div>
      </div>
      
      <a-table
        :loading="logLoading"
        :columns="logColumns"
        :data="logTableData"
        :pagination="logPagination"
        @page-change="handleLogPageChange"
        @page-size-change="handleLogPageSizeChange"
        class="log-table"
      >
        <template #changeType="{ record }">
          <a-tag :color="getChangeTypeColor(record.changeType)" size="large">
            <component :is="getChangeTypeIcon(record.changeType)" />
            {{ record.changeTypeName }}
          </a-tag>
        </template>
        <template #changeNum="{ record }">
          <span :class="['change-num', record.changeNum > 0 ? 'positive' : 'negative']">
            {{ record.changeNum > 0 ? "+" : "" }}{{ record.changeNum }}
          </span>
        </template>
      </a-table>
    </a-modal>

    <a-modal
      v-model:visible="quickAddModalVisible"
      title="快速入库"
      :width="600"
      @ok="onSubmitQuickAdd"
      @cancel="quickAddModalVisible = false"
      class="quick-add-modal"
    >
      <a-form ref="quickAddFormRef" :model="quickAddForm" :rules="quickAddRules" layout="vertical">
        <a-form-item field="productId" label="选择商品">
          <a-select v-model="quickAddForm.productId" placeholder="搜索并选择商品" allow-search style="width: 100%" allow-clear>
            <a-option v-for="product in productList" :key="product.id" :value="product.id">
              {{ product.name }}
            </a-option>
          </a-select>
        </a-form-item>
        <a-form-item field="quantity" label="入库数量">
          <a-input-number 
            v-model="quickAddForm.quantity" 
            :min="1" 
            placeholder="请输入入库数量" 
            style="width: 100%"
            size="large"
          />
        </a-form-item>
        <a-form-item field="remark" label="入库备注">
          <a-textarea 
            v-model="quickAddForm.remark" 
            placeholder="请输入入库备注（选填）" 
            :rows="3"
          />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { stockApi, StockListItem, StockLogItem } from "@/api/modules/product/stock";
import { categoryApi } from "@/api/modules/product/category";
import { productApi } from "@/api/modules/product/product";

const loading = ref(false);
const logLoading = ref(false);
const tableData = ref<StockListItem[]>([]);
const logTableData = ref<StockLogItem[]>([]);
const adjustModalVisible = ref(false);
const logModalVisible = ref(false);
const quickAddModalVisible = ref(false);
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const quickAddFormRef = ref();
const currentSkuId = ref<number>(0);
const currentSku = ref<StockListItem | null>(null);
const categoryTree = ref<any[]>([]);
const productList = ref<any[]>([]);

const defaultProductImage = "https://via.placeholder.com/200x200?text=Product";

const stats = reactive({
  total: 0,
  normal: 0,
  low: 0,
  out: 0
});

const searchForm = reactive({
  productName: "",
  stockStatus: "",
  categoryId: null as number | null
});

const adjustForm = reactive({
  productId: 0,
  productName: "",
  productImage: "",
  skuId: undefined as number | undefined,
  skuCode: "",
  currentStock: 0,
  changeType: 0,
  changeNum: 0,
  remark: ""
});

const quickAddForm = reactive({
  productId: null as number | null,
  quantity: 1,
  remark: ""
});

const quickAddRules = {
  productId: [{ required: true, message: "请选择商品" }],
  quantity: [{ required: true, message: "请输入入库数量" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const logPagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const columns = [
  { type: "selection", width: 60, fixed: "left" },
  { title: "商品图片", dataIndex: "productImage", slotName: "productImage", width: 100 },
  { title: "商品信息", dataIndex: "productName", slotName: "productInfo", width: 280 },
  { title: "库存", slotName: "stock", width: 200 },
  { title: "销量", dataIndex: "sales", width: 80 },
  { title: "预警值", dataIndex: "alertStock", width: 100 },
  { title: "操作", slotName: "optional", width: 180, fixed: "right" }
];

const logColumns = [
  { title: "变动时间", dataIndex: "createdAt", width: 180 },
  { title: "变动类型", slotName: "changeType", width: 140 },
  { title: "变动数量", slotName: "changeNum", width: 100 },
  { title: "变动前", dataIndex: "beforeStock", width: 100 },
  { title: "变动后", dataIndex: "afterStock", width: 100 },
  { title: "订单号", dataIndex: "orderNo", width: 150, ellipsis: true },
  { title: "操作人", dataIndex: "operatorName", width: 100 },
  { title: "备注", dataIndex: "remark", width: 200, ellipsis: true }
];

const getStockClass = (record: StockListItem) => {
  if (record.stock === 0) return "out-of-stock";
  if (record.isAlert) return "low-stock";
  return "normal-stock";
};

const getStockPercentage = (record: StockListItem) => {
  const alertStock = record.alertStock || 10;
  const maxStock = Math.max(record.stock, alertStock * 3, 100);
  return Math.min((record.stock / maxStock) * 100, 100);
};

const getChangeTypeColor = (type: number) => {
  const colors: Record<number, string> = { 0: "green", 1: "red", 2: "arcoblue", 3: "orange", 4: "red" };
  return colors[type] || "gray";
};

const getChangeTypeIcon = (type: number) => {
  const icons: Record<number, string> = { 
    0: "icon-plus-circle-fill", 
    1: "icon-minus-circle-fill", 
    2: "icon-edit-circle-fill",
    3: "icon-exclamation-circle-fill",
    4: "icon-minus-circle-fill"
  };
  return icons[type] || "icon-info-circle-fill";
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await stockApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      productName: searchForm.productName || undefined,
      stockStatus: searchForm.stockStatus || undefined,
      categoryId: searchForm.categoryId || undefined
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

const calculateStats = (list: StockListItem[]) => {
  stats.total = list.length;
  stats.normal = list.filter(item => item.stock > 0 && !item.isAlert).length;
  stats.low = list.filter(item => item.isAlert && item.stock > 0).length;
  stats.out = list.filter(item => item.stock === 0).length;
};

const getCategoryTree = async () => {
  try {
    const data = await categoryApi.tree();
    categoryTree.value = data || [];
  } catch (error) {
    console.error(error);
  }
};

const getProductList = async () => {
  try {
    const data = await productApi.list({ pageNum: 1, pageSize: 100 });
    productList.value = data.list || [];
  } catch (error) {
    console.error(error);
  }
};

const getLogList = async () => {
  logLoading.value = true;
  try {
    const data = await stockApi.logList({
      pageNum: logPagination.current,
      pageSize: logPagination.pageSize,
      skuId: currentSkuId.value || undefined
    });
    logTableData.value = data.list || [];
    logPagination.total = data.total || 0;
  } catch (error) {
    console.error(error);
  } finally {
    logLoading.value = false;
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

const handleLogPageChange = (page: number) => {
  logPagination.current = page;
  getLogList();
};

const handleLogPageSizeChange = (pageSize: number) => {
  logPagination.pageSize = pageSize;
  getLogList();
};

const calculateAfterStock = () => {
  if (adjustForm.changeType === 0) return adjustForm.currentStock + adjustForm.changeNum;
  if (adjustForm.changeType === 1) return adjustForm.currentStock - adjustForm.changeNum;
  return adjustForm.changeNum;
};

const onBatchAdjust = () => {
  arcoMessage("info", "请选择要调整库存的商品");
};

const onAdjust = (record: StockListItem) => {
  adjustForm.productId = record.productId;
  adjustForm.productName = record.productName;
  adjustForm.productImage = record.productImage || "";
  adjustForm.skuId = record.skuId;
  adjustForm.skuCode = record.skuCode || "";
  adjustForm.currentStock = record.stock;
  adjustForm.changeType = 0;
  adjustForm.changeNum = 0;
  adjustForm.remark = "";
  adjustModalVisible.value = true;
};

const onViewLog = (record: StockListItem) => {
  currentSku.value = record;
  currentSkuId.value = record.skuId;
  logPagination.current = 1;
  getLogList();
  logModalVisible.value = true;
};

const onQuickAdd = () => {
  quickAddForm.productId = null;
  quickAddForm.quantity = 1;
  quickAddForm.remark = "";
  quickAddModalVisible.value = true;
};

const onSubmitAdjust = async () => {
  if (adjustForm.changeNum <= 0) {
    arcoMessage("warning", "请输入调整数量");
    return;
  }
  try {
    await stockApi.adjust({
      productId: adjustForm.productId,
      skuId: adjustForm.skuId,
      changeType: adjustForm.changeType,
      changeNum: adjustForm.changeNum,
      remark: adjustForm.remark
    });
    arcoMessage("success", "库存调整成功");
    adjustModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onSubmitQuickAdd = async () => {
  let state = await quickAddFormRef.value?.validate();
  if (state) return;
  arcoMessage("success", "快速入库成功");
  quickAddModalVisible.value = false;
  getList();
};

onMounted(() => {
  getList();
  getCategoryTree();
  getProductList();
});
</script>

<style scoped lang="scss">
.stock-page {
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

.stock-table {
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
  background: #f7f8fa;

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.product-info-cell {
  .product-name {
    font-weight: 500;
    color: #1d2129;
    margin-bottom: 6px;
  }
  
  .product-meta {
    display: flex;
    align-items: center;
    gap: 8px;
    
    .spec-text {
      font-size: 12px;
      color: #86909c;
    }
  }
}

.stock-cell {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.stock-indicator {
  display: flex;
  align-items: center;
  gap: 12px;
}

.stock-bar {
  flex: 1;
  height: 8px;
  background: #f2f3f5;
  border-radius: 4px;
  overflow: hidden;
  
  .stock-fill {
    height: 100%;
    background: #00b42a;
    border-radius: 4px;
    transition: width 0.3s;
  }
}

.normal-stock {
  .stock-fill {
    background: #00b42a;
  }
}

.low-stock {
  .stock-fill {
    background: #ff7d00;
  }
}

.out-of-stock {
  .stock-fill {
    background: #f53f3f;
  }
}

.stock-value {
  font-weight: 600;
  font-size: 16px;
  min-width: 50px;
}

.action-cell {
  display: flex;
  justify-content: flex-start;
}

.adjust-modal,
.log-modal,
.quick-add-modal {
  :deep(.arco-modal-body) {
    padding-top: 24px;
  }
}

.adjust-product-info,
.log-product-info {
  display: flex;
  gap: 16px;
  padding: 16px;
  background: #f7f8fa;
  border-radius: 8px;
}

.product-thumb {
  width: 80px;
  height: 80px;
  border-radius: 8px;
  overflow: hidden;
  flex-shrink: 0;
  background: #ffffff;
  
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
}

.product-details {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
  
  .product-title {
    font-size: 16px;
    font-weight: 600;
    color: #1d2129;
    margin-bottom: 4px;
  }
  
  .product-sku {
    font-size: 13px;
    color: #86909c;
    margin-bottom: 4px;
  }
  
  .current-stock {
    font-size: 14px;
    
    .stock-number {
      font-size: 24px;
      font-weight: 600;
      color: #165dff;
      margin-left: 8px;
    }
  }
}

.change-type-group {
  width: 100%;
  
  .arco-radio {
    margin-bottom: 12px;
  }
}

.radio-option {
  display: flex;
  align-items: flex-start;
  padding: 8px 0;
  
  .radio-title {
    font-weight: 500;
    color: #1d2129;
  }
  
  .radio-desc {
    font-size: 12px;
    color: #86909c;
    margin-top: 2px;
  }
}

.result-stock {
  padding: 16px;
  background: linear-gradient(135deg, #e8f3ff, #dbeafe);
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  
  .result-label {
    font-size: 14px;
    color: #4e5969;
  }
  
  .result-value {
    font-size: 28px;
    font-weight: 700;
    color: #165dff;
    margin-left: 12px;
  }
}

.log-header {
  margin-bottom: 16px;
}

.log-table {
  :deep(.arco-table-th) {
    background-color: #f7f8fa;
  }
}

.change-num {
  font-weight: 600;
  font-size: 16px;
  
  &.positive {
    color: #00b42a;
  }
  
  &.negative {
    color: #f53f3f;
  }
}
</style>

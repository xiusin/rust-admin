<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="商品名称">
              <a-input v-model="searchForm.name" placeholder="请输入商品名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="categoryId" label="商品分类">
              <a-tree-select
                v-model="searchForm.categoryId"
                :data="categoryTree"
                :field-names="{ key: 'id', title: 'name', children: 'children' }"
                placeholder="请选择分类"
                allow-clear
              />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="brandId" label="商品品牌">
              <a-select v-model="searchForm.brandId" placeholder="请选择品牌" allow-clear>
                <a-option v-for="item in brandList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="商品状态">
              <a-select v-model="searchForm.status" placeholder="请选择状态" allow-clear>
                <a-option :value="0">上架</a-option>
                <a-option :value="1">下架</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="auditStatus" label="审核状态">
              <a-select v-model="searchForm.auditStatus" placeholder="请选择审核状态" allow-clear>
                <a-option :value="0">待审核</a-option>
                <a-option :value="1">审核通过</a-option>
                <a-option :value="2">审核拒绝</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="productType" label="商品类型">
              <a-select v-model="searchForm.productType" placeholder="请选择类型" allow-clear>
                <a-option :value="0">实物商品</a-option>
                <a-option :value="1">虚拟商品</a-option>
                <a-option :value="2">服务商品</a-option>
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
              新增商品
            </a-button>
            <a-button size="small" @click="onBatchUpdateStatus(0)" :disabled="selectedIds.length === 0">
              批量上架
            </a-button>
            <a-button size="small" @click="onBatchUpdateStatus(1)" :disabled="selectedIds.length === 0">
              批量下架
            </a-button>
            <a-button type="primary" size="small" status="danger" @click="onBatchDelete" :disabled="selectedIds.length === 0">
              <template #icon><icon-delete /></template>
              批量删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getList"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-table
        ref="tableRef"
        row-key="id"
        :loading="loading"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1400 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="handleSelectionChange"
      >
        <template #coverImage="{ record }">
          <a-avatar v-if="record.coverImage" :size="50" :image-url="record.coverImage" />
          <a-avatar v-else :size="50"><icon-image /></a-avatar>
        </template>
        <template #productInfo="{ record }">
          <div class="product-info">
            <div class="product-name">{{ record.name }}</div>
            <div class="product-subtitle" v-if="record.subtitle">{{ record.subtitle }}</div>
          </div>
        </template>
        <template #priceInfo="{ record }">
          <div class="price-info">
            <div class="sale-price">¥{{ record.salePrice }}</div>
            <div class="line-price" v-if="record.linePrice">¥{{ record.linePrice }}</div>
          </div>
        </template>
        <template #productType="{ record }">
          <a-tag :color="getProductTypeColor(record.productType)">{{ record.productTypeName }}</a-tag>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === 0 ? 'green' : 'red'">{{ record.statusName }}</a-tag>
        </template>
        <template #auditStatus="{ record }">
          <a-tag :color="getAuditStatusColor(record.auditStatus)">{{ record.auditStatusName }}</a-tag>
        </template>
        <template #tags="{ record }">
          <a-space>
            <a-tag v-if="record.isHot === 1" color="red" size="small">热销</a-tag>
            <a-tag v-if="record.isNew === 1" color="blue" size="small">新品</a-tag>
            <a-tag v-if="record.isRecommend === 1" color="orange" size="small">推荐</a-tag>
          </a-space>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onDetail(record)">详情</a-button>
            <a-button type="text" size="mini" @click="onSkuManage(record)">SKU</a-button>
            <a-button type="text" size="mini" @click="onAudit(record)" v-if="record.auditStatus === 0">审核</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-if="record.status === 1">上架</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-else>下架</a-button>
            <a-popconfirm type="warning" content="确定删除该商品吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="1000" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-tabs v-model:active-key="activeTab">
          <a-tab-pane key="basic" title="基本信息">
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item field="name" label="商品名称" :rules="[{ required: true, message: '请输入商品名称' }]">
                  <a-input v-model="form.name" placeholder="请输入商品名称" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="subtitle" label="商品副标题">
                  <a-input v-model="form.subtitle" placeholder="请输入商品副标题" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="categoryId" label="商品分类" :rules="[{ required: true, message: '请选择商品分类' }]">
                  <a-tree-select
                    v-model="form.categoryId"
                    :data="categoryTree"
                    :field-names="{ key: 'id', title: 'name', children: 'children' }"
                    placeholder="请选择分类"
                  />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="brandId" label="商品品牌">
                  <a-select v-model="form.brandId" placeholder="请选择品牌" allow-clear>
                    <a-option v-for="item in brandList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
                  </a-select>
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="productType" label="商品类型">
                  <a-radio-group v-model="form.productType">
                    <a-radio :value="0">实物商品</a-radio>
                    <a-radio :value="1">虚拟商品</a-radio>
                    <a-radio :value="2">服务商品</a-radio>
                  </a-radio-group>
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="unit" label="商品单位">
                  <a-input v-model="form.unit" placeholder="请输入商品单位" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="coverImage" label="商品封面" :rules="[{ required: true, message: '请输入商品封面' }]">
                  <a-input v-model="form.coverImage" placeholder="请输入封面图片URL" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="images" label="商品图片">
                  <a-input v-model="imagesStr" placeholder="多张图片用逗号分隔" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="video" label="商品视频">
                  <a-input v-model="form.video" placeholder="请输入视频URL" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="price" title="价格库存">
            <a-row :gutter="16">
              <a-col :span="8">
                <a-form-item field="salePrice" label="销售价" :rules="[{ required: true, message: '请输入销售价' }]">
                  <a-input-number v-model="form.salePrice" placeholder="请输入销售价" :min="0" :precision="2">
                    <template #prefix>¥</template>
                  </a-input-number>
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="linePrice" label="划线价">
                  <a-input-number v-model="form.linePrice" placeholder="请输入划线价" :min="0" :precision="2">
                    <template #prefix>¥</template>
                  </a-input-number>
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="costPrice" label="成本价">
                  <a-input-number v-model="form.costPrice" placeholder="请输入成本价" :min="0" :precision="2">
                    <template #prefix>¥</template>
                  </a-input-number>
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="stock" label="库存">
                  <a-input-number v-model="form.stock" placeholder="请输入库存" :min="0" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="virtualSales" label="虚拟销量">
                  <a-input-number v-model="form.virtualSales" placeholder="请输入虚拟销量" :min="0" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="limitBuy" label="限购数量">
                  <a-input-number v-model="form.limitBuy" placeholder="0表示不限购" :min="0" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item field="isMultiSpec" label="多规格">
                  <a-switch v-model="form.isMultiSpec" :checked-value="1" :unchecked-value="0" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="spec" title="规格SKU" v-if="form.isMultiSpec === 1">
            <div class="spec-container">
              <div class="spec-header">
                <a-button type="primary" size="small" @click="addSpec">添加规格</a-button>
              </div>
              <div class="spec-list">
                <div v-for="(spec, specIndex) in form.specs" :key="specIndex" class="spec-item">
                  <div class="spec-name">
                    <a-input v-model="spec.name" placeholder="规格名称" style="width: 150px" />
                    <a-button type="text" status="danger" @click="removeSpec(specIndex)">
                      <icon-delete />
                    </a-button>
                  </div>
                  <div class="spec-values">
                    <a-tag
                      v-for="(value, valueIndex) in spec.values"
                      :key="valueIndex"
                      closable
                      @close="removeSpecValue(specIndex, valueIndex)"
                    >
                      {{ value.value }}
                    </a-tag>
                    <a-input
                      v-model="spec.newValue"
                      placeholder="输入规格值"
                      style="width: 120px"
                      @press-enter="addSpecValue(specIndex)"
                    />
                    <a-button type="text" size="small" @click="addSpecValue(specIndex)">添加</a-button>
                  </div>
                </div>
              </div>
              <a-button type="primary" size="small" @click="generateSkus" style="margin-top: 16px">生成SKU</a-button>
            </div>
            <a-table
              v-if="form.skus.length > 0"
              :data="form.skus"
              :pagination="false"
              style="margin-top: 16px"
            >
              <template #columns>
                <a-table-column title="规格" data-index="specText" />
                <a-table-column title="SKU编码">
                  <template #cell="{ record }">
                    <a-input v-model="record.skuCode" placeholder="SKU编码" />
                  </template>
                </a-table-column>
                <a-table-column title="销售价">
                  <template #cell="{ record }">
                    <a-input-number v-model="record.salePrice" :min="0" :precision="2" />
                  </template>
                </a-table-column>
                <a-table-column title="划线价">
                  <template #cell="{ record }">
                    <a-input-number v-model="record.linePrice" :min="0" :precision="2" />
                  </template>
                </a-table-column>
                <a-table-column title="成本价">
                  <template #cell="{ record }">
                    <a-input-number v-model="record.costPrice" :min="0" :precision="2" />
                  </template>
                </a-table-column>
                <a-table-column title="库存">
                  <template #cell="{ record }">
                    <a-input-number v-model="record.stock" :min="0" />
                  </template>
                </a-table-column>
                <a-table-column title="状态">
                  <template #cell="{ record }">
                    <a-switch v-model="record.status" :checked-value="0" :unchecked-value="1" />
                  </template>
                </a-table-column>
              </template>
            </a-table>
          </a-tab-pane>
          <a-tab-pane key="shipping" title="物流设置">
            <a-row :gutter="16">
              <a-col :span="12">
                <a-form-item field="shippingMethod" label="发货方式">
                  <a-radio-group v-model="form.shippingMethod">
                    <a-radio :value="0">快递发货</a-radio>
                    <a-radio :value="1">无需物流</a-radio>
                  </a-radio-group>
                </a-form-item>
              </a-col>
              <a-col :span="12" v-if="form.shippingMethod === 0">
                <a-form-item field="shippingTemplateId" label="运费模板">
                  <a-select v-model="form.shippingTemplateId" placeholder="请选择运费模板" allow-clear>
                    <a-option v-for="item in shippingTemplateList" :key="item.id" :value="item.id">{{ item.name }}</a-option>
                  </a-select>
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="weight" label="重量(kg)">
                  <a-input-number v-model="form.weight" placeholder="请输入重量" :min="0" :precision="2" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="volume" label="体积(m³)">
                  <a-input-number v-model="form.volume" placeholder="请输入体积" :min="0" :precision="4" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
          <a-tab-pane key="detail" title="商品详情">
            <a-form-item field="detail" label="详情描述">
              <a-textarea v-model="form.detail" placeholder="请输入商品详情" :rows="10" />
            </a-form-item>
          </a-tab-pane>
          <a-tab-pane key="other" title="其他设置">
            <a-row :gutter="16">
              <a-col :span="8">
                <a-form-item field="sort" label="排序">
                  <a-input-number v-model="form.sort" placeholder="请输入排序" :min="0" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="isHot" label="热销">
                  <a-switch v-model="form.isHot" :checked-value="1" :unchecked-value="0" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="isNew" label="新品">
                  <a-switch v-model="form.isNew" :checked-value="1" :unchecked-value="0" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item field="isRecommend" label="推荐">
                  <a-switch v-model="form.isRecommend" :checked-value="1" :unchecked-value="0" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="keywords" label="关键词">
                  <a-input v-model="form.keywords" placeholder="多个关键词用逗号分隔" />
                </a-form-item>
              </a-col>
              <a-col :span="24">
                <a-form-item field="description" label="商品描述">
                  <a-textarea v-model="form.description" placeholder="请输入商品描述" :rows="3" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-tab-pane>
        </a-tabs>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="auditModalVisible" title="商品审核" :width="500" @ok="onAuditSubmit">
      <a-form :model="auditForm" auto-label-width>
        <a-form-item label="审核结果">
          <a-radio-group v-model="auditForm.auditStatus">
            <a-radio :value="1">通过</a-radio>
            <a-radio :value="2">拒绝</a-radio>
          </a-radio-group>
        </a-form-item>
        <a-form-item label="审核备注">
          <a-textarea v-model="auditForm.auditRemark" placeholder="请输入审核备注" :rows="3" />
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="skuModalVisible" title="SKU管理" :width="900" :footer="false">
      <a-table :data="skuTableData" :pagination="false">
        <template #columns>
          <a-table-column title="规格" data-index="specText" />
          <a-table-column title="SKU编码" data-index="skuCode" />
          <a-table-column title="销售价">
            <template #cell="{ record }">¥{{ record.salePrice }}</template>
          </a-table-column>
          <a-table-column title="库存" data-index="stock" />
          <a-table-column title="销量" data-index="sales" />
          <a-table-column title="状态">
            <template #cell="{ record }">
              <a-tag :color="record.status === 0 ? 'green' : 'red'">{{ record.status === 0 ? '正常' : '禁用' }}</a-tag>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-modal>

    <a-modal v-model:visible="detailModalVisible" title="商品详情" :width="800" :footer="false">
      <a-descriptions :data="detailData" :column="2" bordered />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, computed } from 'vue';
import { Message } from '@arco-design/web-vue';
import { productApi, ProductListItem, ProductDetail, SpecItem, SkuItem } from '@/api/modules/product/product';
import { categoryApi } from '@/api/modules/product/category';
import { brandApi, BrandListItem } from '@/api/modules/product/brand';
import { shippingTemplateApi } from '@/api/modules/product/shippingTemplate';

const loading = ref(false);
const tableData = ref<ProductListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增商品');
const formRef = ref();
const selectedIds = ref<number[]>([]);
const activeTab = ref('basic');

const categoryTree = ref<any[]>([]);
const brandList = ref<BrandListItem[]>([]);
const shippingTemplateList = ref<any[]>([]);

const auditModalVisible = ref(false);
const skuModalVisible = ref(false);
const detailModalVisible = ref(false);
const skuTableData = ref<SkuItem[]>([]);
const detailData = ref<any[]>([]);
const currentProductId = ref(0);

const searchForm = reactive({
  name: '',
  categoryId: undefined as number | undefined,
  brandId: undefined as number | undefined,
  status: undefined as number | undefined,
  auditStatus: undefined as number | undefined,
  productType: undefined as number | undefined,
});

const form = reactive({
  id: 0,
  categoryId: 0,
  brandId: undefined as number | undefined,
  name: '',
  subtitle: '',
  coverImage: '',
  video: '',
  detail: '',
  productType: 0,
  salePrice: 0,
  linePrice: 0,
  costPrice: 0,
  stock: 0,
  virtualSales: 0,
  limitBuy: 0,
  shippingMethod: 0,
  shippingTemplateId: undefined as number | undefined,
  weight: 0,
  volume: 0,
  unit: '件',
  sort: 0,
  isMultiSpec: 0,
  isHot: 0,
  isNew: 0,
  isRecommend: 0,
  keywords: '',
  description: '',
  specs: [] as SpecAddParams[],
  skus: [] as SkuAddParams[],
});

interface SpecAddParams {
  name: string;
  sort: number;
  values: SpecValueAddParams[];
  newValue: string;
}

interface SpecValueAddParams {
  value: string;
  image?: string;
  colorCode?: string;
  sort: number;
}

interface SkuAddParams {
  skuCode: string;
  specValueIds: string;
  specText: string;
  image?: string;
  salePrice: number;
  linePrice: number;
  costPrice: number;
  stock: number;
  weight: number;
  volume: number;
  status: number;
}

const imagesStr = computed({
  get: () => form.coverImage ? form.coverImage : '',
  set: (val: string) => { form.coverImage = val; }
});

const auditForm = reactive({
  id: 0,
  auditStatus: 1,
  auditRemark: '',
});

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60 },
  {
    title: '商品图片',
    dataIndex: 'coverImage',
    slotName: 'coverImage',
    width: 80,
  },
  {
    title: '商品信息',
    slotName: 'productInfo',
    width: 200,
  },
  {
    title: '价格',
    slotName: 'priceInfo',
    width: 120,
  },
  {
    title: '库存',
    dataIndex: 'stock',
    width: 80,
  },
  {
    title: '销量',
    dataIndex: 'sales',
    width: 80,
  },
  {
    title: '类型',
    dataIndex: 'productType',
    slotName: 'productType',
    width: 100,
  },
  {
    title: '状态',
    dataIndex: 'status',
    slotName: 'status',
    width: 80,
  },
  {
    title: '审核',
    dataIndex: 'auditStatus',
    slotName: 'auditStatus',
    width: 100,
  },
  {
    title: '标签',
    slotName: 'tags',
    width: 150,
  },
  {
    title: '排序',
    dataIndex: 'sort',
    width: 80,
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

const getProductTypeColor = (type: number) => {
  const colors: Record<number, string> = {
    0: 'green',
    1: 'blue',
    2: 'orange',
  };
  return colors[type] || 'gray';
};

const getAuditStatusColor = (status: number) => {
  const colors: Record<number, string> = {
    0: 'orange',
    1: 'green',
    2: 'red',
  };
  return colors[status] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const res = await productApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      categoryId: searchForm.categoryId,
      brandId: searchForm.brandId,
      status: searchForm.status,
      auditStatus: searchForm.auditStatus,
      productType: searchForm.productType,
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

const getCategoryTree = async () => {
  try {
    const res = await categoryApi.tree();
    if (res.code === 200) {
      categoryTree.value = res.data || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const getBrandList = async () => {
  try {
    const res = await brandApi.list({ pageSize: 100 });
    if (res.code === 200) {
      brandList.value = res.data?.list || [];
    }
  } catch (error) {
    console.error(error);
  }
};

const getShippingTemplateList = async () => {
  try {
    const res = await shippingTemplateApi.list({ pageSize: 100 });
    if (res.code === 200) {
      shippingTemplateList.value = res.data?.list || [];
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
  searchForm.categoryId = undefined;
  searchForm.brandId = undefined;
  searchForm.status = undefined;
  searchForm.auditStatus = undefined;
  searchForm.productType = undefined;
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

const resetForm = () => {
  form.id = 0;
  form.categoryId = 0;
  form.brandId = undefined;
  form.name = '';
  form.subtitle = '';
  form.coverImage = '';
  form.video = '';
  form.detail = '';
  form.productType = 0;
  form.salePrice = 0;
  form.linePrice = 0;
  form.costPrice = 0;
  form.stock = 0;
  form.virtualSales = 0;
  form.limitBuy = 0;
  form.shippingMethod = 0;
  form.shippingTemplateId = undefined;
  form.weight = 0;
  form.volume = 0;
  form.unit = '件';
  form.sort = 0;
  form.isMultiSpec = 0;
  form.isHot = 0;
  form.isNew = 0;
  form.isRecommend = 0;
  form.keywords = '';
  form.description = '';
  form.specs = [];
  form.skus = [];
  activeTab.value = 'basic';
};

const onAdd = () => {
  modalTitle.value = '新增商品';
  resetForm();
  modalVisible.value = true;
};

const onEdit = async (record: ProductListItem) => {
  modalTitle.value = '编辑商品';
  try {
    const res = await productApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
      form.id = data.id;
      form.categoryId = data.categoryId;
      form.brandId = data.brandId;
      form.name = data.name;
      form.subtitle = data.subtitle || '';
      form.coverImage = data.coverImage;
      form.video = data.video || '';
      form.detail = data.detail || '';
      form.productType = data.productType;
      form.salePrice = data.salePrice;
      form.linePrice = data.linePrice;
      form.costPrice = data.costPrice;
      form.stock = data.stock;
      form.virtualSales = data.virtualSales;
      form.limitBuy = data.limitBuy;
      form.shippingMethod = data.shippingMethod;
      form.shippingTemplateId = data.shippingTemplateId;
      form.weight = data.weight;
      form.volume = data.volume;
      form.unit = data.unit;
      form.sort = data.sort;
      form.isMultiSpec = data.isMultiSpec;
      form.isHot = data.isHot;
      form.isNew = data.isNew;
      form.isRecommend = data.isRecommend;
      form.keywords = data.keywords || '';
      form.description = data.description || '';
      form.specs = (data.specs || []).map((s: any) => ({
        name: s.name,
        sort: s.sort,
        values: s.values || [],
        newValue: '',
      }));
      form.skus = data.skus || [];
      modalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onDetail = async (record: ProductListItem) => {
  try {
    const res = await productApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
      detailData.value = [
        { label: '商品名称', value: data.name },
        { label: '商品副标题', value: data.subtitle || '-' },
        { label: '商品分类', value: data.categoryName || '-' },
        { label: '商品品牌', value: data.brandName || '-' },
        { label: '商品类型', value: ['实物商品', '虚拟商品', '服务商品'][data.productType] },
        { label: '销售价', value: `¥${data.salePrice}` },
        { label: '划线价', value: data.linePrice ? `¥${data.linePrice}` : '-' },
        { label: '成本价', value: data.costPrice ? `¥${data.costPrice}` : '-' },
        { label: '库存', value: data.stock },
        { label: '销量', value: data.sales },
        { label: '虚拟销量', value: data.virtualSales },
        { label: '商品状态', value: data.status === 0 ? '上架' : '下架' },
        { label: '审核状态', value: ['待审核', '审核通过', '审核拒绝'][data.auditStatus] },
        { label: '创建时间', value: data.createdAt || '-' },
      ];
      detailModalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onSkuManage = async (record: ProductListItem) => {
  currentProductId.value = record.id;
  try {
    const res = await productApi.detail(record.id);
    if (res.code === 200 && res.data) {
      skuTableData.value = res.data.skus || [];
      skuModalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onAudit = (record: ProductListItem) => {
  auditForm.id = record.id;
  auditForm.auditStatus = 1;
  auditForm.auditRemark = '';
  auditModalVisible.value = true;
};

const onAuditSubmit = async () => {
  try {
    const res = await productApi.audit({
      id: auditForm.id,
      auditStatus: auditForm.auditStatus,
      auditRemark: auditForm.auditRemark,
    });
    if (res.code === 200) {
      Message.success('审核成功');
      auditModalVisible.value = false;
      getList();
    } else {
      Message.error(res.message || '审核失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onUpdateStatus = async (record: ProductListItem) => {
  const newStatus = record.status === 0 ? 1 : 0;
  try {
    const res = await productApi.updateStatus(record.id, newStatus);
    if (res.code === 200) {
      Message.success(newStatus === 0 ? '上架成功' : '下架成功');
      getList();
    } else {
      Message.error(res.message || '操作失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: ProductListItem) => {
  try {
    const res = await productApi.delete([record.id]);
    if (res.code === 200) {
      Message.success('删除成功');
      getList();
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
    const res = await productApi.delete(selectedIds.value);
    if (res.code === 200) {
      Message.success('批量删除成功');
      getList();
    } else {
      Message.error(res.message || '删除失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const onBatchUpdateStatus = async (status: number) => {
  if (selectedIds.value.length === 0) return;
  try {
    const res = await productApi.batchUpdateStatus(selectedIds.value, status);
    if (res.code === 200) {
      Message.success(status === 0 ? '批量上架成功' : '批量下架成功');
      getList();
    } else {
      Message.error(res.message || '操作失败');
    }
  } catch (error) {
    console.error(error);
  }
};

const addSpec = () => {
  form.specs.push({
    name: '',
    sort: form.specs.length,
    values: [],
    newValue: '',
  });
};

const removeSpec = (index: number) => {
  form.specs.splice(index, 1);
};

const addSpecValue = (specIndex: number) => {
  const spec = form.specs[specIndex];
  if (spec.newValue && spec.newValue.trim()) {
    spec.values.push({
      value: spec.newValue.trim(),
      sort: spec.values.length,
    });
    spec.newValue = '';
  }
};

const removeSpecValue = (specIndex: number, valueIndex: number) => {
  form.specs[specIndex].values.splice(valueIndex, 1);
};

const generateSkus = () => {
  if (form.specs.length === 0) return;

  const specValues = form.specs.map((s) => s.values.map((v) => v.value));
  const combinations = generateCombinations(specValues);

  form.skus = combinations.map((combo) => ({
    skuCode: '',
    specValueIds: '',
    specText: combo.join('/'),
    salePrice: form.salePrice,
    linePrice: form.linePrice,
    costPrice: form.costPrice,
    stock: Math.floor(form.stock / combinations.length),
    weight: form.weight,
    volume: form.volume,
    status: 0,
  }));
};

const generateCombinations = (arrays: string[][]): string[][] => {
  if (arrays.length === 0) return [[]];
  if (arrays.length === 1) return arrays[0].map((item) => [item]);

  const [first, ...rest] = arrays;
  const restCombinations = generateCombinations(rest);

  return first.flatMap((item) =>
    restCombinations.map((combo) => [item, ...combo])
  );
};

const onSubmit = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  try {
    const submitData = {
      ...form,
      specs: form.specs.map((s) => ({
        name: s.name,
        sort: s.sort,
        values: s.values,
      })),
    };

    let res;
    if (form.id) {
      res = await productApi.edit(submitData);
    } else {
      res = await productApi.add(submitData);
    }

    if (res.code === 200) {
      Message.success(form.id ? '编辑成功' : '新增成功');
      modalVisible.value = false;
      getList();
    } else {
      Message.error(res.message || '操作失败');
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
  getCategoryTree();
  getBrandList();
  getShippingTemplateList();
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

.action-icon {
  cursor: pointer;
  padding: 4px;
  border-radius: 4px;
  &:hover {
    background: var(--color-fill-2);
  }
}

.product-info {
  .product-name {
    font-weight: 500;
  }
  .product-subtitle {
    font-size: 12px;
    color: var(--color-text-3);
    margin-top: 4px;
  }
}

.price-info {
  .sale-price {
    color: rgb(var(--danger-6));
    font-weight: 500;
  }
  .line-price {
    font-size: 12px;
    color: var(--color-text-3);
    text-decoration: line-through;
  }
}

.spec-container {
  .spec-header {
    margin-bottom: 16px;
  }
  .spec-list {
    .spec-item {
      margin-bottom: 16px;
      padding: 12px;
      border: 1px solid var(--color-border);
      border-radius: 4px;
      .spec-name {
        display: flex;
        align-items: center;
        gap: 8px;
        margin-bottom: 8px;
      }
      .spec-values {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        align-items: center;
      }
    }
  }
}
</style>

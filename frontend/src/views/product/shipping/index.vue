<template>
  <div class="shipping-page">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">运费模板管理</h2>
        <div class="page-desc">配置商品配送的运费计算规则，支持按重量、数量、金额等多种方式</div>
      </div>
      <div class="header-right">
        <a-button type="primary" size="large" @click="onAdd">
          <template #icon><icon-plus /></template>
          新建模板
        </a-button>
      </div>
    </div>

    <div class="filter-card">
      <a-form ref="searchFormRef" :model="searchForm" layout="inline">
        <a-form-item field="name" label="模板名称">
          <a-input 
            v-model="searchForm.name" 
            placeholder="搜索模板名称" 
            allow-clear 
            class="search-input"
          >
            <template #prefix><icon-search /></template>
          </a-input>
        </a-form-item>
        <a-form-item field="status" label="模板状态">
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

    <div class="content-area">
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
            class="template-card"
          >
            <template #cover>
              <div class="card-cover">
                <div class="cover-icon">
                  <icon-export />
                </div>
                <a-tag :color="item.status === '0' ? 'green' : 'red'" class="status-tag" size="large">
                  {{ item.status === '0' ? '已启用' : '已禁用' }}
                </a-tag>
              </div>
            </template>
            <a-card-meta :title="item.name">
              <template #description>
                <div class="card-desc">
                  <div class="desc-item">
                    <icon-calculator />
                    <span>{{ getChargeTypeText(item.chargeType) }}</span>
                  </div>
                  <div class="desc-item" v-if="item.isFree">
                    <icon-gift />
                    <span>包邮</span>
                  </div>
                  <div class="desc-item">
                    <icon-enviroment />
                    <span>{{ item.regionCount || 0 }}个配送区域</span>
                  </div>
                </div>
              </template>
            </a-card-meta>
            <div class="card-footer">
              <a-space size="small">
                <a-button type="text" size="small" @click="onEdit(item)">
                  <template #icon><icon-edit /></template>
                  编辑
                </a-button>
                <a-button type="text" size="small" @click="onCopy(item)">
                  <template #icon><icon-copy /></template>
                  复制
                </a-button>
                <a-dropdown trigger="click">
                  <a-button type="text" size="small">
                    更多
                    <template #icon><icon-down /></template>
                  </a-button>
                  <template #content>
                    <a-doption v-if="item.status === '0'" @click="onUpdateStatus(item, '1')">
                      <icon-pause-circle /> 禁用
                    </a-doption>
                    <a-doption v-if="item.status === '1'" @click="onUpdateStatus(item, '0')">
                      <icon-play-circle /> 启用
                    </a-doption>
                    <a-divider style="margin: 4px 0" />
                    <a-doption @click="onDelete(item)" style="color: #f53f3f">
                      <icon-delete /> 删除
                    </a-doption>
                  </template>
                </a-dropdown>
              </a-space>
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

    <a-modal
      v-model:visible="modalVisible"
      :title="modalTitle"
      :width="900"
      @ok="onSubmit"
      @cancel="onCancel"
      class="template-modal"
      :footer-style="{ textAlign: 'right' }"
    >
      <a-form ref="formRef" :rules="rules" :model="form" layout="vertical">
        <a-card title="基本信息" class="form-section">
          <a-row :gutter="20">
            <a-col :span="12">
              <a-form-item field="name" label="模板名称" validate-trigger="blur">
                <a-input v-model="form.name" placeholder="请输入模板名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="chargeType" label="计费方式" validate-trigger="blur">
                <a-radio-group v-model="form.chargeType" direction="horizontal">
                  <a-radio :value="0">按重量</a-radio>
                  <a-radio :value="1">按数量</a-radio>
                  <a-radio :value="2">按金额</a-radio>
                </a-radio-group>
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="20">
            <a-col :span="12">
              <a-form-item field="isFree" label="是否包邮">
                <a-switch type="round" :checked-value="1" :unchecked-value="0" v-model="form.isFree">
                  <template #checked>
                    <icon-check /> 是
                  </template>
                  <template #unchecked>
                    <icon-close /> 否
                  </template>
                </a-switch>
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" label="模板状态">
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
        </a-card>

        <a-card title="配送区域配置" class="form-section" style="margin-top: 16px;">
          <template #extra>
            <a-button type="primary" size="small" @click="addRegion">
              <template #icon><icon-plus /></template>
              添加配送区域
            </a-button>
          </template>
          
          <a-alert 
            type="info" 
            :closable="false" 
            style="margin-bottom: 16px;"
          >
            <template #icon>
              <icon-info-circle-fill />
            </template>
            <template #title>
              设置不同区域的运费规则，未配置的区域将使用默认规则
            </template>
          </a-alert>

          <a-table 
            :data="form.regions" 
            :pagination="false" 
            :bordered="{ cell: true }"
            row-key="id"
            class="regions-table"
          >
            <template #columns>
              <a-table-column title="配送区域" data-index="regionNames" :width="300">
                <template #cell="{ record }">
                  <a-button 
                    type="secondary" 
                    size="small" 
                    @click="openRegionSelector(record)"
                    class="region-select-btn"
                  >
                    <template #icon><icon-enviroment /></template>
                    {{ record.regionNames || "选择区域" }}
                  </a-button>
                </template>
              </a-table-column>
              <a-table-column title="首费" :width="200">
                <template #cell="{ record }">
                  <a-space>
                    <a-input-number 
                      v-model="record.firstUnit" 
                      :min="1" 
                      :style="{ width: '70px' }" 
                      placeholder="首"
                      size="small"
                    />
                    <span class="unit-text">{{ getChargeUnit(form.chargeType) }}</span>
                    <span class="fee-text">运费</span>
                    <a-input-number 
                      v-model="record.firstFee" 
                      :min="0" 
                      :precision="2"
                      :style="{ width: '80px' }" 
                      placeholder="费"
                      size="small"
                    />
                    <span class="currency">元</span>
                  </a-space>
                </template>
              </a-table-column>
              <a-table-column title="续费" :width="200">
                <template #cell="{ record }">
                  <a-space>
                    <a-input-number 
                      v-model="record.continueUnit" 
                      :min="1" 
                      :style="{ width: '70px' }" 
                      placeholder="续"
                      size="small"
                    />
                    <span class="unit-text">{{ getChargeUnit(form.chargeType) }}</span>
                    <span class="fee-text">运费</span>
                    <a-input-number 
                      v-model="record.continueFee" 
                      :min="0" 
                      :precision="2"
                      :style="{ width: '80px' }" 
                      placeholder="费"
                      size="small"
                    />
                    <span class="currency">元</span>
                  </a-space>
                </template>
              </a-table-column>
              <a-table-column title="操作" :width="80" fixed="right">
                <template #cell="{ rowIndex }">
                  <a-popconfirm 
                    type="warning" 
                    content="确定删除该配送区域吗?" 
                    @ok="removeRegion(rowIndex)"
                  >
                    <a-button type="text" status="danger" size="small">
                      <icon-delete />
                    </a-button>
                  </a-popconfirm>
                </template>
              </a-table-column>
            </template>
          </a-table>
          
          <div class="empty-region-hint" v-if="form.regions.length === 0">
            <icon-folder-open />
            <span>暂无配送区域，请点击上方按钮添加</span>
          </div>
        </a-card>
      </a-form>
    </a-modal>

    <a-modal
      v-model:visible="regionModalVisible"
      title="选择配送区域"
      :width="700"
      @ok="confirmRegion"
      @cancel="regionModalVisible = false"
      class="region-modal"
    >
      <a-tabs v-model:active-key="regionTab" type="card">
        <a-tab-pane key="province" title="按省份选择">
          <a-checkbox-group v-model="selectedRegionIds" class="region-checkboxes">
            <a-row :gutter="16">
              <a-col :span="8" v-for="province in provinceList" :key="province.code">
                <a-checkbox :value="province.code">
                  <div class="checkbox-item">
                    <icon-enviroment />
                    {{ province.name }}
                  </div>
                </a-checkbox>
              </a-col>
            </a-row>
          </a-checkbox-group>
        </a-tab-pane>
        <a-tab-pane key="quick" title="快速选择">
          <a-space wrap style="margin-bottom: 16px;">
            <a-tag color="arcoblue" @click="selectAllProvinces" class="quick-tag" checkable>
              全国
            </a-tag>
            <a-tag color="green" @click="selectEastChina" class="quick-tag" checkable>
              华东地区
            </a-tag>
            <a-tag color="orange" @click="selectSouthChina" class="quick-tag" checkable>
              华南地区
            </a-tag>
            <a-tag color="purple" @click="selectNorthChina" class="quick-tag" checkable>
              华北地区
            </a-tag>
          </a-space>
        </a-tab-pane>
      </a-tabs>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { shippingTemplateApi, ShippingTemplateListItem, ShippingRegionItem } from "@/api/modules/product/shippingTemplate";

const loading = ref(false);
const tableData = ref<ShippingTemplateListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增运费模板");
const formRef = ref();
const searchFormRef = ref();
const regionModalVisible = ref(false);
const currentRegion = ref<ShippingRegionItem | null>(null);
const selectedRegionIds = ref<string[]>([]);
const regionTab = ref("province");

const searchForm = reactive({
  name: "",
  status: null as string | null
});

const form = reactive({
  id: 0,
  name: "",
  chargeType: 0,
  isFree: 0,
  status: "0",
  regions: [] as ShippingRegionItem[]
});

const rules = {
  name: [{ required: true, message: "请输入模板名称" }]
};

const pagination = reactive({
  current: 1,
  pageSize: 12,
  showPageSize: true,
  showTotal: true,
  total: 0
});

const provinceList = [
  { code: "110000", name: "北京市", region: "north" },
  { code: "120000", name: "天津市", region: "north" },
  { code: "130000", name: "河北省", region: "north" },
  { code: "140000", name: "山西省", region: "north" },
  { code: "150000", name: "内蒙古", region: "north" },
  { code: "210000", name: "辽宁省", region: "east" },
  { code: "220000", name: "吉林省", region: "east" },
  { code: "230000", name: "黑龙江省", region: "east" },
  { code: "310000", name: "上海市", region: "east" },
  { code: "320000", name: "江苏省", region: "east" },
  { code: "330000", name: "浙江省", region: "east" },
  { code: "340000", name: "安徽省", region: "east" },
  { code: "350000", name: "福建省", region: "south" },
  { code: "360000", name: "江西省", region: "south" },
  { code: "370000", name: "山东省", region: "east" },
  { code: "410000", name: "河南省", region: "north" },
  { code: "420000", name: "湖北省", region: "south" },
  { code: "430000", name: "湖南省", region: "south" },
  { code: "440000", name: "广东省", region: "south" },
  { code: "450000", name: "广西省", region: "south" },
  { code: "460000", name: "海南省", region: "south" },
  { code: "500000", name: "重庆市", region: "south" },
  { code: "510000", name: "四川省", region: "south" },
  { code: "520000", name: "贵州省", region: "south" },
  { code: "530000", name: "云南省", region: "south" },
  { code: "540000", name: "西藏省", region: "south" },
  { code: "610000", name: "陕西省", region: "north" },
  { code: "620000", name: "甘肃省", region: "north" },
  { code: "630000", name: "青海省", region: "north" },
  { code: "640000", name: "宁夏省", region: "north" },
  { code: "650000", name: "新疆省", region: "north" }
];

const getChargeTypeText = (type: number) => {
  const texts: Record<number, string> = { 0: "按重量计费", 1: "按数量计费", 2: "按金额计费" };
  return texts[type] || "未知";
};

const getChargeUnit = (type: number) => {
  const units: Record<number, string> = { 0: "kg", 1: "件", 2: "元" };
  return units[type] || "";
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await shippingTemplateApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
      status: searchForm.status || undefined
    });
    tableData.value = (data.list || []).map((item: any) => ({
      ...item,
      regionCount: item.regions?.length || 0
    }));
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

const handlePageChange = (page: number) => {
  pagination.current = page;
  getList();
};

const handlePageSizeChange = (pageSize: number) => {
  pagination.pageSize = pageSize;
  getList();
};

const onAdd = () => {
  modalTitle.value = "新增运费模板";
  form.id = 0;
  form.name = "";
  form.chargeType = 0;
  form.isFree = 0;
  form.status = "0";
  form.regions = [];
  modalVisible.value = true;
};

const onEdit = async (record: ShippingTemplateListItem) => {
  modalTitle.value = "编辑运费模板";
  try {
    const data = await shippingTemplateApi.detail(record.id);
    form.id = data.id;
    form.name = data.name;
    form.chargeType = data.chargeType;
    form.isFree = data.isFree;
    form.status = data.status;
    form.regions = data.regions || [];
    modalVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onCopy = async (record: ShippingTemplateListItem) => {
  try {
    const data = await shippingTemplateApi.detail(record.id);
    form.id = 0;
    form.name = data.name + "(复制)";
    form.chargeType = data.chargeType;
    form.isFree = data.isFree;
    form.status = "0";
    form.regions = (data.regions || []).map((r: any) => ({ ...r, id: 0 }));
    modalTitle.value = "新增运费模板";
    modalVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onUpdateStatus = async (record: ShippingTemplateListItem, status: string) => {
  try {
    await shippingTemplateApi.edit({ ...record, status });
    arcoMessage("success", status === "0" ? "启用成功" : "禁用成功");
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: ShippingTemplateListItem) => {
  arcoMessage("warning", "确定删除该运费模板吗?");
};

const addRegion = () => {
  form.regions.push({
    id: 0,
    templateId: form.id,
    regionType: 1,
    regionIds: "",
    regionNames: "",
    firstUnit: 1,
    firstFee: 0,
    continueUnit: 1,
    continueFee: 0,
    isFree: 0,
    freeConditionValue: 0
  });
};

const removeRegion = (index: number) => {
  form.regions.splice(index, 1);
};

const openRegionSelector = (record: ShippingRegionItem) => {
  currentRegion.value = record;
  selectedRegionIds.value = record.regionIds ? record.regionIds.split(",") : [];
  regionModalVisible.value = true;
};

const confirmRegion = () => {
  if (currentRegion.value) {
    const names = selectedRegionIds.value
      .map(code => provinceList.find(p => p.code === code)?.name)
      .filter(Boolean)
      .join(",");
    currentRegion.value.regionIds = selectedRegionIds.value.join(",");
    currentRegion.value.regionNames = names || "指定区域";
  }
  regionModalVisible.value = false;
};

const selectAllProvinces = () => {
  selectedRegionIds.value = provinceList.map(p => p.code);
};

const selectEastChina = () => {
  selectedRegionIds.value = provinceList.filter(p => p.region === "east").map(p => p.code);
};

const selectSouthChina = () => {
  selectedRegionIds.value = provinceList.filter(p => p.region === "south").map(p => p.code);
};

const selectNorthChina = () => {
  selectedRegionIds.value = provinceList.filter(p => p.region === "north").map(p => p.code);
};

const onSubmit = async () => {
  let state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await shippingTemplateApi.edit(form);
      arcoMessage("success", "修改成功");
    } else {
      await shippingTemplateApi.add(form);
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
.shipping-page {
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
    margin: 0 0 8px 0;
  }
  
  .page-desc {
    color: #86909c;
    font-size: 14px;
    max-width: 600px;
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

.content-area {
  flex: 1;
  overflow: auto;
  background: #ffffff;
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.template-card {
  margin-bottom: 20px;
  transition: all 0.3s;
  
  &:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
  }
  
  :deep(.arco-card-body) {
    padding: 16px;
  }
}

.card-cover {
  height: 120px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  
  .cover-icon {
    width: 64px;
    height: 64px;
    background: rgba(255, 255, 255, 0.2);
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 32px;
    color: #ffffff;
  }
  
  .status-tag {
    position: absolute;
    top: 12px;
    right: 12px;
  }
}

.card-desc {
  .desc-item {
    display: flex;
    align-items: center;
    gap: 6px;
    margin-bottom: 8px;
    font-size: 13px;
    color: #4e5969;
  }
}

.card-footer {
  padding-top: 12px;
  border-top: 1px solid #e5e6eb;
  margin-top: 12px;
  display: flex;
  justify-content: flex-end;
}

.pagination-wrapper {
  display: flex;
  justify-content: center;
  margin-top: 24px;
}

.template-modal {
  :deep(.arco-modal-body) {
    padding-top: 24px;
    max-height: 70vh;
    overflow-y: auto;
  }
}

.form-section {
  :deep(.arco-card-header) {
    background: #f7f8fa;
  }
}

.regions-table {
  :deep(.arco-table-th) {
    background: #f7f8fa;
  }
}

.region-select-btn {
  width: 100%;
  justify-content: flex-start;
  
  .arco-icon {
    margin-right: 4px;
  }
}

.unit-text,
.fee-text,
.currency {
  color: #86909c;
  font-size: 13px;
}

.empty-region-hint {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 40px;
  color: #86909c;
  font-size: 14px;
}

.region-modal {
  :deep(.arco-modal-body) {
    padding-top: 24px;
  }
}

.region-checkboxes {
  width: 100%;
  
  .checkbox-item {
    display: flex;
    align-items: center;
    gap: 6px;
  }
}

.quick-tag {
  cursor: pointer;
  transition: all 0.2s;
  
  &:hover {
    transform: scale(1.05);
  }
}
</style>

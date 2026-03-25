<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-form ref="searchFormRef" :model="searchForm" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="模板名称">
              <a-input v-model="searchForm.name" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="模板状态">
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
        <template #chargeType="{ record }">
          <a-tag :color="getChargeTypeColor(record.chargeType)" bordered size="small">{{ record.chargeTypeName }}</a-tag>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'" bordered size="small">
            {{ record.status === '0' ? '启用' : '禁用' }}
          </a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onCopy(record)">复制</a-button>
            <a-popconfirm type="warning" content="确定删除该模板吗?" @ok="onDelete(record)">
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
            <a-form-item field="name" label="模板名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入模板名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="chargeType" label="计费方式" validate-trigger="blur">
              <a-radio-group v-model="form.chargeType">
                <a-radio :value="0">按重量</a-radio>
                <a-radio :value="1">按数量</a-radio>
                <a-radio :value="2">按金额</a-radio>
              </a-radio-group>
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="isFree" label="是否包邮" validate-trigger="blur">
              <a-switch type="round" :checked-value="1" :unchecked-value="0" v-model="form.isFree" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="status" label="状态" validate-trigger="blur">
              <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
                <template #checked> 启用 </template>
                <template #unchecked> 禁用 </template>
              </a-switch>
            </a-form-item>
          </a-col>
        </a-row>

        <a-divider>配送区域</a-divider>

        <a-table :data="form.regions" :pagination="false" :bordered="{ cell: true }">
          <template #columns>
            <a-table-column title="配送区域" data-index="regionNames">
              <template #cell="{ record }">
                <a-button size="small" @click="openRegionSelector(record)">
                  {{ record.regionNames || '选择区域' }}
                </a-button>
              </template>
            </a-table-column>
            <a-table-column title="首费">
              <template #cell="{ record }">
                <a-space>
                  <a-input-number v-model="record.firstUnit" :min="1" :style="{ width: '60px' }" placeholder="首" />
                  <span>x</span>
                  <a-input-number v-model="record.firstFee" :min="0" :style="{ width: '80px' }" placeholder="费" />
                </a-space>
              </template>
            </a-table-column>
            <a-table-column title="续费">
              <template #cell="{ record }">
                <a-space>
                  <a-input-number v-model="record.continueUnit" :min="1" :style="{ width: '60px' }" placeholder="续" />
                  <span>x</span>
                  <a-input-number v-model="record.continueFee" :min="0" :style="{ width: '80px' }" placeholder="费" />
                </a-space>
              </template>
            </a-table-column>
            <a-table-column title="操作" :width="80">
              <template #cell="{ rowIndex }">
                <a-button type="text" status="danger" size="small" @click="removeRegion(rowIndex)">
                  <icon-delete />
                </a-button>
              </template>
            </a-table-column>
          </template>
        </a-table>
        <a-button type="primary" size="small" @click="addRegion" style="margin-top: 12px">
          <template #icon><icon-plus /></template>
          添加配送区域
        </a-button>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="regionModalVisible" title="选择配送区域" :width="600" @ok="confirmRegion" @cancel="regionModalVisible = false">
      <a-checkbox-group v-model="selectedRegionIds">
        <a-row>
          <a-col :span="8" v-for="province in provinceList" :key="province.code">
            <a-checkbox :value="province.code">{{ province.name }}</a-checkbox>
          </a-col>
        </a-row>
      </a-checkbox-group>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { shippingTemplateApi, ShippingTemplateListItem, ShippingRegionItem } from '@/api/modules/product/shippingTemplate';

const loading = ref(false);
const tableData = ref<ShippingTemplateListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增模板');
const formRef = ref();
const selectedIds = ref<string[]>([]);
const searchFormRef = ref();
const regionModalVisible = ref(false);
const currentRegion = ref<ShippingRegionItem | null>(null);
const selectedRegionIds = ref<string[]>([]);

const searchForm = reactive({
  name: '',
  status: null as string | null,
});

const form = reactive({
  id: 0,
  name: '',
  chargeType: 0,
  isFree: 0,
  status: '0',
  regions: [] as ShippingRegionItem[],
});

const provinceList = [
  { code: '110000', name: '北京市' }, { code: '120000', name: '天津市' }, { code: '130000', name: '河北省' },
  { code: '140000', name: '山西省' }, { code: '150000', name: '内蒙古' }, { code: '210000', name: '辽宁省' },
  { code: '220000', name: '吉林省' }, { code: '230000', name: '黑龙江省' }, { code: '310000', name: '上海市' },
  { code: '320000', name: '江苏省' }, { code: '330000', name: '浙江省' }, { code: '340000', name: '安徽省' },
  { code: '350000', name: '福建省' }, { code: '360000', name: '江西省' }, { code: '370000', name: '山东省' },
  { code: '410000', name: '河南省' }, { code: '420000', name: '湖北省' }, { code: '430000', name: '湖南省' },
  { code: '440000', name: '广东省' }, { code: '450000', name: '广西省' }, { code: '460000', name: '海南省' },
  { code: '500000', name: '重庆市' }, { code: '510000', name: '四川省' }, { code: '520000', name: '贵州省' },
  { code: '530000', name: '云南省' }, { code: '540000', name: '西藏省' }, { code: '610000', name: '陕西省' },
  { code: '620000', name: '甘肃省' }, { code: '630000', name: '青海省' }, { code: '640000', name: '宁夏省' },
  { code: '650000', name: '新疆省' },
];

const rules = {
  name: [{ required: true, message: '请输入模板名称' }],
};

const pagination = reactive({
  current: 1,
  pageSize: 10,
  showPageSize: true,
  showTotal: true,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60, fixed: 'left' },
  { title: '模板名称', dataIndex: 'name', width: 200 },
  { title: '计费方式', slotName: 'chargeType', width: 120 },
  { title: '是否包邮', dataIndex: 'isFree', width: 100 },
  { title: '状态', slotName: 'status', width: 100 },
  { title: '创建时间', dataIndex: 'createdAt', width: 180 },
  { title: '操作', slotName: 'optional', width: 180, fixed: 'right' },
];

const getChargeTypeColor = (type: number) => {
  const colors: Record<number, string> = { 0: 'blue', 1: 'green', 2: 'orange' };
  return colors[type] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const data = await shippingTemplateApi.list({
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
  modalTitle.value = '新增运费模板';
  form.id = 0;
  form.name = '';
  form.chargeType = 0;
  form.isFree = 0;
  form.status = '0';
  form.regions = [];
  modalVisible.value = true;
};

const onEdit = async (record: ShippingTemplateListItem) => {
  modalTitle.value = '编辑运费模板';
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
    form.name = data.name + '(复制)';
    form.chargeType = data.chargeType;
    form.isFree = data.isFree;
    form.status = '0';
    form.regions = (data.regions || []).map((r: any) => ({ ...r, id: 0 }));
    modalTitle.value = '新增运费模板';
    modalVisible.value = true;
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: ShippingTemplateListItem) => {
  try {
    await shippingTemplateApi.delete([record.id]);
    arcoMessage('success', '删除成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onBatchDelete = async () => {
  if (selectedIds.value.length === 0) return;
  try {
    await shippingTemplateApi.delete(selectedIds.value.map((id) => Number(id)));
    arcoMessage('success', '批量删除成功');
    selectedIds.value = [];
    getList();
  } catch (error) {
    console.error(error);
  }
};

const addRegion = () => {
  form.regions.push({
    id: 0,
    templateId: form.id,
    regionType: 1,
    regionIds: '',
    regionNames: '',
    firstUnit: 1,
    firstFee: 0,
    continueUnit: 1,
    continueFee: 0,
    isFree: 0,
    freeConditionValue: 0,
  });
};

const removeRegion = (index: number) => {
  form.regions.splice(index, 1);
};

const openRegionSelector = (record: ShippingRegionItem) => {
  currentRegion.value = record;
  selectedRegionIds.value = record.regionIds ? record.regionIds.split(',') : [];
  regionModalVisible.value = true;
};

const confirmRegion = () => {
  if (currentRegion.value) {
    const names = selectedRegionIds.value
      .map((code) => provinceList.find((p) => p.code === code)?.name)
      .filter(Boolean)
      .join(',');
    currentRegion.value.regionIds = selectedRegionIds.value.join(',');
    currentRegion.value.regionNames = names || '指定区域';
  }
  regionModalVisible.value = false;
};

const onSubmit = async () => {
  let state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await shippingTemplateApi.edit(form);
      arcoMessage('success', '修改成功');
    } else {
      await shippingTemplateApi.add(form);
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

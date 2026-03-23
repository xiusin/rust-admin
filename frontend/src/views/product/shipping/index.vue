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
              新增模板
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
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columns"
        :data="tableData"
        :pagination="pagination"
        @page-change="handlePageChange"
        @page-size-change="handlePageSizeChange"
        @selection-change="handleSelectionChange"
      >
        <template #chargeType="{ record }">
          <a-tag :color="getChargeTypeColor(record.chargeType)">{{ record.chargeTypeName }}</a-tag>
        </template>
        <template #isFree="{ record }">
          <a-tag :color="record.isFree === 1 ? 'green' : 'gray'">{{ record.isFree === 1 ? '是' : '否' }}</a-tag>
        </template>
        <template #status="{ record }">
          <a-tag :color="record.status === '0' ? 'green' : 'red'">{{ record.status === '0' ? '启用' : '禁用' }}</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onEdit(record)">编辑</a-button>
            <a-button type="text" size="mini" @click="onCopy(record)">复制</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-if="record.status === '0'">禁用</a-button>
            <a-button type="text" size="mini" @click="onUpdateStatus(record)" v-else>启用</a-button>
            <a-popconfirm type="warning" content="确定删除该模板吗?" @ok="onDelete(record)">
              <a-button type="text" size="mini" status="danger">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal v-model:visible="modalVisible" :title="modalTitle" :width="900" @ok="onSubmit" @cancel="onCancel">
      <a-form ref="formRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="name" label="模板名称" :rules="[{ required: true, message: '请输入模板名称' }]">
              <a-input v-model="form.name" placeholder="请输入模板名称" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item field="chargeType" label="计费方式">
              <a-radio-group v-model="form.chargeType">
                <a-radio :value="0">按重量</a-radio>
                <a-radio :value="1">按数量</a-radio>
                <a-radio :value="2">按金额</a-radio>
              </a-radio-group>
            </a-form-item>
          </a-col>
        </a-row>

        <a-divider>包邮设置</a-divider>

        <a-row :gutter="16">
          <a-col :span="12">
            <a-form-item field="isFree" label="是否包邮">
              <a-switch v-model="form.isFree" :checked-value="1" :unchecked-value="0" />
            </a-form-item>
          </a-col>
          <a-col :span="12" v-if="form.isFree === 1">
            <a-form-item field="freeConditionType" label="包邮条件">
              <a-select v-model="form.freeConditionType" placeholder="请选择包邮条件">
                <a-option :value="0">金额</a-option>
                <a-option :value="1">数量</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :span="12" v-if="form.isFree === 1">
            <a-form-item field="freeConditionValue" label="条件值">
              <a-input-number
                v-model="form.freeConditionValue"
                :placeholder="form.freeConditionType === 0 ? '订单金额' : '商品数量'"
                :min="0"
              />
            </a-form-item>
          </a-col>
        </a-row>

        <a-divider>配送区域设置</a-divider>

        <div class="region-list">
          <div class="region-header">
            <a-button type="primary" size="small" @click="addRegion">
              <template #icon><icon-plus /></template>
              添加配送区域
            </a-button>
          </div>
          <a-table :data="form.regions" :pagination="false" :bordered="true" style="margin-top: 16px">
            <template #columns>
              <a-table-column title="配送区域" data-index="regionNames" :width="200">
                <template #cell="{ record, rowIndex }">
                  <a-button size="small" @click="openRegionSelector(rowIndex)">
                    {{ record.regionNames || '请选择区域' }}
                  </a-button>
                </template>
              </a-table-column>
              <a-table-column title="首件/首重/首额" :width="200">
                <template #cell="{ record }">
                  <a-space>
                    <a-input-number v-model="record.firstUnit" :min="1" placeholder="首" style="width: 60px" />
                    <span>×</span>
                    <a-input-number v-model="record.firstFee" :min="0" placeholder="费" style="width: 80px" />
                  </a-space>
                </template>
              </a-table-column>
              <a-table-column title="续件/续重/续额" :width="200">
                <template #cell="{ record }">
                  <a-space>
                    <a-input-number v-model="record.continueUnit" :min="1" placeholder="续" style="width: 60px" />
                    <span>×</span>
                    <a-input-number v-model="record.continueFee" :min="0" placeholder="费" style="width: 80px" />
                  </a-space>
                </template>
              </a-table-column>
              <a-table-column title="是否包邮" :width="100">
                <template #cell="{ record }">
                  <a-switch v-model="record.isFree" :checked-value="1" :unchecked-value="0" size="small" />
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
        </div>

        <a-form-item field="status" label="状态" style="margin-top: 16px">
          <a-radio-group v-model="form.status">
            <a-radio value="0">启用</a-radio>
            <a-radio value="1">禁用</a-radio>
          </a-radio-group>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="regionModalVisible" title="选择配送区域" :width="800" @ok="confirmRegion" @cancel="regionModalVisible = false">
      <div class="region-selector">
        <a-checkbox-group v-model="selectedRegionIds" :max="10">
          <a-row :gutter="16">
            <a-col :span="8" v-for="province in provinceList" :key="province.code">
              <a-checkbox :value="province.code">{{ province.name }}</a-checkbox>
            </a-col>
          </a-row>
        </a-checkbox-group>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { Message } from '@arco-design/web-vue';
import { shippingTemplateApi, ShippingTemplateListItem, ShippingRegionItem } from '@/api/modules/product/shippingTemplate';

const loading = ref(false);
const tableData = ref<ShippingTemplateListItem[]>([]);
const modalVisible = ref(false);
const modalTitle = ref('新增运费模板');
const formRef = ref();
const selectedIds = ref<number[]>([]);
const regionModalVisible = ref(false);
const currentRegionIndex = ref(0);
const selectedRegionIds = ref<string[]>([]);

const searchForm = reactive({
  name: '',
  status: '',
});

const form = reactive({
  id: 0,
  name: '',
  chargeType: 0,
  isFree: 0,
  freeConditionType: 0,
  freeConditionValue: 0,
  status: '0',
  regions: [] as ShippingRegionItem[],
});

const provinceList = ref<{ code: string; name: string }[]>([
  { code: '110000', name: '北京市' },
  { code: '120000', name: '天津市' },
  { code: '130000', name: '河北省' },
  { code: '140000', name: '山西省' },
  { code: '150000', name: '内蒙古' },
  { code: '210000', name: '辽宁省' },
  { code: '220000', name: '吉林省' },
  { code: '230000', name: '黑龙江省' },
  { code: '310000', name: '上海市' },
  { code: '320000', name: '江苏省' },
  { code: '330000', name: '浙江省' },
  { code: '340000', name: '安徽省' },
  { code: '350000', name: '福建省' },
  { code: '360000', name: '江西省' },
  { code: '370000', name: '山东省' },
  { code: '410000', name: '河南省' },
  { code: '420000', name: '湖北省' },
  { code: '430000', name: '湖南省' },
  { code: '440000', name: '广东省' },
  { code: '450000', name: '广西省' },
  { code: '460000', name: '海南省' },
  { code: '500000', name: '重庆市' },
  { code: '510000', name: '四川省' },
  { code: '520000', name: '贵州省' },
  { code: '530000', name: '云南省' },
  { code: '540000', name: '西藏省' },
  { code: '610000', name: '陕西省' },
  { code: '620000', name: '甘肃省' },
  { code: '630000', name: '青海省' },
  { code: '640000', name: '宁夏省' },
  { code: '650000', name: '新疆省' },
]);

const pagination = reactive({
  current: 1,
  pageSize: 10,
  total: 0,
});

const columns = [
  { type: 'selection', width: 60 },
  {
    title: '模板名称',
    dataIndex: 'name',
    width: 200,
  },
  {
    title: '计费方式',
    dataIndex: 'chargeType',
    slotName: 'chargeType',
    width: 120,
  },
  {
    title: '是否包邮',
    dataIndex: 'isFree',
    slotName: 'isFree',
    width: 100,
  },
  {
    title: '商品数量',
    dataIndex: 'productCount',
    width: 100,
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

const getChargeTypeColor = (type: number) => {
  const colors: Record<number, string> = {
    0: 'blue',
    1: 'green',
    2: 'orange',
  };
  return colors[type] || 'gray';
};

const getList = async () => {
  loading.value = true;
  try {
    const res = await shippingTemplateApi.list({
      pageNum: pagination.current,
      pageSize: pagination.pageSize,
      name: searchForm.name || undefined,
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

const search = () => {
  pagination.current = 1;
  getList();
};

const reset = () => {
  searchForm.name = '';
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

const resetForm = () => {
  form.id = 0;
  form.name = '';
  form.chargeType = 0;
  form.isFree = 0;
  form.freeConditionType = 0;
  form.freeConditionValue = 0;
  form.status = '0';
  form.regions = [
    {
      id: 0,
      templateId: 0,
      regionType: 1,
      regionIds: '',
      regionNames: '默认全国',
      firstUnit: 1,
      firstFee: 0,
      continueUnit: 1,
      continueFee: 0,
      isFree: 0,
      freeConditionValue: 0,
    },
  ];
};

const onAdd = () => {
  modalTitle.value = '新增运费模板';
  resetForm();
  modalVisible.value = true;
};

const onEdit = async (record: ShippingTemplateListItem) => {
  modalTitle.value = '编辑运费模板';
  try {
    const res = await shippingTemplateApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
      form.id = data.id;
      form.name = data.name;
      form.chargeType = data.chargeType;
      form.isFree = data.isFree;
      form.freeConditionType = data.freeConditionType;
      form.freeConditionValue = data.freeConditionValue;
      form.status = data.status;
      form.regions = data.regions || [];
      modalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onCopy = async (record: ShippingTemplateListItem) => {
  try {
    const res = await shippingTemplateApi.detail(record.id);
    if (res.code === 200 && res.data) {
      const data = res.data;
      form.id = 0;
      form.name = data.name + '(复制)';
      form.chargeType = data.chargeType;
      form.isFree = data.isFree;
      form.freeConditionType = data.freeConditionType;
      form.freeConditionValue = data.freeConditionValue;
      form.status = '0';
      form.regions = (data.regions || []).map((r: any) => ({ ...r, id: 0 }));
      modalTitle.value = '新增运费模板';
      modalVisible.value = true;
    }
  } catch (error) {
    console.error(error);
  }
};

const onDelete = async (record: ShippingTemplateListItem) => {
  try {
    const res = await shippingTemplateApi.delete([record.id]);
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
    const res = await shippingTemplateApi.delete(selectedIds.value);
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

const onUpdateStatus = async (record: ShippingTemplateListItem) => {
  const newStatus = record.status === '0' ? '1' : '0';
  try {
    const res = await shippingTemplateApi.edit({
      id: record.id,
      name: record.name,
      chargeType: record.chargeType,
      isFree: record.isFree,
      freeConditionType: record.freeConditionType,
      freeConditionValue: record.freeConditionValue,
      status: newStatus,
      regions: [],
    });
    if (res.code === 200) {
      Message.success(newStatus === '0' ? '启用成功' : '禁用成功');
      getList();
    } else {
      Message.error(res.message || '操作失败');
    }
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

const openRegionSelector = (index: number) => {
  currentRegionIndex.value = index;
  const current = form.regions[index];
  selectedRegionIds.value = current.regionIds ? current.regionIds.split(',') : [];
  regionModalVisible.value = true;
};

const confirmRegion = () => {
  const regionNames = selectedRegionIds.value
    .map((code) => provinceList.value.find((p) => p.code === code)?.name)
    .filter(Boolean)
    .join(',');
  form.regions[currentRegionIndex.value].regionIds = selectedRegionIds.value.join(',');
  form.regions[currentRegionIndex.value].regionNames = regionNames || '指定区域';
  regionModalVisible.value = false;
};

const onSubmit = async () => {
  const valid = await formRef.value?.validate();
  if (valid) return;

  try {
    let res;
    const submitData = {
      ...form,
      regions: form.regions.map((r) => ({
        regionType: r.regionType,
        regionIds: r.regionIds,
        regionNames: r.regionNames,
        firstUnit: r.firstUnit,
        firstFee: r.firstFee,
        continueUnit: r.continueUnit,
        continueFee: r.continueFee,
        isFree: r.isFree,
        freeConditionValue: r.freeConditionValue,
      })),
    };

    if (form.id) {
      res = await shippingTemplateApi.edit(submitData);
    } else {
      res = await shippingTemplateApi.add(submitData);
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

.region-list {
  .region-header {
    margin-bottom: 8px;
  }
}

.region-selector {
  max-height: 400px;
  overflow-y: auto;
}
</style>

<template>
  <div class="snow-page">
    <div class="snow-inner">
      <!-- 搜索区域 -->
      <a-form ref="searchFormRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="dict_name" label="字典名称">
              <a-input v-model="form.dict_name" placeholder="请输入字典名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="dict_type" label="字典编码">
              <a-input v-model="form.dict_type" placeholder="请输入字典编码" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="启用状态">
              <a-select v-model="form.status" placeholder="请选择状态" allow-clear>
                <a-option v-for="item in openState" :key="item.value" :value="item.value">{{ item.name }}</a-option>
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

      <!-- 操作栏 -->
      <a-row :gutter="16" style="margin: 16px 0">
        <a-col :span="12">
          <a-space size="medium">
            <a-button type="primary" size="small" @click="onAdd">
              <template #icon><icon-plus /></template>
              新增
            </a-button>
            <a-button type="primary" status="danger" size="small" @click="onBatchDelete" :disabled="selectedKeys.length === 0">
              <template #icon><icon-delete /></template>
              删除
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getDictList"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-dropdown @select="onDensity">
              <a-tooltip content="密度">
                <div class="action-icon"><icon-line-height size="18" /></div>
              </a-tooltip>
              <template #content>
                <a-doption v-for="item in densityType" :value="item.value" :key="item.value" :disabled="item.value === density">{{
                  item.label
                }}</a-doption>
              </template>
            </a-dropdown>
            <a-tooltip content="列设置">
              <a-popover trigger="click" position="br" @popup-visible-change="popupVisibleChange">
                <div class="action-icon"><icon-settings size="18" /></div>
                <template #content>
                  <div id="tableSetting">
                    <div v-for="(item, index) in columns" :key="item.dataIndex" class="setting">
                      <div class="setting-box-icon"><icon-drag-arrow /></div>
                      <div><a-checkbox v-model="item.checked" @change="onCheckbox($event, item, index)"></a-checkbox></div>
                      <div class="title">{{ item.title }}</div>
                    </div>
                  </div>
                </template>
              </a-popover>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <!-- 字典类型表格 -->
      <a-table
        row-key="dict_id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columnsShow"
        :data="dictList"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedKeys"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status == '1' || record.status == 1">启用</a-tag>
          <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onDictData(record)">字典值</a-button>
            <a-button type="text" size="mini" @click="onUpdate(record)">修改</a-button>
            <a-popconfirm type="warning" content="确定删除该字典吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <!-- 字典类型表单弹窗 -->
    <a-modal :width="dialogWidth('30%')" v-model:visible="open" @close="afterClose" @ok="handleOk" @cancel="afterClose">
      <template #title> {{ title }} </template>
      <div>
        <a-form ref="formRef" auto-label-width :layout="formLayout" :rules="rules" :model="addFrom">
          <a-form-item field="dict_name" label="字典名称" validate-trigger="blur">
            <a-input v-model="addFrom.dict_name" placeholder="请输入字典名称" allow-clear />
          </a-form-item>
          <a-form-item field="dict_type" label="字典编码" validate-trigger="blur">
            <a-input v-model="addFrom.dict_type" placeholder="请输入字典编码" allow-clear />
          </a-form-item>
          <a-form-item field="order" label="排序" validate-trigger="blur">
            <a-input-number v-model="addFrom.order" placeholder="请输入排序" :min="0" :step="1" allow-clear />
          </a-form-item>
          <a-form-item field="remark" label="描述" validate-trigger="blur">
            <a-textarea v-model="addFrom.remark" placeholder="请输入字典描述" allow-clear />
          </a-form-item>
          <a-form-item field="status" label="状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="'1'" :unchecked-value="'0'" v-model="addFrom.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
        </a-form>
      </div>
    </a-modal>

    <!-- 字典数据列表弹窗 -->
    <a-modal :width="dialogWidth('60%')" v-model:visible="detailOpen" @ok="detailOk" ok-text="关闭" :hide-cancel="true">
      <template #title> 字典数据 - {{ currentDictType?.dict_name }} </template>
      <div>
        <!-- 字典数据操作栏 -->
        <a-row :gutter="16" style="margin: 16px 0">
          <a-col :span="12">
            <a-space size="medium">
              <a-button type="primary" size="small" @click="onAddDetail">
                <template #icon><icon-plus /></template>
                新增
              </a-button>
              <a-button
                type="primary"
                status="danger"
                size="small"
                @click="onBatchDeleteDetail"
                :disabled="selectedKeysDetail.length === 0"
              >
                <template #icon><icon-delete /></template>
                删除
              </a-button>
            </a-space>
          </a-col>
          <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
            <a-space size="medium">
              <a-tooltip content="刷新">
                <div class="action-icon" @click="getDictDataList"><icon-refresh size="18" /></div>
              </a-tooltip>
              <a-dropdown @select="onDensityDetail">
                <a-tooltip content="密度">
                  <div class="action-icon"><icon-line-height size="18" /></div>
                </a-tooltip>
                <template #content>
                  <a-doption
                    v-for="item in densityType"
                    :value="item.value"
                    :key="item.value"
                    :disabled="item.value === densityDetail"
                    >{{ item.label }}</a-doption
                  >
                </template>
              </a-dropdown>
              <a-tooltip content="列设置">
                <a-popover trigger="click" position="br" @popup-visible-change="popupVisibleChangeDetail">
                  <div class="action-icon"><icon-settings size="18" /></div>
                  <template #content>
                    <div id="tableSettingDetail">
                      <div v-for="(item, index) in detailColumns" :key="item.dataIndex" class="setting">
                        <div class="setting-box-icon"><icon-drag-arrow /></div>
                        <div>
                          <a-checkbox v-model="item.checked" @change="onCheckboxDetail($event, item, index)"></a-checkbox>
                        </div>
                        <div class="title">{{ item.title }}</div>
                      </div>
                    </div>
                  </template>
                </a-popover>
              </a-tooltip>
            </a-space>
          </a-col>
        </a-row>

        <!-- 字典数据表格 -->
        <a-table
          row-key="dict_code"
          column-resizable
          :loading="detailLoading"
          :size="densityDetail"
          :bordered="{ cell: true }"
          :scroll="{ x: '100%', y: '100%' }"
          :columns="detailColumnsShow"
          :data="dictDetailList"
          :row-selection="{ type: 'checkbox', showCheckedAll: true }"
          v-model:selectedKeys="selectedKeysDetail"
          :pagination="detailPagination"
          @page-change="detailPageChange"
          @page-size-change="detailPageSizeChange"
        >
          <template #status="{ record }">
            <a-tag bordered size="small" color="arcoblue" v-if="record.status == '1' || record.status == 1">启用</a-tag>
            <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
          </template>
          <template #optional="{ record }">
            <a-space>
              <a-button type="text" size="mini" @click="onDetailUpdate(record)">修改</a-button>
              <a-popconfirm type="warning" content="确定删除该字典数据吗?" @ok="onDetailDelete(record)">
                <a-button type="text" status="danger" size="mini">删除</a-button>
              </a-popconfirm>
            </a-space>
          </template>
        </a-table>
      </div>
    </a-modal>

    <!-- 字典数据表单弹窗 -->
    <a-modal
      :width="dialogWidth('30%')"
      v-model:visible="detailCaseOpen"
      @close="afterCloseDetail"
      @ok="handleOkDetail"
      @cancel="afterCloseDetail"
    >
      <template #title> {{ detailTitle }} </template>
      <div>
        <a-form ref="detailFormRef" auto-label-width :layout="formLayout" :rules="detailRules" :model="detailForm">
          <a-form-item field="dict_label" label="字典标签" validate-trigger="blur">
            <a-input v-model="detailForm.dict_label" placeholder="请输入字典标签" allow-clear />
          </a-form-item>
          <a-form-item field="dict_value" label="字典值" validate-trigger="blur">
            <a-input v-model="detailForm.dict_value" placeholder="请输入字典值" allow-clear />
          </a-form-item>
          <a-form-item field="dict_sort" label="排序" validate-trigger="blur">
            <a-input-number v-model="detailForm.dict_sort" placeholder="请输入排序" :min="0" :step="1" allow-clear />
          </a-form-item>
          <a-form-item field="remark" label="描述" validate-trigger="blur">
            <a-textarea v-model="detailForm.remark" placeholder="请输入描述" allow-clear />
          </a-form-item>
          <a-form-item field="status" label="状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="'1'" :unchecked-value="'0'" v-model="detailForm.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import {
  getDictTypeListAPI,
  addDictTypeAPI,
  editDictTypeAPI,
  deleteDictTypeAPI,
  getDictDataListAPI,
  addDictDataAPI,
  editDictDataAPI,
  deleteDictDataAPI
} from "@/api/modules/system/dict";
import { deepClone } from "@/utils";
import { useLayoutModel } from "@/hooks/useLayoutModel";
import Sortable from "sortablejs";

const { dialogWidth, formLayout } = useLayoutModel();
const openState = ref(dictFilter("status"));

// 搜索表单
const form = ref({
  dict_name: "",
  dict_type: "",
  status: null as string | null
});
const searchFormRef = ref();

const search = () => {
  pagination.value.current = 1;
  getDictList();
};

const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.value.current = 1;
  getDictList();
};

// 密度设置
const densityType = ref([
  { value: "mini", label: "迷你" },
  { value: "small", label: "偏小" },
  { value: "medium", label: "中等" },
  { value: "large", label: "偏大" }
]);
const density = ref("small");
const onDensity = (e: string) => {
  density.value = e;
};

// 字典类型表格列配置
const columns = ref([
  { title: "字典名称", dataIndex: "dict_name", checked: true },
  { title: "字典编码", dataIndex: "dict_type", checked: true },
  { title: "排序", dataIndex: "order", checked: true },
  { title: "状态", dataIndex: "status", slotName: "status", align: "center", checked: true },
  { title: "描述", dataIndex: "remark", checked: true },
  { title: "创建时间", dataIndex: "created_at", checked: true },
  { title: "操作", slotName: "optional", align: "center", checked: true }
]);
const columnsShow = ref<any[]>([]);
const deepColumns = () => {
  columnsShow.value = deepClone(columns.value);
};
deepColumns();

// 列显示隐藏
const onCheckbox = (checked: any, row: any, index: any) => {
  if (!checked) {
    columnsShow.value = columnsShow.value.filter((item: any) => item.dataIndex != row.dataIndex);
  } else {
    columnsShow.value.splice(index, 0, row);
  }
};

// 列拖拽排序
const popupVisibleChange = (visible: boolean) => {
  if (visible) {
    nextTick(() => {
      const el = document.getElementById("tableSetting") as HTMLElement;
      new Sortable(el, {
        onEnd(e: any) {
          const { oldIndex, newIndex } = e;
          exchangeArray(columns.value, oldIndex, newIndex);
          exchangeArray(columnsShow.value, oldIndex, newIndex);
        }
      });
    });
  }
};

const exchangeArray = (arr: Array<any>, oldIndex: number, newIndex: number) => {
  const temp = arr[newIndex];
  arr[newIndex] = arr[oldIndex];
  arr[oldIndex] = temp;
};

// 字典类型列表
const loading = ref(false);
const pagination = ref({
  pageSize: 10,
  current: 1,
  showPageSize: true,
  showTotal: true,
  total: 0
});
const pageChange = (page: number) => {
  pagination.value.current = page;
  getDictList();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  getDictList();
};
const selectedKeys = ref<string[]>([]);
const dictList = ref<any[]>([]);

const getDictList = async () => {
  loading.value = true;
  try {
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (form.value.dict_name) params.dict_name = form.value.dict_name;
    if (form.value.dict_type) params.dict_type = form.value.dict_type;
    if (form.value.status) params.status = form.value.status;

    const res = await getDictTypeListAPI(params);
    dictList.value = res.data.list || [];
    pagination.value.total = res.data.total || 0;
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

// 字典类型表单
const open = ref(false);
const title = ref("");
const formType = ref(0); // 0新增 1修改
const formRef = ref();
const rules = {
  dict_name: [{ required: true, message: "请输入字典名称" }],
  dict_type: [{ required: true, message: "请输入字典编码" }]
};
const addFrom = ref({
  dict_id: "",
  dict_name: "",
  dict_type: "",
  order: 0,
  remark: "",
  status: "1"
});

const onAdd = () => {
  formType.value = 0;
  title.value = "新增字典";
  open.value = true;
};

const onUpdate = (record: any) => {
  formType.value = 1;
  title.value = "修改字典";
  addFrom.value = deepClone({
    ...record,
    dict_id: String(record.dict_id),
    status: record.status || "1"
  });
  open.value = true;
};

const handleOk = async () => {
  const state = await formRef.value.validate();
  if (state) return;

  try {
    const apiData = {
      ...addFrom.value,
      order: Number(addFrom.value.order) || 0
    };

    if (formType.value === 1) {
      await editDictTypeAPI(apiData);
      arcoMessage("success", "修改成功");
    } else {
      await addDictTypeAPI(apiData);
      arcoMessage("success", "新增成功");
    }
    open.value = false;
    getDictList();
  } catch (e) {
    console.error(e);
  }
};

const afterClose = () => {
  formRef.value?.resetFields();
  addFrom.value = {
    dict_id: "",
    dict_name: "",
    dict_type: "",
    order: 0,
    remark: "",
    status: "1"
  };
};

const onDelete = async (record: any) => {
  try {
    await deleteDictTypeAPI({ dict_id: String(record.dict_id) });
    arcoMessage("success", "删除成功");
    getDictList();
  } catch (e) {
    console.error(e);
  }
};

const onBatchDelete = async () => {
  try {
    for (const dict_id of selectedKeys.value) {
      await deleteDictTypeAPI({ dict_id });
    }
    arcoMessage("success", "批量删除成功");
    selectedKeys.value = [];
    getDictList();
  } catch (e) {
    console.error(e);
  }
};

// 字典数据管理
const detailOpen = ref(false);
const detailLoading = ref(false);
const currentDictType = ref<any>(null);
const dictDetailList = ref<any[]>([]);
const detailPagination = ref({
  pageSize: 10,
  current: 1,
  showPageSize: true,
  showTotal: true,
  total: 0
});
const detailPageChange = (page: number) => {
  detailPagination.value.current = page;
  getDictDataList();
};
const detailPageSizeChange = (pageSize: number) => {
  detailPagination.value.pageSize = pageSize;
  getDictDataList();
};
const selectedKeysDetail = ref<string[]>([]);

// 字典数据密度
const densityDetail = ref("small");
const onDensityDetail = (e: string) => {
  densityDetail.value = e;
};

// 字典数据表格列配置
const detailColumns = ref([
  { title: "字典标签", dataIndex: "dict_label", checked: true },
  { title: "字典值", dataIndex: "dict_value", checked: true },
  { title: "排序", dataIndex: "dict_sort", align: "center", checked: true },
  { title: "状态", dataIndex: "status", slotName: "status", align: "center", checked: true },
  { title: "描述", dataIndex: "remark", checked: true },
  { title: "操作", slotName: "optional", align: "center", checked: true }
]);
const detailColumnsShow = ref<any[]>([]);
const deepDetailColumns = () => {
  detailColumnsShow.value = deepClone(detailColumns.value);
};
deepDetailColumns();

const onCheckboxDetail = (checked: any, row: any, index: any) => {
  if (!checked) {
    detailColumnsShow.value = detailColumnsShow.value.filter((item: any) => item.dataIndex != row.dataIndex);
  } else {
    detailColumnsShow.value.splice(index, 0, row);
  }
};

const popupVisibleChangeDetail = (visible: boolean) => {
  if (visible) {
    nextTick(() => {
      const el = document.getElementById("tableSettingDetail") as HTMLElement;
      new Sortable(el, {
        onEnd(e: any) {
          const { oldIndex, newIndex } = e;
          exchangeArray(detailColumns.value, oldIndex, newIndex);
          exchangeArray(detailColumnsShow.value, oldIndex, newIndex);
        }
      });
    });
  }
};

const onDictData = (record: any) => {
  currentDictType.value = record;
  detailOpen.value = true;
  detailPagination.value.current = 1;
  getDictDataList();
};

const getDictDataList = async () => {
  if (!currentDictType.value) return;
  detailLoading.value = true;
  try {
    const params = {
      dict_type_id: String(currentDictType.value.dict_id),
      page_num: detailPagination.value.current,
      page_size: detailPagination.value.pageSize
    };
    const res = await getDictDataListAPI(params);
    dictDetailList.value = res.data.list || [];
    detailPagination.value.total = res.data.total || 0;
  } catch (e) {
    console.error(e);
  } finally {
    detailLoading.value = false;
  }
};

const detailOk = () => {
  detailOpen.value = false;
  currentDictType.value = null;
  dictDetailList.value = [];
  selectedKeysDetail.value = [];
};

// 字典数据表单
const detailCaseOpen = ref(false);
const detailTitle = ref("");
const detailFormType = ref(0); // 0新增 1修改
const detailFormRef = ref();
const detailRules = {
  dict_label: [{ required: true, message: "请输入字典标签" }],
  dict_value: [{ required: true, message: "请输入字典值" }]
};
const detailForm = ref({
  dict_code: "",
  dict_type_id: "",
  dict_label: "",
  dict_value: "",
  dict_sort: 0,
  remark: "",
  status: "1"
});

const onAddDetail = () => {
  detailFormType.value = 0;
  detailTitle.value = "新增字典数据";
  detailForm.value = {
    dict_code: "",
    dict_type_id: String(currentDictType.value?.dict_id || ""),
    dict_label: "",
    dict_value: "",
    dict_sort: 0,
    remark: "",
    status: "1"
  };
  detailCaseOpen.value = true;
};

const onDetailUpdate = (record: any) => {
  detailFormType.value = 1;
  detailTitle.value = "修改字典数据";
  detailForm.value = deepClone({
    ...record,
    dict_code: String(record.dict_code),
    dict_type_id: String(record.dict_type_id),
    status: record.status || "1"
  });
  detailCaseOpen.value = true;
};

const handleOkDetail = async () => {
  const state = await detailFormRef.value.validate();
  if (state) return;

  try {
    const apiData = {
      ...detailForm.value,
      dict_sort: Number(detailForm.value.dict_sort) || 0
    };

    if (detailFormType.value === 1) {
      await editDictDataAPI(apiData);
      arcoMessage("success", "修改成功");
    } else {
      await addDictDataAPI(apiData);
      arcoMessage("success", "新增成功");
    }
    detailCaseOpen.value = false;
    getDictDataList();
  } catch (e) {
    console.error(e);
  }
};

const afterCloseDetail = () => {
  detailFormRef.value?.resetFields();
  detailForm.value = {
    dict_code: "",
    dict_type_id: "",
    dict_label: "",
    dict_value: "",
    dict_sort: 0,
    remark: "",
    status: "1"
  };
};

const onDetailDelete = async (record: any) => {
  try {
    await deleteDictDataAPI({ dict_code: String(record.dict_code) });
    arcoMessage("success", "删除成功");
    getDictDataList();
  } catch (e) {
    console.error(e);
  }
};

const onBatchDeleteDetail = async () => {
  try {
    for (const dict_code of selectedKeysDetail.value) {
      await deleteDictDataAPI({ dict_code });
    }
    arcoMessage("success", "批量删除成功");
    selectedKeysDetail.value = [];
    getDictDataList();
  } catch (e) {
    console.error(e);
  }
};

onMounted(() => {
  getDictList();
});
</script>

<style lang="scss" scoped>
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
.setting {
  display: flex;
  align-items: center;
  width: 200px;
  .setting-box-icon {
    margin-right: 4px;
    cursor: move;
  }
  .title {
    margin-left: 8px;
  }
}
</style>

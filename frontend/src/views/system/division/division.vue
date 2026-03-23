<template>
  <div class="snow-page">
    <div class="snow-inner">
      <!-- 搜索区域 -->
      <a-form ref="searchFormRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="部门名称">
              <a-input v-model="form.name" placeholder="请输入部门名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="部门状态">
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
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getDivision"><icon-refresh size="18" /></div>
            </a-tooltip>
            <a-dropdown @select="onDensity">
              <a-tooltip content="密度">
                <div class="action-icon"><icon-line-height size="18" /></div>
              </a-tooltip>
              <template #content>
                <a-doption v-for="item in densityType" :value="item.value" :key="item.value" :disabled="item.value === density">{{ item.label }}</a-doption>
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

      <!-- 部门表格 -->
      <a-table
        ref="tableRef"
        row-key="dept_id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 800 }"
        :columns="columnsShow"
        :data="tableData"
      >
        <template #dept_name="{ record }">
          {{ record.dept_name }}
        </template>
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status == '1'">启用</a-tag>
          <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onUpdate(record)">修改</a-button>
            <a-button type="text" size="mini" @click="addSubDivision(record.dept_id)">新增下级</a-button>
            <a-popconfirm type="warning" content="确定删除该部门吗?" @ok="onDelete(record)" v-if="record.dept_id != 100">
              <a-button type="text" status="danger" size="mini">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal :width="dialogWidth()" v-model:visible="open" @close="afterClose" @ok="handleOk" @cancel="afterClose">
      <template #title> {{ title }} </template>
      <div>
        <a-form ref="formRef" auto-label-width :layout="formLayout" :rules="rules" :model="addFrom">
          <a-form-item field="parent_id" label="上级部门" validate-trigger="blur">
            <a-tree-select
              v-model="addFrom.parent_id"
              :data="tableData"
              :field-names="{
                key: 'dept_id',
                title: 'dept_name',
                children: 'children'
              }"
              placeholder="选择上级部门"
            ></a-tree-select>
          </a-form-item>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="dept_name" label="部门名称" validate-trigger="blur">
                <a-input v-model="addFrom.dept_name" placeholder="请输入部门名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" label="部门状态" validate-trigger="blur">
                <a-switch type="round" :checked-value="'1'" :unchecked-value="'0'" v-model="addFrom.status">
                  <template #checked> 启用 </template>
                  <template #unchecked> 禁用 </template>
                </a-switch>
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="leader" label="负责人" validate-trigger="blur">
                <a-input v-model="addFrom.leader" placeholder="请输入负责人" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="phone" label="联系电话" validate-trigger="blur">
                <a-input v-model="addFrom.phone" placeholder="请输入联系电话" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="email" label="邮箱" validate-trigger="blur">
                <a-input v-model="addFrom.email" placeholder="请输入邮箱" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="order" label="显示排序" validate-trigger="blur">
                <a-input-number
                  v-model="addFrom.order"
                  placeholder="请输入排序值"
                  :min="1"
                  :max="999"
                  :step="1"
                  :precision="0"
                  allow-clear
                />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="remark" label="描述" validate-trigger="blur">
            <a-textarea v-model="addFrom.remark" placeholder="请输入描述" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { deepClone } from "@/utils";
import { getDivisionAPI, addDivisionAPI, editDivisionAPI, deleteDivisionAPI } from "@/api/modules/system/index";
import { useLayoutModel } from "@/hooks/useLayoutModel";
import Sortable from "sortablejs";

const { dialogWidth, formLayout } = useLayoutModel();

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

// 表格列配置
const columns = ref([
  { title: "部门名称", dataIndex: "dept_name", slotName: "dept_name", checked: true },
  { title: "排序", dataIndex: "order", align: "center", checked: true },
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

const onCheckbox = (checked: any, row: any, index: any) => {
  if (!checked) {
    columnsShow.value = columnsShow.value.filter((item: any) => item.dataIndex != row.dataIndex);
  } else {
    columnsShow.value.splice(index, 0, row);
  }
};

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

// 新增
const open = ref(false);
const rules = {
  dept_name: [{ required: true, message: "请输入部门名称" }]
};
const addFrom = ref<any>({
  dept_id: null,
  parent_id: null,
  dept_name: "",
  order: 0,
  leader: null,
  phone: "",
  email: "",
  status: '1',
  remark: ""
});
const formType = ref(0);
const title = ref("");
const formRef = ref();
const onAdd = () => {
  title.value = "添加部门";
  formType.value = 0;
  open.value = true;
};
const handleOk = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    const apiData: any = {
      ...addFrom.value,
      dept_id: addFrom.value.dept_id ? String(addFrom.value.dept_id) : undefined,
      parent_id: addFrom.value.parent_id ? String(addFrom.value.parent_id) : "0",
      order: Number(addFrom.value.order) || 0,
    };
    if (addFrom.value.leader) {
      apiData.leader = String(addFrom.value.leader);
    }
    if (formType.value === 1) {
      await editDivisionAPI(apiData);
      arcoMessage("success", "修改成功");
    } else {
      await addDivisionAPI(apiData);
      arcoMessage("success", "新增成功");
    }
    open.value = false;
    getDivision();
  } catch (e) {
    console.error(e);
  }
};
const afterClose = () => {
  formRef.value.resetFields();
  addFrom.value = {
    dept_id: null,
    parent_id: null,
    dept_name: "",
    order: 0,
    leader: null,
    phone: "",
    email: "",
    status: '1',
    remark: ""
  };
};
const onUpdate = (row: any) => {
  title.value = "修改部门";
  formType.value = 1;
  addFrom.value = deepClone(row);
  open.value = true;
};
const addSubDivision = (id: any) => {
  title.value = "新增部门";
  formType.value = 2;
  addFrom.value = {
    dept_id: null,
    parent_id: id,
    dept_name: "",
    order: 0,
    leader: null,
    phone: "",
    email: "",
    status: '1',
    remark: ""
  };
  open.value = true;
};
const onDelete = async (row: any) => {
  try {
    await deleteDivisionAPI({ dept_id: String(row.dept_id) });
    arcoMessage("success", "删除成功");
    getDivision();
  } catch (e) {
    console.error(e);
  }
};

const openState = ref(dictFilter("status"));
const form = ref({
  name: "",
  status: ""
});
const searchFormRef = ref();
const search = () => {
  getDivision();
};
const reset = () => {
  searchFormRef.value?.resetFields();
  getDivision();
};
const loading = ref(false);
const tableRef = ref();
const tableData = ref();
const getDivision = async () => {
  loading.value = true;
  const params: any = {};
  if (form.value.name) params.dept_name = form.value.name;
  if (form.value.status) params.status = form.value.status;
  let res = await getDivisionAPI(params);
  tableData.value = res.data.list || [];
  loading.value = false;
  setTimeout(() => {
    tableRef.value.expandAll();
  }, 0);
};

onMounted(() => {
  getDivision();
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

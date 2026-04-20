<template>
  <div class="snow-fill">
    <div class="snow-fill-inner container">
      <s-fold-page :width="280">
        <template #sider>
          <div class="left-box">
            <a-input placeholder="请输入部门名称">
              <template #prefix>
                <icon-search />
              </template>
            </a-input>
            <div class="tree-box">
              <a-tree ref="treeRef" :field-names="fieldNames" :data="treeData" show-line @select="onSelectTree" v-model:expanded-keys="expandedKeys"> </a-tree>
            </div>
          </div>
        </template>
        <template #content>
          <div class="right-box">
            <!-- 搜索区域 -->
            <a-form ref="searchFormRef" :model="form" auto-label-width>
              <a-row :gutter="16">
                <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
                  <a-form-item field="name" label="用户名称">
                    <a-input v-model="form.name" placeholder="请输入用户名称" allow-clear />
                  </a-form-item>
                </a-col>
                <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
                  <a-form-item field="phone" label="手机号码">
                    <a-input v-model="form.phone" placeholder="请输入手机号码" allow-clear />
                  </a-form-item>
                </a-col>
                <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
                  <a-form-item field="status" label="用户状态">
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
                  <a-button type="primary" status="danger" size="small" :disabled="selectedKeys.length === 0">
                    <template #icon><icon-delete /></template>
                    删除
                  </a-button>
                </a-space>
              </a-col>
              <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
                <a-space size="medium">
                  <a-tooltip content="刷新">
                    <div class="action-icon" @click="getAccount"><icon-refresh size="18" /></div>
                  </a-tooltip>
                  <a-dropdown @select="onDensity">
                    <a-tooltip content="密度">
                      <div class="action-icon"><icon-line-height size="18" /></div>
                    </a-tooltip>
                    <template #content>
                      <a-doption
                        v-for="item in densityType"
                        :value="item.value"
                        :key="item.value"
                        :disabled="item.value === density"
                        >{{ item.label }}</a-doption
                      >
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

            <!-- 账号表格 -->
            <a-table
              row-key="id"
              column-resizable
              :loading="loading"
              :size="density"
              :bordered="{ cell: true }"
              :scroll="{ x: '120%', y: '100%' }"
              :columns="columnsShow"
              :data="accountList"
              :row-selection="{ type: 'checkbox', showCheckedAll: true }"
              v-model:selectedKeys="selectedKeys"
              :pagination="pagination"
              @page-change="pageChange"
              @page-size-change="pageSizeChange"
            >
              <template #sex="{ record }">
                <a-tag bordered size="small" color="arcoblue" v-if="record.sex == 1">男</a-tag>
                <a-tag bordered size="small" color="red" v-else-if="record.sex == 0">女</a-tag>
                <a-tag bordered size="small" v-else>未知</a-tag>
              </template>
              <template #status="{ record }">
                <a-tag bordered size="small" color="arcoblue" v-if="record.status == '0'">启用</a-tag>
                <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
              </template>
              <template #optional="{ record }">
                <a-space>
                  <a-button type="text" size="mini" @click="onUpdate(record)">修改</a-button>
                  <a-popconfirm type="warning" content="确定删除该账号吗?">
                    <a-button type="text" status="danger" size="mini" :disabled="record.admin">删除</a-button>
                  </a-popconfirm>
                  <a-tooltip content="用户详情">
                    <a-button type="text" status="success" size="mini" @click="onDetail(record)">
                      <template #icon><icon-more /></template>
                    </a-button>
                  </a-tooltip>
                </a-space>
              </template>
            </a-table>
          </div>
        </template>
      </s-fold-page>
    </div>

    <a-modal :width="dialogWidth()" v-model:visible="open" @close="afterClose" @ok="handleOk" @cancel="afterClose">
      <template #title> {{ title }} </template>
      <div>
        <a-form ref="formRef" auto-label-width :layout="formLayout" :rules="rules" :model="addFrom">
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="user_name" label="用户名称" validate-trigger="blur">
                <a-input v-model="addFrom.user_name" placeholder="请输入用户名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="nick_name" label="昵称" validate-trigger="blur">
                <a-input v-model="addFrom.nick_name" placeholder="请输入昵称" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24">
            <a-col :span="12">
              <a-form-item field="phonenumber" label="手机号码" validate-trigger="blur">
                <a-input v-model="addFrom.phonenumber" placeholder="请输入手机号码" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="email" label="邮箱" validate-trigger="blur">
                <a-input v-model="addFrom.email" placeholder="请输入邮箱" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
          <a-row>
            <a-col :span="24">
              <a-form-item field="sex" label="性别" validate-trigger="blur">
                <a-radio-group v-model="addFrom.sex" :options="sexOption">
                  <template #label="{ data }">
                    <div>{{ data.name }}</div>
                  </template>
                </a-radio-group>
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="dept_id" label="所属部门" validate-trigger="blur">
            <a-tree-select
              v-model="addFrom.dept_id"
              :data="treeData"
              :field-names="{
                key: 'dept_id',
                title: 'dept_name',
                children: 'children'
              }"
              placeholder="请选择所属部门"
            ></a-tree-select>
          </a-form-item>
          <a-form-item field="role_ids" label="角色" validate-trigger="blur">
            <a-select v-model="addFrom.role_ids" multiple placeholder="请选择角色">
              <a-option
                v-for="item in roleList"
                :key="item.role_id"
                :value="item.role_id"
                :label="item.role_name"
                :disabled="item.admin"
              ></a-option>
            </a-select>
          </a-form-item>
          <a-form-item field="status" label="状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="1" :unchecked-value="0" v-model="addFrom.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
          <a-form-item field="remark" label="描述" validate-trigger="blur">
            <a-textarea v-model="addFrom.remark" placeholder="请输入描述" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { getDivisionAPI, getAccountAPI, getRoleAPI } from "@/api/modules/system/index";
import { deepClone } from "@/utils";
import { useLayoutModel } from "@/hooks/useLayoutModel";
import Sortable from "sortablejs";

const router = useRouter();
const { dialogWidth, formLayout } = useLayoutModel();
const openState = ref(dictFilter("status"));
const sexOption = ref(dictFilter("gender"));

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
  { title: "用户名称", dataIndex: "user_name", checked: true },
  { title: "昵称", dataIndex: "nick_name", checked: true },
  { title: "性别", dataIndex: "sex", slotName: "sex", align: "center", checked: true },
  { title: "部门", dataIndex: "dept_name", checked: true },
  { title: "手机号", dataIndex: "phonenumber", checked: true },
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

const form = ref({
  name: "",
  phone: "",
  status: null as string | null
});
const searchFormRef = ref();
const search = () => {
  pagination.value.current = 1;
  getAccount();
};
const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.value.current = 1;
  getAccount();
};

// 新增
const open = ref(false);
const rules = {
  userName: [{ required: true, message: "请输入用户名称" }],
  nickName: [{ required: true, message: "请输入昵称" }],
  sex: [{ required: true, message: "请选择性别" }],
  deptId: [{ required: true, message: "请选择所属部门" }],
  roles: [{ required: true, message: "请选择角色" }],
  status: [{ required: true, message: "请选择状态" }]
};
const addFrom = ref<any>({
  user_name: "",
  nick_name: "",
  phonenumber: "",
  email: "",
  sex: 2,
  dept_id: null,
  role_ids: [],
  status: "0",
  remark: ""
});
const formType = ref(0);
const title = ref("");
const formRef = ref();
const onAdd = () => {
  title.value = "新增账号";
  formType.value = 0;
  open.value = true;
};
const handleOk = async () => {
  let state = await formRef.value.validate();
  if (state) return (open.value = true);
  arcoMessage("success", "模拟提交成功");
  getAccount();
};

const afterClose = () => {
  formRef.value.resetFields();
  addFrom.value = {
    user_name: "",
    nick_name: "",
    phonenumber: "",
    email: "",
    sex: 2,
    dept_id: null,
    role_ids: [],
    status: "0",
    remark: ""
  };
};

const onUpdate = (row: any) => {
  title.value = "修改账号";
  formType.value = 1;
  addFrom.value = deepClone(row);
  open.value = true;
};

const onDetail = (row: any) => {
  router.push({
    path: "/system/userinfo",
    query: {
      id: row.id,
      userName: row.user_name
    }
  });
};

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
  getAccount();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  getAccount();
};

// 账户
const accountList = ref();
const currentDeptId = ref<number | null>(null);
const getAccount = async () => {
  loading.value = true;
  const params: any = {
    page_num: pagination.value.current,
    page_size: pagination.value.pageSize
  };
  if (currentDeptId.value) params.dept_id = currentDeptId.value;
  if (form.value.name) params.user_name = form.value.name;
  if (form.value.phone) params.phonenumber = form.value.phone;
  if (form.value.status) params.status = form.value.status;
  let res = await getAccountAPI(params);
  const list = res.data.list || res.data || [];
  list.forEach((item: any) => (item.disabled = !!item.admin));
  accountList.value = list;
  pagination.value.total = res.data.total || 0;
  loading.value = false;
};
const selectedKeys = ref<string[]>([]);

// 部门树
const fieldNames = ref({
  key: "dept_id",
  title: "dept_name",
  children: "children"
});
const treeData = ref();
const treeRef = ref();
const expandedKeys = ref<string[]>([]);
const getDivision = async () => {
  let res = await getDivisionAPI();
  const list = res.data.list || res.data || [];
  treeData.value = list;
  if (list.length > 0) {
    expandedKeys.value = list.map((item: any) => item.dept_id);
  }
};
const onSelectTree = (selectedKeys: any) => {
  currentDeptId.value = selectedKeys.length > 0 ? selectedKeys[0] : null;
  pagination.value.current = 1;
  getAccount();
};

// 角色列表
const roleList = ref<any>([]);
const getRole = async () => {
  let res = await getRoleAPI();
  roleList.value = res.data.list || res.data || [];
};

onMounted(() => {
  getDivision();
  getAccount();
  getRole();
});
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-direction: row;
  column-gap: $padding;
  .left-box {
    display: flex;
    flex-direction: column;
    width: 260px;
    height: 100%;
    .tree-box {
      flex: 1;
      margin-top: $padding;
      overflow: auto;
    }
  }
  .right-box {
    flex: 1;
  }
}
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

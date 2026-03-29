<template>
  <div class="snow-page">
    <div class="snow-inner">
      <!-- 搜索区域 -->
      <a-form ref="searchFormRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="角色名称">
              <a-input v-model="form.name" placeholder="请输入角色名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="code" label="角色标识">
              <a-input v-model="form.code" placeholder="请输入角色标识" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="status" label="角色状态">
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
              <div class="action-icon" @click="getRole"><icon-refresh size="18" /></div>
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

      <!-- 角色表格 -->
      <a-table
        row-key="id"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        :scroll="{ x: '100%', y: '100%', minWidth: 1000 }"
        :columns="columnsShow"
        :data="roleList"
        :row-selection="{ type: 'checkbox', showCheckedAll: true }"
        v-model:selectedKeys="selectedKeys"
        :pagination="pagination"
        @page-change="pageChange"
        @page-size-change="pageSizeChange"
      >
        <template #status="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.status == '1'">启用</a-tag>
          <a-tag bordered size="small" color="red" v-else>禁用</a-tag>
        </template>
        <template #menus="{ record }">
          <a-space wrap>
            <a-tag v-for="(menu, index) in getDisplayMenus(record.menus)" :key="index" bordered size="small" color="arcoblue">{{
              menu.title
            }}</a-tag>
            <a-tag
              v-if="getMoreMenusCount(record.menus) > 0"
              bordered
              size="small"
              color="gray"
              style="cursor: pointer"
              @click="showAllMenus(record)"
              >+{{ getMoreMenusCount(record.menus) }}</a-tag
            >
          </a-space>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" :disabled="record.admin" @click="onPrivileges(record)">分配权限</a-button>
            <a-button type="text" size="mini" :disabled="record.admin" @click="onUpdate(record)">修改</a-button>
            <a-popconfirm type="warning" content="确定删除该角色吗?" @ok="onDelete(record)">
              <a-button type="text" status="danger" size="mini" :disabled="record.admin">删除</a-button>
            </a-popconfirm>
          </a-space>
        </template>
      </a-table>
    </div>

    <a-modal :width="dialogWidth()" v-model:visible="open" @close="afterClose" @ok="handleOk" @cancel="afterClose">
      <template #title> {{ title }} </template>
      <div>
        <a-form ref="formRef" auto-label-width :layout="formLayout" :rules="rules" :model="addFrom">
          <a-form-item field="role_name" label="角色名称" validate-trigger="blur">
            <a-input v-model="addFrom.role_name" placeholder="请输入角色名称" allow-clear />
          </a-form-item>
          <a-form-item field="role_key" label="角色编码" validate-trigger="blur">
            <a-input v-model="addFrom.role_key" placeholder="请输入角色编码" allow-clear />
          </a-form-item>
          <a-form-item field="status" label="状态" validate-trigger="blur">
            <a-switch type="round" :checked-value="'1'" :unchecked-value="'0'" v-model="addFrom.status">
              <template #checked> 启用 </template>
              <template #unchecked> 禁用 </template>
            </a-switch>
          </a-form-item>
          <a-form-item field="order" label="排序" validate-trigger="blur">
            <a-input-number
              v-model="addFrom.order"
              :step="1"
              :precision="0"
              :min="1"
              :max="9999"
              :style="{ width: '150px' }"
              placeholder="请输入"
              mode="button"
              class="input-demo"
            />
          </a-form-item>
          <a-form-item field="remark" label="描述" validate-trigger="blur">
            <a-textarea v-model="addFrom.remark" placeholder="请输入描述" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>

    <a-drawer :visible="drawerOpen" :width="dialogWidth('500px', '100%')" @ok="drawerOk" @cancel="drawerCancel">
      <template #title> 分配权限 </template>
      <div>
        <a-card>
          <a-row :gutter="24" justify="center">
            <a-col :span="8">
              <span class="text-right-gap">展开全部</span>
              <a-switch type="round" v-model="treeSwitch.expandAll" @change="onExpandAll">
                <template #checked> 是 </template>
                <template #unchecked> 否 </template>
              </a-switch>
            </a-col>
            <a-col :span="8">
              <span class="text-right-gap">全选节点</span>
              <a-switch type="round" v-model="treeSwitch.selectAll" @change="onSelectAll">
                <template #checked> 是 </template>
                <template #unchecked> 否 </template>
              </a-switch>
            </a-col>
            <a-col :span="8">
              <a-tooltip
                content="权限树的父子节点独立，因为若节点关联，父节点会存在半选情况，半选节点的ID不会返回，会导致菜单无法渲染"
              >
                <span>父子关联 <icon-question-circle-fill /></span>
              </a-tooltip>
            </a-col>
          </a-row>
        </a-card>

        <a-tree
          ref="treeRef"
          :fieldNames="{
            key: 'id',
            title: 'i18n',
            children: 'children'
          }"
          :check-strictly="true"
          :checkable="true"
          :show-line="true"
          :unmount-on-close="true"
          v-model:checked-keys="permissionKeys"
          :data="permissionTree"
        />
      </div>
    </a-drawer>

    <!-- 查看全部权限弹窗 -->
    <a-modal v-model:visible="menuModalVisible" :width="600" :footer="false">
      <template #title> {{ menuModalTitle }} </template>
      <div>
        <a-tree
          :fieldNames="{
            key: 'id',
            title: 'i18n',
            children: 'children'
          }"
          :checkable="false"
          :show-line="true"
          :default-expand-all="true"
          :data="roleMenuTree"
        />
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import useGlobalProperties from "@/hooks/useGlobalProperties";
import {
  getRoleAPI,
  getMenuListAPI,
  getUserPermissionAPI,
  addRoleAPI,
  editRoleAPI,
  deleteRoleAPI,
  assignMenuAPI
} from "@/api/modules/system/index";
import { deepClone } from "@/utils";
import { useLayoutModel } from "@/hooks/useLayoutModel";
import Sortable from "sortablejs";

const proxy = useGlobalProperties();
const openState = ref(dictFilter("status"));
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
  { title: "角色名称", dataIndex: "role_name", checked: true },
  { title: "角色标识", dataIndex: "role_key", checked: true },
  { title: "排序", dataIndex: "order", align: "center", checked: true },
  { title: "状态", dataIndex: "status", slotName: "status", align: "center", checked: true },
  { title: "权限", dataIndex: "menus", slotName: "menus", checked: true },
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
  code: "",
  status: null as string | null
});
const searchFormRef = ref();
const search = () => {
  pagination.value.current = 1;
  getRole();
};
const reset = () => {
  searchFormRef.value?.resetFields();
  pagination.value.current = 1;
  getRole();
};

// 新增
const open = ref(false);
const rules = {
  role_name: [{ required: true, message: "请输入角色名称" }],
  role_key: [{ required: true, message: "请输入角色编码" }],
  status: [{ required: true, message: "请选择状态" }]
};
const addFrom = ref<any>({
  role_name: "",
  role_key: "",
  status: "1",
  order: 1,
  remark: ""
});

const title = ref("");
const formRef = ref();
const onAdd = () => {
  title.value = "新增角色";
  open.value = true;
};
const handleOk = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    if (addFrom.value.role_id) {
      await editRoleAPI(addFrom.value);
      arcoMessage("success", "修改成功");
    } else {
      await addRoleAPI(addFrom.value);
      arcoMessage("success", "新增成功");
    }
    open.value = false;
    getRole();
  } catch (e) {
    console.error(e);
  }
};

const afterClose = () => {
  formRef.value.resetFields();
  addFrom.value = {
    role_name: "",
    role_key: "",
    status: "1",
    order: 1,
    remark: ""
  };
};

const onUpdate = (row: any) => {
  title.value = "修改角色";
  addFrom.value = deepClone(row);
  open.value = true;
};

const onDelete = async (row: any) => {
  try {
    await deleteRoleAPI({ role_id: Number(row.role_id) });
    arcoMessage("success", "删除成功");
    getRole();
  } catch (e) {
    console.error(e);
  }
};

// 获取列表
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
  getRole();
};
const pageSizeChange = (pageSize: number) => {
  pagination.value.pageSize = pageSize;
  getRole();
};
const roleList = ref([]);
const getRole = async () => {
  try {
    loading.value = true;
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (form.value.name) params.role_name = form.value.name;
    if (form.value.code) params.role_key = form.value.code;
    if (form.value.status) params.status = form.value.status;
    let res = await getRoleAPI(params);
    const list = res.data.list || res.data || [];
    list.forEach((item: any) => item.admin && (item.disabled = true));
    roleList.value = list;
    pagination.value.total = res.data.total || 0;
  } finally {
    loading.value = false;
  }
};

// 获取显示的权限（最多3个）
const getDisplayMenus = (menus: any[]) => {
  if (!menus || menus.length === 0) return [];
  return menus.slice(0, 3);
};

// 获取超出显示的权限数量
const getMoreMenusCount = (menus: any[]) => {
  if (!menus || menus.length <= 3) return 0;
  return menus.length - 3;
};

// 查看全部权限弹窗
const menuModalVisible = ref(false);
const menuModalTitle = ref("");
const roleMenuTree = ref([]);

const showAllMenus = (record: any) => {
  menuModalTitle.value = `${record.role_name} - 权限列表`;
  const menuIds = new Set(record.menus?.map((m: any) => m.menu_id) || []);
  roleMenuTree.value = filterMenuTree(permissionTree.value, menuIds);
  menuModalVisible.value = true;
};

const filterMenuTree = (tree: any[], menuIds: Set<number>): any[] => {
  return tree
    .map((node: any) => {
      const newNode = { ...node };
      if (newNode.children && newNode.children.length > 0) {
        newNode.children = filterMenuTree(newNode.children, menuIds);
      }
      if (menuIds.has(newNode.id) || (newNode.children && newNode.children.length > 0)) {
        return newNode;
      }
      return null;
    })
    .filter((node): node is any => node !== null);
};

const selectedKeys = ref<string[]>([]);

// 获取权限树
const treeRef = ref();
const treeSwitch = ref({
  expandAll: true,
  selectAll: false
});

const onExpandAll = (state: boolean) => {
  treeRef.value.expandAll(state);
};

const onSelectAll = (state: boolean) => {
  treeRef.value.checkAll(state);
};

const treeSwitchReset = () => {
  treeSwitch.value = {
    expandAll: true,
    selectAll: false
  };
};

const permissionTree = ref([]);
const permissionKeys = ref([]);
const getMenuList = async () => {
  let { data } = await getMenuListAPI();
  translation(data);
  permissionTree.value = data;
};

// 分配权限
const drawerOpen = ref(false);
const currentRoleId = ref<number | null>(null);
const onPrivileges = async (row: any) => {
  currentRoleId.value = Number(row.role_id);
  let res = await getUserPermissionAPI({ role: row.role_key });
  permissionKeys.value = (res.data || []).map((id: any) => Number(id));
  drawerOpen.value = true;
  treeRef.value.expandAll(true);
};

const drawerOk = async () => {
  try {
    await assignMenuAPI({
      role_id: currentRoleId.value!,
      menu_ids: permissionKeys.value
    });
    arcoMessage("success", "分配成功");
  } catch (e) {
    console.error(e);
  }
  drawerOpen.value = false;
  treeSwitchReset();
  getRole();
};

const drawerCancel = () => {
  drawerOpen.value = false;
  treeSwitchReset();
};

const translation = (tree: any) => {
  tree.forEach((item: any) => {
    if (item.children) translation(item.children);
    if (item.meta.title) {
      item.i18n = proxy.$t(`menu.${item.meta.title}`);
    }
  });
};

getRole();
getMenuList();
</script>

<style lang="scss" scoped>
.text-right-gap {
  margin-right: $margin;
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

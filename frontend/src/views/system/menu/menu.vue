<template>
  <div class="snow-fill">
    <div class="snow-fill-inner">
      <!-- 搜索区域 -->
      <a-form ref="searchFormRef" :model="form" auto-label-width>
        <a-row :gutter="16">
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="name" label="菜单名称">
              <a-input v-model="form.name" placeholder="请输入菜单名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="hide" label="显示状态">
              <a-select v-model="form.hide" placeholder="请选择显示状态" allow-clear>
                <a-option v-for="item in openState" :key="item.value" :value="item.value">{{ item.name }}</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-form-item field="disable" label="启用状态">
              <a-select v-model="form.disable" placeholder="请选择启用状态" allow-clear>
                <a-option v-for="item in openState" :key="item.value" :value="item.value">{{ item.name }}</a-option>
              </a-select>
            </a-form-item>
          </a-col>
          <a-col :xs="24" :sm="24" :md="12" :lg="12" :xl="6" :xxl="6">
            <a-space class="search-btn">
              <a-button type="primary" size="small" @click="onSearch">
                <template #icon><icon-search /></template>
                <template #default>查询</template>
              </a-button>
              <a-button size="small" @click="onReset">
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
            <a-button type="primary" status="success" size="small" @click="onExpand">
              <template #icon><icon-swap /></template>
              {{ expand ? "收起" : "展开" }}
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space size="medium">
            <a-tooltip content="刷新">
              <div class="action-icon" @click="getMenuList"><icon-refresh size="18" /></div>
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

      <!-- 菜单表格 -->
      <a-table
        ref="tableRef"
        row-key="name"
        column-resizable
        :loading="loading"
        :size="density"
        :bordered="{ cell: true }"
        show-empty-tree
        :pagination="false"
        :scroll="{ x: '150%', y: '93%' }"
        :columns="columnsShow"
        :data="tableTree"
      >
        <template #title="{ record }">
          {{ $t(`menu.${record.meta.title}`) }}
        </template>
        <template #type="{ record }">
          <a-tag v-if="record.meta.type == 1" bordered size="small" color="purple">目录</a-tag>
          <a-tag v-else-if="record.meta.type == 2" bordered size="small" color="green">菜单</a-tag>
          <a-tag v-else bordered size="small" color="gray">按钮</a-tag>
        </template>
        <template #icon="{ record }">
          <MenuItemIcon :svg-icon="record.meta.svgIcon" :icon="record.meta.icon" />
        </template>
        <template #component="{ record }">
          {{ record.redirect ? record.redirect : record.component }}
        </template>
        <template #roles="{ record }">
          {{ record.meta.roles }}
        </template>
        <template #sort="{ record }">
          {{ record.meta.sort }}
        </template>
        <template #hide="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.meta.hide">是</a-tag>
          <a-tag bordered size="small" color="red" v-else>否</a-tag>
        </template>
        <template #disable="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.meta.disable">是</a-tag>
          <a-tag bordered size="small" color="red" v-else>否</a-tag>
        </template>
        <template #keepAlive="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.meta.keepAlive">是</a-tag>
          <a-tag bordered size="small" color="red" v-else>否</a-tag>
        </template>
        <template #link="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.meta.link">是</a-tag>
          <a-tag bordered size="small" color="red" v-else>否</a-tag>
        </template>
        <template #isFull="{ record }">
          <a-tag bordered size="small" color="arcoblue" v-if="record.meta.isFull">是</a-tag>
          <a-tag bordered size="small" color="red" v-else>否</a-tag>
        </template>
        <template #optional="{ record }">
          <a-space>
            <a-button type="text" size="mini" @click="onUpdate(record)">修改</a-button>
            <a-button type="text" size="mini" v-if="record.meta.type != 3" @click="onCurrentAdd(record)">新增</a-button>
            <a-popconfirm type="warning" content="确定删除该项吗?" @ok="onDelete(record)">
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
          <a-form-item field="type" label="菜单类型" validate-trigger="blur">
            <a-radio-group type="button" :disabled="!!addFrom.id" v-model="addFrom.type" @change="typeChange">
              <a-radio v-for="item in menuType" :key="item.value" :value="item.value">{{ item.name }}</a-radio>
            </a-radio-group>
          </a-form-item>
          <a-form-item field="parentId" label="上级菜单" validate-trigger="blur" :disabled="!!addFrom.id">
            <a-tree-select
              v-model="addFrom.parentId"
              :data="menuTree"
              :field-names="{
                key: 'id',
                title: 'i18n',
                children: 'children'
              }"
              allow-clear
              placeholder="请选择上级菜单"
            ></a-tree-select>
            <template #extra>
              <div>未选择则默认第一级</div>
            </template>
          </a-form-item>
          <a-row :gutter="24" v-if="[1, 2].includes(addFrom.type)">
            <a-col :span="12">
              <a-form-item field="svgIcon" label="自定义图标" validate-trigger="blur">
                <s-select-icon type="svg" v-model="addFrom.svgIcon" />
                <template #extra>
                  <div>自定义图标优先级高于菜单图标</div>
                </template>
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="icon" label="菜单图标" validate-trigger="blur">
                <s-select-icon type="arco" v-model="addFrom.icon" />
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="title" label="菜单标题" validate-trigger="blur">
            <a-input
              v-model="addFrom.title"
              placeholder="请输入菜单标题"
              allow-clear
              @input="(e: string) => onTrim(e, 'title')"
            />
            <template #extra>
              <div>
                优先匹配国际化Key
                <a-typography-text code v-if="addFrom.title"> menu.{{ addFrom.title }} </a-typography-text>
                （无对应Key则直接取标题展示）
              </div>
            </template>
          </a-form-item>
          <a-form-item v-if="[1, 2].includes(addFrom.type)" field="path" label="路由路径" validate-trigger="blur">
            <a-input v-model="addFrom.path" placeholder="请输入路由路径，如：/home" allow-clear @input="pathChange" />
            <template #extra>
              <div>
                菜单名称由路径自动生成
                <a-typography-text code v-if="addFrom.name"> {{ addFrom.name }} </a-typography-text>
              </div>
            </template>
          </a-form-item>
          <a-form-item v-if="addFrom.type == 3" field="permission" label="权限标识" validate-trigger="blur">
            <a-input
              v-model="addFrom.permission"
              placeholder="请输入权限标识，如：sys:btn:add"
              allow-clear
              @input="(e: string) => onTrim(e, 'permission')"
            />
          </a-form-item>

          <a-form-item v-if="[1, 2].includes(addFrom.type)" field="redirect" label="路由重定向" validate-trigger="blur">
            <a-input
              v-model="addFrom.redirect"
              placeholder="请输入路由重定向"
              allow-clear
              @input="(e: string) => onTrim(e, 'redirect')"
            />
          </a-form-item>
          <a-form-item
            v-if="addFrom.type == 2"
            field="component"
            label="组件路径"
            validate-trigger="blur"
            :disabled="addFrom.isLink"
          >
            <a-input
              v-model="addFrom.component"
              placeholder="请输入组件路径"
              allow-clear
              @input="(e: string) => onTrim(e, 'component')"
            >
              <template #prepend>@/views/</template>
              <template #append>.vue</template>
            </a-input>
          </a-form-item>
          <a-row :gutter="24">
            <a-col :span="8" v-if="[1, 2].includes(addFrom.type)">
              <a-form-item field="hide" label="显示状态" validate-trigger="blur">
                <a-switch type="round" v-model="addFrom.hide" :checked-value="false" :unchecked-value="true">
                  <template #checked> 显示 </template>
                  <template #unchecked> 隐藏 </template>
                </a-switch>
              </a-form-item>
            </a-col>
            <a-col :span="8" v-if="[1, 2].includes(addFrom.type)">
              <a-form-item field="disable" label="启用状态" validate-trigger="blur">
                <a-switch type="round" v-model="addFrom.disable" :checked-value="false" :unchecked-value="true">
                  <template #checked> 启用 </template>
                  <template #unchecked> 禁用 </template>
                </a-switch>
              </a-form-item>
            </a-col>
            <a-col :span="8" v-if="addFrom.type == 2">
              <a-form-item field="keepAlive" label="是否缓存" validate-trigger="blur">
                <a-switch type="round" v-model="addFrom.keepAlive">
                  <template #checked> 是 </template>
                  <template #unchecked> 否 </template>
                </a-switch>
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="24" v-if="addFrom.type == 2">
            <a-col :span="8">
              <a-form-item field="affix" label="固定Tabs" validate-trigger="blur">
                <a-switch type="round" v-model="addFrom.affix">
                  <template #checked> 是 </template>
                  <template #unchecked> 否 </template>
                </a-switch>
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="isLink" label="是否外链" validate-trigger="blur">
                <a-switch type="round" v-model="addFrom.isLink" @change="onIsLink">
                  <template #checked> 是 </template>
                  <template #unchecked> 否 </template>
                </a-switch>
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item field="iframe" label="内嵌窗口" validate-trigger="blur" :disabled="!addFrom.isLink">
                <a-switch type="round" v-model="addFrom.iframe" @change="onIframe">
                  <template #checked> 是 </template>
                  <template #unchecked> 否 </template>
                </a-switch>
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="link" label="外链路径" validate-trigger="blur" v-if="addFrom.type == 2 && addFrom.isLink">
            <a-input v-model="addFrom.link" placeholder="请输入路由路径" allow-clear />
          </a-form-item>
          <a-form-item field="affix" label="全屏显示" validate-trigger="blur" v-if="addFrom.type == 2">
            <a-switch type="round" v-model="addFrom.isFull">
              <template #checked> 是 </template>
              <template #unchecked> 否 </template>
            </a-switch>
          </a-form-item>
          <a-form-item field="sort" label="排序" validate-trigger="blur">
            <a-input-number
              v-model="addFrom.sort"
              :step="1"
              :precision="0"
              :min="1"
              :max="9999"
              :style="{ width: '120px' }"
              placeholder="请输入"
              mode="button"
              class="input-demo"
            />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import MenuItemIcon from "@/layout/components/Menu/menu-item-icon.vue";
import useGlobalProperties from "@/hooks/useGlobalProperties";
import { getMenuListAPI, addMenuAPI, editMenuAPI, deleteMenuAPI } from "@/api/modules/system/index";
import { deepClone, getPascalCase } from "@/utils";
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

const form = ref({
  name: "",
  hide: "",
  disable: ""
});
const searchFormRef = ref();

const onReset = () => {
  searchFormRef.value?.resetFields();
  getMenuList();
};

// 表格列配置
const columns = ref([
  { title: "菜单名称", dataIndex: "title", slotName: "title", checked: true },
  { title: "菜单类型", dataIndex: "type", slotName: "type", align: "center", checked: true },
  { title: "图标", dataIndex: "icon", slotName: "icon", align: "center", checked: true },
  { title: "路由路径", dataIndex: "path", checked: true },
  { title: "路由名称", dataIndex: "name", checked: true },
  { title: "组件路径", dataIndex: "component", slotName: "component", checked: true },
  { title: "权限标识", dataIndex: "roles", slotName: "roles", checked: true },
  { title: "排序", dataIndex: "sort", slotName: "sort", align: "center", checked: true },
  { title: "是否隐藏", dataIndex: "hide", slotName: "hide", align: "center", checked: true },
  { title: "是否禁用", dataIndex: "disable", slotName: "disable", align: "center", checked: true },
  { title: "是否缓存", dataIndex: "keepAlive", slotName: "keepAlive", align: "center", checked: true },
  { title: "是否外链", dataIndex: "link", slotName: "link", align: "center", checked: true },
  { title: "是否全屏", dataIndex: "isFull", slotName: "isFull", align: "center", checked: true },
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
const rules = ref({
  parentId: [{ required: false, message: "请选择上级菜单" }],
  name: [{ required: true, message: "请输入菜单名称" }],
  title: [{ required: true, message: "请输入菜单标题" }],
  path: [{ required: true, message: "请输入路由路径" }],
  permission: [{ required: true, message: "请输入权限标识" }]
});

const menuType = ref([
  { name: "目录", value: 1 },
  { name: "菜单", value: 2 },
  { name: "按钮", value: 3 }
]);
const addFrom = ref<any>({
  type: 1,
  parentId: "",
  svgIcon: "",
  icon: "",
  name: "",
  title: "",
  isFull: false,
  permission: "",
  path: "",
  redirect: "",
  component: "",
  hide: false,
  disable: false,
  keepAlive: true,
  affix: false,
  isLink: false,
  link: "",
  iframe: false,
  sort: 1
});
const formType = ref(0);
const title = ref("");
const formRef = ref();
const onAdd = () => {
  title.value = "新增菜单";
  formType.value = 0;
  open.value = true;
};
const handleOk = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    const typeMap: Record<number, string> = { 1: "M", 2: "C", 3: "F" };
    const form = addFrom.value;
    const apiData = {
      id: form.id ? Number(form.id) : undefined,
      pid: form.parentId ? Number(form.parentId) : 0,
      order: Number(form.sort) || 0,
      menu_type: typeMap[form.type] || "M",
      name: form.name,
      title: form.title || form.name,
      path: form.path,
      component: form.component,
      redirect: form.redirect,
      status: form.disable ? "0" : "1",
      remark: form.remark,
      icon: form.icon,
      hidden: form.hide ? "1" : "0",
      noCache: form.keepAlive ? "0" : "1",
      breadcrumb: "1",
      affix: form.affix ? "1" : "0",
      noTagsView: "0",
      activeMenu: "0",
      canTo: "0",
      alwaysShow: "0"
    };
    if (formType.value === 1) {
      await editMenuAPI(apiData);
      arcoMessage("success", "修改成功");
    } else {
      await addMenuAPI(apiData);
      arcoMessage("success", "新增成功");
    }
    open.value = false;
    getMenuList();
  } catch (e) {
    console.error(e);
  }
};

const afterClose = () => {
  formRef.value.resetFields();
  addFrom.value = {
    type: 1,
    parentId: "",
    svgIcon: "",
    icon: "",
    name: "",
    title: "",
    isFull: false,
    permission: "",
    path: "",
    redirect: "",
    component: "",
    hide: false,
    disable: false,
    keepAlive: true,
    affix: false,
    isLink: false,
    link: "",
    iframe: false,
    sort: 1
  };
};

const onUpdate = (row: Menu.MenuOptions) => {
  let data = deepClone(row);
  delete data.children;
  if (data.parentId == "0") data.parentId = "";
  let form = {
    ...data,
    ...data.meta
  };
  if (form.meta) delete form.meta;
  typeChange(form.type);
  addFrom.value = form;
  title.value = "修改菜单";
  formType.value = 1;
  open.value = true;
};

const onCurrentAdd = (record: any) => {
  let {
    id,
    meta: { type }
  } = record;
  addFrom.value.parentId = id;
  addFrom.value.type = type == 2 ? 3 : type;
  title.value = "新增菜单";
  formType.value = 0;
  open.value = true;
};

const onDelete = async (record: any) => {
  try {
    await deleteMenuAPI({ id: Number(record.id) });
    arcoMessage("success", "删除成功");
    getMenuList();
  } catch (e) {
    console.error(e);
  }
};

const typeChange = (val: number) => {
  rules.value.parentId[0].required = val == 3;
  formRef.value.clearValidate();
};

const onTrim = (val: string, key: string) => {
  addFrom.value[key] = val.trim();
};

const pathChange = (str: string) => {
  addFrom.value.path = str.trim();
  addFrom.value.name = getPascalCase(str.trim().replace(/[./:?=&"-]/g, "_"));
};

const onIsLink = (is: boolean) => {
  if (!is) {
    addFrom.value.iframe = false;
    addFrom.value.link = "";
    addFrom.value.component = "";
  } else {
    addFrom.value.component = "link/external/external";
  }
};

const onIframe = (is: boolean) => {
  if (!is) {
    addFrom.value.component = "link/external/external";
  } else {
    addFrom.value.component = "link/internal/internal";
  }
};

const onSearch = () => getMenuList();
const loading = ref(false);
const tableRef = ref();
const tableTree = ref([]);
const menuTree = ref<any>([]);
const getMenuList = async () => {
  try {
    loading.value = true;
    const params: any = {};
    if (form.value.name) params.title = form.value.name;
    if (form.value.hide) params.hidden = form.value.hide;
    if (form.value.disable) params.status = form.value.disable;
    let { data } = await getMenuListAPI(params);
    translation(data);
    tableTree.value = data;
    menuTree.value = filterTree(data);
  } finally {
    loading.value = false;
  }
};

const expand = ref(false);
const onExpand = () => {
  expand.value = !expand.value;
  tableRef.value.expandAll(expand.value);
};

const translation = (tree: any) => {
  tree.forEach((item: any) => {
    if (item.children) translation(item.children);
    if (item.meta.title) {
      item.i18n = proxy.$t(`menu.${item.meta.title}`);
    }
  });
};

const filterTree = (nodes: Menu.MenuOptions[]) => {
  return nodes
    .filter((node: any) => node.meta.type !== 3)
    .map((node: any) => {
      const newNode = { ...node };
      if (newNode.children) {
        const filteredChildren = filterTree(newNode.children);
        if (filteredChildren.length > 0) {
          newNode.children = filteredChildren;
        } else {
          delete newNode.children;
        }
      }
      return newNode;
    });
};

getMenuList();
</script>

<style lang="scss" scoped>
:deep(.arco-typography code) {
  font-size: 100%;
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

<template>
  <div class="action-config">
    <div class="config-header">
      <span>操作按钮配置</span>
      <a-button type="primary" size="small" @click="handleAddAction">
        <template #icon><icon-plus /></template>
        添加操作
      </a-button>
    </div>

    <div v-if="actions.length === 0" class="empty-actions">
      <a-empty description="暂无操作按钮" />
    </div>

    <div v-else class="actions-list">
      <div v-for="(action, index) in actions" :key="action.id" class="action-item">
        <div class="action-header">
          <a-input v-model="action.label" placeholder="按钮名称" style="width: 120px" />
          <a-select v-model="action.type" placeholder="按钮类型" style="width: 100px">
            <a-option value="primary">主要</a-option>
            <a-option value="secondary">次要</a-option>
            <a-option value="dashed">虚线</a-option>
            <a-option value="outline">线形</a-option>
            <a-option value="text">文字</a-option>
          </a-select>
          <a-select v-model="action.status" placeholder="状态" style="width: 100px" allow-clear>
            <a-option value="success">成功</a-option>
            <a-option value="warning">警告</a-option>
            <a-option value="danger">危险</a-option>
          </a-select>
          <a-button type="text" status="danger" size="small" @click="handleDelete(index)">
            <icon-delete />
          </a-button>
        </div>

        <div class="action-content">
          <a-form layout="vertical">
            <a-row :gutter="12">
              <a-col :span="12">
                <a-form-item label="图标">
                  <a-input v-model="action.icon" placeholder="图标名称" />
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item label="权限标识">
                  <a-input v-model="action.permission" placeholder="权限标识" />
                </a-form-item>
              </a-col>
            </a-row>

            <a-form-item label="操作类型">
              <a-radio-group v-model="action.actionType">
                <a-radio value="modal">弹窗</a-radio>
                <a-radio value="drawer">抽屉</a-radio>
                <a-radio value="link">跳转</a-radio>
                <a-radio value="api">接口</a-radio>
                <a-radio value="custom">自定义</a-radio>
              </a-radio-group>
            </a-form-item>

            <a-form-item v-if="action.actionType === 'modal' || action.actionType === 'drawer'" label="表单配置">
              <a-select v-model="action.formId" placeholder="选择表单">
                <a-option value="form1">表单1</a-option>
                <a-option value="form2">表单2</a-option>
              </a-select>
            </a-form-item>

            <a-form-item v-if="action.actionType === 'link'" label="跳转路径">
              <a-input v-model="action.linkPath" placeholder="/path/:id" />
            </a-form-item>

            <a-form-item v-if="action.actionType === 'api'" label="接口地址">
              <a-input v-model="action.apiUrl" placeholder="/api/xxx" />
            </a-form-item>

            <a-form-item v-if="action.actionType === 'custom'" label="处理函数">
              <a-textarea v-model="action.handler" placeholder="请输入处理函数代码" :auto-size="{ minRows: 3 }" />
            </a-form-item>

            <a-row :gutter="12">
              <a-col :span="8">
                <a-form-item label="显示条件">
                  <a-input v-model="action.condition" placeholder="显示条件" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item label="确认提示">
                  <a-input v-model="action.confirmText" placeholder="确认提示文案" />
                </a-form-item>
              </a-col>
              <a-col :span="8">
                <a-form-item label="排序">
                  <a-input-number v-model="action.sort" :min="0" />
                </a-form-item>
              </a-col>
            </a-row>
          </a-form>
        </div>
      </div>
    </div>

    <a-divider orientation="left">快捷操作模板</a-divider>

    <div class="action-templates">
      <a-space wrap>
        <a-button v-for="template in actionTemplates" :key="template.type" size="small" @click="handleApplyTemplate(template)">
          {{ template.label }}
        </a-button>
      </a-space>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";

interface TableAction {
  id: string;
  label: string;
  type: string;
  status?: string;
  icon?: string;
  permission?: string;
  actionType: string;
  formId?: string;
  linkPath?: string;
  apiUrl?: string;
  handler?: string;
  condition?: string;
  confirmText?: string;
  sort: number;
}

interface Props {
  modelValue: TableAction[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: TableAction[]];
}>();

const actions = ref<TableAction[]>([...props.modelValue]);

watch(
  () => props.modelValue,
  val => {
    actions.value = [...val];
  },
  { deep: true }
);

watch(
  actions,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleAddAction = () => {
  actions.value.push({
    id: `action_${Date.now()}`,
    label: "",
    type: "text",
    actionType: "modal",
    sort: actions.value.length
  });
};

const handleDelete = (index: number) => {
  actions.value.splice(index, 1);
  Message.success("删除成功");
};

const actionTemplates = [
  {
    type: "edit",
    label: "编辑",
    action: {
      label: "编辑",
      type: "text",
      icon: "icon-edit",
      actionType: "modal",
      permission: "edit"
    }
  },
  {
    type: "delete",
    label: "删除",
    action: {
      label: "删除",
      type: "text",
      status: "danger",
      icon: "icon-delete",
      actionType: "api",
      confirmText: "确定要删除吗？",
      permission: "delete"
    }
  },
  {
    type: "view",
    label: "查看",
    action: {
      label: "查看",
      type: "text",
      icon: "icon-eye",
      actionType: "drawer",
      permission: "view"
    }
  },
  {
    type: "enable",
    label: "启用/禁用",
    action: {
      label: "启用",
      type: "text",
      icon: "icon-check",
      actionType: "api",
      permission: "status"
    }
  }
];

const handleApplyTemplate = (template: { action: Partial<TableAction> }) => {
  actions.value.push({
    id: `action_${Date.now()}`,
    ...template.action,
    sort: actions.value.length
  } as TableAction);
};
</script>

<style lang="scss" scoped>
.action-config {
  .config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    font-weight: 500;
  }

  .empty-actions {
    padding: 20px 0;
  }

  .actions-list {
    .action-item {
      margin-bottom: 16px;
      padding: 16px;
      background: var(--color-fill-1);
      border-radius: 4px;

      .action-header {
        display: flex;
        gap: 12px;
        align-items: center;
        margin-bottom: 12px;
      }

      .action-content {
        :deep(.arco-form-item) {
          margin-bottom: 12px;
        }
      }
    }
  }
}
</style>

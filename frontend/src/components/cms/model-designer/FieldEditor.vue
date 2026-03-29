<template>
  <a-modal
    v-model:visible="modalVisible"
    :title="mode === 'add' ? '添加字段' : '编辑字段'"
    :width="800"
    :unmount-on-close="true"
    @cancel="handleCancel"
    @ok="handleOk"
  >
    <a-tabs v-model:active-key="activeTab">
      <a-tab-pane key="basic" title="基本信息">
        <a-form :model="formData" layout="vertical">
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="字段名称" field="name" :rules="[{ required: true, message: '请输入字段名称' }]">
                <a-input v-model="formData.name" placeholder="请输入字段名称（英文）" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="显示名称" field="label" :rules="[{ required: true, message: '请输入显示名称' }]">
                <a-input v-model="formData.label" placeholder="请输入显示名称" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="字段类型" field="type" :rules="[{ required: true, message: '请选择字段类型' }]">
                <FieldTypeSelect v-model="formData.type" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="数据库类型">
                <a-select v-model="formData.dbType" placeholder="自动推断">
                  <a-option value="string">VARCHAR</a-option>
                  <a-option value="text">TEXT</a-option>
                  <a-option value="int">INT</a-option>
                  <a-option value="bigint">BIGINT</a-option>
                  <a-option value="decimal">DECIMAL</a-option>
                  <a-option value="datetime">DATETIME</a-option>
                  <a-option value="json">JSON</a-option>
                </a-select>
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="默认值">
                <a-input v-model="formData.defaultValue" placeholder="请输入默认值" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="占位符">
                <a-input v-model="formData.placeholder" placeholder="请输入占位符提示" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="16">
            <a-col :span="8">
              <a-form-item label="是否必填">
                <a-switch v-model="formData.required" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="唯一字段">
                <a-switch v-model="formData.unique" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="可搜索">
                <a-switch v-model="formData.searchable" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-form-item label="字段描述">
            <a-textarea v-model="formData.description" placeholder="请输入字段描述" :auto-size="{ minRows: 2 }" />
          </a-form-item>
        </a-form>
      </a-tab-pane>

      <a-tab-pane key="form" title="表单配置">
        <a-form :model="formData.formConfig" layout="vertical">
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="表单控件">
                <a-select v-model="formData.formConfig.component" placeholder="请选择表单控件">
                  <a-option value="input">输入框</a-option>
                  <a-option value="textarea">文本域</a-option>
                  <a-option value="select">下拉选择</a-option>
                  <a-option value="radio">单选框</a-option>
                  <a-option value="checkbox">复选框</a-option>
                  <a-option value="date-picker">日期选择</a-option>
                  <a-option value="upload">上传</a-option>
                  <a-option value="editor">富文本</a-option>
                </a-select>
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="表单提示">
                <a-input v-model="formData.formConfig.tooltip" placeholder="请输入表单提示" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="是否显示">
                <a-switch v-model="formData.formConfig.visible" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="是否禁用">
                <a-switch v-model="formData.formConfig.disabled" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-form-item label="选项配置" v-if="['select', 'radio', 'checkbox'].includes(formData.formConfig.component)">
            <a-textarea
              v-model="formData.formConfig.optionsJson"
              placeholder="请输入选项配置（JSON格式）"
              :auto-size="{ minRows: 3 }"
            />
          </a-form-item>

          <a-form-item label="联动配置">
            <a-textarea
              v-model="formData.formConfig.linkageJson"
              placeholder="请输入联动配置（JSON格式）"
              :auto-size="{ minRows: 2 }"
            />
          </a-form-item>
        </a-form>
      </a-tab-pane>

      <a-tab-pane key="table" title="表格配置">
        <a-form :model="formData.tableConfig" layout="vertical">
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item label="列宽">
                <a-input-number v-model="formData.tableConfig.width" placeholder="请输入列宽" :min="50" />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item label="最小列宽">
                <a-input-number v-model="formData.tableConfig.minWidth" placeholder="请输入最小列宽" :min="50" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-row :gutter="16">
            <a-col :span="8">
              <a-form-item label="显示列">
                <a-switch v-model="formData.tableConfig.visible" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="可排序">
                <a-switch v-model="formData.tableConfig.sortable" />
              </a-form-item>
            </a-col>
            <a-col :span="8">
              <a-form-item label="固定列">
                <a-switch v-model="formData.tableConfig.fixed" />
              </a-form-item>
            </a-col>
          </a-row>

          <a-form-item label="渲染器">
            <a-select v-model="formData.tableConfig.renderer" placeholder="请选择渲染器">
              <a-option value="text">文本</a-option>
              <a-option value="image">图片</a-option>
              <a-option value="tag">标签</a-option>
              <a-option value="link">链接</a-option>
              <a-option value="button">按钮</a-option>
              <a-option value="date">日期</a-option>
            </a-select>
          </a-form-item>

          <a-form-item label="自定义渲染">
            <a-textarea
              v-model="formData.tableConfig.customRender"
              placeholder="请输入自定义渲染代码"
              :auto-size="{ minRows: 3 }"
            />
          </a-form-item>
        </a-form>
      </a-tab-pane>

      <a-tab-pane key="validation" title="验证规则">
        <ValidationRules v-model="formData.validationRules" />
      </a-tab-pane>
    </a-tabs>
  </a-modal>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";
import FieldTypeSelect from "./FieldTypeSelect.vue";
import ValidationRules from "./ValidationRules.vue";

interface ModelField {
  id: string;
  name: string;
  label: string;
  type: string;
  required: boolean;
  unique?: boolean;
  searchable?: boolean;
  defaultValue?: string;
  placeholder?: string;
  description?: string;
  dbType?: string;
  formConfig?: {
    component?: string;
    tooltip?: string;
    visible?: boolean;
    disabled?: boolean;
    optionsJson?: string;
    linkageJson?: string;
  };
  tableConfig?: {
    width?: number;
    minWidth?: number;
    visible?: boolean;
    sortable?: boolean;
    fixed?: boolean;
    renderer?: string;
    customRender?: string;
  };
  validationRules?: any[];
  sort: number;
}

interface Props {
  visible: boolean;
  field: ModelField | null;
  mode: "add" | "edit";
}

const props = withDefaults(defineProps<Props>(), {
  visible: false,
  field: null,
  mode: "add"
});

const emit = defineEmits<{
  "update:visible": [value: boolean];
  save: [field: ModelField];
}>();

const modalVisible = computed({
  get: () => props.visible,
  set: val => emit("update:visible", val)
});

const activeTab = ref("basic");

const defaultFormData = (): ModelField => ({
  id: "",
  name: "",
  label: "",
  type: "text",
  required: false,
  unique: false,
  searchable: false,
  formConfig: {
    component: "input",
    visible: true,
    disabled: false
  },
  tableConfig: {
    visible: true,
    sortable: false,
    fixed: false,
    renderer: "text"
  },
  validationRules: [],
  sort: 0
});

const formData = ref<ModelField>(defaultFormData());

watch(
  () => props.visible,
  val => {
    if (val) {
      if (props.field) {
        formData.value = { ...defaultFormData(), ...props.field };
      } else {
        formData.value = defaultFormData();
      }
      activeTab.value = "basic";
    }
  }
);

const handleCancel = () => {
  modalVisible.value = false;
};

const handleOk = () => {
  if (!formData.value.name) {
    Message.warning("请输入字段名称");
    return;
  }
  if (!formData.value.label) {
    Message.warning("请输入显示名称");
    return;
  }
  if (!formData.value.type) {
    Message.warning("请选择字段类型");
    return;
  }

  emit("save", formData.value);
  modalVisible.value = false;
};
</script>

<style lang="scss" scoped>
:deep(.arco-modal-body) {
  padding: 16px 20px;
}

:deep(.arco-form-item) {
  margin-bottom: 16px;
}
</style>

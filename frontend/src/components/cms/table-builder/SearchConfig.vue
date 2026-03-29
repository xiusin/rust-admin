<template>
  <div class="search-config">
    <div class="config-header">
      <span>搜索字段配置</span>
      <a-button type="primary" size="small" @click="handleAddField">
        <template #icon><icon-plus /></template>
        添加
      </a-button>
    </div>

    <div v-if="fields.length === 0" class="empty-fields">
      <a-empty description="暂无搜索字段" />
    </div>

    <div v-else class="fields-list">
      <div v-for="(field, index) in fields" :key="field.id" class="field-item">
        <a-row :gutter="12">
          <a-col :span="8">
            <a-form-item label="字段">
              <a-input v-model="field.field" placeholder="字段名" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="标签">
              <a-input v-model="field.label" placeholder="显示标签" />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item label="类型">
              <a-select v-model="field.type" placeholder="搜索类型">
                <a-option value="input">输入框</a-option>
                <a-option value="select">下拉选择</a-option>
                <a-option value="date">日期</a-option>
                <a-option value="dateRange">日期范围</a-option>
                <a-option value="number">数字</a-option>
                <a-option value="numberRange">数字范围</a-option>
              </a-select>
            </a-form-item>
          </a-col>
        </a-row>

        <a-row :gutter="12">
          <a-col :span="12">
            <a-form-item label="占位符">
              <a-input v-model="field.placeholder" placeholder="占位提示" />
            </a-form-item>
          </a-col>
          <a-col :span="12">
            <a-form-item label="操作符">
              <a-select v-model="field.operator" placeholder="查询操作符">
                <a-option value="=">等于</a-option>
                <a-option value="like">包含</a-option>
                <a-option value="startWith">开头</a-option>
                <a-option value="endWith">结尾</a-option>
                <a-option value=">">大于</a-option>
                <a-option value=">=">大于等于</a-option>
                <a-option value="<">小于</a-option>
                <a-option value="<=">小于等于</a-option>
                <a-option value="between">区间</a-option>
                <a-option value="in">包含于</a-option>
              </a-select>
            </a-form-item>
          </a-col>
        </a-row>

        <div class="field-actions">
          <a-button type="text" status="danger" size="small" @click="handleDelete(index)"> <icon-delete /> 删除 </a-button>
        </div>
      </div>
    </div>

    <a-divider orientation="left">搜索布局</a-divider>

    <a-form layout="vertical">
      <a-form-item label="每行字段数">
        <a-select v-model="layoutConfig.columns">
          <a-option :value="2">2列</a-option>
          <a-option :value="3">3列</a-option>
          <a-option :value="4">4列</a-option>
        </a-select>
      </a-form-item>

      <a-form-item label="默认展开">
        <a-switch v-model="layoutConfig.collapsed" />
      </a-form-item>

      <a-form-item label="折叠显示数量">
        <a-input-number v-model="layoutConfig.showCount" :min="1" :max="10" />
      </a-form-item>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { Message } from "@arco-design/web-vue";

interface SearchField {
  id: string;
  field: string;
  label: string;
  type: string;
  placeholder?: string;
  operator?: string;
}

interface LayoutConfig {
  columns: number;
  collapsed: boolean;
  showCount: number;
}

interface Props {
  modelValue: SearchField[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: SearchField[]];
}>();

const fields = ref<SearchField[]>([...props.modelValue]);
const layoutConfig = ref<LayoutConfig>({
  columns: 3,
  collapsed: false,
  showCount: 3
});

watch(
  () => props.modelValue,
  val => {
    fields.value = [...val];
  },
  { deep: true }
);

watch(
  fields,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleAddField = () => {
  fields.value.push({
    id: `search_${Date.now()}`,
    field: "",
    label: "",
    type: "input",
    operator: "="
  });
};

const handleDelete = (index: number) => {
  fields.value.splice(index, 1);
  Message.success("删除成功");
};
</script>

<style lang="scss" scoped>
.search-config {
  .config-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    font-weight: 500;
  }

  .empty-fields {
    padding: 20px 0;
  }

  .fields-list {
    .field-item {
      padding: 16px;
      margin-bottom: 12px;
      background: var(--color-fill-1);
      border-radius: 4px;

      .field-actions {
        display: flex;
        justify-content: flex-end;
        margin-top: 8px;
      }
    }
  }
}
</style>

<template>
  <div class="form-rule-config">
    <div class="rules-header">
      <span>表单规则</span>
      <a-button type="primary" size="small" @click="handleAddRule">
        <template #icon><icon-plus /></template>
        添加规则
      </a-button>
    </div>

    <div v-if="rules.length === 0" class="empty-rules">
      <a-empty description="暂无表单规则" />
    </div>

    <div v-else class="rules-list">
      <div v-for="(rule, index) in rules" :key="index" class="rule-item">
        <a-card size="small">
          <template #title>
            <a-select v-model="rule.type" placeholder="规则类型" style="width: 150px">
              <a-option value="validate">字段验证</a-option>
              <a-option value="linkage">字段联动</a-option>
              <a-option value="async">异步验证</a-option>
              <a-option value="custom">自定义规则</a-option>
            </a-select>
          </template>
          <template #extra>
            <a-button type="text" status="danger" size="small" @click="handleDeleteRule(index)">
              <icon-delete />
            </a-button>
          </template>

          <div class="rule-content">
            <a-row :gutter="12">
              <a-col :span="12">
                <a-form-item label="源字段">
                  <a-select v-model="rule.sourceField" placeholder="请选择源字段">
                    <a-option value="field1">字段1</a-option>
                    <a-option value="field2">字段2</a-option>
                  </a-select>
                </a-form-item>
              </a-col>
              <a-col :span="12">
                <a-form-item label="目标字段">
                  <a-select v-model="rule.targetField" placeholder="请选择目标字段">
                    <a-option value="field1">字段1</a-option>
                    <a-option value="field2">字段2</a-option>
                  </a-select>
                </a-form-item>
              </a-col>
            </a-row>

            <a-form-item label="条件表达式">
              <a-input v-model="rule.condition" placeholder="请输入条件表达式" />
            </a-form-item>

            <a-form-item label="触发时机">
              <a-checkbox-group v-model="rule.trigger">
                <a-checkbox value="change">值变化</a-checkbox>
                <a-checkbox value="blur">失焦</a-checkbox>
                <a-checkbox value="submit">提交</a-checkbox>
              </a-checkbox-group>
            </a-form-item>

            <a-form-item label="错误提示">
              <a-input v-model="rule.message" placeholder="请输入错误提示信息" />
            </a-form-item>
          </div>
        </a-card>
      </div>
    </div>

    <a-divider orientation="left">规则模板</a-divider>

    <div class="rule-templates">
      <a-card
        v-for="template in ruleTemplates"
        :key="template.type"
        class="template-card"
        hoverable
        @click="handleApplyTemplate(template)"
      >
        <template #title>
          <icon-check-circle v-if="template.type === 'validate'" />
          <icon-link v-else-if="template.type === 'linkage'" />
          <icon-sync v-else-if="template.type === 'async'" />
          <icon-code v-else />
          {{ template.name }}
        </template>
        <div class="template-desc">{{ template.description }}</div>
      </a-card>
    </div>
  </div>
</template>

<script setup lang="ts">
interface FormRule {
  type: string;
  sourceField?: string;
  targetField?: string;
  condition?: string;
  trigger?: string[];
  message?: string;
}

interface Props {
  modelValue: FormRule[];
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
});

const emit = defineEmits<{
  "update:modelValue": [value: FormRule[]];
}>();

const rules = ref<FormRule[]>([...props.modelValue]);

watch(
  () => props.modelValue,
  val => {
    rules.value = [...val];
  },
  { deep: true }
);

watch(
  rules,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleAddRule = () => {
  rules.value.push({
    type: "validate",
    trigger: ["change"]
  });
};

const handleDeleteRule = (index: number) => {
  rules.value.splice(index, 1);
};

const ruleTemplates = [
  {
    type: "validate",
    name: "必填验证",
    description: "字段值不能为空"
  },
  {
    type: "linkage",
    name: "字段联动",
    description: "一个字段值变化影响另一个字段"
  },
  {
    type: "async",
    name: "异步验证",
    description: "异步校验字段值"
  },
  {
    type: "custom",
    name: "自定义规则",
    description: "自定义验证逻辑"
  }
];

const handleApplyTemplate = (template: { type: string }) => {
  rules.value.push({
    type: template.type,
    trigger: ["change"]
  });
};
</script>

<style lang="scss" scoped>
.form-rule-config {
  .rules-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 16px;
    font-weight: 500;
  }

  .empty-rules {
    padding: 20px 0;
  }

  .rules-list {
    .rule-item {
      margin-bottom: 12px;

      .rule-content {
        :deep(.arco-form-item) {
          margin-bottom: 12px;
        }
      }
    }
  }

  .rule-templates {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 12px;

    .template-card {
      cursor: pointer;

      .template-desc {
        font-size: 12px;
        color: var(--color-text-3);
      }
    }
  }
}
</style>

<template>
  <div class="validation-rules">
    <div class="rules-header">
      <span>验证规则</span>
      <a-button type="primary" size="small" @click="handleAddRule">
        <template #icon><icon-plus /></template>
        添加规则
      </a-button>
    </div>
    
    <div v-if="rules.length === 0" class="empty-rules">
      <a-empty description="暂无验证规则" />
    </div>
    
    <div v-else class="rules-list">
      <div v-for="(rule, index) in rules" :key="index" class="rule-item">
        <a-select v-model="rule.type" placeholder="规则类型" style="width: 120px">
          <a-option value="required">必填</a-option>
          <a-option value="minLength">最小长度</a-option>
          <a-option value="maxLength">最大长度</a-option>
          <a-option value="min">最小值</a-option>
          <a-option value="max">最大值</a-option>
          <a-option value="pattern">正则表达式</a-option>
          <a-option value="email">邮箱格式</a-option>
          <a-option value="phone">手机号格式</a-option>
          <a-option value="url">URL格式</a-option>
          <a-option value="custom">自定义</a-option>
        </a-select>
        
        <a-input
          v-if="['minLength', 'maxLength', 'min', 'max', 'pattern', 'custom'].includes(rule.type)"
          v-model="rule.value"
          :placeholder="getPlaceholder(rule.type)"
          style="flex: 1"
        />
        
        <a-input v-model="rule.message" placeholder="错误提示信息" style="flex: 1" />
        
        <a-button type="text" status="danger" @click="handleDeleteRule(index)">
          <template #icon><icon-delete /></template>
        </a-button>
      </div>
    </div>
    
    <a-divider orientation="left">常用规则模板</a-divider>
    
    <div class="rule-templates">
      <a-tag
        v-for="template in ruleTemplates"
        :key="template.type"
        color="arcoblue"
        style="cursor: pointer"
        @click="handleApplyTemplate(template)"
      >
        {{ template.label }}
      </a-tag>
    </div>
  </div>
</template>

<script setup lang="ts">
interface ValidationRule {
  type: string
  value?: string | number
  message: string
}

interface Props {
  modelValue: ValidationRule[]
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => []
})

const emit = defineEmits<{
  'update:modelValue': [value: ValidationRule[]]
}>()

const rules = ref<ValidationRule[]>([...props.modelValue])

watch(
  () => props.modelValue,
  (val) => {
    rules.value = [...val]
  },
  { deep: true }
)

watch(
  rules,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)

const handleAddRule = () => {
  rules.value.push({
    type: 'required',
    message: ''
  })
}

const handleDeleteRule = (index: number) => {
  rules.value.splice(index, 1)
}

const getPlaceholder = (type: string) => {
  const placeholders: Record<string, string> = {
    minLength: '请输入最小长度',
    maxLength: '请输入最大长度',
    min: '请输入最小值',
    max: '请输入最大值',
    pattern: '请输入正则表达式',
    custom: '请输入验证函数'
  }
  return placeholders[type] || ''
}

const ruleTemplates = [
  { type: 'required', label: '必填', rules: [{ type: 'required', message: '此字段为必填项' }] },
  { type: 'email', label: '邮箱验证', rules: [{ type: 'email', message: '请输入有效的邮箱地址' }] },
  { type: 'phone', label: '手机号验证', rules: [{ type: 'phone', message: '请输入有效的手机号' }] },
  { type: 'url', label: 'URL验证', rules: [{ type: 'url', message: '请输入有效的URL' }] },
  {
    type: 'password',
    label: '密码强度',
    rules: [
      { type: 'minLength', value: 8, message: '密码长度不能少于8位' },
      { type: 'pattern', value: '^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)', message: '密码必须包含大小写字母和数字' }
    ]
  }
]

const handleApplyTemplate = (template: { type: string; rules: ValidationRule[] }) => {
  template.rules.forEach((rule) => {
    rules.value.push({ ...rule })
  })
}
</script>

<style lang="scss" scoped>
.validation-rules {
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
      display: flex;
      gap: 8px;
      align-items: center;
      margin-bottom: 12px;
    }
  }

  .rule-templates {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
}
</style>

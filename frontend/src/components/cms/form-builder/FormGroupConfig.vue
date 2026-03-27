<template>
  <div class="form-group-config">
    <a-form layout="vertical">
      <a-form-item label="占位提示">
        <a-input v-model="configData.placeholder" placeholder="请输入占位提示" />
      </a-form-item>
      
      <a-form-item label="默认值">
        <a-input v-model="configData.defaultValue" placeholder="请输入默认值" />
      </a-form-item>
      
      <a-form-item label="是否必填">
        <a-switch v-model="configData.required" />
      </a-form-item>
      
      <a-form-item label="是否禁用">
        <a-switch v-model="configData.disabled" />
      </a-form-item>
      
      <a-form-item label="是否只读">
        <a-switch v-model="configData.readonly" />
      </a-form-item>
      
      <a-divider orientation="left">显示条件</a-divider>
      
      <a-form-item label="条件显示">
        <a-switch v-model="configData.conditionEnabled" />
      </a-form-item>
      
      <template v-if="configData.conditionEnabled">
        <a-form-item label="依赖字段">
          <a-select v-model="configData.conditionField" placeholder="请选择依赖字段">
            <a-option value="status">状态</a-option>
            <a-option value="type">类型</a-option>
          </a-select>
        </a-form-item>
        
        <a-form-item label="条件值">
          <a-input v-model="configData.conditionValue" placeholder="请输入条件值" />
        </a-form-item>
      </template>
      
      <a-divider orientation="left">字段联动</a-divider>
      
      <a-form-item label="启用联动">
        <a-switch v-model="configData.linkageEnabled" />
      </a-form-item>
      
      <template v-if="configData.linkageEnabled">
        <a-form-item label="联动配置">
          <a-textarea
            v-model="configData.linkageConfig"
            placeholder="请输入联动配置（JSON格式）"
            :auto-size="{ minRows: 3 }"
          />
        </a-form-item>
      </template>
      
      <a-divider orientation="left">自定义样式</a-divider>
      
      <a-form-item label="自定义类名">
        <a-input v-model="configData.customClass" placeholder="请输入自定义类名" />
      </a-form-item>
      
      <a-form-item label="自定义样式">
        <a-textarea
          v-model="configData.customStyle"
          placeholder="请输入自定义样式（CSS）"
          :auto-size="{ minRows: 2 }"
        />
      </a-form-item>
    </a-form>
  </div>
</template>

<script setup lang="ts">
interface FieldConfig {
  placeholder?: string
  defaultValue?: string
  required?: boolean
  disabled?: boolean
  readonly?: boolean
  conditionEnabled?: boolean
  conditionField?: string
  conditionValue?: string
  linkageEnabled?: boolean
  linkageConfig?: string
  customClass?: string
  customStyle?: string
}

interface Props {
  modelValue: FieldConfig
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({})
})

const emit = defineEmits<{
  'update:modelValue': [value: FieldConfig]
}>()

const configData = ref<FieldConfig>({
  placeholder: props.modelValue.placeholder,
  defaultValue: props.modelValue.defaultValue,
  required: props.modelValue.required,
  disabled: props.modelValue.disabled,
  readonly: props.modelValue.readonly,
  conditionEnabled: props.modelValue.conditionEnabled,
  conditionField: props.modelValue.conditionField,
  conditionValue: props.modelValue.conditionValue,
  linkageEnabled: props.modelValue.linkageEnabled,
  linkageConfig: props.modelValue.linkageConfig,
  customClass: props.modelValue.customClass,
  customStyle: props.modelValue.customStyle
})

watch(
  () => props.modelValue,
  (val) => {
    configData.value = { ...val }
  },
  { deep: true }
)

watch(
  configData,
  (val) => {
    emit('update:modelValue', val)
  },
  { deep: true }
)
</script>

<style lang="scss" scoped>
.form-group-config {
  :deep(.arco-divider) {
    margin: 16px 0;
  }
}
</style>

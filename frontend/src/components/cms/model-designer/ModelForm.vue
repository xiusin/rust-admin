<template>
  <a-form :model="formData" layout="vertical" @submit="handleSubmit">
    <a-form-item label="模型名称" field="name" :rules="[{ required: true, message: '请输入模型名称' }]">
      <a-input v-model="formData.name" placeholder="请输入模型名称（英文）" />
    </a-form-item>

    <a-form-item label="显示名称" field="label" :rules="[{ required: true, message: '请输入显示名称' }]">
      <a-input v-model="formData.label" placeholder="请输入显示名称" />
    </a-form-item>

    <a-form-item label="模型描述" field="description">
      <a-textarea v-model="formData.description" placeholder="请输入模型描述" :auto-size="{ minRows: 2, maxRows: 4 }" />
    </a-form-item>

    <a-form-item label="数据表名" field="tableName">
      <a-input v-model="formData.tableName" placeholder="请输入数据表名">
        <template #prefix>
          <icon-table />
        </template>
      </a-input>
    </a-form-item>
  </a-form>
</template>

<script setup lang="ts">
interface ModelFormData {
  name: string;
  label: string;
  description?: string;
  tableName?: string;
}

interface Props {
  modelValue: ModelFormData;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  "update:modelValue": [value: ModelFormData];
}>();

const formData = ref<ModelFormData>({
  name: props.modelValue.name,
  label: props.modelValue.label,
  description: props.modelValue.description,
  tableName: props.modelValue.tableName
});

watch(
  () => props.modelValue,
  val => {
    formData.value = { ...val };
  },
  { deep: true }
);

watch(
  formData,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const handleSubmit = () => {
  emit("update:modelValue", formData.value);
};
</script>

<style lang="scss" scoped>
:deep(.arco-form-item) {
  margin-bottom: 16px;
}
</style>

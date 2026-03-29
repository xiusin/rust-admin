<template>
  <a-form :model="configData" layout="vertical">
    <a-form-item label="启用缓存">
      <a-switch v-model="configData.enableCache" />
      <template #extra>
        <span class="form-tip">开启后将对模型数据进行缓存</span>
      </template>
    </a-form-item>

    <a-form-item label="软删除">
      <a-switch v-model="configData.enableSoftDelete" />
      <template #extra>
        <span class="form-tip">开启后删除数据将标记为已删除而非物理删除</span>
      </template>
    </a-form-item>

    <a-form-item label="自动时间戳">
      <a-switch v-model="configData.enableTimestamp" />
      <template #extra>
        <span class="form-tip">开启后将自动维护created_at和updated_at字段</span>
      </template>
    </a-form-item>

    <a-form-item label="数据权限">
      <a-select v-model="configData.dataPermission" placeholder="请选择数据权限">
        <a-option value="all">全部数据</a-option>
        <a-option value="dept">本部门数据</a-option>
        <a-option value="self">仅本人数据</a-option>
        <a-option value="custom">自定义</a-option>
      </a-select>
    </a-form-item>

    <a-form-item label="状态字段">
      <a-select v-model="configData.statusField" placeholder="请选择状态字段" allow-clear>
        <a-option v-for="field in statusFields" :key="field.value" :value="field.value">
          {{ field.label }}
        </a-option>
      </a-select>
      <template #extra>
        <span class="form-tip">用于控制数据的启用/禁用状态</span>
      </template>
    </a-form-item>

    <a-form-item label="排序字段">
      <a-select v-model="configData.sortField" placeholder="请选择排序字段" allow-clear>
        <a-option v-for="field in sortFields" :key="field.value" :value="field.value">
          {{ field.label }}
        </a-option>
      </a-select>
    </a-form-item>

    <a-divider orientation="left">高级配置</a-divider>

    <a-form-item label="索引配置">
      <a-textarea
        v-model="configData.indexConfig"
        placeholder="请输入索引配置（JSON格式）"
        :auto-size="{ minRows: 2, maxRows: 4 }"
      />
    </a-form-item>

    <a-form-item label="钩子配置">
      <a-textarea
        v-model="configData.hookConfig"
        placeholder="请输入钩子配置（JSON格式）"
        :auto-size="{ minRows: 2, maxRows: 4 }"
      />
    </a-form-item>
  </a-form>
</template>

<script setup lang="ts">
interface ModelConfigData {
  enableCache: boolean;
  enableSoftDelete: boolean;
  enableTimestamp: boolean;
  dataPermission?: string;
  statusField?: string;
  sortField?: string;
  indexConfig?: string;
  hookConfig?: string;
}

interface Props {
  modelValue: ModelConfigData;
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => ({
    enableCache: false,
    enableSoftDelete: true,
    enableTimestamp: true
  })
});

const emit = defineEmits<{
  "update:modelValue": [value: ModelConfigData];
}>();

const configData = ref<ModelConfigData>({
  enableCache: props.modelValue.enableCache,
  enableSoftDelete: props.modelValue.enableSoftDelete,
  enableTimestamp: props.modelValue.enableTimestamp,
  dataPermission: props.modelValue.dataPermission,
  statusField: props.modelValue.statusField,
  sortField: props.modelValue.sortField,
  indexConfig: props.modelValue.indexConfig,
  hookConfig: props.modelValue.hookConfig
});

watch(
  () => props.modelValue,
  val => {
    configData.value = { ...val };
  },
  { deep: true }
);

watch(
  configData,
  val => {
    emit("update:modelValue", val);
  },
  { deep: true }
);

const statusFields = ref([
  { value: "status", label: "状态" },
  { value: "state", label: "状态(state)" }
]);

const sortFields = ref([
  { value: "sort", label: "排序" },
  { value: "order", label: "顺序" },
  { value: "created_at", label: "创建时间" }
]);
</script>

<style lang="scss" scoped>
.form-tip {
  font-size: 12px;
  color: var(--color-text-3);
}

:deep(.arco-divider) {
  margin: 16px 0;
}
</style>

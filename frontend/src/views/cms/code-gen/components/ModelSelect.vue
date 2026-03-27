<template>
  <a-select v-model="modelValue" placeholder="请选择模型" allow-clear>
    <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
  </a-select>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue';
import { modelApi, type CmsModel } from '@/api/modules/cms/model';

const modelValue = defineModel<number | null>({ default: null });

const emit = defineEmits<{
  change: [];
}>();

const models = ref<CmsModel[]>([]);

const loadModels = async () => {
  try {
    models.value = await modelApi.simpleList();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadModels();
});

watch(modelValue, () => {
  emit('change');
});
</script>

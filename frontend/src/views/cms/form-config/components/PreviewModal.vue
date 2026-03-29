<template>
  <div class="preview-modal">
    <a-form :model="formData" layout="vertical">
      <a-row :gutter="16">
        <a-col :span="12" v-for="field in fields" :key="field.id">
          <a-form-item :label="field.name" :required="field.required">
            <component
              :is="getFieldComponent(field.type)"
              v-model="formData[field.code]"
              :placeholder="field.placeholder"
              :disabled="field.disabled"
            />
          </a-form-item>
        </a-col>
      </a-row>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from "vue";
import type { FormConfigDetail, FormFieldConfig } from "@/api/modules/cms/form";

interface Props {
  config: FormConfigDetail;
}

const props = defineProps<Props>();

const fields = computed(() => props.config.config?.fields || []);

const formData = ref<Record<string, any>>({});

const getFieldComponent = (type: string) => {
  const components: Record<string, string> = {
    text: "a-input",
    textarea: "a-textarea",
    number: "a-input-number",
    select: "a-select",
    radio: "a-radio-group",
    checkbox: "a-checkbox-group",
    date: "a-date-picker",
    datetime: "a-date-picker",
    image: "a-upload",
    file: "a-upload",
    editor: "a-textarea",
    switch: "a-switch"
  };
  return components[type] || "a-input";
};

watch(
  () => props.config,
  () => {
    formData.value = {};
    fields.value.forEach(f => {
      formData.value[f.code] = f.defaultValue || "";
    });
  },
  { immediate: true }
);
</script>

<style scoped lang="scss">
.preview-modal {
  padding: 16px;
}
</style>

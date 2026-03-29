<template>
  <a-card class="config-card" :bordered="false" hoverable>
    <div class="card-header">
      <div class="card-title">
        <span class="name">{{ data.name }}</span>
        <a-tag v-if="data.isDefault" color="arcoblue" size="small">默认</a-tag>
      </div>
      <div class="card-status">
        <a-tag :color="data.status ? 'green' : 'red'" size="small">
          {{ data.status ? "启用" : "禁用" }}
        </a-tag>
      </div>
    </div>
    <div class="card-body">
      <div class="card-info">
        <div class="info-item">
          <span class="label">编码：</span>
          <span class="value">{{ data.code }}</span>
        </div>
        <div class="info-item">
          <span class="label">列数：</span>
          <span class="value">{{ data.columnCount || 0 }}</span>
        </div>
      </div>
    </div>
    <div class="card-footer">
      <a-space>
        <a-button type="text" size="mini" @click="emit('edit', data)">
          <template #icon><icon-edit /></template>
          编辑
        </a-button>
        <a-button type="text" size="mini" @click="emit('preview', data)">
          <template #icon><icon-eye /></template>
          预览
        </a-button>
        <a-dropdown trigger="click">
          <a-button type="text" size="mini">
            <template #icon><icon-more /></template>
          </a-button>
          <template #content>
            <a-doption v-if="!data.isDefault" @click="emit('set-default', data)"> <icon-star /> 设为默认 </a-doption>
            <a-doption @click="emit('copy', data)"> <icon-copy /> 复制 </a-doption>
            <a-doption @click="handleDelete"> <icon-delete /> 删除 </a-doption>
          </template>
        </a-dropdown>
      </a-space>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import type { TableConfigItem } from "@/api/modules/cms/table";

interface Props {
  data: TableConfigItem;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  edit: [record: TableConfigItem];
  preview: [record: TableConfigItem];
  delete: [record: TableConfigItem];
  copy: [record: TableConfigItem];
  "set-default": [record: TableConfigItem];
}>();

const handleDelete = () => {
  Modal.warning({
    title: "确认删除",
    content: `确定要删除表格配置"${props.data.name}"吗？`,
    hideCancel: false,
    onOk: () => {
      emit("delete", props.data);
    }
  });
};
</script>

<style scoped lang="scss">
.config-card {
  height: 100%;
  display: flex;
  flex-direction: column;

  &:hover {
    .card-footer {
      opacity: 1;
    }
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;

    .card-title {
      display: flex;
      align-items: center;
      gap: 8px;

      .name {
        font-size: 14px;
        font-weight: 500;
      }
    }
  }

  .card-body {
    flex: 1;

    .card-info {
      .info-item {
        display: flex;
        margin-bottom: 8px;
        font-size: 13px;

        .label {
          color: var(--color-text-3);
          width: 60px;
        }

        .value {
          color: var(--color-text-1);
        }
      }
    }
  }

  .card-footer {
    padding-top: 12px;
    border-top: 1px solid var(--color-border-2);
    opacity: 0;
    transition: opacity 0.3s;
  }
}
</style>

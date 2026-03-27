<template>
  <a-card class="model-card" :bordered="false" hoverable>
    <div class="card-header">
      <div class="card-icon">
        <component :is="data.icon || 'icon-apps'" />
      </div>
      <div class="card-status">
        <a-tag :color="data.isEnabled ? 'green' : 'red'" size="small">
          {{ data.isEnabled ? '启用' : '禁用' }}
        </a-tag>
      </div>
    </div>
    <div class="card-body">
      <div class="card-title">{{ data.name }}</div>
      <div class="card-code">{{ data.code }}</div>
      <div class="card-desc">{{ data.description || '暂无描述' }}</div>
    </div>
    <div class="card-footer">
      <div class="card-stats">
        <div class="stat-item">
          <span class="stat-value">{{ data.fieldCount || 0 }}</span>
          <span class="stat-label">字段</span>
        </div>
        <div class="stat-item">
          <span class="stat-value">{{ data.contentCount || 0 }}</span>
          <span class="stat-label">内容</span>
        </div>
      </div>
    </div>
    <div class="card-actions">
      <a-button type="text" size="mini" @click="emit('design', data)">
        <template #icon><icon-settings /></template>
        设计
      </a-button>
      <a-button type="text" size="mini" @click="emit('content', data)">
        <template #icon><icon-file /></template>
        内容
      </a-button>
      <a-dropdown trigger="click">
        <a-button type="text" size="mini">
          <template #icon><icon-more /></template>
        </a-button>
        <template #content>
          <a-doption @click="emit('edit', data)">
            <icon-edit /> 编辑
          </a-doption>
          <a-doption @click="emit('copy', data)">
            <icon-copy /> 复制
          </a-doption>
          <a-doption @click="emit('toggle-status', data)">
            <icon-swap /> {{ data.isEnabled ? '禁用' : '启用' }}
          </a-doption>
          <a-doption @click="handleDelete">
            <icon-delete /> 删除
          </a-doption>
        </template>
      </a-dropdown>
    </div>
  </a-card>
</template>

<script setup lang="ts">
import type { CmsModelList } from '@/api/modules/cms/model';

interface Props {
  data: CmsModelList;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  edit: [record: CmsModelList];
  delete: [record: CmsModelList];
  design: [record: CmsModelList];
  content: [record: CmsModelList];
  copy: [record: CmsModelList];
  'toggle-status': [record: CmsModelList];
}>();

const handleDelete = () => {
  Modal.warning({
    title: '确认删除',
    content: `确定要删除模型"${props.data.name}"吗？`,
    hideCancel: false,
    onOk: () => {
      emit('delete', props.data);
    },
  });
};
</script>

<style scoped lang="scss">
.model-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: all 0.3s;

  &:hover {
    .card-actions {
      opacity: 1;
    }
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;

    .card-icon {
      width: 40px;
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: center;
      background: linear-gradient(135deg, rgb(var(--primary-6)), rgb(var(--primary-5)));
      border-radius: 8px;
      color: #fff;
      font-size: 20px;
    }
  }

  .card-body {
    flex: 1;

    .card-title {
      font-size: 16px;
      font-weight: 500;
      color: var(--color-text-1);
      margin-bottom: 4px;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }

    .card-code {
      font-size: 12px;
      color: var(--color-text-3);
      margin-bottom: 8px;
    }

    .card-desc {
      font-size: 13px;
      color: var(--color-text-2);
      overflow: hidden;
      text-overflow: ellipsis;
      display: -webkit-box;
      -webkit-line-clamp: 2;
      -webkit-box-orient: vertical;
      min-height: 36px;
    }
  }

  .card-footer {
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--color-border-2);

    .card-stats {
      display: flex;
      gap: 24px;

      .stat-item {
        display: flex;
        flex-direction: column;
        align-items: center;

        .stat-value {
          font-size: 18px;
          font-weight: 500;
          color: var(--color-text-1);
        }

        .stat-label {
          font-size: 12px;
          color: var(--color-text-3);
        }
      }
    }
  }

  .card-actions {
    display: flex;
    justify-content: space-between;
    margin-top: 12px;
    padding-top: 12px;
    border-top: 1px solid var(--color-border-2);
    opacity: 0;
    transition: opacity 0.3s;
  }
}
</style>

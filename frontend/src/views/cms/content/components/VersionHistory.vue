<template>
  <a-dropdown trigger="click" @select="onSelect">
    <a-button>
      <template #icon><icon-history /></template>
      版本历史
    </a-button>
    <template #content>
      <a-doption v-for="version in versions" :key="version.id" :value="version.id">
        <div class="version-item">
          <div class="version-info">
            <span class="version-number">v{{ version.version }}</span>
            <span class="version-title">{{ version.title }}</span>
          </div>
          <div class="version-meta">
            <span class="version-operator">{{ version.operatorName }}</span>
            <span class="version-time">{{ version.createdAt }}</span>
          </div>
        </div>
      </a-doption>
      <a-doption v-if="versions.length === 0" disabled> 暂无历史版本 </a-doption>
    </template>
  </a-dropdown>

  <a-modal v-model:visible="previewVisible" :width="800" :footer="false" :title="`版本 v${currentVersion?.version}`">
    <div class="version-preview">
      <a-descriptions :column="2" bordered>
        <a-descriptions-item label="标题" :span="2">{{ currentVersion?.title }}</a-descriptions-item>
        <a-descriptions-item label="操作者">{{ currentVersion?.operatorName }}</a-descriptions-item>
        <a-descriptions-item label="操作时间">{{ currentVersion?.createdAt }}</a-descriptions-item>
        <a-descriptions-item label="备注" :span="2">{{ currentVersion?.remark || "-" }}</a-descriptions-item>
      </a-descriptions>
      <a-divider>内容预览</a-divider>
      <div class="preview-content" v-html="currentVersion?.content || '暂无内容'"></div>
      <div class="preview-actions">
        <a-button type="primary" @click="handleRollback"> 回滚到此版本 </a-button>
      </div>
    </div>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { contentApi, type ContentVersion } from "@/api/modules/cms/content";

interface Props {
  contentId: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  rollback: [versionId: number];
}>();

const versions = ref<ContentVersion[]>([]);
const previewVisible = ref(false);
const currentVersion = ref<ContentVersion | null>(null);

const loadVersions = async () => {
  if (!props.contentId) return;
  try {
    versions.value = await contentApi.versions(props.contentId);
  } catch (error) {
    console.error(error);
  }
};

const onSelect = (versionId: string | number | Record<string, any> | undefined) => {
  const version = versions.value.find(v => v.id === versionId);
  if (version) {
    currentVersion.value = version;
    previewVisible.value = true;
  }
};

const handleRollback = () => {
  if (currentVersion.value) {
    Modal.warning({
      title: "确认回滚",
      content: `确定要回滚到版本 v${currentVersion.value.version} 吗？`,
      hideCancel: false,
      onOk: () => {
        emit("rollback", currentVersion.value!.id);
        previewVisible.value = false;
      }
    });
  }
};

onMounted(() => {
  loadVersions();
});
</script>

<style scoped lang="scss">
.version-item {
  display: flex;
  flex-direction: column;
  gap: 4px;

  .version-info {
    display: flex;
    align-items: center;
    gap: 8px;

    .version-number {
      font-weight: 500;
      color: rgb(var(--primary-6));
    }

    .version-title {
      color: var(--color-text-1);
    }
  }

  .version-meta {
    display: flex;
    gap: 12px;
    font-size: 12px;
    color: var(--color-text-3);
  }
}

.version-preview {
  .preview-content {
    min-height: 200px;
    padding: 16px;
    background: var(--color-fill-1);
    border-radius: 4px;
    line-height: 1.8;
  }

  .preview-actions {
    margin-top: 16px;
    text-align: center;
  }
}
</style>

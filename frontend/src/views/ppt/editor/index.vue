<template>
  <div class="ppt-editor-page">
    <div class="editor-header">
      <div class="header-left">
        <a-button type="text" @click="handleBack">
          <template #icon><icon-left /></template>
          返回
        </a-button>
        <a-divider direction="vertical" />
        <a-input v-model="projectTitle" :style="{ width: '300px' }" placeholder="请输入PPT标题" />
      </div>
      <div class="header-right">
        <a-space>
          <a-button type="outline" @click="handlePreview">
            <template #icon><icon-play-arrow /></template>
            预览
          </a-button>
          <a-button type="primary" :loading="saving" @click="handleSave">
            <template #icon><icon-save /></template>
            保存
          </a-button>
          <a-button type="primary" status="success" @click="handleExport">
            <template #icon><icon-download /></template>
            导出
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="editor-content">
      <CanvasEditor />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useRouter, useRoute } from "vue-router";
import { Message } from "@arco-design/web-vue";
import { projectApi } from "@/api/modules/ppt";
import { usePPTStore } from "@/store/modules/ppt";
import { CanvasEditor } from "@/components/ppt/editor";

const router = useRouter();
const route = useRoute();
const pptStore = usePPTStore();

const projectId = computed(() => route.params.id as string);
const projectTitle = ref("");
const saving = ref(false);
const loading = ref(false);

const loadProject = async () => {
  if (!projectId.value) return;

  loading.value = true;
  try {
    const project = await projectApi.detail(Number(projectId.value));
    projectTitle.value = project.title;
    pptStore.setCurrentProject(project);
  } catch (error: any) {
    Message.error(error?.message || "加载项目失败");
  } finally {
    loading.value = false;
  }
};

const handleBack = () => {
  router.push("/ppt/history");
};

const handlePreview = () => {
  Message.info("预览功能开发中...");
};

const handleSave = async () => {
  saving.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 1000));
    Message.success("保存成功");
  } catch (error: any) {
    Message.error(error?.message || "保存失败");
  } finally {
    saving.value = false;
  }
};

const handleExport = () => {
  Message.info("导出功能开发中...");
};

onMounted(() => {
  loadProject();
});
</script>

<style scoped lang="scss">
.ppt-editor-page {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--color-bg-2);
}

.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 16px;
  background: var(--color-bg-1);
  border-bottom: 1px solid var(--color-border);
  flex-shrink: 0;

  .header-left,
  .header-right {
    display: flex;
    align-items: center;
  }
}

.editor-content {
  flex: 1;
  overflow: hidden;
}
</style>

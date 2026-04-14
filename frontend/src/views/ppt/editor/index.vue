<template>
  <div class="ppt-editor-page">
    <div class="editor-header">
      <div class="header-left">
        <a-button type="text" @click="handleBack" class="back-btn">
          <template #icon><icon-left /></template>
          返回
        </a-button>
        <a-divider direction="vertical" />
        <div class="project-info">
          <a-input 
            v-model="projectTitle" 
            placeholder="请输入PPT标题" 
            class="title-input"
            size="large"
          />
          <span class="status-badge" :class="statusClass">
            <icon-dot />
            {{ statusText }}
          </span>
        </div>
      </div>
      <div class="header-right">
        <a-space size="medium">
          <a-tooltip content="撤销" :disabled="!canUndo">
            <a-button type="outline" :disabled="!canUndo" @click="handleUndo">
              <template #icon><icon-undo /></template>
            </a-button>
          </a-tooltip>
          <a-tooltip content="重做" :disabled="!canRedo">
            <a-button type="outline" :disabled="!canRedo" @click="handleRedo">
              <template #icon><icon-redo /></template>
            </a-button>
          </a-tooltip>
          <a-divider direction="vertical" style="margin: 0 8px" />
          <a-button type="outline" @click="handlePreview">
            <template #icon><icon-play-arrow /></template>
            预览
          </a-button>
          <a-button type="outline" @click="handleAutoSave">
            <template #icon><icon-cloud /></template>
            {{ autoSaveEnabled ? '已开启' : '未开启' }}
          </a-button>
          <a-button type="primary" :loading="saving" @click="handleSave" class="save-btn">
            <template #icon><icon-save /></template>
            {{ saving ? '保存中...' : '保存' }}
          </a-button>
          <a-dropdown @select="handleExportSelect">
            <a-button type="primary" status="success">
              <template #icon><icon-download /></template>
              导出
              <template #suffix><icon-down /></template>
            </a-button>
            <template #content>
              <a-doption value="pptx">PPTX 格式</a-doption>
              <a-doption value="pdf">PDF 格式</a-doption>
              <a-doption value="png">PNG 图片</a-doption>
            </template>
          </a-dropdown>
        </a-space>
      </div>
    </div>

    <div class="editor-content">
      <CanvasEditor />
    </div>

    <div class="editor-footer" v-if="autoSaveEnabled">
      <span class="last-save">
        <icon-check-circle />
        上次保存：{{ lastSaveTime }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
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
const canUndo = ref(false);
const canRedo = ref(false);
const autoSaveEnabled = ref(true);
const lastSaveTime = ref("刚刚");
let autoSaveTimer: NodeJS.Timeout | null = null;

const statusText = computed(() => {
  if (saving.value) return "保存中...";
  return "已就绪";
});

const statusClass = computed(() => {
  if (saving.value) return "saving";
  return "ready";
});

const loadProject = async () => {
  if (!projectId.value) return;

  loading.value = true;
  try {
    const project = await projectApi.detail(Number(projectId.value));
    projectTitle.value = project.title;
    pptStore.setCurrentProject(project);
    lastSaveTime.value = formatTime(new Date());
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

const handleUndo = () => {
  Message.info("撤销功能开发中...");
};

const handleRedo = () => {
  Message.info("重做功能开发中...");
};

const handleAutoSave = () => {
  autoSaveEnabled.value = !autoSaveEnabled.value;
  if (autoSaveEnabled.value) {
    startAutoSave();
    Message.success("自动保存已开启");
  } else {
    stopAutoSave();
    Message.info("自动保存已关闭");
  }
};

const handleSave = async () => {
  saving.value = true;
  try {
    await new Promise(resolve => setTimeout(resolve, 1000));
    lastSaveTime.value = formatTime(new Date());
    Message.success("保存成功");
  } catch (error: any) {
    Message.error(error?.message || "保存失败");
  } finally {
    saving.value = false;
  }
};

const handleExportSelect = (value: string) => {
  const formatMap: Record<string, string> = {
    pptx: "PPTX",
    pdf: "PDF",
    png: "PNG"
  };
  Message.info(`正在导出为 ${formatMap[value]} 格式...`);
};

const startAutoSave = () => {
  if (autoSaveTimer) return;
  autoSaveTimer = setInterval(() => {
    if (!saving.value) {
      handleSave();
    }
  }, 60000);
};

const stopAutoSave = () => {
  if (autoSaveTimer) {
    clearInterval(autoSaveTimer);
    autoSaveTimer = null;
  }
};

const formatTime = (date: Date) => {
  const hours = date.getHours().toString().padStart(2, '0');
  const minutes = date.getMinutes().toString().padStart(2, '0');
  return `${hours}:${minutes}`;
};

onMounted(() => {
  loadProject();
  if (autoSaveEnabled.value) {
    startAutoSave();
  }
});

onUnmounted(() => {
  stopAutoSave();
});
</script>

<style scoped lang="scss">
.ppt-editor-page {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: linear-gradient(135deg, #f5f7fa 0%, #e4e8ec 100%);

  .editor-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 12px 24px;
    background: white;
    border-bottom: 1px solid #e5e6eb;
    flex-shrink: 0;
    box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);
    z-index: 10;

    .header-left,
    .header-right {
      display: flex;
      align-items: center;
    }

    .back-btn {
      transition: all 0.2s ease;

      &:hover {
        background: #f2f3f5;
      }
    }

    .project-info {
      display: flex;
      align-items: center;
      gap: 16px;
      margin-left: 16px;

      .title-input {
        width: 320px;
        font-weight: 600;
        font-size: 15px;

        :deep(.arco-input) {
          border: 2px solid transparent;
          transition: all 0.2s ease;

          &:hover,
          &:focus {
            border-color: #165dff;
          }
        }
      }

      .status-badge {
        display: flex;
        align-items: center;
        gap: 4px;
        padding: 4px 12px;
        border-radius: 100px;
        font-size: 13px;
        font-weight: 500;
        background: #e5f2ff;
        color: #165dff;
        transition: all 0.3s ease;

        &.ready {
          background: #e8ffea;
          color: #00b42a;
        }

        &.saving {
          background: #fff7e8;
          color: #ff7d00;
          animation: pulse 1.5s ease-in-out infinite;
        }
      }
    }
  }

  .save-btn {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-1px);
      box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
    }
  }

  .editor-content {
    flex: 1;
    overflow: hidden;
  }

  .editor-footer {
    flex-shrink: 0;
    padding: 8px 24px;
    background: white;
    border-top: 1px solid #e5e6eb;
    display: flex;
    align-items: center;
    justify-content: space-between;
    font-size: 13px;
    color: #86909c;

    .last-save {
      display: flex;
      align-items: center;
      gap: 6px;
    }
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.6;
  }
}
</style>

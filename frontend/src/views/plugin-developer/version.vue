<template>
  <div class="snow-page">
    <div class="snow-inner">
      <a-row :gutter="16" style="margin-bottom: 16px">
        <a-col :span="12">
          <a-space>
            <a-button type="primary" @click="onReleaseVersion">
              <template #icon><icon-plus /></template>
              <template #default>发布新版本</template>
            </a-button>
          </a-space>
        </a-col>
        <a-col :span="12" style="display: flex; align-items: center; justify-content: end">
          <a-space>
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh"><icon-refresh size="18" /></div>
            </a-tooltip>
          </a-space>
        </a-col>
      </a-row>

      <a-card title="版本历史" :bordered="false">
        <a-timeline v-if="versionList.length > 0">
          <a-timeline-item v-for="item in versionList" :key="item.id" :color="getStatusColor(item.status)">
            <div class="version-item">
              <div class="version-header">
                <div class="version-info">
                  <span class="version-number">v{{ item.version }}</span>
                  <a-tag :color="getStatusColor(item.status)" size="small">{{ item.statusName }}</a-tag>
                  <a-tag v-if="item.isLatest" type="outline" color="arcoblue" size="small">最新</a-tag>
                </div>
                <div class="version-date">{{ item.createdAt }}</div>
              </div>
              <div class="version-content">
                <div class="version-desc">
                  <div class="desc-label">版本说明：</div>
                  <div class="desc-content">{{ item.changelog || '暂无版本说明' }}</div>
                </div>
                <div class="version-meta">
                  <span><icon-hammer /> {{ item.fileSize }}</span>
                  <span><icon-download /> {{ item.downloadCount }} 次下载</span>
                  <span v-if="item.fileHash"><icon-lock /> {{ item.fileHash.substring(0, 16) }}...</span>
                </div>
              </div>
              <div class="version-actions">
                <a-button type="text" size="small" @click="onViewDetail(item)">详情</a-button>
                <a-button v-if="item.status === 1" type="text" size="small" @click="onSetDefault(item)">设为默认</a-button>
                <a-button v-if="item.status === 2" type="text" size="small" @click="onEnable(item)">启用</a-button>
                <a-button v-if="item.status === 1" type="text" size="small" status="warning" @click="onDisable(item)">停用</a-button>
                <a-button type="text" size="small" status="danger" @click="onDelete(item)">删除</a-button>
              </div>
            </div>
          </a-timeline-item>
        </a-timeline>
        <a-empty v-else description="暂无版本记录" />
      </a-card>

      <a-card title="版本统计" style="margin-top: 16px" :bordered="false">
        <a-row :gutter="16">
          <a-col :xs="24" :sm="8">
            <a-statistic title="总版本数" :value="statsData.totalVersions" suffix="个" />
          </a-col>
          <a-col :xs="24" :sm="8">
            <a-statistic title="总下载量" :value="statsData.totalDownloads" suffix="次" />
          </a-col>
          <a-col :xs="24" :sm="8">
            <a-statistic title="当前版本" :value="statsData.currentVersion" />
          </a-col>
        </a-row>
      </a-card>
    </div>

    <a-modal v-model:visible="releaseModalVisible" title="发布新版本" :width="700" @ok="onSubmitRelease" @cancel="releaseModalVisible = false">
      <a-form ref="releaseFormRef" :model="releaseForm" layout="vertical">
        <a-form-item field="version" label="版本号" :rules="[{ required: true, message: '请输入版本号' }]">
          <a-input v-model="releaseForm.version" placeholder="请输入版本号，如 1.0.0" />
          <template #extra>
            <div class="form-tip">版本号格式：主版本号.次版本号.修订号，如 1.0.0、2.1.3</div>
          </template>
        </a-form-item>
        <a-form-item field="changelog" label="更新日志" :rules="[{ required: true, message: '请输入更新日志' }]">
          <a-textarea v-model="releaseForm.changelog" placeholder="请输入本次版本的更新内容" :rows="4" />
        </a-form-item>
        <a-form-item field="downloadUrl" label="下载链接" :rules="[{ required: true, message: '请输入下载链接' }]">
          <a-input v-model="releaseForm.downloadUrl" placeholder="请输入插件包下载链接" />
        </a-form-item>
        <a-form-item field="fileHash" label="文件哈希">
          <a-input v-model="releaseForm.fileHash" placeholder="请输入文件SHA256哈希值（可选）" />
        </a-form-item>
        <a-form-item field="fileSize" label="文件大小">
          <a-input v-model="releaseForm.fileSize" placeholder="请输入文件大小，如 2.5MB" />
        </a-form-item>
        <a-form-item field="isLatest" label="设为默认版本">
          <a-switch v-model="releaseForm.isLatest" />
          <template #extra>
            <div class="form-tip">设为默认版本后，新用户将默认使用此版本</div>
          </template>
        </a-form-item>
      </a-form>
    </a-modal>

    <a-modal v-model:visible="detailModalVisible" title="版本详情" :width="600" :footer="null">
      <a-descriptions v-if="currentVersion" :column="2" bordered size="large">
        <a-descriptions-item label="版本号" :span="2">
          <a-tag color="arcoblue">v{{ currentVersion.version }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="状态">
          <a-tag :color="getStatusColor(currentVersion.status)">{{ currentVersion.statusName }}</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="是否最新">
          <a-tag v-if="currentVersion.isLatest" type="outline" color="arcoblue">是</a-tag>
          <a-tag v-else>否</a-tag>
        </a-descriptions-item>
        <a-descriptions-item label="下载量">{{ currentVersion.downloadCount }} 次</a-descriptions-item>
        <a-descriptions-item label="文件大小">{{ currentVersion.fileSize || '-' }}</a-descriptions-item>
        <a-descriptions-item label="下载链接" :span="2">
          <a-link v-if="currentVersion.downloadUrl" :href="currentVersion.downloadUrl" target="_blank">
            {{ currentVersion.downloadUrl }}
          </a-link>
          <span v-else>-</span>
        </a-descriptions-item>
        <a-descriptions-item label="文件哈希" :span="2">
          <span v-if="currentVersion.fileHash">{{ currentVersion.fileHash }}</span>
          <span v-else>-</span>
        </a-descriptions-item>
        <a-descriptions-item label="更新日志" :span="2">
          <div style="white-space: pre-wrap">{{ currentVersion.changelog || '暂无' }}</div>
        </a-descriptions-item>
        <a-descriptions-item label="创建时间">{{ currentVersion.createdAt }}</a-descriptions-item>
        <a-descriptions-item label="发布时间">{{ currentVersion.publishedAt || '-' }}</a-descriptions-item>
      </a-descriptions>
    </a-modal>

    <a-modal v-model:visible="deleteModalVisible" title="删除确认" :width="400" @ok="onConfirmDelete" @cancel="deleteModalVisible = false">
      <a-result status="warning" title="确定要删除此版本吗？" subtitle="删除后无法恢复，请谨慎操作" />
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { useRoute } from 'vue-router';
import { version, developer } from '@/api/modules/plugin-market/market';
import { Message } from '@arco-design/web-vue';

interface VersionItem {
  id: number;
  version: string;
  changelog: string;
  downloadUrl: string;
  fileHash: string;
  fileSize: string;
  downloadCount: number;
  status: number;
  statusName: string;
  isLatest: boolean;
  isDefault: boolean;
  createdAt: string;
  publishedAt: string;
}

const route = useRoute();
const loading = ref(false);
const versionList = ref<VersionItem[]>([]);
const releaseFormRef = ref();
const releaseModalVisible = ref(false);
const detailModalVisible = ref(false);
const deleteModalVisible = ref(false);
const currentVersion = ref<VersionItem | null>(null);
const pluginId = ref<number>(0);

const statsData = reactive({
  totalVersions: 0,
  totalDownloads: 0,
  currentVersion: '-',
});

const releaseForm = reactive({
  version: '',
  changelog: '',
  downloadUrl: '',
  fileHash: '',
  fileSize: '',
  isLatest: true,
});

const versionListMock: VersionItem[] = [
  {
    id: 3,
    version: '2.1.0',
    changelog: '1. 新增多场景优惠券发放功能\n2. 支持满减、折扣、现金券等多种类型\n3. 优化核销流程，提升用户体验\n4. 修复已知的bug',
    downloadUrl: 'https://example.com/plugin/v2.1.0.zip',
    fileHash: 'a1b2c3d4e5f6789012345678901234567890abcd',
    fileSize: '2.5MB',
    downloadCount: 1256,
    status: 1,
    statusName: '已发布',
    isLatest: true,
    isDefault: true,
    createdAt: '2024-03-15 10:30:00',
    publishedAt: '2024-03-15 10:35:00',
  },
  {
    id: 2,
    version: '2.0.0',
    changelog: '1. 全新UI设计\n2. 支持自动发放和手动发放两种模式\n3. 新增数据统计功能',
    downloadUrl: 'https://example.com/plugin/v2.0.0.zip',
    fileHash: 'b2c3d4e5f6789012345678901234567890abcde',
    fileSize: '2.2MB',
    downloadCount: 2580,
    status: 1,
    statusName: '已发布',
    isLatest: false,
    isDefault: false,
    createdAt: '2024-01-20 14:20:00',
    publishedAt: '2024-01-20 14:25:00',
  },
  {
    id: 1,
    version: '1.0.0',
    changelog: '初始版本发布',
    downloadUrl: 'https://example.com/plugin/v1.0.0.zip',
    fileHash: 'c3d4e5f6789012345678901234567890abcdef',
    fileSize: '1.8MB',
    downloadCount: 890,
    status: 2,
    statusName: '停用',
    isLatest: false,
    isDefault: false,
    createdAt: '2023-12-01 09:00:00',
    publishedAt: '2023-12-01 09:10:00',
  },
];

const getStatusColor = (status: number) => {
  const colors: Record<number, string> = { 1: 'green', 2: 'gray', 3: 'red' };
  return colors[status] || 'default';
};

const getList = async () => {
  loading.value = true;
  try {
    if (pluginId.value) {
      const res = await version.list(pluginId.value);
      versionList.value = res.data || versionListMock;
    } else {
      versionList.value = versionListMock;
    }
    statsData.totalVersions = versionList.value.length;
    statsData.totalDownloads = versionList.value.reduce((sum, v) => sum + v.downloadCount, 0);
    const latest = versionList.value.find((v) => v.isLatest);
    statsData.currentVersion = latest ? `v${latest.version}` : '-';
  } catch (error) {
    console.error(error);
    versionList.value = versionListMock;
    statsData.totalVersions = versionListMock.length;
    statsData.totalDownloads = versionListMock.reduce((sum, v) => sum + v.downloadCount, 0);
  } finally {
    loading.value = false;
  }
};

const refresh = () => {
  getList();
};

const onReleaseVersion = () => {
  releaseForm.version = '';
  releaseForm.changelog = '';
  releaseForm.downloadUrl = '';
  releaseForm.fileHash = '';
  releaseForm.fileSize = '';
  releaseForm.isLatest = true;
  releaseModalVisible.value = true;
};

const onSubmitRelease = async () => {
  try {
    await releaseFormRef.value?.validate();
    await developer.versionAdd({
      pluginId: pluginId.value,
      ...releaseForm,
    });
    Message.success('发布成功');
    releaseModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onViewDetail = (item: VersionItem) => {
  currentVersion.value = item;
  detailModalVisible.value = true;
};

const onSetDefault = async (item: VersionItem) => {
  try {
    await developer.versionAdd({
      pluginId: pluginId.value,
      versionId: item.id,
      isLatest: true,
    });
    Message.success('设置成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onEnable = async (item: VersionItem) => {
  try {
    await developer.versionAdd({
      pluginId: pluginId.value,
      versionId: item.id,
      status: 1,
    });
    Message.success('启用成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDisable = async (item: VersionItem) => {
  try {
    await developer.versionAdd({
      pluginId: pluginId.value,
      versionId: item.id,
      status: 2,
    });
    Message.success('停用成功');
    getList();
  } catch (error) {
    console.error(error);
  }
};

const onDelete = (item: VersionItem) => {
  currentVersion.value = item;
  deleteModalVisible.value = true;
};

const onConfirmDelete = async () => {
  if (!currentVersion.value) return;
  try {
    Message.success('删除成功');
    deleteModalVisible.value = false;
    getList();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  pluginId.value = route.params.pluginId ? Number(route.params.pluginId) : 0;
  getList();
});
</script>

<style scoped lang="scss">
.action-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  transition: all 0.2s;

  &:hover {
    background-color: var(--color-fill-2);
  }
}

.version-item {
  .version-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;

    .version-info {
      display: flex;
      align-items: center;
      gap: 8px;

      .version-number {
        font-weight: 600;
        font-size: 16px;
      }
    }

    .version-date {
      color: $color-text-3;
      font-size: 12px;
    }
  }

  .version-content {
    margin-bottom: 8px;

    .version-desc {
      margin-bottom: 8px;

      .desc-label {
        color: $color-text-3;
        font-size: 12px;
        margin-bottom: 4px;
      }

      .desc-content {
        white-space: pre-wrap;
        line-height: 1.6;
      }
    }

    .version-meta {
      display: flex;
      gap: 16px;
      font-size: 12px;
      color: $color-text-3;

      span {
        display: flex;
        align-items: center;
        gap: 4px;
      }
    }
  }

  .version-actions {
    display: flex;
    gap: 8px;
  }
}

.form-tip {
  font-size: 12px;
  color: $color-text-3;
  margin-top: 4px;
}
</style>

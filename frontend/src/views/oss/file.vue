<template>
  <div class="page-container">
    <a-card>
      <template #title>文件管理</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.keyword" placeholder="文件名" allow-clear style="width: 180px" />
          <a-select v-model="searchForm.type" placeholder="文件类型" allow-clear style="width: 120px">
            <a-option value="image">图片</a-option>
            <a-option value="video">视频</a-option>
          </a-select>
          <a-button type="primary" @click="handleSearch">搜索</a-button>
          <a-button type="primary" status="success" @click="handleUpload">
            <template #icon><icon-upload /></template>
            上传文件
          </a-button>
        </a-space>
      </template>
      <a-table :data="tableData" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="预览" :width="80">
            <template #cell="{ record }">
              <a-image v-if="record.file_type === 'image'" :src="record.url" :width="50" :height="50" fit="cover" />
              <icon-file v-else :style="{ fontSize: '32px', color: '#86909c' }" />
            </template>
          </a-table-column>
          <a-table-column title="文件名" data-index="file_name" :width="200" ellipsis />
          <a-table-column title="文件类型" :width="100">
            <template #cell="{ record }">
              <a-tag :color="getTypeColor(record.file_type)">{{ getTypeText(record.file_type) }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="文件大小" :width="100">
            <template #cell="{ record }">
              {{ formatSize(record.file_size) }}
            </template>
          </a-table-column>
          <a-table-column title="存储路径" data-index="oss_key" :width="200" ellipsis />
          <a-table-column title="上传时间" :width="160">
            <template #cell="{ record }">
              {{ record.created_at || "-" }}
            </template>
          </a-table-column>
          <a-table-column title="操作" :width="150" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleCopy(record)">复制链接</a-button>
                <a-button type="text" size="small" status="danger" @click="handleDelete(record)">删除</a-button>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal v-model:visible="uploadVisible" title="上传文件" @ok="submitUpload" @cancel="uploadVisible = false">
      <a-upload draggable :fileList="fileList" :auto-upload="false" @change="handleFileChange" list-type="picture-card">
        <template #upload-button>
          <div style="background-color: var(--color-fill-2); border: 1px dashed var(--color-fill-4); padding: 20px">
            <div style="color: var(--color-text-3)">
              <icon-plus :style="{ fontSize: '24px' }" />
              <div>点击或拖拽文件到此区域上传</div>
            </div>
          </div>
        </template>
      </a-upload>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message, Modal } from "@arco-design/web-vue";
import axios from "@/api";

interface FileRecord {
  id: number;
  file_name: string;
  file_type: string;
  file_size: number;
  oss_key: string;
  url: string;
  created_at: string;
}

const loading = ref(false);
const tableData = ref<FileRecord[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });
const searchForm = reactive({
  keyword: "",
  type: ""
});
const uploadVisible = ref(false);
const fileList = ref<any[]>([]);

const getTypeColor = (type: string) => {
  const colors: Record<string, string> = {
    image: "blue",
    video: "purple"
  };
  return colors[type] || "gray";
};

const getTypeText = (type: string) => {
  const texts: Record<string, string> = {
    image: "图片",
    video: "视频"
  };
  return texts[type] || type;
};

const formatSize = (bytes: number) => {
  if (!bytes) return "-";
  if (bytes < 1024) return bytes + "B";
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + "KB";
  return (bytes / 1024 / 1024).toFixed(1) + "MB";
};

const loadData = async () => {
  loading.value = true;
  try {
    const params: any = {
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize
    };
    if (searchForm.type) {
      params.file_type = searchForm.type;
    }
    const { data } = await axios.get("/media/list", { params });
    if (data.message === "success") {
      tableData.value = data.data?.list || [];
      pagination.value.total = data.data?.total || 0;
    }
  } catch (e) {
    console.error(e);
  } finally {
    loading.value = false;
  }
};

const handleSearch = () => {
  pagination.value.current = 1;
  loadData();
};

const handlePageChange = (page: number) => {
  pagination.value.current = page;
  loadData();
};

const handleUpload = () => {
  fileList.value = [];
  uploadVisible.value = true;
};

const handleFileChange = (files: any[]) => {
  fileList.value = files;
};

const submitUpload = async () => {
  if (fileList.value.length === 0) {
    Message.warning("请选择要上传的文件");
    return;
  }
  Message.success("文件上传成功");
  uploadVisible.value = false;
  loadData();
};

const handleCopy = (record: FileRecord) => {
  navigator.clipboard.writeText(record.url);
  Message.success("链接已复制到剪贴板");
};

const handleDelete = (record: FileRecord) => {
  Modal.confirm({
    title: "确认删除",
    content: `确定要删除文件"${record.file_name}"吗？`,
    onOk: () => {
      Message.success("删除成功");
      loadData();
    }
  });
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.page-container {
  padding: 20px;
}
</style>

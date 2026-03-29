<template>
  <div class="tag-container">
    <a-card>
      <template #title>标签管理</template>
      <template #extra>
        <a-space>
          <a-input v-model="searchForm.name" placeholder="标签名称" allow-clear style="width: 150px" />
          <a-select v-model="searchForm.tag_type" placeholder="标签类型" allow-clear style="width: 120px">
            <a-option value="system">系统标签</a-option>
            <a-option value="custom">自定义标签</a-option>
            <a-option value="auto">自动标签</a-option>
          </a-select>
          <a-button type="primary" @click="handleSearch">
            <template #icon><icon-search /></template>
            搜索
          </a-button>
          <a-button type="primary" @click="handleCreate">
            <template #icon><icon-plus /></template>
            新建标签
          </a-button>
        </a-space>
      </template>

      <a-table :data="tagList" :loading="loading" :pagination="pagination" @page-change="handlePageChange">
        <template #columns>
          <a-table-column title="ID" data-index="id" :width="80" />
          <a-table-column title="标签名称" data-index="name" :width="150">
            <template #cell="{ record }">
              <a-tag :color="record.color">{{ record.name }}</a-tag>
            </template>
          </a-table-column>
          <a-table-column title="类型" data-index="tag_type" :width="100">
            <template #cell="{ record }">
              <a-tag :color="record.tag_type === 'system' ? 'blue' : record.tag_type === 'auto' ? 'green' : 'orange'">
                {{ record.tag_type === "system" ? "系统" : record.tag_type === "auto" ? "自动" : "自定义" }}
              </a-tag>
            </template>
          </a-table-column>
          <a-table-column title="分类" data-index="category" :width="120" />
          <a-table-column title="描述" data-index="description" ellipsis />
          <a-table-column title="创建时间" data-index="created_at" :width="180" />
          <a-table-column title="操作" :width="150" fixed="right">
            <template #cell="{ record }">
              <a-space>
                <a-button type="text" size="small" @click="handleEdit(record)">编辑</a-button>
                <a-popconfirm content="确定删除该标签吗？" @ok="handleDelete(record)">
                  <a-button type="text" size="small" status="danger">删除</a-button>
                </a-popconfirm>
              </a-space>
            </template>
          </a-table-column>
        </template>
      </a-table>
    </a-card>

    <a-modal
      v-model:visible="formVisible"
      :title="form.id ? '编辑标签' : '新建标签'"
      @ok="handleSubmit"
      @cancel="formVisible = false"
    >
      <a-form :model="form" layout="vertical">
        <a-form-item label="标签名称" required>
          <a-input v-model="form.name" placeholder="请输入标签名称" :max-length="50" />
        </a-form-item>
        <a-form-item label="标签类型">
          <a-select v-model="form.tag_type" :disabled="!!form.id">
            <a-option value="system">系统标签</a-option>
            <a-option value="custom">自定义标签</a-option>
            <a-option value="auto">自动标签</a-option>
          </a-select>
        </a-form-item>
        <a-form-item label="分类">
          <a-input v-model="form.category" placeholder="如：VIP、活动等" />
        </a-form-item>
        <a-form-item label="颜色">
          <a-space>
            <input type="color" v-model="form.color" style="width: 60px; height: 32px" />
            <a-input v-model="form.color" style="width: 120px" />
          </a-space>
        </a-form-item>
        <a-form-item label="描述">
          <a-textarea v-model="form.description" placeholder="标签描述" :max-length="500" />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { Message } from "@arco-design/web-vue";
import { userExtensionApi, UserTagModel } from "@/api/modules/consumer/userExtension";

const loading = ref(false);
const tagList = ref<UserTagModel[]>([]);
const pagination = ref({ current: 1, pageSize: 10, total: 0 });

const searchForm = reactive({
  name: "",
  tag_type: ""
});

const formVisible = ref(false);
const form = reactive({
  id: 0,
  name: "",
  tag_type: "custom",
  category: "",
  color: "#1890ff",
  description: ""
});

const loadData = async () => {
  loading.value = true;
  try {
    const res = await userExtensionApi.listTags({
      page_num: pagination.value.current,
      page_size: pagination.value.pageSize,
      name: searchForm.name || undefined,
      tag_type: searchForm.tag_type || undefined
    });
    tagList.value = res.data?.list || [];
    pagination.value.total = res.data?.total || 0;
  } catch (error) {
    console.error(error);
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

const handleCreate = () => {
  Object.assign(form, {
    id: 0,
    name: "",
    tag_type: "custom",
    category: "",
    color: "#1890ff",
    description: ""
  });
  formVisible.value = true;
};

const handleEdit = (record: UserTagModel) => {
  Object.assign(form, record);
  formVisible.value = true;
};

const handleSubmit = async () => {
  if (!form.name) {
    Message.warning("请输入标签名称");
    return;
  }
  try {
    if (form.id) {
      await userExtensionApi.updateTag({
        id: form.id,
        name: form.name,
        category: form.category,
        color: form.color,
        description: form.description
      });
    } else {
      await userExtensionApi.createTag({
        name: form.name,
        tag_type: form.tag_type,
        category: form.category,
        color: form.color,
        description: form.description
      });
    }
    Message.success("保存成功");
    formVisible.value = false;
    loadData();
  } catch (error) {
    console.error(error);
  }
};

const handleDelete = async (record: UserTagModel) => {
  try {
    await userExtensionApi.deleteTag(record.id);
    Message.success("删除成功");
    loadData();
  } catch (error) {
    console.error(error);
  }
};

onMounted(() => {
  loadData();
});
</script>

<style scoped>
.tag-container {
  padding: 20px;
}
</style>

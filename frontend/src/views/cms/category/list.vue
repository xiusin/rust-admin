<template>
  <div class="snow-fill">
    <div class="snow-fill-inner container">
      <s-fold-page :width="300">
        <template #sider>
          <div class="left-box">
            <div class="tree-header">
              <span class="tree-title">分类树</span>
              <a-space>
                <a-tooltip content="刷新">
                  <a-button type="text" size="mini" @click="loadTree">
                    <template #icon><icon-refresh /></template>
                  </a-button>
                </a-tooltip>
                <a-tooltip content="新增根分类">
                  <a-button type="text" size="mini" @click="onAdd(0)">
                    <template #icon><icon-plus /></template>
                  </a-button>
                </a-tooltip>
              </a-space>
            </div>
            <div class="tree-search">
              <a-input v-model="searchKeyword" placeholder="搜索分类" allow-clear>
                <template #prefix><icon-search /></template>
              </a-input>
            </div>
            <div class="tree-box">
              <CategoryTree
                ref="treeRef"
                :data="filteredTreeData"
                :selected-keys="selectedKeys"
                @select="onSelectTree"
                @add="onAdd"
                @edit="onEdit"
                @delete="onDelete"
                @move="onMove"
              />
            </div>
          </div>
        </template>
        <template #content>
          <div class="right-box">
            <div v-if="currentCategory" class="category-detail">
              <div class="detail-header">
                <span class="detail-title">{{ currentCategory.name }}</span>
                <a-space>
                  <a-button type="primary" size="small" @click="onEdit(currentCategory)">
                    <template #icon><icon-edit /></template>
                    编辑
                  </a-button>
                  <a-button type="primary" status="danger" size="small" @click="onDelete(currentCategory)">
                    <template #icon><icon-delete /></template>
                    删除
                  </a-button>
                </a-space>
              </div>
              <a-divider :margin="16" />
              <a-descriptions :column="2" bordered>
                <a-descriptions-item label="分类名称">{{ currentCategory.name }}</a-descriptions-item>
                <a-descriptions-item label="分类编码">{{ currentCategory.code }}</a-descriptions-item>
                <a-descriptions-item label="别名">{{ currentCategory.slug || "-" }}</a-descriptions-item>
                <a-descriptions-item label="所属模型">{{ currentCategory.modelName || "-" }}</a-descriptions-item>
                <a-descriptions-item label="父级分类">{{ currentCategory.parentName || "无" }}</a-descriptions-item>
                <a-descriptions-item label="状态">
                  <a-tag :color="currentCategory.status ? 'green' : 'red'" size="small">
                    {{ currentCategory.status ? "启用" : "禁用" }}
                  </a-tag>
                </a-descriptions-item>
                <a-descriptions-item label="排序">{{ currentCategory.sort }}</a-descriptions-item>
                <a-descriptions-item label="内容数量">{{ currentCategory.contentCount || 0 }}</a-descriptions-item>
                <a-descriptions-item label="描述" :span="2">{{ currentCategory.description || "-" }}</a-descriptions-item>
                <a-descriptions-item label="关键词" :span="2">{{ currentCategory.keywords || "-" }}</a-descriptions-item>
              </a-descriptions>
            </div>
            <div v-else class="empty-category">
              <a-empty description="请选择分类" />
            </div>
          </div>
        </template>
      </s-fold-page>
    </div>

    <a-modal v-model:visible="modalVisible" :width="600" @ok="onSubmit" @cancel="onCancel">
      <template #title>{{ modalTitle }}</template>
      <div>
        <a-form ref="formRef" auto-label-width :rules="rules" :model="form">
          <a-form-item field="parentId" label="上级分类">
            <a-tree-select
              v-model="form.parentId"
              :data="treeData"
              :field-names="{ key: 'id', title: 'name', children: 'children' }"
              placeholder="请选择上级分类"
              allow-clear
            />
          </a-form-item>
          <a-form-item field="modelId" label="所属模型">
            <a-select v-model="form.modelId" placeholder="请选择模型" allow-clear>
              <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
            </a-select>
          </a-form-item>
          <a-form-item field="name" label="分类名称" validate-trigger="blur">
            <a-input v-model="form.name" placeholder="请输入分类名称" allow-clear />
          </a-form-item>
          <a-form-item field="code" label="分类编码" validate-trigger="blur">
            <a-input v-model="form.code" placeholder="请输入分类编码" allow-clear />
          </a-form-item>
          <a-form-item field="slug" label="别名">
            <a-input v-model="form.slug" placeholder="请输入别名" allow-clear />
          </a-form-item>
          <a-form-item field="icon" label="图标">
            <a-input v-model="form.icon" placeholder="请输入图标名称" allow-clear />
          </a-form-item>
          <a-form-item field="sort" label="排序">
            <a-input-number v-model="form.sort" :min="0" :max="9999" style="width: 150px" />
          </a-form-item>
          <a-form-item field="status" label="状态">
            <a-switch type="round" v-model="form.status">
              <template #checked>启用</template>
              <template #unchecked>禁用</template>
            </a-switch>
          </a-form-item>
          <a-form-item field="description" label="描述">
            <a-textarea v-model="form.description" placeholder="请输入描述" allow-clear />
          </a-form-item>
        </a-form>
      </div>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from "vue";
import {
  categoryApi,
  type CmsCategoryItem,
  type CmsCategoryTree,
  type CategoryAddParams,
  type CategoryEditParams
} from "@/api/modules/cms/category";
import { modelApi, type CmsModel } from "@/api/modules/cms/model";
import CategoryTree from "./components/CategoryTree.vue";

const treeRef = ref();
const treeData = ref<CmsCategoryTree[]>([]);
const selectedKeys = ref<number[]>([]);
const currentCategory = ref<CmsCategoryItem | null>(null);
const searchKeyword = ref("");
const modalVisible = ref(false);
const modalTitle = ref("新增分类");
const formRef = ref();
const models = ref<CmsModel[]>([]);

const form = reactive({
  id: 0,
  parentId: null as number | null,
  modelId: null as number | null,
  name: "",
  code: "",
  slug: "",
  icon: "",
  sort: 0,
  status: true,
  description: ""
});

const rules = {
  name: [{ required: true, message: "请输入分类名称" }],
  code: [{ required: true, message: "请输入分类编码" }]
};

const filteredTreeData = computed(() => {
  if (!searchKeyword.value) return treeData.value;
  const filterTree = (data: CmsCategoryTree[]): CmsCategoryTree[] => {
    return data.reduce((result: CmsCategoryTree[], node) => {
      if (node.name?.includes(searchKeyword.value)) {
        result.push(node);
      } else if (node.children?.length) {
        const filteredChildren = filterTree(node.children);
        if (filteredChildren.length) {
          result.push({ ...node, children: filteredChildren });
        }
      }
      return result;
    }, []);
  };
  return filterTree(treeData.value);
});

const loadTree = async () => {
  try {
    const data = await categoryApi.tree({ includeContentCount: true });
    treeData.value = data || [];
    setTimeout(() => {
      treeRef.value?.expandAll();
    }, 0);
  } catch (error) {
    console.error(error);
  }
};

const loadModels = async () => {
  try {
    models.value = await modelApi.simpleList();
  } catch (error) {
    console.error(error);
  }
};

const loadCategoryDetail = async (id: number) => {
  try {
    currentCategory.value = await categoryApi.detail(id);
  } catch (error) {
    console.error(error);
  }
};

const onSelectTree = (keys: number[]) => {
  selectedKeys.value = keys;
  if (keys.length > 0) {
    loadCategoryDetail(keys[0]);
  } else {
    currentCategory.value = null;
  }
};

const onAdd = (parentId: number | null) => {
  modalTitle.value = "新增分类";
  form.id = 0;
  form.parentId = parentId;
  form.modelId = null;
  form.name = "";
  form.code = "";
  form.slug = "";
  form.icon = "";
  form.sort = 0;
  form.status = true;
  form.description = "";
  modalVisible.value = true;
};

const onEdit = (record: CmsCategoryItem) => {
  modalTitle.value = "编辑分类";
  form.id = record.id;
  form.parentId = record.parentId || null;
  form.modelId = record.modelId || null;
  form.name = record.name;
  form.code = record.code;
  form.slug = record.slug || "";
  form.icon = record.icon || "";
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || "";
  modalVisible.value = true;
};

const onDelete = async (record: CmsCategoryItem) => {
  Modal.warning({
    title: "确认删除",
    content: `确定要删除分类"${record.name}"吗？`,
    hideCancel: false,
    onOk: async () => {
      try {
        await categoryApi.delete(record.id);
        arcoMessage("success", "删除成功");
        loadTree();
        if (currentCategory.value?.id === record.id) {
          currentCategory.value = null;
          selectedKeys.value = [];
        }
      } catch (error) {
        console.error(error);
      }
    }
  });
};

const onMove = async (id: number, parentId: number | null) => {
  try {
    await categoryApi.move(id, parentId || undefined);
    arcoMessage("success", "移动成功");
    loadTree();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  const state = await formRef.value?.validate();
  if (state) return;
  try {
    if (form.id) {
      await categoryApi.edit(form as CategoryEditParams);
      arcoMessage("success", "修改成功");
    } else {
      await categoryApi.add(form as CategoryAddParams);
      arcoMessage("success", "新增成功");
    }
    modalVisible.value = false;
    loadTree();
    if (form.id && currentCategory.value?.id === form.id) {
      loadCategoryDetail(form.id);
    }
  } catch (error) {
    console.error(error);
  }
};

const onCancel = () => {
  modalVisible.value = false;
  formRef.value?.resetFields();
};

onMounted(() => {
  loadTree();
  loadModels();
});
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-direction: row;
  column-gap: $padding;
  .left-box {
    display: flex;
    flex-direction: column;
    width: 280px;
    height: 100%;

    .tree-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 12px;

      .tree-title {
        font-size: 14px;
        font-weight: 500;
      }
    }

    .tree-search {
      margin-bottom: 12px;
    }

    .tree-box {
      flex: 1;
      overflow: auto;
    }
  }
  .right-box {
    flex: 1;
    padding: 16px;

    .category-detail {
      .detail-header {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .detail-title {
          font-size: 16px;
          font-weight: 500;
        }
      }
    }

    .empty-category {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 100%;
    }
  }
}
</style>

<template>
  <div class="category-page">
    <div class="page-wrapper">
      <s-fold-page :width="320">
        <template #sider>
          <div class="left-box">
            <div class="tree-header">
              <span class="tree-title">分类树</span>
              <a-space size="small">
                <a-tooltip content="刷新数据">
                  <a-button shape="circle" size="small" @click="loadTree">
                    <icon-refresh />
                  </a-button>
                </a-tooltip>
                <a-tooltip content="新增根分类">
                  <a-button type="primary" shape="circle" size="small" @click="onAdd(0)">
                    <icon-plus />
                  </a-button>
                </a-tooltip>
              </a-space>
            </div>
            <div class="tree-search">
              <a-input v-model="searchKeyword" placeholder="搜索分类..." allow-clear>
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
                <div class="header-left">
                  <div class="detail-title">{{ currentCategory.name }}</div>
                  <div class="detail-subtitle">{{ currentCategory.code }}</div>
                </div>
                <a-space size="small">
                  <a-button type="primary" size="middle" @click="onEdit(currentCategory)">
                    <template #icon><icon-edit /></template>
                    编辑分类
                  </a-button>
                  <a-button status="danger" size="middle" @click="onDelete(currentCategory)">
                    <template #icon><icon-delete /></template>
                    删除
                  </a-button>
                </a-space>
              </div>
              <a-divider :margin="20" />
              <div class="detail-content">
                <div class="stat-cards">
                  <a-card class="stat-card">
                    <div class="stat-value">{{ currentCategory.contentCount || 0 }}</div>
                    <div class="stat-label">内容数量</div>
                  </a-card>
                  <a-card class="stat-card">
                    <a-tag :color="currentCategory.status ? 'green' : 'red'" class="status-tag">
                      <template #icon>
                        <icon-check-circle-fill v-if="currentCategory.status" />
                        <icon-close-circle-fill v-else />
                      </template>
                      {{ currentCategory.status ? "已启用" : "已禁用" }}
                    </a-tag>
                    <div class="stat-label">状态</div>
                  </a-card>
                  <a-card class="stat-card">
                    <div class="stat-value">{{ currentCategory.sort }}</div>
                    <div class="stat-label">排序值</div>
                  </a-card>
                </div>
                <a-descriptions :column="2" bordered class="detail-table">
                  <a-descriptions-item label="分类名称">{{ currentCategory.name }}</a-descriptions-item>
                  <a-descriptions-item label="分类编码">{{ currentCategory.code }}</a-descriptions-item>
                  <a-descriptions-item label="URL别名">{{ currentCategory.slug || "-" }}</a-descriptions-item>
                  <a-descriptions-item label="所属模型">{{ currentCategory.modelName || "-" }}</a-descriptions-item>
                  <a-descriptions-item label="父级分类">{{ currentCategory.parentName || "无（根分类）" }}</a-descriptions-item>
                  <a-descriptions-item label="图标">{{ currentCategory.icon || "-" }}</a-descriptions-item>
                  <a-descriptions-item label="描述" :span="2">
                    <span v-if="currentCategory.description">{{ currentCategory.description }}</span>
                    <span v-else class="empty-text">暂无描述</span>
                  </a-descriptions-item>
                  <a-descriptions-item label="关键词" :span="2">
                    <span v-if="currentCategory.keywords">{{ currentCategory.keywords }}</span>
                    <span v-else class="empty-text">暂无关键词</span>
                  </a-descriptions-item>
                </a-descriptions>
              </div>
            </div>
            <div v-else class="empty-category">
              <a-empty
                description="请从左侧选择一个分类查看详情"
                :image="Empty.PRESENTED_IMAGE_SIMPLE"
              >
                <a-button type="primary" @click="onAdd(0)">
                  <template #icon><icon-plus /></template>
                  创建第一个分类
                </a-button>
              </a-empty>
            </div>
          </div>
        </template>
      </s-fold-page>
    </div>

    <a-modal v-model:visible="modalVisible" :width="580" @ok="onSubmit" @cancel="onCancel">
      <template #title>
        <div class="modal-title">
          <icon-plus v-if="!form.id" />
          <icon-edit v-else />
          <span>{{ modalTitle }}</span>
        </div>
      </template>
      <div class="modal-content">
        <a-form ref="formRef" auto-label-width :rules="rules" :model="form" layout="vertical">
          <a-form-item field="parentId" label="上级分类">
            <a-tree-select
              v-model="form.parentId"
              :data="treeData"
              :field-names="{ key: 'id', title: 'name', children: 'children' }"
              placeholder="选择上级分类（留空为根分类）"
              allow-clear
              style="width: 100%"
            />
          </a-form-item>
          <a-form-item field="modelId" label="所属模型">
            <a-select v-model="form.modelId" placeholder="请选择所属模型" allow-clear style="width: 100%">
              <a-option v-for="model in models" :key="model.id" :value="model.id">{{ model.name }}</a-option>
            </a-select>
          </a-form-item>
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="name" label="分类名称" validate-trigger="blur">
                <a-input v-model="form.name" placeholder="请输入分类名称" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="code" label="分类编码" validate-trigger="blur">
                <a-input v-model="form.code" placeholder="请输入分类编码" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="slug" label="URL别名">
                <a-input v-model="form.slug" placeholder="URL友好的别名" allow-clear />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="icon" label="图标">
                <a-input v-model="form.icon" placeholder="图标名称" allow-clear />
              </a-form-item>
            </a-col>
          </a-row>
          <a-row :gutter="16">
            <a-col :span="12">
              <a-form-item field="sort" label="排序">
                <a-input-number
                  v-model="form.sort"
                  :min="0"
                  :max="9999"
                  style="width: 100%"
                  placeholder="排序值"
                  mode="button"
                />
              </a-form-item>
            </a-col>
            <a-col :span="12">
              <a-form-item field="status" label="分类状态">
                <a-switch type="round" v-model="form.status">
                  <template #checked>启用</template>
                  <template #unchecked>禁用</template>
                </a-switch>
              </a-form-item>
            </a-col>
          </a-row>
          <a-form-item field="description" label="分类描述">
            <a-textarea
              v-model="form.description"
              placeholder="请输入分类描述"
              allow-clear
              :auto-size="{ minRows: 3, maxRows: 5 }"
            />
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
.category-page {
  height: 100%;
  background: var(--color-bg-1);

  .page-wrapper {
    height: 100%;
    padding: 20px;
  }

  .left-box {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: 16px;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);

    .tree-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;
      padding-bottom: 12px;
      border-bottom: 1px solid var(--color-border-1);

      .tree-title {
        font-size: 16px;
        font-weight: 600;
        color: var(--color-text-1);
      }
    }

    .tree-search {
      margin-bottom: 16px;
    }

    .tree-box {
      flex: 1;
      overflow-y: auto;
    }
  }

  .right-box {
    flex: 1;
    height: 100%;
    padding: 16px;
    background: var(--color-bg-2);
    border-radius: 8px;
    border: 1px solid var(--color-border-1);
    overflow-y: auto;

    .category-detail {
      .detail-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;

        .header-left {
          flex: 1;

          .detail-title {
            font-size: 20px;
            font-weight: 600;
            color: var(--color-text-1);
            margin-bottom: 4px;
          }

          .detail-subtitle {
            font-size: 14px;
            color: var(--color-text-3);
          }
        }
      }

      .detail-content {
        .stat-cards {
          display: grid;
          grid-template-columns: repeat(3, 1fr);
          gap: 16px;
          margin-bottom: 24px;

          .stat-card {
            text-align: center;
            padding: 20px 12px;
            border-radius: 8px;
            border: 1px solid var(--color-border-1);
            background: var(--color-bg-3);

            .stat-value {
              font-size: 28px;
              font-weight: 600;
              color: var(--arcoblue-6);
              margin-bottom: 8px;
            }

            .stat-label {
              font-size: 13px;
              color: var(--color-text-2);
            }

            .status-tag {
              display: inline-flex;
              align-items: center;
              gap: 4px;
              font-weight: 500;
              font-size: 14px;
              margin-bottom: 8px;
            }
          }
        }

        .detail-table {
          :deep(.arco-descriptions-label) {
            background: var(--color-bg-3);
            font-weight: 500;
          }
        }

        .empty-text {
          color: var(--color-text-4);
          font-style: italic;
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

.modal-title {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 16px;
  font-weight: 600;
}

.modal-content {
  padding: 8px 0;
}
</style>

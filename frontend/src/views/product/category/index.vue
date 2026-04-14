<template>
  <div class="category-page">
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">产品分类管理</h2>
        <div class="page-desc">管理商品的分类体系，支持多级分类结构</div>
      </div>
      <div class="header-right">
        <a-space size="medium">
          <a-button type="primary" size="large" @click="onAdd(0)">
            <template #icon><icon-plus /></template>
            添加一级分类
          </a-button>
        </a-space>
      </div>
    </div>

    <div class="page-content">
      <div class="tree-panel">
        <div class="panel-header">
          <span class="panel-title">分类树</span>
          <a-space size="small">
            <a-tooltip content="展开全部">
              <div class="action-icon" @click="expandAll">
                <icon-plus-square />
              </div>
            </a-tooltip>
            <a-tooltip content="收起全部">
              <div class="action-icon" @click="collapseAll">
                <icon-minus-square />
              </div>
            </a-tooltip>
            <a-tooltip content="刷新">
              <div class="action-icon" @click="refresh">
                <icon-refresh />
              </div>
            </a-tooltip>
          </a-space>
        </div>
        <div class="tree-container">
          <a-tree
            ref="treeRef"
            :data="categoryTree"
            :field-names="{ key: 'id', title: 'name', children: 'children' }"
            :default-expand-all="true"
            :draggable="true"
            @node-drop="handleNodeDrop"
            @node-click="handleNodeClick"
          >
            <template #title="{ record }">
              <div class="tree-node">
                <div class="node-icon">
                  <icon-tag v-if="!record.icon" />
                  <component :is="record.icon" v-else />
                </div>
                <div class="node-name">{{ record.name }}</div>
                <a-tag :color="record.status === '0' ? 'green' : 'red'" size="small" class="node-status">
                  {{ record.status === '0' ? '启用' : '禁用' }}
                </a-tag>
              </div>
            </template>
          </a-tree>
        </div>
      </div>

      <div class="detail-panel">
        <div class="panel-header" v-if="currentNode">
          <span class="panel-title">分类详情</span>
          <a-space size="small">
            <a-tooltip content="添加子分类">
              <div class="action-icon" @click="onAdd(currentNode!.id)">
                <icon-plus />
              </div>
            </a-tooltip>
            <a-tooltip content="编辑分类">
              <div class="action-icon" @click="onEdit(currentNode!)">
                <icon-edit />
              </div>
            </a-tooltip>
            <a-popconfirm type="warning" content="确定删除该分类吗? 删除后子分类也将被删除" @ok="onDelete(currentNode!)">
              <a-tooltip content="删除分类">
                <div class="action-icon delete-icon">
                  <icon-delete />
                </div>
              </a-tooltip>
            </a-popconfirm>
          </a-space>
        </div>

        <div class="detail-content" v-if="currentNode">
          <a-descriptions :column="1" bordered size="large">
            <a-descriptions-item label="分类名称">
              <div class="detail-name">
                <div class="node-icon-lg">
                  <icon-tag v-if="!currentNode.icon" />
                  <component :is="currentNode.icon" v-else />
                </div>
                {{ currentNode.name }}
              </div>
            </a-descriptions-item>
            <a-descriptions-item label="分类层级">
              <a-tag :color="getLevelColor(currentNode.level)" size="large">
                {{ getLevelText(currentNode.level) }}
              </a-tag>
            </a-descriptions-item>
            <a-descriptions-item label="分类状态">
              <a-tag :color="currentNode.status === '0' ? 'green' : 'red'" size="large">
                {{ currentNode.status === '0' ? '启用' : '禁用' }}
              </a-tag>
            </a-descriptions-item>
            <a-descriptions-item label="排序">
              {{ currentNode.sort }}
            </a-descriptions-item>
            <a-descriptions-item label="分类描述">
              {{ currentNode.description || '-' }}
            </a-descriptions-item>
            <a-descriptions-item label="创建时间">
              {{ currentNode.createdAt }}
            </a-descriptions-item>
            <a-descriptions-item label="更新时间">
              {{ currentNode.updatedAt || '-' }}
            </a-descriptions-item>
          </a-descriptions>

          <div class="quick-actions">
            <a-space size="medium">
              <a-button type="primary" @click="onAdd(currentNode!.id)">
                <template #icon><icon-plus /></template>
                添加子分类
              </a-button>
              <a-button @click="onEdit(currentNode!)">
                <template #icon><icon-edit /></template>
                编辑分类
              </a-button>
            </a-space>
          </div>
        </div>

        <div class="empty-state" v-else>
          <div class="empty-icon">
            <icon-folder-open />
          </div>
          <div class="empty-text">请在左侧选择一个分类</div>
          <div class="empty-desc">查看分类详情信息或进行操作</div>
        </div>
      </div>
    </div>

    <a-modal
      v-model:visible="modalVisible"
      :title="modalTitle"
      :width="560"
      @ok="onSubmit"
      @cancel="onCancel"
      class="category-modal"
    >
      <a-form ref="formRef" :rules="rules" :model="form" layout="vertical">
        <a-form-item field="parentId" label="上级分类">
          <a-tree-select
            v-model="form.parentId"
            :data="categoryTree"
            :field-names="{ key: 'id', title: 'name', children: 'children' }"
            placeholder="请选择上级分类（不选则为一级分类）"
            allow-clear
            style="width: 100%"
            :dropdown-style="{ maxHeight: '400px', overflow: 'auto' }"
          />
        </a-form-item>
        <a-row :gutter="16">
          <a-col :span="16">
            <a-form-item field="name" label="分类名称" validate-trigger="blur">
              <a-input v-model="form.name" placeholder="请输入分类名称" allow-clear />
            </a-form-item>
          </a-col>
          <a-col :span="8">
            <a-form-item field="sort" label="排序" validate-trigger="blur">
              <a-input-number
                v-model="form.sort"
                :min="0"
                :max="9999"
                style="width: 100%"
                placeholder="排序"
                mode="button"
              />
            </a-form-item>
          </a-col>
        </a-row>
        <a-form-item field="icon" label="分类图标">
          <div class="icon-selector">
            <div class="selected-icon" v-if="form.icon">
              <component :is="form.icon" />
            </div>
            <a-input v-model="form.icon" placeholder="请输入图标名称" allow-clear style="flex: 1" />
          </div>
        </a-form-item>
        <a-form-item field="status" label="分类状态">
          <a-switch type="round" :checked-value="'0'" :unchecked-value="'1'" v-model="form.status">
            <template #checked>
              <icon-check /> 启用
            </template>
            <template #unchecked>
              <icon-close /> 禁用
            </template>
          </a-switch>
        </a-form-item>
        <a-form-item field="description" label="分类描述">
          <a-textarea v-model="form.description" placeholder="请输入分类描述（选填）" :rows="4" :max-length="200" show-word-limit />
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { categoryApi, CategoryListItem } from "@/api/modules/product/category";

const loading = ref(false);
const treeRef = ref();
const categoryTree = ref<any[]>([]);
const modalVisible = ref(false);
const modalTitle = ref("新增分类");
const formRef = ref();
const currentNode = ref<CategoryListItem | null>(null);

const form = reactive({
  id: 0,
  parentId: 0,
  name: "",
  icon: "",
  sort: 0,
  status: "0",
  description: ""
});

const rules = {
  name: [{ required: true, message: "请输入分类名称" }]
};

const getLevelColor = (level: number) => {
  const colors: Record<number, string> = { 1: "arcoblue", 2: "green", 3: "orange" };
  return colors[level] || "gray";
};

const getLevelText = (level: number) => {
  const texts: Record<number, string> = { 1: "一级分类", 2: "二级分类", 3: "三级分类" };
  return texts[level] || "未知层级";
};

const getTree = async () => {
  loading.value = true;
  try {
    const data = await categoryApi.tree();
    categoryTree.value = data || [];
  } catch (error) {
    console.error(error);
  } finally {
    loading.value = false;
  }
};

const refresh = () => {
  getTree();
};

const expandAll = () => {
  treeRef.value?.expandAll(true);
};

const collapseAll = () => {
  treeRef.value?.expandAll(false);
};

const handleNodeClick = (node: any) => {
  currentNode.value = node;
};

const handleNodeDrop = (dragNode: any, dropNode: any, dropPosition: number) => {
  arcoMessage("info", "分类位置已更新");
};

const findNodeById = (nodes: any[], id: number): any => {
  for (const node of nodes) {
    if (node.id === id) return node;
    if (node.children) {
      const found = findNodeById(node.children, id);
      if (found) return found;
    }
  }
  return null;
};

const onAdd = (parentId: number) => {
  modalTitle.value = parentId === 0 ? "新增一级分类" : "新增子分类";
  form.id = 0;
  form.parentId = parentId;
  form.name = "";
  form.icon = "";
  form.sort = 0;
  form.status = "0";
  form.description = "";
  modalVisible.value = true;
};

const onEdit = (record: CategoryListItem) => {
  modalTitle.value = "编辑分类";
  form.id = record.id;
  form.parentId = record.parentId;
  form.name = record.name;
  form.icon = record.icon || "";
  form.sort = record.sort;
  form.status = record.status;
  form.description = record.description || "";
  modalVisible.value = true;
};

const onDelete = async (record: CategoryListItem) => {
  try {
    await categoryApi.delete([record.id]);
    arcoMessage("success", "删除成功");
    if (currentNode.value?.id === record.id) {
      currentNode.value = null;
    }
    getTree();
  } catch (error) {
    console.error(error);
  }
};

const onSubmit = async () => {
  let state = await formRef.value.validate();
  if (state) return;
  try {
    if (form.id) {
      await categoryApi.edit(form);
      arcoMessage("success", "修改成功");
    } else {
      await categoryApi.add(form);
      arcoMessage("success", "新增成功");
    }
    modalVisible.value = false;
    getTree();
  } catch (error) {
    console.error(error);
  }
};

const onCancel = () => {
  modalVisible.value = false;
  formRef.value?.resetFields();
};

onMounted(() => {
  getTree();
});
</script>

<style scoped lang="scss">
.category-page {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 20px;
}

.header-left {
  .page-title {
    font-size: 24px;
    font-weight: 600;
    color: #1d2129;
    margin: 0 0 8px 0;
  }
  
  .page-desc {
    color: #86909c;
    font-size: 14px;
  }
}

.page-content {
  flex: 1;
  display: flex;
  gap: 20px;
  overflow: hidden;
}

.tree-panel,
.detail-panel {
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  display: flex;
  flex-direction: column;
}

.tree-panel {
  width: 360px;
  flex-shrink: 0;
}

.detail-panel {
  flex: 1;
  min-width: 0;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #e5e6eb;
  
  .panel-title {
    font-weight: 600;
    font-size: 16px;
    color: #1d2129;
  }
}

.action-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
  
  &:hover {
    background-color: var(--color-fill-2);
  }
  
  &.delete-icon:hover {
    background-color: #fde2e2;
    color: #f53f3f;
  }
}

.tree-container {
  flex: 1;
  padding: 16px;
  overflow: auto;
}

.tree-node {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 4px 0;
}

.node-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  border-radius: 4px;
  background: #f2f3f5;
  color: #165dff;
}

.node-name {
  flex: 1;
  font-size: 14px;
}

.node-status {
  flex-shrink: 0;
}

.detail-content {
  flex: 1;
  padding: 20px;
  overflow: auto;
}

.detail-name {
  display: flex;
  align-items: center;
  gap: 12px;
  font-size: 16px;
  font-weight: 500;
}

.node-icon-lg {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background: linear-gradient(135deg, #165dff, #4080ff);
  color: #ffffff;
  font-size: 20px;
}

.quick-actions {
  margin-top: 24px;
  padding-top: 24px;
  border-top: 1px solid #e5e6eb;
}

.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
}

.empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: #f2f3f5;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36px;
  color: #86909c;
  margin-bottom: 20px;
}

.empty-text {
  font-size: 16px;
  font-weight: 500;
  color: #1d2129;
  margin-bottom: 8px;
}

.empty-desc {
  font-size: 14px;
  color: #86909c;
}

.icon-selector {
  display: flex;
  align-items: center;
  gap: 12px;
}

.selected-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background: #f2f3f5;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  color: #165dff;
  flex-shrink: 0;
}

.category-modal {
  :deep(.arco-modal-body) {
    padding-top: 24px;
  }
}
</style>

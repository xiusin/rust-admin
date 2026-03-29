<template>
  <div class="file-tree">
    <a-input v-model="searchKeyword" placeholder="搜索文件" allow-clear style="margin-bottom: 12px">
      <template #prefix>
        <icon-search />
      </template>
    </a-input>

    <div class="tree-container">
      <a-tree
        :data="treeData"
        :selected-keys="selectedKeys"
        :expanded-keys="expandedKeys"
        :field-names="{ key: 'id', title: 'name', children: 'children' }"
        @select="handleSelect"
        @expand="handleExpand"
      >
        <template #title="node">
          <div class="tree-node" :class="{ 'is-active': activeFile === node.id }">
            <span class="node-icon">
              <icon-folder v-if="node.children && node.children.length" />
              <icon-file v-else />
            </span>
            <span class="node-name">{{ node.name }}</span>
            <span v-if="node.language" class="node-lang">{{ node.language }}</span>
          </div>
        </template>
      </a-tree>
    </div>

    <div class="tree-actions">
      <a-button long size="small" @click="handleExpandAll">
        <icon-unfold />
        展开全部
      </a-button>
      <a-button long size="small" @click="handleCollapseAll">
        <icon-fold />
        收起全部
      </a-button>
    </div>
  </div>
</template>

<script setup lang="ts">
interface CodeFile {
  id: string;
  name: string;
  path: string;
  language: string;
  content: string;
  children?: CodeFile[];
}

interface Props {
  files: CodeFile[];
  activeFile?: string;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  select: [file: CodeFile];
}>();

const searchKeyword = ref("");
const selectedKeys = ref<string[]>([]);
const expandedKeys = ref<string[]>([]);

const treeData = computed(() => {
  if (!searchKeyword.value) {
    return props.files;
  }

  const filterTree = (nodes: CodeFile[]): CodeFile[] => {
    return nodes.reduce((acc: CodeFile[], node) => {
      if (node.name.toLowerCase().includes(searchKeyword.value.toLowerCase())) {
        acc.push(node);
      } else if (node.children) {
        const filteredChildren = filterTree(node.children);
        if (filteredChildren.length > 0) {
          acc.push({ ...node, children: filteredChildren });
        }
      }
      return acc;
    }, []);
  };

  return filterTree(props.files);
});

watch(
  () => props.activeFile,
  val => {
    if (val) {
      selectedKeys.value = [val];
    }
  },
  { immediate: true }
);

const handleSelect = (keys: string[], { node }: { node: any }) => {
  selectedKeys.value = keys;
  if (!node.children) {
    emit("select", node);
  }
};

const handleExpand = (keys: string[]) => {
  expandedKeys.value = keys;
};

const handleExpandAll = () => {
  const getAllIds = (nodes: CodeFile[]): string[] => {
    return nodes.reduce((acc: string[], node) => {
      acc.push(node.id);
      if (node.children) {
        acc.push(...getAllIds(node.children));
      }
      return acc;
    }, []);
  };
  expandedKeys.value = getAllIds(props.files);
};

const handleCollapseAll = () => {
  expandedKeys.value = [];
};
</script>

<style lang="scss" scoped>
.file-tree {
  display: flex;
  flex-direction: column;
  height: 100%;

  .tree-container {
    flex: 1;
    overflow-y: auto;

    .tree-node {
      display: flex;
      gap: 6px;
      align-items: center;

      .node-icon {
        color: var(--color-text-3);
      }

      .node-name {
        flex: 1;
      }

      .node-lang {
        font-size: 10px;
        color: var(--color-text-3);
      }

      &.is-active {
        color: rgb(var(--primary-6));
      }
    }
  }

  .tree-actions {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-top: 12px;
  }
}
</style>

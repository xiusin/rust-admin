<template>
  <a-tree
    ref="treeRef"
    :data="data"
    :selected-keys="selectedKeys"
    :field-names="{ key: 'id', title: 'name', children: 'children' }"
    show-line
    draggable
    block-node
    @select="onSelect"
    @drop="onDrop"
  >
    <template #extra="node">
      <a-space class="tree-node-actions" @click.stop>
        <a-button type="text" size="mini" @click="emit('add', node.id)">
          <template #icon><icon-plus /></template>
        </a-button>
        <a-button type="text" size="mini" @click="emit('edit', node)">
          <template #icon><icon-edit /></template>
        </a-button>
        <a-button type="text" size="mini" status="danger" @click="emit('delete', node)">
          <template #icon><icon-delete /></template>
        </a-button>
      </a-space>
    </template>
  </a-tree>
</template>

<script setup lang="ts">
import { ref } from "vue";
import type { CmsCategoryTree } from "@/api/modules/cms/category";

interface Props {
  data: CmsCategoryTree[];
  selectedKeys: number[];
}

defineProps<Props>();

const emit = defineEmits<{
  select: [keys: number[]];
  add: [parentId: number | null];
  edit: [record: CmsCategoryTree];
  delete: [record: CmsCategoryTree];
  move: [id: number, parentId: number | null];
}>();

const treeRef = ref();

const expandAll = () => {
  treeRef.value?.expandAll();
};

const onSelect = (keys: (string | number)[]) => {
  emit("select", keys as number[]);
};

const onDrop = (info: { dragNode: any; dropNode: any; dropPosition: number }) => {
  const { dragNode, dropNode, dropPosition } = info;
  const dragId = dragNode.id;
  let targetParentId: number | null = null;

  if (dropPosition === 0) {
    targetParentId = dropNode.id;
  } else {
    targetParentId = dropNode.parentId || null;
  }

  emit("move", dragId, targetParentId);
};

defineExpose({
  expandAll
});
</script>

<style scoped lang="scss">
.tree-node-actions {
  opacity: 0;
  transition: opacity 0.2s;
}

:deep(.arco-tree-node:hover) {
  .tree-node-actions {
    opacity: 1;
  }
}
</style>

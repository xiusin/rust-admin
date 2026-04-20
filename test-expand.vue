<template>
  <a-table ref="tableRef" :data="data" row-key="id" />
</template>
<script setup>
import { ref, onMounted } from 'vue';
const tableRef = ref();
const data = ref([{ id: 1, children: [{ id: 2 }] }]);
onMounted(() => {
  console.log(tableRef.value.expand);
});
</script>

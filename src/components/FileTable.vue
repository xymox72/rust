<!-- src/components/FileTable.vue -->
<template>
    <DemoGrid :filter-key="searchQuery" :columns="columns" :data="data" class="mt-4" />
</template>

<script setup lang="ts">
import { defineProps } from "vue";
import DemoGrid from './Table.vue';

defineProps({
    columns: Array,
    data: Array,
    searchQuery: String,
});
</script>
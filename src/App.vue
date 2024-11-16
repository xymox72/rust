<!-- src/App.vue -->
<template>
  <div v-if="fileManager.isLoading" class="container mx-auto p-4">
    <EnvsView class="mb-4" />
    <StepLabel :step="currentStep" />

    <DateSelector @step-changed="updateStep" @dateSelected="handleDateSelection" class="mb-4" />

    <div class="text-gray-700 mb-4">Загруженные файлы - {{ countFails.length }}</div>

    <FileActions :countData="fileManager.countData.value" :countFails="fileManager.countFails.value.length"
      @removeFiles="fileManager.removeFiles" @showFiles="fileManager.getMessages" @toggleFails="fileManager.toggleFails"
      class="mb-4" />

    <div class="font-bold mb-4">SUMMARY: {{ fileManager.countData }}</div>

    <FileTable :filter-key="searchQuery" :columns="gridColumns" :data="fileManager.gridData.value" />

    <div v-if="isShowInfoFails" class="mt-4 space-y-2">
      <div v-for="mes in countFails" :key="mes" class="text-red-500">
        FAILS - {{ mes }}
      </div>
    </div>
  </div>
  <Loader v-else />
</template>

<script setup lang="ts">
import { Ref, computed, onMounted, ref } from "vue";
import StepLabel from "./components/StepLabel.vue";
import EnvsView from "./compositions/Envs.vue";
import DateSelector from "./components/DateSelector.vue";
import FileActions from "./components/FileActions.vue";
import FileTable from "./components/FileTable.vue";
import Loader from "./components/Loader.vue";

import { listen } from '@tauri-apps/api/event';
import { useStepTracker } from "./compositions/useStepTracker";
import { useFileManager } from "./compositions/useFileManager";

const { currentStep, updateStep } = useStepTracker();
const fileManager = useFileManager();
const gridColumns = ["File Name", "Created time"];
const searchQuery = ref('');
const countFails = ref<Array<string>>([]);
const isShowInfoFails = ref(false);


// Обработка выбранной даты или вычисленной по дням
const handleDateSelection = async (days: number) => {
  await fileManager.getCount(days)

  updateStep(1);
};









onMounted(() => {
  listen<string>('file', (event) => {
    countFails.value.push(event.payload);
  });
});
</script>

<style lang="css" scoped>
.container {
  display: flex;
  flex-direction: column;
  align-items: center;
}
</style>
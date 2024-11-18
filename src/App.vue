<!-- src/App.vue -->
<template>
  <div v-if="fileManager.isLoading" class="container mx-auto p-4">
    <EnvsView class="mb-4" />
    <StepLabel :step="currentStep" />

    <DateSelector @reset="resetHandler" ref="dateSelectorRef" @step-changed="updateStep" @dateSelected="handleDateSelection" class="mb-4" />

    <FileActions

      :is-disabled="!fileManager.countData.value"
      @removeFiles="fileManager.removeFiles"
      @reset="resetHandler"
      @showFiles="fileManager.getMessages" @toggleFails="fileManager.toggleFails"
      class="mb-4" />



    <FileTable :count-data="fileManager.countData.value" :filter-key="searchQuery" :columns="gridColumns" :data="fileManager.gridData.value" />

    <div v-if="isShowInfoFails" class="mt-4 space-y-2">
      <div v-for="mes in countFails" :key="mes" class="text-red-500">
        FAILS - {{ mes }}
      </div>
    </div>
  </div>
  <Loader v-else />
</template>

<script setup lang="ts">
import {  onMounted, ref } from "vue";
import StepLabel from "./components/StepLabel.vue";
import EnvsView from "./compositions/Envs.vue";
import DateSelector from "./components/DateSelector.vue";
import FileActions from "./components/FileActions.vue";
import FileTable from "./components/FileTable.vue";
import Loader from "./components/Loader.vue";

import { listen } from '@tauri-apps/api/event';
import { useStepTracker } from "./compositions/useStepTracker";
import { useFileManager } from "./compositions/useFileManager";
import eventBus from "./eventBus";


const { currentStep, updateStep } = useStepTracker();
const fileManager = useFileManager();
const gridColumns = ["File Name", "Created time"];
const searchQuery = ref('');
const countFails = ref<Array<string>>([]);
const isShowInfoFails = ref(false);
const dateSelectorRef = ref(null);


// Обработка выбранной даты или вычисленной по дням
const handleDateSelection = async (days: number) => {
  await fileManager.getMessages(days);

  updateStep(3);
};



const resetHandler = () => {
  fileManager.reset();

   
  eventBus.emit("resetDateSelector");
  updateStep(0);

}

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
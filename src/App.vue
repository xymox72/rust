<!-- src/App.vue -->
<template>
 <div v-if="!isLoading" class="container mx-auto p-4">
    <EnvsView class="mb-4"/>
    <StepLabel :step="currentStep" />

    <DateSelector @step-changed="updateStep" @dateSelected="handleDateSelection" class="mb-4" />

    <div class="text-gray-700 mb-4">Загруженные файлы - {{ countFails.length }}</div>

    <FileActions
      :countData="countData"
      :countFails="countFails.length"
      @removeFiles="removeFiles"
      @showFiles="getMessages"
      @toggleFails="toggleFails"
      class="mb-4"
    />

    <div class="font-bold mb-4">SUMMARY: {{ countData }}</div>

    <FileTable :filter-key="searchQuery" :columns="gridColumns" :data="gridData" />

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
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { useStepTracker } from "./compositions/useStepTracker";

const { currentStep, updateStep } = useStepTracker();

const gridColumns = ["File Name", "Created time"];
const searchQuery = ref('');
const data: Ref<Array<any>> = ref([]);
const count = ref(0);
const isLoading = ref(false);
const countFails = ref<Array<string>>([]);
const isShowInfoFails = ref(false);
const selectedDate = ref<string | null>(null);

// Обработка выбранной даты или вычисленной по дням
const handleDateSelection = (date: string) => {
  selectedDate.value = date;
  getCount(date);
  updateStep(1);
};

// Получение сообщений
async function getMessages() {
  isLoading.value = true;
  try {
    const [mes, coun] = await Promise.all([invoke("get_meesages", { selectedDate: selectedDate.value }), invoke("count", { selectedDate: selectedDate.value })]);
    data.value = mes as any;
    count.value = coun as any;
  } catch (err) {
    console.log(err);
  } finally {
    isLoading.value = false;
  }
}

// Удаление файлов
async function removeFiles() {
  countFails.value = [];
  await invoke("remove_files", { selectedDate: selectedDate.value });
  alert("Файлы удалены");
  reset();
}

const gridData = computed(() => data.value.map((e) => ({ "File Name": e.messagefilename, "Created time": e.messagecreatetime })));
const countData = computed(() => count.value);

const toggleFails = () => {
  isShowInfoFails.value = !isShowInfoFails.value;
}

const reset = () => {
  isLoading.value = false;
  data.value = [];
  count.value = 0;
  countFails.value = [];
  isShowInfoFails.value = false;
};

const getCount = async (date: string) => {
  isLoading.value = true;
  count.value = await invoke("count", { selectedDate: date });
  data.value = [];
  isLoading.value = false;
};

onMounted(() => {
  listen<string>('file', (event) => {
    countFails.value.push(event.payload);
  });
});
</script>

<style lang="css" scoped>
  .container{
    display: flex;
    flex-direction: column;
    align-items: center;
  }

</style>
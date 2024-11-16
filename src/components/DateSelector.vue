<template>

  <div class="flex gap-4 items-center">

    <VueDatePicker v-model="calendarDate" :enable-time-picker="false" :max-date="new Date()" class="border rounded p-2"
      @cleared="clearDaysAgo" />

    <input type="number" placeholder="Выберите дней" v-model="daysAgo" class="border rounded p-2" />

    <button @click="emitDaysAgo" :disabled="isNullDays" class="bg-blue-500 w-full text-white font-semibold  rounded p-2"
      :class="{ 'opacity-50 cursor-not-allowed': isNullDays }">
      Начать поиск
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, defineEmits, watch, inject, onMounted, onBeforeUnmount } from "vue";
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css';
import { subDays, differenceInCalendarDays } from "date-fns";
import eventBus from "../eventBus";


const daysAgo = ref<number | null>(null);
const emit = defineEmits(["dateSelected", "stepChanged"]);
watch(daysAgo, (newVal) => {
  if (newVal !== null && newVal <= 0) {
    daysAgo.value = null;
  }
  emit("stepChanged", daysAgo.value === null ? 1 : 2);
});

onMounted(() => {
  eventBus.on("resetDateSelector", clearDaysAgo);
});

onBeforeUnmount(() => {
  eventBus.off("resetDateSelector", clearDaysAgo);
});


const isNullDays = computed(() => daysAgo.value === null);

const calendarDate = computed({
  get() {

    if (daysAgo.value !== null) {
      return subDays(new Date(), daysAgo.value);
    }
    return null;
  },
  set(value) {

    if (value) {
      daysAgo.value = differenceInCalendarDays(new Date(), new Date(value));
    }
  },
});

function clearDaysAgo() {
  console.log("dsada");
  daysAgo.value = null;
  //props.onReset && props.onReset();
}




function emitDaysAgo() {
  emit("dateSelected", daysAgo.value);
}
</script>

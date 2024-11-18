<!-- src/components/StepLabel.vue -->
<template>
    <div class="text-lg font-semibold text-gray-700 text-center mb-4">
      {{ stepMessage }}
    </div>
  </template>
  
  <script setup lang="ts">
  import { computed, defineProps } from "vue";
  
  const props = defineProps<{
    step: number;
  }>();
  
  // Определяем сообщения для каждого шага
  const stepMessage = computed(() => {
    switch (props.step) {
      default:
      case 1:
        return "Шаг 1: Выберите дату или введите количество дней назад";
      case 2:
        return "Шаг 2: Нажмите на кнопку Начать поиск, чтобы найти файлы";
      case 3:
        return "Шаг 3:"
      


    }
  });
  </script>
  
  <style scoped>
  .text-center {
    text-align: center;
  }
  .mb-4 {
    margin-bottom: 1rem;
  }
  </style>
  
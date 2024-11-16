// src/composables/useStepTracker.ts
import { ref } from "vue";

export function useStepTracker() {
  const currentStep = ref(1);

  // Функция для обновления шага
  function updateStep(step: number) {
    currentStep.value = step;
  }

  return {
    currentStep,
    updateStep,
  };
}

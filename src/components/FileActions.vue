<!-- src/components/FileActions.vue -->
<template>
  <div class="flex gap-4 mt-4">
    <button class="bg-red-500 text-white font-semibold py-2 px-4 rounded"  :class="{ 'opacity-50 cursor-not-allowed': isDisabled }" :disabled="isDisabled"
      @click="$emit('removeFiles')" type="submit">
      Удалить файлы
    </button>

    <button  class="bg-green-500 text-white font-semibold py-2 px-4 rounded"     :class="{ 'opacity-50 cursor-not-allowed': isDisabled }"  :disabled="isDisabled"
      @click="$emit('reset')" type="submit">
      Сброс состояния
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
const emit = defineEmits(["removeFiles", "reset"]);
defineProps({
  isDisabled: Boolean

});
</script>

<style scoped>
.file-actions {
  display: flex;
  gap: 10px;
}
</style>
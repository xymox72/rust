<!-- src/components/FileActions.vue -->
<template>
  <div class="flex gap-4 mt-4">
    <button class="bg-red-500 text-white font-semibold py-2 px-4 rounded" :disabled="!countData"
      @click="$emit('removeFiles')" type="submit">
      Remove All files
    </button>
    <button class="bg-green-500 text-white font-semibold py-2 px-4 rounded" :disabled="!countData"
      @click="$emit('showFiles')" type="submit">
      Show table of files
    </button>
    <button class="bg-red-300 text-white font-semibold py-2 px-4 rounded" @click="$emit('toggleFails')">
      FAILS - {{ countFails }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "vue";

defineProps({
  countData: Number,
  countFails: Number,
});
</script>

<style scoped>
.file-actions {
  display: flex;
  gap: 10px;
}
</style>
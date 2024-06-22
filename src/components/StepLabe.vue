<script setup lang="ts">
import {  computed } from "vue";
const props = defineProps({
  step: { type: Number, required: true },
});

const label = computed(() => {
    switch(props.step){
        case 1: 
            return "Выберите дату";
        case 2:
            return "Можете удалить файлы"
    }
});

</script>

<template>
    <div class="label">{{ label }} </div>
</template>

<style>
 .label{
    font-size: 30px;
 }
</style>
<script setup lang="ts">

import { invoke } from "@tauri-apps/api/tauri";
import { computed, onMounted, ref } from "vue";

interface EnvVars {
  [key: string]: string;
}
const envVars = ref<EnvVars | null>(null);

const fetchEnvVars = async () => {
    try {
      const vars: EnvVars = await invoke('get_envs');
      console.log(vars);
      envVars.value = vars;
    } catch (err) {
     console.error("err",err);
    } 
  };

onMounted(() => {
  fetchEnvVars();
});

const hasEnvVars = computed(() => {
      return envVars.value !== null && Object.keys(envVars.value).length > 0;
    });


</script>

<template>
    <div v-if="hasEnvVars" class="container">

    <div class="row">
        Выбранная папка: {{envVars!["WORKING_DIRECTORY"]}}
    </div>
</div>
</template>

<style>
.container, .row{
    display: flex;
    justify-content: center;
}
</style>
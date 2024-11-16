<script setup lang="ts">

import { invoke } from "@tauri-apps/api/tauri";
import { computed, onMounted, ref } from "vue";

interface EnvVars {
  [key: string]: string;
}
const envVars = ref<EnvVars | null>(null);

const listEnvs = ["WORKING_DIRECTORY", "DATABASE_URL"];

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

const humanEnvVars = computed(() => {
  if (envVars.value){
    const value = envVars.value;
    return Object.keys(value).filter((e) => listEnvs.includes(e)).map((key) => `${key}: ${value[key]}`).join("\n")
  }

  return "envs is not definded";
})


const showEnvs = () =>{

  alert(humanEnvVars);
};



const hasEnvVars = computed(() => {
      return envVars.value !== null && Object.keys(envVars.value).length > 0;
    });


</script>

<template>
    <div v-if="hasEnvVars" class="container">
    {{ humanEnvVars }}
    <button @click="showEnvs">Показать envs</button>
</div>
</template>

<style>
.container, .row{
    display: flex;
    justify-content: center;
}
</style>
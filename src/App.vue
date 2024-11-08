<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { Ref, computed, onMounted, ref } from "vue";
import StepLabe from "./components/StepLabe.vue";
import DemoGrid from './components/Table.vue';
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { IMessage } from "./models/Message";
import EnvsView from "./compositions/Envs.vue";
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css';
import { format } from "date-fns";
import Loader from "./components/Loader.vue";

const gridColumns = ["File Name", "Created time"];
const searchQuery = ref('')



const data: Ref<Array<IMessage>> = ref([]);
const date = ref<string | null>(null);
const count = ref(0);
const isLoading = ref(false);


  const countFails = ref<Array<string>>([]);
async function getMessages() {
  const selectedDate = formatedDate.value;
  isLoading.value = true;
  try {
    const [mes, coun] = await Promise.all([invoke("get_meesages", { selectedDate }), invoke("count", { selectedDate })]);

    data.value = mes as any;
    count.value = coun as any;
  }
  catch (err) {
    console.log(err);
  }
  finally {
    isLoading.value = false;
  }
}


async function removeFiles() {
  countFails.value = [];
  await invoke("remove_files", {selectedDate: formatedDate.value});
  alert("Файлы удалены");

  reset();

}

const gridData = computed(() =>
  data.value.map((e) => ({ "File Name": e.messagefilename, "Created time": e.messagecreatetime }))
);

const countData = computed(() => {
  return count.value;
});



const formatedDate = computed(() => {
  if (date.value !== null) {
    return format(date.value, "dd.MM.yyyy");
  }
  return "";
});

const getCount = async (newDate: any) => {
  date.value = newDate;
  isLoading.value = true;
  const selectedDate = formatedDate.value;

  count.value = await invoke("count", { selectedDate });

  data.value = [];

  isLoading.value = false;
};

const step = computed<number>(() => {
  
  return !date.value ? 1 : 2;
});

onMounted(() => {
  listen<string>('file', (event) => {
       console.log("event", event.payload);
       countFails.value.push(event.payload);
      });
});

const isShowInfoFails = ref<boolean>(false);

const showInfo = () => {
  isShowInfoFails.value = !isShowInfoFails.value;
}
const reset = () => {
  isLoading.value = false;
  data.value = [];
  count.value = 0;
  countFails.value = [];
  isShowInfoFails.value = false; 
}
</script>

<template>
  <div v-if="!isLoading" class="container">
    <StepLabe :step="step"/>
    <EnvsView/>
 
    <VueDatePicker :enable-time-picker="false" @cleared="reset" @update:model-value="getCount" v-model="date" />
    <div>
      Загруженные файлы - {{ countFails.length }}
    </div>
    <div v-if="countData" class="buttons">
      <button class="red button" :disabled="!countData" @click="removeFiles" type="submit">Remove All files</button>
      <button  class="green button"  :disabled="!countData" @click="getMessages" type="submit">Show table of files</button>
      <button  class="red button" @click="showInfo" >
      FAILS - {{ countFails.length }}
    </button>
    </div>

    <div>SUMMARY: {{ countData }}</div>
    <DemoGrid :filter-key="searchQuery" :columns="gridColumns" :data="gridData" />
   
    <div v-if="isShowInfoFails" v-for="mes in countFails">
      FAILS - {{ mes }}
    </div>
  </div>
   <Loader v-else/>   

</template>

<style scoped>
.buttons {
  display: flex;
  align-content: center;
  justify-content: center;
  gap: 10px;
}

.button{
  color: white;
}

.red{
  background-color: red;
}

.green{
  background-color: #42b983;
}

.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

:root {
  font-family:  Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
  gap: 10px;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>Ref,

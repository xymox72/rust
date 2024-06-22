<script setup ts>
import {
  VueSpinner
} from 'vue3-spinners';


</script>

<template>
    <div class="loader">
      Подождите
     <VueSpinner size="40"/>
    </div>
</template>

<style>
.loader{
    display: flex;
    justify-content: center;
    align-items: center;
    height: 90vh;
    flex-direction: column;
    gap: 10px;
}
</style>
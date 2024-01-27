<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { RegisteredQuestionnaire } from "./models";

const registered_questionnaires = ref<RegisteredQuestionnaire>();

async function get_questionnaires() {
  registered_questionnaires.value = await invoke("get_registered_questionnaires");
  console.log(registered_questionnaires)
}

onMounted(() => {
  get_questionnaires();
})
</script>

<template>
  <header>
    <h1>
    Cuestionektor
    </h1>
  </header>
  <nav>
    <ul>Nuevo cuestionario</ul>
    <ul>Ordenar por</ul>
  </nav>
  <div>
    {{ registered_questionnaires }}
  </div>
  <Greet></Greet>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
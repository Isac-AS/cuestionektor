<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "../components/Greet.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { RegisteredQuestionnaire } from "../models";

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
    <div class="page column-nowrap">
        <header>
            <h1>
                Cuestionarios
            </h1>
        </header>
        <nav>
            <ul>Nuevo cuestionario</ul>
            <ul>Ordenar por s</ul>
        </nav>
        <div>
            {{ registered_questionnaires }}
        </div>
        <Greet></Greet>
    </div>
</template>

<style scoped>
</style>
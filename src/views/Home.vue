<script setup lang="ts">
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "../components/Greet.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { RegisteredQuestionnaire } from "../models";

const registered_questionnaires = ref<RegisteredQuestionnaire>();

async function get_questionnaires() {
    registered_questionnaires.value = await invoke("get_registered_questionnaires");
    console.log(registered_questionnaires.element)
}

onMounted(() => {
    get_questionnaires();
})
</script>

<template>
    <div id="home_page">
        <header>
            <h1>
                Cuestionektor
            </h1>
        </header>
        <div>
            {{ registered_questionnaires }}
        </div>
        <Greet></Greet>
    </div>
</template>

<style scoped>
#home_page {
    width: 100%;
    padding: 2rem;
    display: flex;
    flex-flow: column nowrap;
    align-items: center;
}
</style>
<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { RegisteredQuestionnaire, RegisteredQuestionnaires, OperationResultStruct, OperationResult } from "../models";
import icons from '../assets/icons/'

const registered_questionnaires = ref<RegisteredQuestionnaire[]>();
const no_registered_questionnaires = ref<boolean>(false);

async function get_questionnaires() {
    let questionnaires_attempt = await invoke<OperationResultStruct<RegisteredQuestionnaires>>("get_registered_questionnaires");
    if (questionnaires_attempt.result == OperationResult.Fail) {
        alert("Error fetching questionnaires");
        return;
    }
    registered_questionnaires.value = questionnaires_attempt.element.questionnaires;
    if (registered_questionnaires.value.length <= 0) {
        no_registered_questionnaires.value = true;
    }
}

onMounted(() => {
    get_questionnaires();
})
</script>

<template>
    <div id="home_page" class="column-nowrap">
        <header>
            <h1 class="text-3xl font-bold">
                Cuestionektor
            </h1>
        </header>
        <div v-if="no_registered_questionnaires" class="column-nowrap jcc aic gap">
            <h2>No hay cuestionarios registrados</h2>
            <div class="row-nowrap no-questionnaire-card-container">
                <div class="column-nowrap no-questionnaire-card">
                    <h3>Crea un cuestionario a mano</h3>
                    Para cuestionarios pequeños
                    <router-link to="/create" class="button">
                        <img :src="icons.add" class="icon">
                        <span class="text">Crear un cuesitonario</span>
                    </router-link>
                </div>
                <div class="column-nowrap no-questionnaire-card">
                    <h3>Crear a partir de un fichero</h3>
                    Sube un fichero
                    <router-link to="/create-from-file" class="button">
                        <img :src="icons.file" class="icon">
                        <span class="text">Subir un fichero</span>
                    </router-link>
                </div>
            </div>
        </div>
        <div v-else>
            <h1 class="bg-primary">
                cuestionarios
            </h1>
            <div class="flex">

                <button class="rounded bg-primary">Some example button</button>
            </div>
        </div>
    </div>
</template>

<style scoped>
#home_page {
    width: 100%;
    padding: 2rem;
    align-items: center;
    gap: 3em;
}

.no-questionnaire-card-container {
    gap: 10em;
}

.no-questionnaire-card {
    padding: 2em;
    box-shadow: 0 5px 5px rgba(0, 0, 0, 0.2);
    gap: 2em;
    align-items: center;
}
</style>
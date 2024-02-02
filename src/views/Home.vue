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
    <div class="flex flex-col items-center w-full">
        <h1 class="text-5xl font-bold mt-5">
            Cuestionektor
        </h1>
        <div v-if="no_registered_questionnaires" class="flex flex-col items-center mt-7 gap-10">
            <h2 class="text-2xl font-semibold">No hay cuestionarios registrados</h2>
            <div class="grid grid-cols-2 gap-10">
                <div class="bg-surface-dp6 rounded p-2 shadow-lg">
                    <h3 class="text-xl">Crea un cuestionario a mano</h3>
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

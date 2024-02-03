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
            <h2
                class="text-2xl font-semibold p-3 dark:bg-error bg-wm-error rounded shadow-md dark:text-OnError text-wm-OnError">
                No hay cuestionarios registrados
            </h2>
            <div class="grid grid-cols-2 gap-10">
                <div class="bg-surface-dp6 rounded p-3 shadow-lg flex flex-col gap-3">
                    <h3 class="text-xl font-bold">Crea un cuestionario a mano</h3>
                    <p>Para cuestionarios pequeños</p>
                    <router-link to="/create" class="btn-primary">
                        <img :src="icons.add" class="mr-2 w-6">
                        <span class="font-semibold">Crear un cuesitonario</span>
                    </router-link>
                </div>
                <div class="bg-surface-dp6 rounded p-3 shadow-lg flex flex-col gap-3">
                    <h3 class="text-xl font-bold">Crear a partir de un fichero</h3>
                    <p>Sube un fichero</p>
                    <router-link to="/create-from-file" class="btn-primary">
                        <img :src="icons.file" class="mr-2 w-6">
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

                <button class="rounded bg-primary">some example button</button>
            </div>
        </div>
    </div>
</template>

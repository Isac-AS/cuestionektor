<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { inject, onMounted, ref } from "vue";
import { RegisteredQuestionnaire, RegisteredQuestionnaires, OperationResultStruct, OperationResult } from "../models";
import icons from '../assets/icons/'
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
import { CreateNotification } from "../services/notifications.service";

const createNotification = <CreateNotification>inject('create-notification');
const registered_questionnaires = ref<RegisteredQuestionnaire[]>();
const no_registered_questionnaires = ref<boolean>(false);

async function get_questionnaires() {
    let questionnaires_attempt = await invoke<OperationResultStruct<RegisteredQuestionnaires>>("get_registered_questionnaires");
    if (questionnaires_attempt.result == OperationResult.Fail) {
        createNotification({
            type: 'Error',
            message: 'No se pudieron leer los cuestionarios',
            title: 'Error',
            duration: 3,
        })
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
        <div v-if="no_registered_questionnaires">
            <NoQuestionnaires />
        </div>
        <div v-else class="w-full flex flex-col items-center">
            <h1 class="text-3xl my-7 font-bold">Cuestionarios registrados</h1>
            <div class="w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 p-3">
                <div v-for="questionnaire in registered_questionnaires"
                    class="bg-surface-dp12 rounded shadow-md p-3 flex flex-col">
                    <div>

                    </div>
                    {{ questionnaire.name }}
                </div>
            </div>
        </div>
    </div>
</template>

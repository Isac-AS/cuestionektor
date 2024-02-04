<script setup lang="ts">
import { inject, onMounted, ref } from "vue";

// import icons from '../assets/icons/'
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
import { CreateNotification } from "../services/notifications.service";
import { BackendResponse, OperationResult } from "../models/view-models";
import { Questionnaire } from "../models/questionnaire";
import { getQuestionnaires } from "../services/questionnaire.service";

const createNotification = <CreateNotification>inject('create-notification');
const registeredQuestionnaires = ref<Questionnaire[]>();
const noRegisteredQuestionnaires = ref<boolean>(false);

function handleResponse(response: BackendResponse<Questionnaire[]>) {
    if (response.result == OperationResult.Fail) {
        createNotification({
            type: 'Error',
            message: 'No se pudieron leer los cuestionarios',
            title: 'Error',
            duration: 3,
        })
        return;
    }
}

async function loadQuestionnaires() {
    let questionnaireResponse = await getQuestionnaires();
    handleResponse(questionnaireResponse);
    registeredQuestionnaires.value = questionnaireResponse.data;
    if (registeredQuestionnaires.value.length <= 0) {
        noRegisteredQuestionnaires.value = true;
    }
}

onMounted(() => {
    loadQuestionnaires()
})
</script>

<template>
    <div class="flex flex-col items-center w-full">
        <h1 class="text-5xl font-bold mt-5">
            Cuestionektor
        </h1>
        <div v-if="noRegisteredQuestionnaires">
            <NoQuestionnaires />
        </div>
        <div v-else class="w-full flex flex-col items-center">
            <h1 class="text-3xl my-7 font-bold">Cuestionarios registrados</h1>
            <div class="w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 p-3">
                <div v-for="questionnaire in registeredQuestionnaires"
                    class="bg-surface-dp12 rounded shadow-md p-3 flex flex-col">
                    <div>

                    </div>
                    {{ questionnaire.name }}
                </div>
            </div>
        </div>
    </div>
</template>

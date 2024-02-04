<script setup lang="ts">
import { inject, onMounted, ref } from "vue";
import icons from '../assets/icons/'
import { Questionnaire } from "../models/questionnaire";
import { BackendResponse, OperationResult } from "../models/view-models";
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
import { CreateNotification } from "../services/notifications.service";
import { updateQuestionnaireName, deleteQuestionnaire, getQuestionnaires } from "../services/questionnaire.service"

const createNotification = <CreateNotification>inject('create-notification');
const registeredQuestionnaires = ref<Questionnaire[]>();
const noRegisteredQuestionnaires = ref<boolean>(false);
const editing_name = ref<boolean>(false);
const newName = ref('');
const editIndex = ref(-1);

function informAboutResult(result: OperationResult, successMessage: string, errorMessage: string): boolean {
    let returnValue = result == OperationResult.Success;
    createNotification({
        type: returnValue ? 'success' : 'error',
        title: returnValue ? 'Exito: ' : 'Error: ',
        message: returnValue ? successMessage : errorMessage,
        duration: 3
    });
    return returnValue;
}
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

async function changeName(id: number) {
    await updateQuestionnaireName(id, newName.value).then(
        (modificationResult) => {
            informAboutResult(
                modificationResult.result,
                "Nombre modificado con exito.",
                "Error al cambiar el nombre.",
            );
            editing_name.value = false;
            editIndex.value = -1;
        }
    );
}

async function deleteQuestionnaireCompletely(id: number) {
    await deleteQuestionnaire(id).then(
        (modificationResult) => {
            informAboutResult(
                modificationResult.result,
                "Cuestionario eliminado con exito.",
                "Error al eliminar el cuestionario.",
            );
        }
    );
}

onMounted(() => {
    loadQuestionnaires();
})
</script>

<template>
    <div class="flex flex-col items-center w-full">
        <div v-if="noRegisteredQuestionnaires">
            <NoQuestionnaires />
        </div>
        <div v-else class="w-full flex flex-col items-center">
            <h1 class="text-3xl my-7 font-bold">Cuestionarios registrados</h1>
            <div class="w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5 p-3">
                <div v-for="(questionnaire, index) in registeredQuestionnaires"
                    class="bg-surface-dp6 rounded shadow-md p-3 grid grid-cols-5 gap-2">
                    <div class="col-span-2">
                        Nombre
                    </div>
                    <div class="col-span-3 flex items-center justify-between bg-surface-dp24 p-1 rounded shadow-md">
                        <div>
                            <div v-if="!editing_name || editIndex != index" class="ml-2">
                                {{ questionnaire.name }}
                            </div>
                            <div v-else>
                                <input type="text" :placeholder="questionnaire.name.toString()" v-model="newName"
                                    class="my-2 px-2 py-1 text-OnPrimary bg-white border shadow-sm border-slate-300 placeholder-slate-500 focus:outline-none focus:border-sky-500 focus:ring-sky-500 block w-full rounded-md sm:text-sm focus:ring-1">
                            </div>
                        </div>
                        <div class="flex">
                            <div v-if="!editing_name || editIndex != index" class="flex">
                                <button
                                    @click="editing_name = true; editIndex = index; newName = questionnaire.name.toString()"
                                    class="p-1 mx-1 dark:hover:bg-primary/50 rounded">
                                    <img :src="icons.edit" class="w-6 dark:invert">
                                </button>
                                <button class="p-1 mx-1 dark:hover:bg-primary/50 rounded"
                                    @click="deleteQuestionnaireCompletely(questionnaire.id)">
                                    <img :src="icons.delete_icon" class="w-6 dark:invert">
                                </button>
                            </div>
                            <div v-else class="flex">
                                <button :disabled="newName.length == 0" @click="changeName(questionnaire.id)"
                                    class="p-1 mx-1 dark:hover:bg-primary/50 rounded disabled:cursor-not-allowed">
                                    <img :src="icons.done" class="w-6 dark:invert">
                                </button>
                                <button @click="newName = ''; editing_name = false; editIndex = -1;"
                                    class="p-1 mx-1 dark:hover:bg-primary/50 rounded">
                                    <img :src="icons.close" class="w-6 dark:invert">
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="col-span-2">
                        Abierto por ultima vez
                    </div>
                    <div class="col-span-3 bg-surface-dp24 p-1 rounded shadow-md">
                        <div class="ml-2">
                            {{ questionnaire.last_accessed == '0' ? 'Nunca' : questionnaire.last_accessed }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>
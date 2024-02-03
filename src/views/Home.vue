<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { RegisteredQuestionnaire, RegisteredQuestionnaires, OperationResultStruct, OperationResult } from "../models";
import icons from '../assets/icons/'
import NoQuestionnaires from "../components/NoQuestionnaires.vue"

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
        <div v-if="no_registered_questionnaires">
            <NoQuestionnaires />
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

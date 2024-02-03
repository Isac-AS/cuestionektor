<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { onMounted, ref } from "vue";
import { OperationResult, OperationResultStruct, RegisteredQuestionnaire, RegisteredQuestionnaires } from "../models";
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
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
        <div v-if="no_registered_questionnaires">
            <NoQuestionnaires />
        </div>
        <div v-else class="w-full grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
            {{ registered_questionnaires }}
        </div>
    </div>
</template>
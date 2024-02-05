<script setup lang="ts">
import { inject } from "vue";
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
import { Questionnaire } from "../models/questionnaire";
import { REGISTERED_QUESTIONNAIRES_KEY } from "../injectionKeys";
// import icons from '../assets/icons/'

const registeredQuestionnaires = inject<Questionnaire[]>(REGISTERED_QUESTIONNAIRES_KEY);
</script>

<template>
    <div class="flex flex-col items-center w-full">
        <h1 class="text-5xl font-bold mt-5">
            Cuestionektor
        </h1>
        <div v-if="registeredQuestionnaires && registeredQuestionnaires.length <= 0">
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

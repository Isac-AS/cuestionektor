<script setup lang="ts">
import { inject } from "vue";
import NoQuestionnaires from "../components/NoQuestionnaires.vue"
import { Questionnaire } from "../models/questionnaire";
import { OPEN_QUESTIONNAIRE_KEY, REGISTERED_QUESTIONNAIRES_KEY } from "../injectionKeys";
import { OpenQuestionnaire } from "../services/context.service";
import icons from '../assets/icons';
// import icons from '../assets/icons/'

const registeredQuestionnaires = inject<Questionnaire[]>(REGISTERED_QUESTIONNAIRES_KEY);
const openQuestionnaire = inject<OpenQuestionnaire>(OPEN_QUESTIONNAIRE_KEY);
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
                <div v-for="questionnaire in registeredQuestionnaires">
                    <router-link to="/questionnaire"
                        class="flex items-center bg-surface-dp12 transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md"
                        @click="openQuestionnaire!(questionnaire.id)">
                        <img :src="icons.question" class="invert w-6 lg:w-8">
                        <span>{{ questionnaire.name }}</span>
                    </router-link>
                </div>
            </div>
        </div>
    </div>
</template>

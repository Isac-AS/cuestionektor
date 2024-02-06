<script setup lang="ts">
import { inject, onUnmounted, ref } from 'vue';
import { Question } from '../models/questionnaire';
import { GET_QUESTIONS_KEY, REFRESH_QUESTIONNAIRES_KEY } from '../injectionKeys';
import { LoadQuestionnaires } from '../services/context.service';
import QuestionComponent from '../components/QuestionComponent.vue';
import icons from '../assets/icons';

// Injections
const loadQuestionnaires = inject<LoadQuestionnaires>(REFRESH_QUESTIONNAIRES_KEY);
const questions = inject<Question[]>(GET_QUESTIONS_KEY);

// Filter sidebar
const filtersExpanded = ref(false);
const ToggleFilters = () => {
    filtersExpanded.value = !filtersExpanded.value;
}

// Sidebar booleans
const showAnswersInGrid = ref(false);
const questionnaireMode = ref(false);

// Questions shown
const startIndex = ref(0);
const endIndex = ref(20);
const questionsPerPage = ref(20);
function prev() {
    if ((startIndex.value - questionsPerPage.value) < 0) {
        startIndex.value = 0;
        endIndex.value = questionsPerPage.value;
        return;
    }
    startIndex.value -= questionsPerPage.value;
    endIndex.value -= questionsPerPage.value;
}

function next() {
    if ((startIndex.value + questionsPerPage.value) > questions!.length) {
        startIndex.value = questions!.length - questionsPerPage.value;
        endIndex.value = questions!.length;
        return;
    }
    startIndex.value += questionsPerPage.value;
    endIndex.value += questionsPerPage.value;
}

function updateEndIndex() {
    if (startIndex.value + questionsPerPage.value > questions!.length) {
        endIndex.value = questions!.length;
        return;
    }
    endIndex.value = startIndex.value + questionsPerPage.value;
}

onUnmounted(() => {
    loadQuestionnaires!();
})
</script>

<template>
    <div class="w-100 flex">
        <div class="flex flex-col w-full p-6 gap-6">
            <div v-for="question in questions?.slice(startIndex, endIndex)">
                <QuestionComponent v-bind:question="question" v-bind:showAnswersInGrid="showAnswersInGrid" v-bind:questionnaireMode="questionnaireMode" />
            </div>
        </div>
        <aside :class="`${filtersExpanded ? 'w-80 lg:w-96' : 'w-16 lg:w-20'}`"
            class="sticky top-0 h-screen flex flex-col dark:bg-surface-dp6 shadow transition-all duration-300 items-center gap-5 pr-1">
            <button @click="ToggleFilters"
                class="my-5 p-2 w-5/6 flex justify-center rounded bg-surface-dp12 shadow-lg transition-all hover:bg-surface-dp24 duration-200">
                <img :src="icons.arrow_left" :class="`${filtersExpanded ? 'rotate-180' : ''}`"
                    class="invert w-7 lg:w-9 transition-all duration-300">
            </button>
            <button @click="questionnaireMode = !questionnaireMode"
                class="flex items-center justify-start transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md w-5/6"
                :class="`${questionnaireMode ? 'dark:bg-primary/30 border-l-4 border-primary' : ''}`">
                <img :src="icons.dynamic_form" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Modo cuestionario</span>
            </button>
            <button @click="showAnswersInGrid = !showAnswersInGrid"
                class="flex items-center justify-start transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md w-5/6">
                <img :src="showAnswersInGrid ? icons.table_rows : icons.grid_view" class="invert w-6 lg:w-8 transition-all duration-500">
                <span v-if="filtersExpanded">Apariencia</span>
            </button>
            <button
                class="flex items-center justify-start transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md w-5/6"
                @click="next()">
                <img :src="icons.arrow_right" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Siguiente</span>
            </button>
            <button
                class="flex items-center justify-start transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md w-5/6"
                @click="prev()">
                <img :src="icons.arrow_left" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Anterior</span>
            </button>
            <div class="flex items-center m-1 p-2 gap-4 w-5/6">
                <span v-if="filtersExpanded">Numero de preguntas</span>
                <input type="number" class="p-1 w-[95%] text-OnPrimary text-xl rounded outline-none text-center" min="1"
                    :max="questions!.length" v-model="questionsPerPage" v-on:change="updateEndIndex()">
            </div>
            <div class="flex items-center m-1 p-2 gap-4 w-5/6">
                <span v-if="filtersExpanded">Agrupar por tema</span>
                <input type="text" class="p-1 w-[95%] text-OnPrimary text-xl rounded outline-none text-center">
            </div>
        </aside>
    </div>
</template>
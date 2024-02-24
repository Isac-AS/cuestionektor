<script setup lang="ts">
import { inject, onMounted, ref } from 'vue';
import { Question } from '../models/questionnaire';
import { ADD_EMPTY_QUESTION_KEY, CLEAR_LOADED_QUESTIONS_KEY, CREATE_NOTIFICATION_KEY, GET_QUESTIONS_KEY, INFORM_ABOUT_RESULT_KEY, OPEN_QUESTIONNAIRE_KEY, REFRESH_QUESTIONNAIRES_KEY, REFRESH_QUESTIONS_KEY } from '../injectionKeys';
import EditableQuestion from '../components/EditableQuestion.vue'
import icons from '../assets/icons';
import { AddEmptyQuestion, ClearLoadedQuestions, LoadQuestionnaires, OpenQuestionnaire, RefreshQuestions } from '../services/context.service';
import { InformAboutResult } from '../App.vue';
import { createEmptyQuestionnaire } from '../services/questionnaire.service';
import { CreateNotification } from '../services/notifications.service';

// Injections
const questions = inject<Question[]>(GET_QUESTIONS_KEY);
const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);
const clearLoadedQuestions = inject<ClearLoadedQuestions>(CLEAR_LOADED_QUESTIONS_KEY);
const openQuestionnaire = inject<OpenQuestionnaire>(OPEN_QUESTIONNAIRE_KEY);
const createNotification = <CreateNotification>inject(CREATE_NOTIFICATION_KEY);
const refreshQuestionnaires = inject<LoadQuestionnaires>(REFRESH_QUESTIONNAIRES_KEY);
const addEmptyQuestion = inject<AddEmptyQuestion>(ADD_EMPTY_QUESTION_KEY);
const refreshQuestions = inject<RefreshQuestions>(REFRESH_QUESTIONS_KEY);

// Filter sidebar
const filtersExpanded = ref(false);
const ToggleFilters = () => {
    filtersExpanded.value = !filtersExpanded.value;
}

// Sidebar booleans
const showAnswersInGrid = ref(false);

// Filters
const topicFilter = ref('');
function applyTopicFilter(question: Question): boolean {
    if (topicFilter.value.length == 0) {
        return true;
    }
    return question.topic.includes(topicFilter.value);
}

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

function createEmptyQuestion() {
    addEmptyQuestion!();
    refreshQuestions!();
}

const questionnaireCreated = ref(false);
const newQuestionnaireName = ref('');
async function createQuestionnaire() {
    if (newQuestionnaireName.value.length <= 0) {
        createNotification({
            type: 'error',
            message: 'Dale un nombre al cuestionario',
            duration: 3,
        });
        return;
    }

    let questionCreateResponse = await createEmptyQuestionnaire(newQuestionnaireName.value);
    informAboutResult!(
        questionCreateResponse.result,
        "Cuestionario creado",
        questionCreateResponse.data
    );
    openQuestionnaire!(parseInt(questionCreateResponse.data));
    refreshQuestionnaires!();
    questionnaireCreated.value = true;
}

onMounted(() => {
    clearLoadedQuestions!();
})
</script>

<template>
    <div v-if="!questionnaireCreated" class="w-100 flex flex-col items-center gap-6 mt-10">
        <h1 class="text-3xl">Nombre del nuevo cuestionario</h1>
        <input type="text" placeholder="Nombre del cuestionario" v-model="newQuestionnaireName"
                        class="rounded shadow h-7 pl-5 text-OnPrimary focus:ring-3 focus:ring-sky-500">
        <button @click="createQuestionnaire" class="btn-primary">
            Crear cuestionario
        </button>
    </div>
    <div v-else class="w-100 flex">
        <div class="flex flex-col w-full p-6 gap-12">
            <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md w-11 self-center" @click="createEmptyQuestion()">
                <img src="/src/assets/icons/add.svg" class="invert w-5 lg:w-7">
            </button>
            <div v-for="question in questions?.filter((q) => applyTopicFilter(q)).slice(startIndex, endIndex)">
                <EditableQuestion v-bind:question="question" v-bind:showAnswersInGrid="showAnswersInGrid" />
            </div>
        </div>
        <aside :class="`${filtersExpanded ? 'w-80 lg:w-96' : 'w-16 lg:w-20'}`"
            class="sticky top-0 h-screen flex flex-col dark:bg-surface-dp6 shadow transition-all duration-300 items-center gap-5 pr-1">
            <button @click="ToggleFilters"
                class="my-5 p-2 w-5/6 flex justify-center rounded bg-surface-dp12 shadow-lg transition-all hover:bg-surface-dp24 duration-200">
                <img :src="icons.arrow_left" :class="`${filtersExpanded ? 'rotate-180' : ''}`"
                    class="invert w-7 lg:w-9 transition-all duration-300">
            </button>

            <button @click="showAnswersInGrid = !showAnswersInGrid"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`">
                <img :src="showAnswersInGrid ? icons.table_rows : icons.grid_view"
                    class="invert w-6 lg:w-8 transition-all duration-500">
                <span v-if="filtersExpanded">Apariencia</span>
            </button>
            <hr class="w-3/4 h-px">
            <button :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                @click="next()">
                <img :src="icons.arrow_right" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Siguiente</span>
            </button>
            <button class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`" @click="prev()">
                <img :src="icons.arrow_left" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Anterior</span>
            </button>
            <div class="flex items-center m-1 p-1 gap-4 w-5/6"
                :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`">
                <span v-if="filtersExpanded">Numero de preguntas</span>
                <input type="number" class="p-1 w-[95%] text-OnPrimary text-xl rounded outline-none text-center" min="1"
                    :max="questions?.length" v-model="questionsPerPage" v-on:change="updateEndIndex()">
            </div>
            <hr class="w-3/4 h-px">
            <div class="flex items-center m-1 p-1 gap-4 w-5/6"
                :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`">
                <span v-if="filtersExpanded">Agrupar por tema</span>
                <input type="text" class="p-1 w-[95%] text-OnPrimary text-xl rounded outline-none text-center"
                    v-model="topicFilter">
            </div>
        </aside>
    </div>
</template>
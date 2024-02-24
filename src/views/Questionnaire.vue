<script setup lang="ts">
import { inject, ref } from 'vue';
import { AnswerState, Question } from '../models/questionnaire';
import { ADD_EMPTY_QUESTION_KEY, GET_QUESTIONS_KEY, REFRESH_QUESTIONS_KEY } from '../injectionKeys';
import QuestionComponent from '../components/QuestionComponent.vue';
import EditableQuestion from '../components/EditableQuestion.vue'
import icons from '../assets/icons';
import { AddEmptyQuestion, RefreshQuestions } from '../services/context.service';

// Injections
const questions = inject<Question[]>(GET_QUESTIONS_KEY);
const addEmptyQuestion = inject<AddEmptyQuestion>(ADD_EMPTY_QUESTION_KEY);
const refreshQuestions = inject<RefreshQuestions>(REFRESH_QUESTIONS_KEY);

// Filter sidebar
const filtersExpanded = ref(false);
const ToggleFilters = () => {
    filtersExpanded.value = !filtersExpanded.value;
}

// Sidebar booleans
const showAnswersInGrid = ref(false);
const editMode = ref(false);

// Filters
const answerStateFilter = ref<AnswerState>(AnswerState.All);

function updateAnswerState(newAnswerState: AnswerState) {
    if (answerStateFilter.value == newAnswerState) {
        answerStateFilter.value = AnswerState.All
    } else {
        answerStateFilter.value = newAnswerState
    }
}
function applyStateFilter(question: Question): boolean {
    switch (answerStateFilter.value) {
        case AnswerState.All:
            return true;
        case AnswerState.Correct:
            return question.answeredCorrectly == true;
        case AnswerState.Incorrect:
            return question.answeredCorrectly == false;
        case AnswerState.Unanswered:
            return question.answeredCorrectly == undefined;
    }
}

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
</script>

<template>
    <div class="w-100 flex">
        <div v-if="!editMode" class="flex flex-col w-full p-6 gap-12">
            <div v-for="question in questions?.filter((q) => applyTopicFilter(q)).filter((q) => applyStateFilter(q)).slice(startIndex, endIndex)">
                <QuestionComponent v-bind:question="question" v-bind:showAnswersInGrid="showAnswersInGrid" />
            </div>
        </div>
        <div v-else class="flex flex-col w-full p-6 gap-12">
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
            <button @click="editMode = !editMode"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${editMode ? 'dark:bg-primary/30 border-l-4 border-primary' : ''} ${filtersExpanded ? 'justify-start' : 'justify-center'}`">
                <img :src="icons.edit" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Habilitar edicion</span>
            </button>
            <hr class="w-3/4 h-px">
            <div class="flex items-center m-1 p-1 gap-4 w-5/6"
                :class="`${filtersExpanded ? 'justify-start' : 'justify-center'}`">
                <span v-if="filtersExpanded">Agrupar por tema</span>
                <input type="text" class="p-1 w-[95%] text-OnPrimary text-xl rounded outline-none text-center" v-model="topicFilter">
            </div>
            <button v-if="!editMode"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${answerStateFilter == AnswerState.Correct ? 'dark:bg-primary/30 border-l-4 border-primary' : ''} ${filtersExpanded ? 'justify-start' : 'justify-center'}`"
                @click="updateAnswerState(AnswerState.Correct)">
                <img :src="icons.check_circle" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Mostrar aciertos</span>
            </button>
            <button v-if="!editMode"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${answerStateFilter == AnswerState.Incorrect ? 'dark:bg-primary/30 border-l-4 border-primary' : ''} ${filtersExpanded ? 'justify-start' : 'justify-center'}`"
                @click="updateAnswerState(AnswerState.Incorrect)">
                <img :src="icons.cancel_circle" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Mostrar fallos</span>
            </button>
            <button v-if="!editMode"
                class="flex items-center transition-all duration-200 hover:bg-primary/30 p-1 m-1 gap-4 rounded-md w-5/6"
                :class="`${answerStateFilter == AnswerState.Unanswered ? 'dark:bg-primary/30 border-l-4 border-primary' : ''} ${filtersExpanded ? 'justify-start' : 'justify-center'}`"
                @click="updateAnswerState(AnswerState.Unanswered)">
                <img :src="icons.help_circle" class="invert w-6 lg:w-8">
                <span v-if="filtersExpanded">Preguntas sin contestar</span>
            </button>
        </aside>
    </div>
</template>
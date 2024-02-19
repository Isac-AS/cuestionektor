<script lang="ts" setup>
import { Question, Answer } from '../models/questionnaire';
import { marked } from "marked";
import icons from '../assets/icons';
import { computed, inject, ref } from 'vue';
import { deleteQuestion, updateQuestion } from '../services/question.service';
import { InformAboutResult } from '../App.vue';
import { INFORM_ABOUT_RESULT_KEY, REMOVE_QUESTION_KEY } from '../injectionKeys';
import input from 'postcss/lib/input';
import { RemoveQuestion } from '../services/context.service';

const props = defineProps<{
    question: Question,
    showAnswersInGrid: boolean
}>()

const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);
const removeQuestionFromArray = inject<RemoveQuestion>(REMOVE_QUESTION_KEY);

const editingHeading = ref(false);
const editingAnswer = ref(false);
const editingTopic = ref(false);
const editingExplanation = ref(false);
const explanation = computed(() => marked.parse(props.question.explanation));

const tempHeading = ref('');
const tempAnswer = ref('');
const tempPrefix = ref('');
const tempIsCorrect = ref(false);
const editingIndex = ref(-1);
const tempTopic = ref('');
const tempExplanation = ref('');


const addNewQuestion = ref(false);
const newPrefix = ref('');
const newAnswerText = ref('');
const newIsCorrect = ref(false);
function addAnswer() {
    let newAnswer: Answer = {
        is_correct: newIsCorrect.value,
        prefix: newPrefix.value,
        text: newAnswerText.value
    }
    props.question.answers.push(newAnswer);
    saveQuestion();
    addNewQuestion.value = false;
    newPrefix.value = '';
    newAnswerText.value = '';
    newIsCorrect.value = false;
}

async function saveQuestion() {
    let questionUpdateResponse = await updateQuestion(props.question.id, props.question);
    informAboutResult!(
        questionUpdateResponse.result,
        "Pregunta modificada",
        questionUpdateResponse.data
    );
}

async function removeQuestion() {
    let questionUpdateResponse = await deleteQuestion(props.question.id);
    informAboutResult!(
        questionUpdateResponse.result,
        "Pregunta eliminada",
        questionUpdateResponse.data
    );
    removeQuestionFromArray!(props.question);
}
</script>

<template>
    <div class="flex flex-col dark:bg-surface-dp6 p-4 rounded shadow-md">
        <div class="flex justify-between">
            <h1 v-if="!editingHeading"
                class="text-2xl font-semibold text-justify rounded shadow-lg dark:bg-surface-dp8 p-2 max-w-[90%]">
                {{ props.question.heading }}
            </h1>
            <input type="text" v-else v-model="tempHeading"
                class="rounded w-3/4 self-center p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary text-xl">
            <div v-if="!editingHeading" class="flex gap-4">
                <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                    @click="editingHeading = true; tempHeading = props.question.heading;">
                    <img :src="icons.edit" class="invert w-5 lg:w-7">
                </button>
                <button class="bg-wm-error hover:brightness-125 p-2 rounded-md" @click="removeQuestion()">
                    <img :src="icons.delete_icon" class="invert w-5 lg:w-7">
                </button>
            </div>
            <div v-if="editingHeading" class="flex gap-4">
                <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                    @click="props.question.heading = tempHeading; editingHeading = false; saveQuestion()">
                    <img :src="icons.done" class="invert w-5 lg:w-7">
                </button>
                <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="editingHeading = false">
                    <img :src="icons.close" class="invert w-5 lg:w-7">
                </button>
            </div>
        </div>
        <div class="grid grid-cols-1 gap-5 mt-5" :class="`${showAnswersInGrid ? 'lg:grid-cols-2' : ''}`">
            <div v-for="answer, index in props.question!.answers" class="dark:bg-surface-dp8">
                <div class="flex justify-between">
                    <div v-if="!editingAnswer || editingIndex != index" class="flex w-11/12 gap-2">
                        <div class="text-xl font-semibold text-justify rounded shadow-lg dark:bg-surface-dp16 p-2 px-5">
                            {{ answer.prefix }}
                        </div>
                        <div class="text-xl font-semibold text-justify rounded shadow-lg dark:bg-surface-dp16 p-2 w-3/5">
                            {{ answer.text }}
                        </div>
                    </div>
                    <div v-else class="flex w-11/12 gap-2 items-center">
                        <input type="text" v-model="tempPrefix"
                            class="text-xl rounded p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary w-1/12">
                        <input type="text" v-model="tempAnswer"
                            class="rounded w-3/5 self-center p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary text-xl">
                        <div class="text-xl font-semibold">Es correcta?</div>
                        <button class="hover:brightness-125 p-2 rounded-md"
                            :class="`${tempIsCorrect ? 'bg-green-400/50' : 'bg-error'}`"
                            @click="tempIsCorrect = !tempIsCorrect">
                            <img v-if="tempIsCorrect" :src="icons.done" class="invert w-5 lg:w-7">
                            <img v-else :src="icons.close" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                    <div v-if="!editingAnswer && editingIndex < 0" class="flex gap-4">
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="editingAnswer = true; tempAnswer = answer.text; tempPrefix = answer.prefix;
                        tempIsCorrect = answer.is_correct; editingIndex = index;">
                            <img :src="icons.edit" class="invert w-5 lg:w-7">
                        </button>
                        <button class="bg-surface-dp24 hover:bg-error p-2 rounded-md"
                            @click="props.question.answers.splice(index, 1); saveQuestion()">
                            <img :src="icons.delete_icon" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                    <div v-if="editingAnswer && editingIndex == index" class="flex gap-4">
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                            @click="answer.text = tempAnswer; answer.prefix = tempPrefix; answer.is_correct = tempIsCorrect; editingAnswer = false; editingIndex = -1; saveQuestion()">
                            <img :src="icons.done" class="invert w-5 lg:w-7">
                        </button>
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                            @click="editingAnswer = false; editingIndex = -1">
                            <img :src="icons.close" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                </div>
            </div>
            <div class="flex flex-col items-center">
                <button v-if="!addNewQuestion" class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                    @click="addNewQuestion = true">
                    <img :src="icons.add" class="invert w-5 lg:w-7">
                </button>
                <div v-else class="flex flex-col gap-4 w-full items-center">
                    <div class="flex gap-5">
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="addAnswer()">
                            <img :src="icons.done" class="invert w-5 lg:w-7">
                        </button>
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="addNewQuestion = false">
                            <img :src="icons.close" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                    <div class="flex gap-6 w-11/12 items-center">
                        <input type="text" v-model="newPrefix"
                            class="text-xl rounded p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary w-1/12">
                        <input type="text" v-model="newAnswerText"
                            class="rounded w-3/5 self-center p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary text-xl">
                        <div class="text-xl font-semibold">Es correcta?</div>
                        <button class="hover:brightness-125 p-2 rounded-md"
                            :class="`${newIsCorrect ? 'bg-green-400/50' : 'bg-error'}`"
                            @click="newIsCorrect = !newIsCorrect">
                            <img v-if="newIsCorrect" :src="icons.done" class="invert w-5 lg:w-7">
                            <img v-else :src="icons.close" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                </div>
            </div>
        </div>
        <div class="mt-6 dark:bg-surface-dp12 rounded flex flex-col gap-4 p-3">
            <div class="flex justify-between items-center">
                <p class="text-xl font-semibold">Tema / Grupo:</p>
                <button v-if="!editingTopic" class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                    @click="editingTopic = true; tempTopic = props.question.topic;">
                    <img :src="icons.edit" class="invert w-5 lg:w-7">
                </button>
                <div v-if="editingTopic" class="flex gap-4">
                    <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                        @click="props.question.topic = tempTopic; editingTopic = false; saveQuestion()">
                        <img :src="icons.done" class="invert w-5 lg:w-7">
                    </button>
                    <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="editingTopic = false">
                        <img :src="icons.close" class="invert w-5 lg:w-7">
                    </button>
                </div>
            </div>
            <p v-if="!editingTopic">{{ props.question.topic }}</p>
            <input type="text" v-else v-model="tempTopic"
                class="rounded w-1/2 self-center p-2 text-OnPrimary dark:bg-gray-200 focus:outline-secondary">
        </div>
        <div class="mt-6 dark:bg-surface-dp12 rounded flex flex-col gap-4 p-3">
            <div class="flex justify-between items-center">
                <p class="text-xl font-semibold">Explicacion / Descripcion:</p>
                <button v-if="!editingExplanation" class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                    @click="editingExplanation = true; tempExplanation = props.question.explanation;">
                    <img :src="icons.edit" class="invert w-5 lg:w-7">
                </button>
                <div v-if="editingExplanation" class="flex gap-4">
                    <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                        @click="props.question.explanation = tempExplanation; editingExplanation = false; saveQuestion()">
                        <img :src="icons.done" class="invert w-5 lg:w-7">
                    </button>
                    <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md" @click="editingExplanation = false">
                        <img :src="icons.close" class="invert w-5 lg:w-7">
                    </button>
                </div>
            </div>
            <div v-if="!editingExplanation" v-html="explanation"></div>
            <textarea v-if="editingExplanation" v-model="tempExplanation"
                class="rounded text-OnPrimary h-32 dark:bg-gray-300 focus:outline-secondary"></textarea>
        </div>
    </div>
</template>
<script lang="ts" setup>
import { Question } from '../models/questionnaire';
import { marked } from "marked";
import icons from '../assets/icons';
import { computed, inject, ref } from 'vue';
import { updateQuestion } from '../services/question.service';
import { InformAboutResult } from '../App.vue';
import { INFORM_ABOUT_RESULT_KEY } from '../injectionKeys';
import input from 'postcss/lib/input';

const props = defineProps<{
    question: Question,
    showAnswersInGrid: boolean
}>()
const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);

const editingTopic = ref(false);
const editingExplanation = ref(false);
const explanation = computed(() => marked.parse(props.question.explanation));

const tempTopic = ref('');
const tempExplanation = ref('');

function answerQuestion(isCorrect: boolean) {
    if (props.question.answeredCorrectly !== undefined) return;
    props.question.answeredCorrectly = isCorrect;
}

async function saveQuestion() {
    let questionUpdateResponse = await updateQuestion(props.question.id, props.question);
    informAboutResult!(
        questionUpdateResponse.result,
        "Pregunta modificada",
        questionUpdateResponse.data
    );
}
</script>

<template>
    <div class="flex flex-col dark:bg-surface-dp6 p-4 rounded shadow-md">
        <h1 class="text-2xl font-semibold text-justify rounded shadow-lg dark:bg-surface-dp8 p-2">
            {{ props.question.heading }}
        </h1>
        <div class="grid grid-cols-1 gap-5 mt-5" :class="`${showAnswersInGrid ? 'lg:grid-cols-2' : ''}`">
            <div v-for="answer in props.question!.answers" class="dark:bg-surface-dp24">
                <button @click="answerQuestion(answer.is_correct)"
                    class="text-lg dark:hover:bg-primary/15 transition-all duration-200 px-5 py-2 rounded shadow-md w-full text-justify flex h-full"
                    :class="`
                        ${props.question.answeredCorrectly !== undefined && answer.is_correct ? 'bg-green-600/50 dark:bg-green-400/40 hover:bg-green-600/50 dark:hover:bg-green-300/40' : ''} 
                        ${props.question.answeredCorrectly === false && !answer.is_correct ? 'bg-wm-error/50 dark:bg-error/45 hover:bg-wm-error/55 dark:hover:bg-error/60' : ''}
                        `">
                    <p class="mr-4">{{ answer.prefix + ")" }}</p>
                    <p>{{ answer.text }}</p>
                </button>
            </div>
        </div>
        <div v-if="props.question.answeredCorrectly !== undefined">
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
                        <button class="bg-surface-dp24 hover:brightness-125 p-2 rounded-md"
                            @click="editingExplanation = false">
                            <img :src="icons.close" class="invert w-5 lg:w-7">
                        </button>
                    </div>
                </div>
                <div v-if="!editingExplanation" v-html="explanation"></div>
                <textarea v-if="editingExplanation" v-model="tempExplanation"
                    class="rounded text-OnPrimary h-32 dark:bg-gray-200 focus:outline-secondary"></textarea>
            </div>
        </div>
    </div>
</template>
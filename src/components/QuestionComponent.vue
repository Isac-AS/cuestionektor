<script lang="ts" setup>
import { Question } from '../models/questionnaire';

const props = defineProps<{
    question: Question,
    showAnswersInGrid: boolean
}>()

function answerQuestion(isCorrect: boolean) {
    if (props.question.answeredCorrectly !== undefined) return;
    props.question.answeredCorrectly = isCorrect;
}
</script>

<template>
    <div class="flex flex-col dark:bg-surface-dp6 p-4 rounded shadow-md">
        <h1 class="text-2xl font-semibold text-justify rounded shadow-lg dark:bg-surface-dp8 p-2">
            {{ props.question.heading }}
        </h1>
        <div class="grid grid-cols-1 gap-5 mt-5" :class="`${showAnswersInGrid ? 'lg:grid-cols-2' : ''}`">
            <div v-for="answer in props.question!.answers">
                <button @click="answerQuestion(answer.is_correct)"
                    class="text-lg dark:hover:bg-primary/15 transition-all duration-200 px-5 py-2 rounded shadow-md w-full text-justify flex h-full"
                    :class="`
                        ${props.question.answeredCorrectly !== undefined && answer.is_correct ? 'bg-green-600/50 dark:bg-green-400/40' : ''} 
                        ${props.question.answeredCorrectly === false && !answer.is_correct ? 'bg-wm-error/50 dark:bg-error/45' : 'dark:bg-surface-dp24'}
                        `">
                    <p class="mr-4">{{ answer.prefix + ")" }}</p>
                    <p>{{ answer.text }}</p>
                </button>
            </div>
        </div>
        <div v-if="props.question.answeredCorrectly !== undefined">
            <div>
                <p>Tema / Grupo:</p>
                <p>{{ props.question.topic }}</p>
            </div>
            <div>
                <p>Explicacion / Descripcion:</p>
                <div v-html="props.question.explanation"></div>
            </div>
        </div>
    </div>
</template>
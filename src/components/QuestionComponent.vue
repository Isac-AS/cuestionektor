<script lang="ts" setup>
import { Question } from '../models/questionnaire';

defineProps<{
  question: Question
}>()

</script>

<template>
    <div class="flex flex-col dark:bg-surface-dp6 p-4 rounded shadow-md">
        <h1 class="text-xl text-justify rounded shadow-lg dark:bg-surface-dp24 p-2">
            {{ question.heading }}
        </h1>
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-5 mt-5">
            <div v-for="answer in question!.answers">
                <button class="text-lg dark:bg-surface-dp24 dark:hover:bg-primary/15 transition-all duration-200 px-5 py-2 rounded shadow-md w-full text-justify flex h-full">
                    <p class="mr-4">{{ answer.prefix + ")"}}</p>
                    <p>{{ answer.text }}</p> 
                </button>
            </div>
        </div>
    </div>
</template>
<script setup lang="ts">
import { inject, ref } from 'vue';
import { InformAboutResult } from '../App.vue';
import { Question } from '../models/questionnaire';
import { GET_QUESTIONS_KEY, INFORM_ABOUT_RESULT_KEY } from '../injectionKeys';

const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);
const questions = inject<Question[]>(GET_QUESTIONS_KEY);


</script>

<template>
    <div class="w-full">
        <div v-for="question in questions">
            {{ question.heading }}
        </div>
    </div>
</template>

<style scoped>
</style>
<script setup lang="ts">
import { inject, onUnmounted, ref } from 'vue';
import { InformAboutResult } from '../App.vue';
import { Question } from '../models/questionnaire';
import { GET_QUESTIONS_KEY, INFORM_ABOUT_RESULT_KEY, REFRESH_QUESTIONNAIRES_KEY } from '../injectionKeys';
import { LoadQuestionnaires } from '../services/context.service';

const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);
const loadQuestionnaires = inject<LoadQuestionnaires>(REFRESH_QUESTIONNAIRES_KEY);
const questions = inject<Question[]>(GET_QUESTIONS_KEY);

onUnmounted(() => {
    loadQuestionnaires!();
})
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
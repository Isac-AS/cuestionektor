<script setup lang="ts">
import { inject, onUpdated, ref } from 'vue';
import { getQuestions } from '../services/question.service';
import { InformAboutResult } from '../App.vue';
import { Question } from '../models/questionnaire';
import { INFORM_ABOUT_RESULT_KEY } from '../injectionKeys';

const informAboutResult = inject<InformAboutResult>(INFORM_ABOUT_RESULT_KEY);
const questions = ref<Question[]>();

const props = defineProps<{id: number}>();

async function fetchQuestions(questionnaireId: number) {
    await getQuestions(questionnaireId).then(
        (questionsResponse) => {
            informAboutResult!(
                questionsResponse.result,
                "Cuestionarios cargados con exito.",
                "Error cargando cuestionarios.",
            );
            questions.value = questionsResponse.data;
        }
    );
}

onUpdated(() => {
    fetchQuestions(+props.id);
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
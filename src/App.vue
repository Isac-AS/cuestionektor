<script setup lang="ts">
import { onMounted, provide } from 'vue';
import AppBar from './components/AppBar.vue'
import Sidebar from './components/Sidebar.vue'
import Toast from './components/Toast.vue';
import useNotifications from './services/notifications.service';
import { OperationResult } from './models/view-models';
import useContext from './services/context.service';
import { ADD_EMPTY_QUESTION_KEY, CLEAR_LOADED_QUESTIONS_KEY, CREATE_NOTIFICATION_KEY, CURRENT_QUESTIONNAIRE_ID_KEY, GET_QUESTIONS_KEY, INFORM_ABOUT_RESULT_KEY, OPEN_QUESTIONNAIRE_KEY, REFRESH_QUESTIONNAIRES_KEY, REFRESH_QUESTIONS_KEY, REGISTERED_QUESTIONNAIRES_KEY, REMOVE_QUESTION_KEY } from './injectionKeys';

const {
	notifications,
	createNotification,
	removeNotifications,
} = useNotifications();

const {
	registeredQuestionnaires,
	loadQuestionnaires,
	loadedQuestions,
	currentQuestionnaireId,
	openQuestionnaire,
	removeQuestion,
	clearLoadedQuestions,
	addEmptyQuestion,
	refreshQuestions
} = useContext();

export type InformAboutResult = {
	(
		result: OperationResult,
		successMessage: string,
		errorMessage: string
	): boolean;
}
const informAboutResult: InformAboutResult = (result: OperationResult, successMessage: string, errorMessage: string): boolean => {
	let returnValue = result == OperationResult.Success;
	createNotification({
		type: returnValue ? 'success' : 'error',
		title: returnValue ? 'Exito: ' : 'Error: ',
		message: returnValue ? successMessage : errorMessage,
		duration: 4
	});
	return returnValue;
}

provide(INFORM_ABOUT_RESULT_KEY, informAboutResult);
provide(CREATE_NOTIFICATION_KEY, createNotification);

provide(REGISTERED_QUESTIONNAIRES_KEY, registeredQuestionnaires);
provide(REFRESH_QUESTIONNAIRES_KEY, loadQuestionnaires);
provide(CURRENT_QUESTIONNAIRE_ID_KEY, currentQuestionnaireId);
provide(GET_QUESTIONS_KEY, loadedQuestions);
provide(OPEN_QUESTIONNAIRE_KEY, openQuestionnaire);
provide(REMOVE_QUESTION_KEY, removeQuestion);
provide(CLEAR_LOADED_QUESTIONS_KEY, clearLoadedQuestions);
provide(ADD_EMPTY_QUESTION_KEY, addEmptyQuestion);
provide(REFRESH_QUESTIONS_KEY, refreshQuestions);

onMounted(() => {
	loadQuestionnaires();
	
})
</script>

<template>
	<div class="app flex flex-col h-full">
		<AppBar />
		<div class="flex">
			<Sidebar />
			<main class="h-full w-full">
				<router-view />
			</main>
		</div>
	</div>

	<Toast v-for="(item) in notifications" :key="item.id" :id="item.id" :type="item.type" :title="item.title"
		:message="item.message" :auto-close="item.autoClose" :duration="item.duration" @close="() => {
			removeNotifications(item.id);
		}"></Toast>
</template>
<script setup lang="ts">
import { onMounted, provide } from 'vue';
import AppBar from './components/AppBar.vue'
import Sidebar from './components/Sidebar.vue'
import Toast from './components/Toast.vue';
import useNotifications from './services/notifications.service';
import { OperationResult } from './models/view-models';
import useContext from './services/context.service';
import { CREATE_NOTIFICATION_KEY, CURRENT_QUESTIONNAIRE_ID_KEY, INFORM_ABOUT_RESULT_KEY, REFRESH_QUESTIONNAIRES_KEY, REGISTERED_QUESTIONNAIRES_KEY, SET_CURRENT_QUESTIONNAIRE_ID_KEY } from './injectionKeys';

const {
	notifications,
	createNotification,
	removeNotifications,
} = useNotifications();

const {
	registeredQuestionnaires,
	loadQuestionnaires,
	currentQuestionnaireId,
	setCurrentQuestionnaireId
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
provide(SET_CURRENT_QUESTIONNAIRE_ID_KEY, setCurrentQuestionnaireId);

onMounted(() => {
	loadQuestionnaires()
})
</script>

<template>
	<div class="app flex flex-col h-full">
		<AppBar />
		<div class="flex h-full">
			<Sidebar />
			<router-view />
		</div>
	</div>

	<Toast v-for="(item) in notifications" :key="item.id" :id="item.id" :type="item.type" :title="item.title"
		:message="item.message" :auto-close="item.autoClose" :duration="item.duration" @close="() => {
			removeNotifications(item.id);
		}"></Toast>
</template>
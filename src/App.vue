<script setup lang="ts">
import { onMounted, provide } from 'vue';
import AppBar from './components/AppBar.vue'
import Sidebar from './components/Sidebar.vue'
import Toast from './components/Toast.vue';
import useNotifications from './services/notifications.service';
import { OperationResult } from './models/view-models';
import useContext from './services/context.service';

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

provide("inform-about-result", informAboutResult);
provide("create-notification", createNotification);

provide("registered-questionnaires", registeredQuestionnaires);
provide("refresh-questionnaires", loadQuestionnaires);
provide("current-questionnaire-id", currentQuestionnaireId);
provide("set-current-questionnaire-id", setCurrentQuestionnaireId);

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
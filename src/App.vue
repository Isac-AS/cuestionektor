<script setup lang="ts">
import { provide } from 'vue';
import AppBar from './components/AppBar.vue'
import Sidebar from './components/Sidebar.vue'
import Toast from './components/Toast.vue';
import useNotifications from './services/notifications.service';

const {
	notifications,
	createNotification,
	removeNotifications,
} = useNotifications();

provide("create-notification", createNotification);
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
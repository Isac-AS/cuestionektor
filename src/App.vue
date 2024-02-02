<script setup lang="ts">
  import AppBar from './components/AppBar.vue'
  import Sidebar from './components/Sidebar.vue'
</script>

<template>
	<div class="app flex flex-col h-full">
		<AppBar />	
		<div class="flex h-full">
			<Sidebar />
			<router-view />
		</div>
	</div>
</template>
<script setup lang="ts">
  import Sidebar from './components/Sidebar.vue'
</script>

<template>
	<div class="app">
		<!-- Sidebar -->
		<Sidebar />

		<!-- Content -->
		<router-view />
	</div>
</template>


<style lang="scss">
:root {
	--primary: #FA7D09;
	--primary-alt: #b3944d;
	--grey: #64748b;
	--dark: #1e293b;
	--dark-alt: #262729;
	--light: #f1f5f9;
	--sidebar-width: 350px;
}

.button {
	border-radius: 8px;
    display: flex;
    align-items: center;
    text-decoration: none;
	box-shadow: 0 5px 5px rgba(0, 0, 0, 0.2);
    transition: 0.2s ease-in-out;
    padding: 0.5rem 1rem;
	background-color: var(--dark-alt);

	.text {
		margin-left: 1em;
	}
	
    &:hover {
        background-color: var(--dark-alt);
        .text {
			transition: 0.2s ease-in-out;
            color: var(--primary-alt);
        }
    }
}

.app {
	display: flex;

	main {
		flex: 1 1 0;
		padding: 2rem;

		@media (max-width: 1024px) {
			padding-left: 6rem;
		}
	}
}
</style>
<script setup lang="ts">
import { ref } from 'vue'
import icons from '../assets/icons';

const is_expanded = ref(localStorage.getItem("is_expanded") === "true");

const ToggleMenu = () => {
    is_expanded.value = !is_expanded.value;
    localStorage.setItem("is_expanded", is_expanded.value.toString());
}
</script>

<template>
    <aside :class="`${is_expanded ? 'w-80 lg:w-96' : 'w-16 lg:w-20'}`"
        class="flex flex-col dark:bg-surface-dp6 shadow transition-all duration-300 items-center z-30">

        <button @click="ToggleMenu" class="my-5 p-2 w-5/6 flex justify-center rounded bg-surface-dp12 shadow-lg transition-all hover:bg-surface-dp24 duration-200">
            <img :src="icons.arrow_right" :class="`${is_expanded ? 'rotate-180' : ''}`" class="invert w-7 lg:w-9 transition-all duration-300">
        </button>

        <div class="flex flex-col gap-1 w-full items-center">
            <router-link to="/" class="sidebar-button">
                <img :src="icons.home" class="invert w-6 lg:w-8">
                <span :class="`${is_expanded ? '' : 'hidden'}`">Inicio</span>
            </router-link>
            <router-link to="/create" class="sidebar-button">
                <img :src="icons.add" class="invert w-6 lg:w-8">
                <span :class="`${is_expanded ? '' : 'hidden'}`">Crear cuestionario</span>
            </router-link>
            <router-link to="/create-from-file" class="sidebar-button">
                <img :src="icons.file" class="invert w-6 lg:w-8">
                <span :class="`${is_expanded ? '' : 'hidden'}`">Subir un fichero</span>
            </router-link>
            <router-link to="/manage" class="sidebar-button">
                <img :src="icons.settings" class="invert w-6 lg:w-8">
                <span :class="`${is_expanded ? '' : 'hidden'}`">Gestionar cuestionarios</span>
            </router-link>
        </div>
    </aside>
</template>

<style lang="scss" scoped>
.sidebar-button {
    @apply flex items-center transition-all duration-200 hover:bg-primary/30 p-2 m-1 gap-4 rounded-md w-5/6;

    &.router-link-exact-active {
        @apply bg-primary/30 border-r-4 border-solid border-primary;
    }
}
</style>
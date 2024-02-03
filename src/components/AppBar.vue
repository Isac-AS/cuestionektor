<script setup lang="ts">
import logoURL from '../assets/logo.png';
import icons from '../assets/icons';
</script>

<template>
    <div class="flex h-14 lg:h-16 items-center bg-surface-dp24 shadow-md justify-between z-50">
        <div class="flex items-center">
            <router-link to="/">
                <img :src="logoURL" class="h-14 lg:h-16 ml-1" alt="Cuestionektor logo" />
            </router-link>
            <h1 class="text-2xl ml-7 font-semibold text-wm-OnSurface dark:text-OnSurface">{{ this.$route.name }}</h1>
        </div>
        <div class="flex">
            <router-link to="/" class="mx-2 dark:hover:bg-slate-500/65 rounded-md p-1">
                <img :src="icons.settings" class="invert w-[40px]">
            </router-link>
            <router-link to="/" class="mx-2 dark:hover:bg-slate-500/65 rounded-md p-1">
                <img :src="icons.question" class="invert w-[40px]">
            </router-link>
        </div>
    </div>
</template>


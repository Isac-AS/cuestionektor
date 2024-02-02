<script setup lang="ts">
import logoURL from '../assets/logo.png';
import icons from '../assets/icons';
</script>

<template>
    <div class="flex h-[56px] lg:h-[80px] items-center bg-surface-dp24 shadow-md justify-between">
        <div class="">
            <router-link to="/">
                <img :src="logoURL" class="h-[56px] lg:h-[80px]" alt="Cuestionektor logo" />
            </router-link>
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


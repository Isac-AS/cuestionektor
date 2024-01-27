<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
</script>

<template>
  <header>
    <h1>
    Cuestionektor
    </h1>
  </header>
  <nav>
    <ul>Nuevo cuestionario</ul>
    <ul>Ordenar por</ul>
  </nav>
  <Greet></Greet>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>

<script setup lang="ts">
// This component injects the SVG sprite into the DOM
// Include it once at the root of your app (e.g., in App.vue)
import spriteContent from "@/assets/icons/sprite.svg?raw";
</script>

<template>
  <!-- Hidden container for SVG sprite symbols -->
  <div
    v-html="spriteContent"
    style="position: absolute; width: 0; height: 0; overflow: hidden"
    aria-hidden="true"
  />
</template>

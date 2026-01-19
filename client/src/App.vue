<template>
  <TpSpriteProvider />
  <router-view />
</template>

<script setup lang="ts">
import { onMounted } from "vue";
import { TpSpriteProvider } from "@/components/ui";
import { useTheme } from "@/composables/useTheme";

// Initialize theme system
const { initialize } = useTheme();

onMounted(() => {
  initialize();
});
</script>

<style>
/* Keyboard shortcut styling - using design tokens */
.kbd {
  background-color: var(--tp-bg-tertiary);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  box-shadow: 0 1px 0 rgba(0, 0, 0, 0.1);
  color: var(--tp-text-secondary);
  display: inline-block;
  font-size: 0.85em;
  font-family: var(--tp-font-mono);
  padding: 2px 6px;
  white-space: nowrap;
}
</style>

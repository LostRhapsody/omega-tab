<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    name: string;
    size?: "xs" | "sm" | "md" | "lg" | "xl";
    spin?: boolean;
  }>(),
  {
    size: "md",
    spin: false,
  },
);

const sizePixels = computed(() => {
  switch (props.size) {
    case "xs":
      return 14;
    case "sm":
      return 16;
    case "md":
      return 20;
    case "lg":
      return 24;
    case "xl":
      return 32;
    default:
      return 20;
  }
});
</script>

<template>
  <svg
    :class="['tp-icon', { 'tp-icon--spin': spin }]"
    :width="sizePixels"
    :height="sizePixels"
    aria-hidden="true"
  >
    <use :href="`#icon-${name}`"></use>
  </svg>
</template>

<style>
.tp-icon {
  display: inline-block;
  vertical-align: middle;
  flex-shrink: 0;
  fill: none;
  stroke: currentColor;
}

@keyframes tp-spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.tp-icon--spin {
  animation: tp-spin 1s linear infinite;
}

@media (prefers-reduced-motion: reduce) {
  .tp-icon--spin {
    animation: none;
  }
}
</style>

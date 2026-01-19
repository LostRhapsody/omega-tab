<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    size?: "sm" | "md" | "lg" | "xl";
    color?: "primary" | "accent" | "muted";
  }>(),
  {
    size: "md",
    color: "primary",
  },
);

const sizePixels = computed(() => {
  switch (props.size) {
    case "sm":
      return 16;
    case "md":
      return 24;
    case "lg":
      return 32;
    case "xl":
      return 48;
    default:
      return 24;
  }
});
</script>

<template>
  <svg
    :class="['tp-spinner', `tp-spinner--${color}`]"
    :width="sizePixels"
    :height="sizePixels"
    viewBox="0 0 24 24"
    fill="none"
    role="status"
    aria-label="Loading"
  >
    <circle
      class="tp-spinner__track"
      cx="12"
      cy="12"
      r="10"
      stroke="currentColor"
      stroke-width="2"
    />
    <path
      class="tp-spinner__head"
      d="M12 2a10 10 0 0 1 10 10"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
    />
  </svg>
</template>

<style>
@keyframes tp-spinner-rotate {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.tp-spinner {
  animation: tp-spinner-rotate 1s linear infinite;
}

.tp-spinner__track {
  opacity: 0.2;
}

.tp-spinner__head {
  opacity: 1;
}

/* Colors */
.tp-spinner--primary {
  color: var(--tp-text-primary);
}

.tp-spinner--accent {
  color: var(--tp-accent);
}

.tp-spinner--muted {
  color: var(--tp-text-muted);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .tp-spinner {
    animation: none;
  }
}
</style>

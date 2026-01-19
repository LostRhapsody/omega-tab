<script setup lang="ts">
import { computed } from "vue";
import TpIcon from "./TpIcon.vue";

const props = withDefaults(
  defineProps<{
    variant?: "primary" | "secondary" | "ghost" | "danger";
    size?: "sm" | "md" | "lg";
    loading?: boolean;
    disabled?: boolean;
    iconOnly?: boolean;
    icon?: string;
    iconPosition?: "left" | "right";
  }>(),
  {
    variant: "secondary",
    size: "md",
    loading: false,
    disabled: false,
    iconOnly: false,
    iconPosition: "left",
  },
);

const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

const classes = computed(() => [
  "tp-button",
  `tp-button--${props.variant}`,
  `tp-button--${props.size}`,
  {
    "tp-button--loading": props.loading,
    "tp-button--icon-only": props.iconOnly,
    "tp-button--disabled": props.disabled || props.loading,
  },
]);

const handleClick = (event: MouseEvent) => {
  if (!props.disabled && !props.loading) {
    emit("click", event);
  }
};
</script>

<template>
  <button :class="classes" :disabled="disabled || loading" @click="handleClick" v-bind="$attrs">
    <span v-if="loading" class="tp-button__spinner">
      <TpIcon name="spinner" :size="size === 'sm' ? 'xs' : 'sm'" spin />
    </span>
    <span class="tp-button__content" :class="{ 'tp-button__content--hidden': loading }">
      <TpIcon
        v-if="icon && iconPosition === 'left'"
        :name="icon"
        :size="size === 'sm' ? 'xs' : 'sm'"
        class="tp-button__icon tp-button__icon--left"
      />
      <slot />
      <TpIcon
        v-if="icon && iconPosition === 'right'"
        :name="icon"
        :size="size === 'sm' ? 'xs' : 'sm'"
        class="tp-button__icon tp-button__icon--right"
      />
    </span>
  </button>
</template>

<style>
.tp-button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--tp-space-2);
  font-family: var(--tp-font-sans);
  font-weight: var(--tp-font-medium);
  cursor: pointer;
  border: var(--tp-border-width) solid transparent;
  border-radius: var(--tp-radius-sm);
  transition:
    background-color var(--tp-transition-fast),
    border-color var(--tp-transition-fast),
    color var(--tp-transition-fast),
    opacity var(--tp-transition-fast);
  position: relative;
  white-space: nowrap;
}

/* Sizes */
.tp-button--sm {
  padding: var(--tp-space-1) var(--tp-space-2);
  font-size: var(--tp-text-sm);
  min-height: 28px;
}

.tp-button--md {
  padding: var(--tp-space-2) var(--tp-space-4);
  font-size: var(--tp-text-base);
  min-height: 36px;
}

.tp-button--lg {
  padding: var(--tp-space-3) var(--tp-space-6);
  font-size: var(--tp-text-lg);
  min-height: 44px;
}

/* Variants */
.tp-button--primary {
  background: var(--tp-text-primary);
  color: var(--tp-bg-primary);
  border-color: var(--tp-text-primary);
}

.tp-button--primary:hover:not(:disabled) {
  background: var(--tp-text-secondary);
  border-color: var(--tp-text-secondary);
}

.tp-button--primary:active:not(:disabled) {
  background: var(--tp-text-primary);
}

.tp-button--secondary {
  background: transparent;
  color: var(--tp-text-primary);
  border-color: var(--tp-border-strong);
}

.tp-button--secondary:hover:not(:disabled) {
  border-color: var(--tp-text-primary);
  background: var(--tp-bg-secondary);
}

.tp-button--ghost {
  background: transparent;
  color: var(--tp-text-primary);
  border-color: transparent;
}

.tp-button--ghost:hover:not(:disabled) {
  background: var(--tp-bg-secondary);
}

.tp-button--danger {
  background: transparent;
  color: var(--tp-error);
  border-color: var(--tp-error);
}

.tp-button--danger:hover:not(:disabled) {
  background: var(--tp-error);
  color: var(--tp-bg-primary);
}

/* States */
.tp-button--disabled,
.tp-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-button--loading {
  cursor: wait;
}

/* Icon only */
.tp-button--icon-only.tp-button--sm {
  padding: var(--tp-space-1);
  min-width: 28px;
}

.tp-button--icon-only.tp-button--md {
  padding: var(--tp-space-2);
  min-width: 36px;
}

.tp-button--icon-only.tp-button--lg {
  padding: var(--tp-space-3);
  min-width: 44px;
}

/* Content and spinner */
.tp-button__content {
  display: inline-flex;
  align-items: center;
  gap: var(--tp-space-2);
}

.tp-button__content--hidden {
  visibility: hidden;
}

.tp-button__spinner {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* Focus styles */
.tp-button:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

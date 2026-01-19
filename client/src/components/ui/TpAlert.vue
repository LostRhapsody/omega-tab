<script setup lang="ts">
import { computed } from "vue";
import TpIcon from "./TpIcon.vue";

const props = withDefaults(
  defineProps<{
    type?: "info" | "success" | "warning" | "error";
    title?: string;
    closable?: boolean;
    icon?: string | boolean;
  }>(),
  {
    type: "info",
    closable: false,
    icon: true,
  },
);

const emit = defineEmits<{
  close: [];
}>();

const defaultIcons: Record<string, string> = {
  info: "info",
  success: "check",
  warning: "warning",
  error: "warning",
};

const iconName = computed(() => {
  if (props.icon === false) return null;
  if (typeof props.icon === "string") return props.icon;
  return defaultIcons[props.type];
});
</script>

<template>
  <div :class="['tp-alert', `tp-alert--${type}`]" role="alert">
    <TpIcon v-if="iconName" :name="iconName" class="tp-alert__icon" />

    <div class="tp-alert__content">
      <div v-if="title" class="tp-alert__title">{{ title }}</div>
      <div class="tp-alert__message">
        <slot />
      </div>
    </div>

    <button
      v-if="closable"
      type="button"
      class="tp-alert__close"
      aria-label="Close alert"
      @click="emit('close')"
    >
      [x]
    </button>
  </div>
</template>

<style>
.tp-alert {
  display: flex;
  align-items: flex-start;
  gap: var(--tp-space-3);
  padding: var(--tp-space-3) var(--tp-space-4);
  border: var(--tp-border-width) solid;
  border-radius: var(--tp-radius-sm);
}

/* Types */
.tp-alert--info {
  background: var(--tp-bg-secondary);
  border-color: var(--tp-border);
  color: var(--tp-text-primary);
}

.tp-alert--success {
  background: var(--tp-accent-glow);
  border-color: var(--tp-accent);
  color: var(--tp-text-primary);
}

.tp-alert--success .tp-alert__icon {
  color: var(--tp-success);
}

.tp-alert--warning {
  background: rgba(245, 158, 11, 0.1);
  border-color: var(--tp-warning);
  color: var(--tp-text-primary);
}

.tp-alert--warning .tp-alert__icon {
  color: var(--tp-warning);
}

.tp-alert--error {
  background: var(--tp-error-bg);
  border-color: var(--tp-error);
  color: var(--tp-text-primary);
}

.tp-alert--error .tp-alert__icon {
  color: var(--tp-error);
}

.tp-alert__icon {
  flex-shrink: 0;
  margin-top: 2px;
}

.tp-alert__content {
  flex: 1;
  min-width: 0;
}

.tp-alert__title {
  font-weight: var(--tp-font-bold);
  margin-bottom: var(--tp-space-1);
}

.tp-alert__message {
  font-size: var(--tp-text-sm);
  line-height: var(--tp-leading-normal);
}

.tp-alert__close {
  flex-shrink: 0;
  font-family: var(--tp-font-mono);
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  transition: color var(--tp-transition-fast);
}

.tp-alert__close:hover {
  color: var(--tp-text-primary);
}

.tp-alert__close:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

<script setup lang="ts">
import { ref, watch, onUnmounted } from "vue";
import TpIcon from "./TpIcon.vue";

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    message: string;
    type?: "info" | "success" | "warning" | "error";
    duration?: number;
    position?: "top" | "bottom";
    closable?: boolean;
  }>(),
  {
    type: "info",
    duration: 5000,
    position: "bottom",
    closable: true,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  close: [];
}>();

let timeout: ReturnType<typeof setTimeout> | null = null;

const close = () => {
  emit("update:modelValue", false);
  emit("close");
};

const startTimer = () => {
  if (props.duration > 0) {
    timeout = setTimeout(close, props.duration);
  }
};

const clearTimer = () => {
  if (timeout) {
    clearTimeout(timeout);
    timeout = null;
  }
};

watch(
  () => props.modelValue,
  (isVisible) => {
    if (isVisible) {
      startTimer();
    } else {
      clearTimer();
    }
  },
);

onUnmounted(() => {
  clearTimer();
});

const typeIcons: Record<string, string> = {
  info: "info",
  success: "check",
  warning: "warning",
  error: "warning",
};
</script>

<template>
  <Teleport to="body">
    <Transition name="tp-snackbar">
      <div
        v-if="modelValue"
        :class="['tp-snackbar', `tp-snackbar--${type}`, `tp-snackbar--${position}`]"
        role="alert"
        @mouseenter="clearTimer"
        @mouseleave="startTimer"
      >
        <TpIcon :name="typeIcons[type]" size="sm" class="tp-snackbar__icon" />

        <span class="tp-snackbar__message">{{ message }}</span>

        <button
          v-if="closable"
          type="button"
          class="tp-snackbar__close"
          aria-label="Close notification"
          @click="close"
        >
          [x]
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<style>
.tp-snackbar {
  position: fixed;
  left: 50%;
  transform: translateX(-50%);
  z-index: var(--tp-z-toast);
  display: flex;
  align-items: center;
  gap: var(--tp-space-3);
  min-width: 300px;
  max-width: calc(100vw - var(--tp-space-8));
  padding: var(--tp-space-3) var(--tp-space-4);
  background: var(--tp-text-primary);
  color: var(--tp-bg-primary);
  border-radius: var(--tp-radius-sm);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

/* Position */
.tp-snackbar--top {
  top: var(--tp-space-4);
}

.tp-snackbar--bottom {
  bottom: var(--tp-space-4);
}

/* Types */
.tp-snackbar--success .tp-snackbar__icon {
  color: var(--tp-success);
}

.tp-snackbar--warning .tp-snackbar__icon {
  color: var(--tp-warning);
}

.tp-snackbar--error .tp-snackbar__icon {
  color: var(--tp-error);
}

.tp-snackbar__icon {
  flex-shrink: 0;
}

.tp-snackbar__message {
  flex: 1;
  font-size: var(--tp-text-sm);
  line-height: var(--tp-leading-normal);
}

.tp-snackbar__close {
  flex-shrink: 0;
  font-family: var(--tp-font-mono);
  font-size: var(--tp-text-sm);
  color: inherit;
  opacity: 0.7;
  transition: opacity var(--tp-transition-fast);
}

.tp-snackbar__close:hover {
  opacity: 1;
}

.tp-snackbar__close:focus-visible {
  outline: 2px solid var(--tp-bg-primary);
  outline-offset: 2px;
}

/* Transitions */
.tp-snackbar-enter-active,
.tp-snackbar-leave-active {
  transition:
    opacity var(--tp-transition-base),
    transform var(--tp-transition-base);
}

.tp-snackbar-enter-from,
.tp-snackbar-leave-to {
  opacity: 0;
}

.tp-snackbar--top.tp-snackbar-enter-from,
.tp-snackbar--top.tp-snackbar-leave-to {
  transform: translateX(-50%) translateY(-20px);
}

.tp-snackbar--bottom.tp-snackbar-enter-from,
.tp-snackbar--bottom.tp-snackbar-leave-to {
  transform: translateX(-50%) translateY(20px);
}
</style>

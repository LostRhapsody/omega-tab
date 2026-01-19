<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    title?: string;
    size?: "sm" | "md" | "lg" | "xl" | "full";
    closable?: boolean;
    closeOnOverlay?: boolean;
    closeOnEscape?: boolean;
    persistent?: boolean;
    initialFocus?: string;
  }>(),
  {
    size: "md",
    closable: true,
    closeOnOverlay: true,
    closeOnEscape: true,
    persistent: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  close: [];
  open: [];
}>();

const modalRef = ref<HTMLDivElement>();
const previousActiveElement = ref<HTMLElement | null>(null);

const close = () => {
  if (!props.closable && !props.closeOnOverlay) return;
  emit("update:modelValue", false);
  emit("close");
};

const handleOverlayClick = (event: MouseEvent) => {
  if (props.closeOnOverlay && event.target === event.currentTarget) {
    close();
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === "Escape" && props.closeOnEscape) {
    close();
  }

  // Trap focus within modal
  if (event.key === "Tab" && modalRef.value) {
    const focusableElements = modalRef.value.querySelectorAll<HTMLElement>(
      'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
    );
    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];

    if (event.shiftKey && document.activeElement === firstElement) {
      event.preventDefault();
      lastElement?.focus();
    } else if (!event.shiftKey && document.activeElement === lastElement) {
      event.preventDefault();
      firstElement?.focus();
    }
  }
};

watch(
  () => props.modelValue,
  async (isOpen) => {
    if (isOpen) {
      previousActiveElement.value = document.activeElement as HTMLElement;
      document.body.style.overflow = "hidden";
      document.addEventListener("keydown", handleKeydown);
      emit("open");

      await nextTick();
      // Focus the initial focus element if specified, otherwise the first focusable
      let focusTarget: HTMLElement | null | undefined = null;
      if (props.initialFocus) {
        focusTarget = modalRef.value?.querySelector<HTMLElement>(props.initialFocus);
      }
      if (!focusTarget) {
        focusTarget = modalRef.value?.querySelector<HTMLElement>(
          'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])',
        );
      }
      focusTarget?.focus();
    } else {
      document.body.style.overflow = "";
      document.removeEventListener("keydown", handleKeydown);
      // Restore focus to the previously focused element
      previousActiveElement.value?.focus();
    }
  },
);

onUnmounted(() => {
  document.body.style.overflow = "";
  document.removeEventListener("keydown", handleKeydown);
});
</script>

<template>
  <Teleport to="body">
    <Transition name="tp-modal">
      <div v-if="modelValue" class="tp-modal-overlay" @click="handleOverlayClick">
        <div
          ref="modalRef"
          :class="['tp-modal', `tp-modal--${size}`]"
          role="dialog"
          aria-modal="true"
          :aria-labelledby="title ? 'tp-modal-title' : undefined"
        >
          <div v-if="title || closable || $slots.header" class="tp-modal__header">
            <slot name="header">
              <h2 v-if="title" id="tp-modal-title" class="tp-modal__title">
                {{ title }}
              </h2>
            </slot>
            <button
              v-if="closable"
              type="button"
              class="tp-modal__close"
              aria-label="Close modal"
              @click="close"
            >
              [x]
            </button>
          </div>

          <div class="tp-modal__body">
            <slot />
          </div>

          <div v-if="$slots.actions" class="tp-modal__actions">
            <slot name="actions" />
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style>
.tp-modal-overlay {
  position: fixed;
  inset: 0;
  z-index: var(--tp-z-modal);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--tp-space-4);
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(2px);
}

.tp-modal {
  position: relative;
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - var(--tp-space-8));
  background: var(--tp-bg-primary);
  border: var(--tp-border-width-thick) solid var(--tp-rule);
  overflow: hidden;
}

/* Sizes */
.tp-modal--sm {
  width: 100%;
  max-width: 400px;
}

.tp-modal--md {
  width: 100%;
  max-width: 560px;
}

.tp-modal--lg {
  width: 100%;
  max-width: 720px;
}

.tp-modal--xl {
  width: 100%;
  max-width: 960px;
}

.tp-modal--full {
  width: calc(100vw - var(--tp-space-8));
  height: calc(100vh - var(--tp-space-8));
  max-width: none;
  max-height: none;
}

/* Header */
.tp-modal__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--tp-space-4);
  padding: var(--tp-space-4) var(--tp-space-6);
  border-bottom: var(--tp-border-width) solid var(--tp-border);
  flex-shrink: 0;
}

.tp-modal__title {
  font-size: var(--tp-text-lg);
  font-weight: var(--tp-font-bold);
  margin: 0;
  line-height: var(--tp-leading-tight);
}

.tp-modal__close {
  flex-shrink: 0;
  padding: var(--tp-space-1) var(--tp-space-2);
  font-family: var(--tp-font-mono);
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  transition: color var(--tp-transition-fast);
}

.tp-modal__close:hover {
  color: var(--tp-text-primary);
}

.tp-modal__close:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

/* Body */
.tp-modal__body {
  flex: 1;
  padding: var(--tp-space-6);
  overflow-y: auto;
}

/* Actions */
.tp-modal__actions {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--tp-space-3);
  padding: var(--tp-space-4) var(--tp-space-6);
  border-top: var(--tp-border-width) solid var(--tp-border);
  flex-shrink: 0;
}

/* Transitions - defined in tokens.css */
</style>

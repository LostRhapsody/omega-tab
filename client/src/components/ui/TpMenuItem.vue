<script setup lang="ts">
import TpIcon from "./TpIcon.vue";

const props = withDefaults(
  defineProps<{
    icon?: string;
    danger?: boolean;
    disabled?: boolean;
  }>(),
  {
    danger: false,
    disabled: false,
  },
);

const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

const handleClick = (event: MouseEvent) => {
  if (!props.disabled) {
    emit("click", event);
  }
};

const handleKeydown = (event: KeyboardEvent) => {
  if ((event.key === "Enter" || event.key === " ") && !props.disabled) {
    event.preventDefault();
    emit("click", event as unknown as MouseEvent);
  }
};
</script>

<template>
  <button
    :class="[
      'tp-menu-item',
      {
        'tp-menu-item--danger': danger,
        'tp-menu-item--disabled': disabled,
      },
    ]"
    role="menuitem"
    :tabindex="disabled ? -1 : 0"
    :disabled="disabled"
    @click="handleClick"
    @keydown="handleKeydown"
  >
    <TpIcon v-if="icon" :name="icon" size="sm" class="tp-menu-item__icon" />
    <span class="tp-menu-item__content">
      <slot />
    </span>
    <span v-if="$slots.append" class="tp-menu-item__append">
      <slot name="append" />
    </span>
  </button>
</template>

<style>
.tp-menu-item {
  display: flex;
  align-items: center;
  gap: var(--tp-space-3);
  width: 100%;
  padding: var(--tp-space-2) var(--tp-space-3);
  background: transparent;
  border: none;
  font-family: var(--tp-font-sans);
  font-size: var(--tp-text-base);
  color: var(--tp-text-primary);
  text-align: left;
  cursor: pointer;
  transition: background-color var(--tp-transition-fast);
}

.tp-menu-item:hover:not(:disabled) {
  background: var(--tp-bg-secondary);
}

.tp-menu-item--danger {
  color: var(--tp-error);
}

.tp-menu-item--danger:hover:not(:disabled) {
  background: var(--tp-error-bg);
}

.tp-menu-item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-menu-item__icon {
  flex-shrink: 0;
  color: var(--tp-text-muted);
}

.tp-menu-item--danger .tp-menu-item__icon {
  color: var(--tp-error);
}

.tp-menu-item__content {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tp-menu-item__append {
  flex-shrink: 0;
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
}

/* Focus styles */
.tp-menu-item:focus-visible {
  outline: none;
  background: var(--tp-bg-secondary);
}
</style>

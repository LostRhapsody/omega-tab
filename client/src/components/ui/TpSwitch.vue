<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    label?: string;
    disabled?: boolean;
    size?: "sm" | "md";
  }>(),
  {
    modelValue: false,
    disabled: false,
    size: "md",
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  change: [value: boolean];
}>();

const toggle = () => {
  if (props.disabled) return;
  console.log("Toggling switch:", props.modelValue);
  const newValue = !props.modelValue;
  console.log("New value:", newValue);
  emit("update:modelValue", newValue);
  emit("change", newValue);
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === "Enter" || event.key === " ") {
    event.preventDefault();
    toggle();
  }
};
</script>

<template>
  <label
    :class="[
      'tp-switch',
      `tp-switch--${size}`,
      {
        'tp-switch--checked': modelValue,
        'tp-switch--disabled': disabled,
      },
    ]"
  >
    <span
      class="tp-switch__track"
      role="switch"
      :aria-checked="modelValue"
      :tabindex="disabled ? -1 : 0"
      @click="toggle"
      @keydown="handleKeydown"
    >
      <span class="tp-switch__thumb" />
    </span>
    <span v-if="label || $slots.default" class="tp-switch__label">
      <slot>{{ label }}</slot>
    </span>
  </label>
</template>

<style>
.tp-switch {
  display: inline-flex;
  align-items: center;
  gap: var(--tp-space-3);
  cursor: pointer;
}

.tp-switch--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-switch__track {
  position: relative;
  display: inline-flex;
  align-items: center;
  flex-shrink: 0;
  background: var(--tp-bg-tertiary);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: 999px;
  transition:
    background-color var(--tp-transition-fast),
    border-color var(--tp-transition-fast);
}

/* Sizes */
.tp-switch--sm .tp-switch__track {
  width: 32px;
  height: 18px;
}

.tp-switch--md .tp-switch__track {
  width: 40px;
  height: 22px;
}

.tp-switch--sm .tp-switch__thumb {
  width: 12px;
  height: 12px;
}

.tp-switch--md .tp-switch__thumb {
  width: 16px;
  height: 16px;
}

.tp-switch__thumb {
  position: absolute;
  left: 2px;
  background: var(--tp-text-muted);
  border-radius: 50%;
  transition:
    transform var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
}

/* Checked state */
.tp-switch--checked .tp-switch__track {
  background: var(--tp-accent-dim);
  border-color: var(--tp-accent);
}

.tp-switch--checked .tp-switch__thumb {
  background: var(--tp-accent);
}

.tp-switch--sm.tp-switch--checked .tp-switch__thumb {
  transform: translateX(14px);
}

.tp-switch--md.tp-switch--checked .tp-switch__thumb {
  transform: translateX(18px);
}

.tp-switch__label {
  font-size: var(--tp-text-base);
  color: var(--tp-text-primary);
  user-select: none;
}

/* Focus styles */
.tp-switch__track:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

/* Hover */
.tp-switch:not(.tp-switch--disabled) .tp-switch__track:hover {
  border-color: var(--tp-border-strong);
}

.tp-switch--checked:not(.tp-switch--disabled) .tp-switch__track:hover {
  border-color: var(--tp-accent);
}
</style>

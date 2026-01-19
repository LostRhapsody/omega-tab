<script setup lang="ts">
import { computed } from "vue";

export interface RadioOption {
  value: string | number;
  label: string;
  disabled?: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue: string | number | null;
    options: RadioOption[];
    name?: string;
    label?: string;
    disabled?: boolean;
    inline?: boolean;
  }>(),
  {
    modelValue: null,
    disabled: false,
    inline: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string | number];
  change: [value: string | number];
}>();

const groupName = computed(() => props.name || `tp-radio-${Math.random().toString(36).slice(2)}`);

const select = (value: string | number, optionDisabled?: boolean) => {
  if (props.disabled || optionDisabled) return;
  emit("update:modelValue", value);
  emit("change", value);
};
</script>

<template>
  <fieldset
    :class="[
      'tp-radio-group',
      {
        'tp-radio-group--inline': inline,
        'tp-radio-group--disabled': disabled,
      },
    ]"
  >
    <legend v-if="label" class="tp-radio-group__label">
      {{ label }}
    </legend>

    <div class="tp-radio-group__options">
      <label
        v-for="option in options"
        :key="option.value"
        :class="[
          'tp-radio',
          {
            'tp-radio--checked': modelValue === option.value,
            'tp-radio--disabled': disabled || option.disabled,
          },
        ]"
      >
        <input
          type="radio"
          :name="groupName"
          :value="option.value"
          :checked="modelValue === option.value"
          :disabled="disabled || option.disabled"
          class="tp-radio__input"
          @change="select(option.value, option.disabled)"
        />
        <span class="tp-radio__indicator">
          <span class="tp-radio__dot" />
        </span>
        <span class="tp-radio__label">{{ option.label }}</span>
      </label>
    </div>
  </fieldset>
</template>

<style>
.tp-radio-group {
  border: none;
  padding: 0;
  margin: 0;
}

.tp-radio-group__label {
  display: block;
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-2);
}

.tp-radio-group__options {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.tp-radio-group--inline .tp-radio-group__options {
  flex-direction: row;
  flex-wrap: wrap;
  gap: var(--tp-space-4);
}

.tp-radio-group--disabled {
  opacity: 0.5;
}

.tp-radio {
  display: inline-flex;
  align-items: center;
  gap: var(--tp-space-2);
  cursor: pointer;
}

.tp-radio--disabled {
  cursor: not-allowed;
}

.tp-radio__input {
  position: absolute;
  opacity: 0;
  width: 0;
  height: 0;
}

.tp-radio__indicator {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: 50%;
  background: transparent;
  flex-shrink: 0;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
}

.tp-radio:hover:not(.tp-radio--disabled) .tp-radio__indicator {
  border-color: var(--tp-border-strong);
}

.tp-radio--checked .tp-radio__indicator {
  border-color: var(--tp-accent);
}

.tp-radio__dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--tp-accent);
  transform: scale(0);
  transition: transform var(--tp-transition-fast);
}

.tp-radio--checked .tp-radio__dot {
  transform: scale(1);
}

.tp-radio__label {
  font-size: var(--tp-text-base);
  color: var(--tp-text-primary);
  user-select: none;
}

/* Focus styles */
.tp-radio__input:focus-visible + .tp-radio__indicator {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

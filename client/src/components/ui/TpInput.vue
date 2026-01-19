<script setup lang="ts">
import { ref, computed, useAttrs } from "vue";
import TpIcon from "./TpIcon.vue";

const props = withDefaults(
  defineProps<{
    modelValue?: string;
    inputId?: string;
    label?: string;
    placeholder?: string;
    type?: "text" | "email" | "password" | "url" | "search" | "tel" | "number";
    error?: string;
    hint?: string;
    disabled?: boolean;
    required?: boolean;
    showCursor?: boolean;
    prependIcon?: string;
    appendIcon?: string;
  }>(),
  {
    modelValue: "",
    inputId: "",
    type: "text",
    disabled: false,
    required: false,
    showCursor: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
  focus: [event: FocusEvent];
  blur: [event: FocusEvent];
  enter: [event: KeyboardEvent];
  prependClick: [];
  appendClick: [];
}>();

const attrs = useAttrs();
const inputRef = ref<HTMLInputElement>();
const isFocused = ref(false);

const inputValue = computed({
  get: () => props.modelValue,
  set: (value: string) => emit("update:modelValue", value),
});

const handleFocus = (event: FocusEvent) => {
  isFocused.value = true;
  emit("focus", event);
};

const handleBlur = (event: FocusEvent) => {
  isFocused.value = false;
  emit("blur", event);
};

const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === "Enter") {
    emit("enter", event);
  }
};

const focus = () => inputRef.value?.focus();
const blur = () => inputRef.value?.blur();
const select = () => inputRef.value?.select();

defineExpose({ focus, blur, select, inputRef });
</script>

<template>
  <div
    :class="[
      'tp-input-wrapper',
      {
        'tp-input-wrapper--focused': isFocused,
        'tp-input-wrapper--error': error,
        'tp-input-wrapper--disabled': disabled,
      },
    ]"
  >
    <label v-if="label" class="tp-input__label">
      {{ label }}
      <span v-if="required" class="tp-input__required">*</span>
    </label>

    <div class="tp-input__container">
      <span
        v-if="prependIcon || $slots.prepend"
        class="tp-input__prepend"
        @click="emit('prependClick')"
      >
        <slot name="prepend">
          <TpIcon v-if="prependIcon" :name="prependIcon" size="sm" />
        </slot>
      </span>

      <input
        ref="inputRef"
        v-model="inputValue"
        :id="inputId"
        :type="type"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        class="tp-input"
        v-bind="attrs"
        @focus="handleFocus"
        @blur="handleBlur"
        @keydown="handleKeydown"
      />

      <span v-if="showCursor && isFocused" class="tp-cursor" />

      <span
        v-if="appendIcon || $slots.append"
        class="tp-input__append"
        @click="emit('appendClick')"
      >
        <slot name="append">
          <TpIcon v-if="appendIcon" :name="appendIcon" size="sm" />
        </slot>
      </span>
    </div>

    <span v-if="error" class="tp-input__error">{{ error }}</span>
    <span v-else-if="hint" class="tp-input__hint">{{ hint }}</span>
  </div>
</template>

<style>
.tp-input-wrapper {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  width: 100%;
}

.tp-input__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.tp-input__required {
  color: var(--tp-error);
  margin-left: var(--tp-space-1);
}

.tp-input__container {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  border-bottom: var(--tp-border-width) solid var(--tp-border);
  background: transparent;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
  padding: var(--tp-space-2) 0;
}

.tp-input-wrapper--focused .tp-input__container {
  border-color: var(--tp-accent);
}

.tp-input-wrapper--error .tp-input__container {
  border-color: var(--tp-error);
}

.tp-input-wrapper--disabled .tp-input__container {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-input {
  flex: 1;
  border: none;
  background: transparent;
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-sans);
  padding: 0;
  min-width: 0;
}

.tp-input::placeholder {
  color: var(--tp-text-muted);
}

.tp-input:focus {
  outline: none;
}

.tp-input:disabled {
  cursor: not-allowed;
}

.tp-input__prepend,
.tp-input__append {
  display: flex;
  align-items: center;
  color: var(--tp-text-muted);
  flex-shrink: 0;
}

.tp-input__prepend:hover,
.tp-input__append:hover {
  color: var(--tp-text-primary);
  cursor: pointer;
}

.tp-input__error {
  font-size: var(--tp-text-sm);
  color: var(--tp-error);
}

.tp-input__hint {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}

/* Password toggle - hide browser default */
.tp-input[type="password"]::-ms-reveal,
.tp-input[type="password"]::-ms-clear {
  display: none;
}

/* Number input - hide spinners */
.tp-input[type="number"]::-webkit-outer-spin-button,
.tp-input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.tp-input[type="number"] {
  -moz-appearance: textfield;
}
</style>

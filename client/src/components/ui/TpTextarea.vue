<script setup lang="ts">
import { ref, computed, watch, onMounted, useAttrs } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue?: string;
    label?: string;
    placeholder?: string;
    error?: string;
    hint?: string;
    disabled?: boolean;
    required?: boolean;
    rows?: number;
    maxRows?: number;
    autoResize?: boolean;
  }>(),
  {
    modelValue: "",
    rows: 3,
    disabled: false,
    required: false,
    autoResize: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string];
  focus: [event: FocusEvent];
  blur: [event: FocusEvent];
}>();

const attrs = useAttrs();
const textareaRef = ref<HTMLTextAreaElement>();
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

const resize = () => {
  if (!props.autoResize || !textareaRef.value) return;

  const textarea = textareaRef.value;
  textarea.style.height = "auto";

  const lineHeight = parseInt(getComputedStyle(textarea).lineHeight);
  const minHeight = lineHeight * props.rows;
  const maxHeight = props.maxRows ? lineHeight * props.maxRows : Infinity;

  const newHeight = Math.min(Math.max(textarea.scrollHeight, minHeight), maxHeight);
  textarea.style.height = `${newHeight}px`;
};

watch(
  () => props.modelValue,
  () => {
    if (props.autoResize) {
      // Use nextTick to ensure DOM is updated
      setTimeout(resize, 0);
    }
  },
);

onMounted(() => {
  if (props.autoResize) {
    resize();
  }
});

const focus = () => textareaRef.value?.focus();
const blur = () => textareaRef.value?.blur();
const select = () => textareaRef.value?.select();

defineExpose({ focus, blur, select, textareaRef });
</script>

<template>
  <div
    :class="[
      'tp-textarea-wrapper',
      {
        'tp-textarea-wrapper--focused': isFocused,
        'tp-textarea-wrapper--error': error,
        'tp-textarea-wrapper--disabled': disabled,
      },
    ]"
  >
    <label v-if="label" class="tp-textarea__label">
      {{ label }}
      <span v-if="required" class="tp-textarea__required">*</span>
    </label>

    <div class="tp-textarea__container">
      <textarea
        ref="textareaRef"
        v-model="inputValue"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :rows="rows"
        class="tp-textarea"
        v-bind="attrs"
        @focus="handleFocus"
        @blur="handleBlur"
        @input="resize"
      />
    </div>

    <span v-if="error" class="tp-textarea__error">{{ error }}</span>
    <span v-else-if="hint" class="tp-textarea__hint">{{ hint }}</span>
  </div>
</template>

<style>
.tp-textarea-wrapper {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  width: 100%;
}

.tp-textarea__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.tp-textarea__required {
  color: var(--tp-error);
  margin-left: var(--tp-space-1);
}

.tp-textarea__container {
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  background: transparent;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
}

.tp-textarea-wrapper--focused .tp-textarea__container {
  border-color: var(--tp-accent);
}

.tp-textarea-wrapper--error .tp-textarea__container {
  border-color: var(--tp-error);
}

.tp-textarea-wrapper--disabled .tp-textarea__container {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-textarea {
  width: 100%;
  border: none;
  background: transparent;
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-sans);
  line-height: var(--tp-leading-normal);
  padding: var(--tp-space-2) var(--tp-space-3);
  resize: vertical;
  min-height: auto;
}

.tp-textarea::placeholder {
  color: var(--tp-text-muted);
}

.tp-textarea:focus {
  outline: none;
}

.tp-textarea:disabled {
  cursor: not-allowed;
  resize: none;
}

.tp-textarea__error {
  font-size: var(--tp-text-sm);
  color: var(--tp-error);
}

.tp-textarea__hint {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}
</style>

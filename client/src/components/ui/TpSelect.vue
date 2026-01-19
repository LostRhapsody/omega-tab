<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from "vue";
import TpIcon from "./TpIcon.vue";

export interface SelectOption {
  value: string | number;
  label: string;
  disabled?: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue?: string | number | null;
    options: SelectOption[];
    label?: string;
    placeholder?: string;
    error?: string;
    disabled?: boolean;
    required?: boolean;
  }>(),
  {
    modelValue: null,
    disabled: false,
    required: false,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string | number | null];
  change: [option: SelectOption | null];
}>();

const triggerRef = ref<HTMLButtonElement>();
const menuRef = ref<HTMLUListElement>();
const isOpen = ref(false);
const focusedIndex = ref(-1);
const menuPosition = ref({ top: "0px", left: "0px", width: "0px" });

const selectedOption = computed(
  () => props.options.find((opt) => opt.value === props.modelValue) ?? null,
);

const displayText = computed(() => selectedOption.value?.label ?? props.placeholder ?? "Select...");

const updateMenuPosition = () => {
  if (!triggerRef.value) return;
  const rect = triggerRef.value.getBoundingClientRect();
  menuPosition.value = {
    top: `${rect.bottom + window.scrollY + 4}px`,
    left: `${rect.left + window.scrollX}px`,
    width: `${rect.width}px`,
  };
};

const toggle = () => {
  if (props.disabled) return;
  isOpen.value = !isOpen.value;
  if (isOpen.value) {
    focusedIndex.value = props.options.findIndex((opt) => opt.value === props.modelValue);
    if (focusedIndex.value === -1) focusedIndex.value = 0;
    nextTick(updateMenuPosition);
  }
};

const close = () => {
  isOpen.value = false;
  focusedIndex.value = -1;
};

const select = (option: SelectOption) => {
  if (option.disabled) return;
  emit("update:modelValue", option.value);
  emit("change", option);
  close();
  triggerRef.value?.focus();
};

const handleKeydown = (event: KeyboardEvent) => {
  if (props.disabled) return;

  switch (event.key) {
    case "Enter":
    case " ":
      event.preventDefault();
      if (isOpen.value && focusedIndex.value >= 0) {
        const option = props.options[focusedIndex.value];
        if (option && !option.disabled) select(option);
      } else {
        toggle();
      }
      break;

    case "ArrowDown":
      event.preventDefault();
      if (!isOpen.value) {
        isOpen.value = true;
        focusedIndex.value = 0;
      } else {
        focusedIndex.value = Math.min(focusedIndex.value + 1, props.options.length - 1);
      }
      break;

    case "ArrowUp":
      event.preventDefault();
      if (isOpen.value) {
        focusedIndex.value = Math.max(focusedIndex.value - 1, 0);
      }
      break;

    case "Escape":
      event.preventDefault();
      close();
      break;

    case "Tab":
      close();
      break;

    case "Home":
      event.preventDefault();
      if (isOpen.value) focusedIndex.value = 0;
      break;

    case "End":
      event.preventDefault();
      if (isOpen.value) focusedIndex.value = props.options.length - 1;
      break;
  }
};

const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node;
  if (
    triggerRef.value &&
    !triggerRef.value.contains(target) &&
    menuRef.value &&
    !menuRef.value.contains(target)
  ) {
    close();
  }
};

watch(isOpen, (open) => {
  if (open) {
    document.addEventListener("click", handleClickOutside);
  } else {
    document.removeEventListener("click", handleClickOutside);
  }
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
});
</script>

<template>
  <div
    :class="[
      'tp-select',
      {
        'tp-select--open': isOpen,
        'tp-select--error': error,
        'tp-select--disabled': disabled,
      },
    ]"
  >
    <label v-if="label" class="tp-select__label">
      {{ label }}
      <span v-if="required" class="tp-select__required">*</span>
    </label>

    <button
      ref="triggerRef"
      type="button"
      class="tp-select__trigger"
      :disabled="disabled"
      :aria-expanded="isOpen"
      aria-haspopup="listbox"
      @click="toggle"
      @keydown="handleKeydown"
    >
      <span class="tp-select__value" :class="{ 'tp-select__value--placeholder': !selectedOption }">
        <slot name="selection" :option="selectedOption">
          {{ displayText }}
        </slot>
      </span>
      <span class="tp-select__arrow">
        <TpIcon :name="isOpen ? 'chevron-up' : 'chevron-down'" size="sm" />
      </span>
    </button>

    <Teleport to="body">
      <Transition name="tp-dropdown">
        <ul
          v-if="isOpen"
          ref="menuRef"
          class="tp-select__menu"
          role="listbox"
          :style="{
            position: 'absolute',
            top: menuPosition.top,
            left: menuPosition.left,
            width: menuPosition.width,
          }"
        >
          <li
            v-for="(option, index) in options"
            :key="option.value"
            :class="[
              'tp-select__item',
              {
                'tp-select__item--focused': focusedIndex === index,
                'tp-select__item--selected': option.value === modelValue,
                'tp-select__item--disabled': option.disabled,
              },
            ]"
            role="option"
            :aria-selected="option.value === modelValue"
            @click="select(option)"
            @mouseenter="focusedIndex = index"
          >
            <slot name="option" :option="option">
              {{ option.label }}
            </slot>
            <TpIcon
              v-if="option.value === modelValue"
              name="check"
              size="sm"
              class="tp-select__check"
            />
          </li>
        </ul>
      </Transition>
    </Teleport>

    <span v-if="error" class="tp-select__error">{{ error }}</span>
  </div>
</template>

<style>
.tp-select {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  width: 100%;
}

.tp-select__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.tp-select__required {
  color: var(--tp-error);
  margin-left: var(--tp-space-1);
}

.tp-select__trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--tp-space-2);
  width: 100%;
  padding: var(--tp-space-2) var(--tp-space-3);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  background: transparent;
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-sans);
  cursor: pointer;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
  text-align: left;
}

.tp-select__trigger:hover:not(:disabled) {
  border-color: var(--tp-border-strong);
}

.tp-select--open .tp-select__trigger {
  border-color: var(--tp-accent);
}

.tp-select--error .tp-select__trigger {
  border-color: var(--tp-error);
}

.tp-select--disabled .tp-select__trigger {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-select__value {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tp-select__value--placeholder {
  color: var(--tp-text-muted);
}

.tp-select__arrow {
  flex-shrink: 0;
  color: var(--tp-text-muted);
  transition: transform var(--tp-transition-fast);
}

.tp-select--open .tp-select__arrow {
  color: var(--tp-accent);
}

.tp-select__menu {
  z-index: var(--tp-z-toast);
  padding: var(--tp-space-1) 0;
  list-style: none;
  background: var(--tp-bg-primary);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  max-height: 240px;
  overflow-y: auto;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.tp-select__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--tp-space-2);
  padding: var(--tp-space-2) var(--tp-space-3);
  cursor: pointer;
  transition: background-color var(--tp-transition-fast);
}

.tp-select__item:hover,
.tp-select__item--focused {
  background: var(--tp-bg-secondary);
}

.tp-select__item--selected {
  color: var(--tp-accent);
}

.tp-select__item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-select__check {
  color: var(--tp-accent);
  flex-shrink: 0;
}

.tp-select__error {
  font-size: var(--tp-text-sm);
  color: var(--tp-error);
}

/* Focus styles */
.tp-select__trigger:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

<script setup lang="ts">
import { ref, computed, watch, onUnmounted, nextTick } from "vue";
import TpIcon from "./TpIcon.vue";

export interface ComboboxOption {
  value: string | number;
  label: string;
  disabled?: boolean;
}

const props = withDefaults(
  defineProps<{
    modelValue?: string | number | null;
    options: ComboboxOption[];
    label?: string;
    placeholder?: string;
    error?: string;
    disabled?: boolean;
    required?: boolean;
    inputId?: string;
    creatable?: boolean;
    createLabel?: string;
  }>(),
  {
    modelValue: null,
    disabled: false,
    required: false,
    creatable: true,
    createLabel: "Create '{input}'",
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: string | number | null];
  change: [option: ComboboxOption | null];
  create: [value: string];
}>();

const inputRef = ref<HTMLInputElement>();
const controlRef = ref<HTMLDivElement>();
const menuRef = ref<HTMLUListElement>();
const isOpen = ref(false);
const focusedIndex = ref(0);
const searchQuery = ref("");
const menuPosition = ref({ top: "0px", left: "0px", width: "0px" });

const selectedOption = computed(
  () => props.options.find((opt) => opt.value === props.modelValue) ?? null,
);

const filteredOptions = computed(() => {
  if (!searchQuery.value) return props.options;
  const query = searchQuery.value.toLowerCase();
  return props.options.filter((opt) => opt.label.toLowerCase().includes(query));
});

const showCreateOption = computed(() => {
  if (!props.creatable || !searchQuery.value.trim()) return false;
  const exactMatch = props.options.some(
    (opt) => opt.label.toLowerCase() === searchQuery.value.trim().toLowerCase(),
  );
  return !exactMatch;
});

const totalOptions = computed(
  () => filteredOptions.value.length + (showCreateOption.value ? 1 : 0),
);

const updateMenuPosition = () => {
  if (!controlRef.value) return;
  const rect = controlRef.value.getBoundingClientRect();
  menuPosition.value = {
    top: `${rect.bottom + window.scrollY + 4}px`,
    left: `${rect.left + window.scrollX}px`,
    width: `${rect.width}px`,
  };
};

const open = () => {
  if (props.disabled || isOpen.value) return;
  isOpen.value = true;
  focusedIndex.value = 0;
  nextTick(updateMenuPosition);
};

const close = () => {
  isOpen.value = false;
  focusedIndex.value = 0;
};

const select = (option: ComboboxOption) => {
  if (option.disabled) return;
  emit("update:modelValue", option.value);
  emit("change", option);
  searchQuery.value = "";
  close();
  inputRef.value?.focus();
};

const handleCreate = () => {
  const value = searchQuery.value.trim();
  if (!value) return;
  emit("create", value);
  emit("update:modelValue", value);
  searchQuery.value = "";
  close();
  inputRef.value?.focus();
};

const handleFocus = () => {
  // Copy current value to searchQuery so user can edit it character by character
  searchQuery.value =
    selectedOption.value?.label ?? (props.modelValue ? String(props.modelValue) : "");
  open();
};

const handleInput = () => {
  if (!isOpen.value) {
    open();
  }
  focusedIndex.value = 0;
};

const handleKeydown = (event: KeyboardEvent) => {
  if (props.disabled) return;

  switch (event.key) {
    case "Enter":
      event.preventDefault();
      if (isOpen.value && focusedIndex.value >= 0) {
        if (focusedIndex.value < filteredOptions.value.length) {
          const option = filteredOptions.value[focusedIndex.value];
          if (option && !option.disabled) select(option);
        } else if (showCreateOption.value) {
          handleCreate();
        }
      }
      break;

    case "ArrowDown":
      event.preventDefault();
      if (!isOpen.value) {
        open();
      } else if (totalOptions.value > 0) {
        focusedIndex.value = Math.min(focusedIndex.value + 1, totalOptions.value - 1);
      }
      break;

    case "ArrowUp":
      event.preventDefault();
      if (isOpen.value && totalOptions.value > 0) {
        focusedIndex.value = Math.max(focusedIndex.value - 1, 0);
      }
      break;

    case "Escape":
      event.preventDefault();
      searchQuery.value = "";
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
      if (isOpen.value && totalOptions.value > 0) {
        focusedIndex.value = totalOptions.value - 1;
      }
      break;
  }
};

const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Node;
  if (
    controlRef.value &&
    !controlRef.value.contains(target) &&
    menuRef.value &&
    !menuRef.value.contains(target)
  ) {
    searchQuery.value = "";
    close();
  }
};

const toggleMenu = () => {
  if (isOpen.value) {
    close();
  } else {
    open();
    inputRef.value?.focus();
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

// Display the selected value in the input when not searching
const displayValue = computed(() => {
  // When open/editing, always show searchQuery (even if empty)
  if (isOpen.value) return searchQuery.value;
  // When closed, show selected value
  if (selectedOption.value) return selectedOption.value.label;
  // If modelValue is set but doesn't match an option (newly created), show the value itself
  if (props.modelValue) return String(props.modelValue);
  return "";
});
</script>

<template>
  <div
    :class="[
      'tp-combobox',
      {
        'tp-combobox--open': isOpen,
        'tp-combobox--error': error,
        'tp-combobox--disabled': disabled,
      },
    ]"
  >
    <label v-if="label" class="tp-combobox__label">
      {{ label }}
      <span v-if="required" class="tp-combobox__required">*</span>
    </label>

    <div ref="controlRef" class="tp-combobox__control">
      <input
        ref="inputRef"
        :id="inputId"
        :value="displayValue"
        @input="
          searchQuery = ($event.target as HTMLInputElement).value;
          handleInput();
        "
        type="text"
        class="tp-combobox__input"
        :placeholder="placeholder ?? 'Select or type...'"
        :disabled="disabled"
        role="combobox"
        aria-autocomplete="list"
        :aria-expanded="isOpen"
        @focus="handleFocus"
        @keydown="handleKeydown"
      />
      <button
        type="button"
        class="tp-combobox__toggle"
        :disabled="disabled"
        tabindex="-1"
        @click="toggleMenu"
      >
        <TpIcon :name="isOpen ? 'chevron-up' : 'chevron-down'" size="sm" />
      </button>
    </div>

    <Teleport to="body">
      <Transition name="tp-dropdown">
        <ul
          v-if="isOpen"
          ref="menuRef"
          class="tp-combobox__menu"
          role="listbox"
          :style="{
            position: 'absolute',
            top: menuPosition.top,
            left: menuPosition.left,
            width: menuPosition.width,
          }"
        >
          <li
            v-for="(option, index) in filteredOptions"
            :key="option.value"
            :class="[
              'tp-combobox__item',
              {
                'tp-combobox__item--focused': focusedIndex === index,
                'tp-combobox__item--selected': option.value === modelValue,
                'tp-combobox__item--disabled': option.disabled,
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
              class="tp-combobox__check"
            />
          </li>

          <!-- Create option -->
          <li
            v-if="showCreateOption"
            :class="[
              'tp-combobox__item',
              'tp-combobox__item--create',
              { 'tp-combobox__item--focused': focusedIndex === filteredOptions.length },
            ]"
            role="option"
            @click="handleCreate"
            @mouseenter="focusedIndex = filteredOptions.length"
          >
            <TpIcon name="plus" size="sm" />
            <span>{{ createLabel.replace("{input}", searchQuery.trim()) }}</span>
          </li>

          <!-- Empty state -->
          <li v-if="filteredOptions.length === 0 && !showCreateOption" class="tp-combobox__empty">
            No options found
          </li>
        </ul>
      </Transition>
    </Teleport>

    <span v-if="error" class="tp-combobox__error">{{ error }}</span>
  </div>
</template>

<style>
.tp-combobox {
  position: relative;
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  width: 100%;
}

.tp-combobox__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.tp-combobox__required {
  color: var(--tp-error);
  margin-left: var(--tp-space-1);
}

.tp-combobox__control {
  display: flex;
  align-items: center;
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  background: transparent;
  transition: border-color var(--tp-transition-fast);
}

.tp-combobox__control:hover:not(.tp-combobox--disabled .tp-combobox__control) {
  border-color: var(--tp-border-strong);
}

.tp-combobox--open .tp-combobox__control {
  border-color: var(--tp-accent);
}

.tp-combobox--error .tp-combobox__control {
  border-color: var(--tp-error);
}

.tp-combobox--disabled .tp-combobox__control {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-combobox__input {
  flex: 1;
  padding: var(--tp-space-2) var(--tp-space-3);
  border: none;
  background: transparent;
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-sans);
  outline: none;
  min-width: 0;
}

.tp-combobox__input::placeholder {
  color: var(--tp-text-muted);
}

.tp-combobox__input:disabled {
  cursor: not-allowed;
}

.tp-combobox__toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--tp-space-2);
  border: none;
  background: transparent;
  color: var(--tp-text-muted);
  cursor: pointer;
  transition: color var(--tp-transition-fast);
}

.tp-combobox__toggle:hover:not(:disabled) {
  color: var(--tp-text-primary);
}

.tp-combobox--open .tp-combobox__toggle {
  color: var(--tp-accent);
}

.tp-combobox__menu {
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

.tp-combobox__item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--tp-space-2);
  padding: var(--tp-space-2) var(--tp-space-3);
  cursor: pointer;
  transition: background-color var(--tp-transition-fast);
}

.tp-combobox__item:hover,
.tp-combobox__item--focused {
  background: var(--tp-bg-secondary);
}

.tp-combobox__item--selected {
  color: var(--tp-accent);
}

.tp-combobox__item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-combobox__item--create {
  justify-content: flex-start;
  gap: var(--tp-space-2);
  color: var(--tp-accent);
  border-top: var(--tp-border-width) solid var(--tp-border);
}

.tp-combobox__check {
  color: var(--tp-accent);
  flex-shrink: 0;
}

.tp-combobox__empty {
  padding: var(--tp-space-4);
  text-align: center;
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
}

.tp-combobox__error {
  font-size: var(--tp-text-sm);
  color: var(--tp-error);
}

/* Focus styles */
.tp-combobox__input:focus-visible {
  outline: none;
}

.tp-combobox__control:focus-within {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

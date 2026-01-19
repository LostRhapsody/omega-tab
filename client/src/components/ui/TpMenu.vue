<script setup lang="ts">
import { ref, watch, onUnmounted, nextTick, computed } from "vue";

const props = withDefaults(
  defineProps<{
    modelValue?: boolean;
    position?: "bottom-start" | "bottom-end" | "top-start" | "top-end";
    offset?: number;
    closeOnSelect?: boolean;
  }>(),
  {
    modelValue: false,
    position: "bottom-start",
    offset: 4,
    closeOnSelect: true,
  },
);

const emit = defineEmits<{
  "update:modelValue": [value: boolean];
  open: [];
  close: [];
}>();

const triggerRef = ref<HTMLElement>();
const menuRef = ref<HTMLElement>();
const isOpen = ref(props.modelValue);
const focusedIndex = ref(-1);

const menuItems = computed(() => {
  if (!menuRef.value) return [];
  return Array.from(menuRef.value.querySelectorAll<HTMLElement>('[role="menuitem"]'));
});

watch(
  () => props.modelValue,
  (value) => {
    isOpen.value = value;
  },
);

watch(isOpen, (value) => {
  emit("update:modelValue", value);
  if (value) {
    emit("open");
    document.addEventListener("click", handleClickOutside);
    document.addEventListener("keydown", handleKeydown);
    focusedIndex.value = 0;
    nextTick(() => {
      menuItems.value[0]?.focus();
    });
  } else {
    emit("close");
    document.removeEventListener("click", handleClickOutside);
    document.removeEventListener("keydown", handleKeydown);
    focusedIndex.value = -1;
  }
});

const toggle = () => {
  isOpen.value = !isOpen.value;
};

const open = () => {
  isOpen.value = true;
};

const close = () => {
  isOpen.value = false;
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

const handleKeydown = (event: KeyboardEvent) => {
  const items = menuItems.value;
  if (!items.length) return;

  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value + 1) % items.length;
      items[focusedIndex.value]?.focus();
      break;

    case "ArrowUp":
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value - 1 + items.length) % items.length;
      items[focusedIndex.value]?.focus();
      break;

    case "Home":
      event.preventDefault();
      focusedIndex.value = 0;
      items[0]?.focus();
      break;

    case "End":
      event.preventDefault();
      focusedIndex.value = items.length - 1;
      items[items.length - 1]?.focus();
      break;

    case "Escape":
      event.preventDefault();
      close();
      triggerRef.value?.focus();
      break;

    case "Tab":
      close();
      break;
  }
};

const handleItemClick = () => {
  if (props.closeOnSelect) {
    close();
  }
};

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
  document.removeEventListener("keydown", handleKeydown);
});

defineExpose({ open, close, toggle, triggerRef, menuRef });
</script>

<template>
  <div class="tp-menu">
    <div ref="triggerRef" class="tp-menu__trigger" @click="toggle">
      <slot name="trigger" :is-open="isOpen" :toggle="toggle" />
    </div>

    <Transition name="tp-dropdown">
      <div
        v-if="isOpen"
        ref="menuRef"
        :class="['tp-menu__content', `tp-menu__content--${position}`]"
        :style="{ '--tp-menu-offset': `${offset}px` }"
        role="menu"
        @click="handleItemClick"
      >
        <slot :close="close" />
      </div>
    </Transition>
  </div>
</template>

<style>
.tp-menu {
  position: relative;
  display: inline-block;
}

.tp-menu__trigger {
  display: inline-flex;
}

.tp-menu__content {
  position: absolute;
  z-index: var(--tp-z-dropdown);
  min-width: 160px;
  padding: var(--tp-space-1) 0;
  background: var(--tp-bg-primary);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* Positioning */
.tp-menu__content--bottom-start {
  top: calc(100% + var(--tp-menu-offset));
  left: 0;
}

.tp-menu__content--bottom-end {
  top: calc(100% + var(--tp-menu-offset));
  right: 0;
}

.tp-menu__content--top-start {
  bottom: calc(100% + var(--tp-menu-offset));
  left: 0;
}

.tp-menu__content--top-end {
  bottom: calc(100% + var(--tp-menu-offset));
  right: 0;
}
</style>

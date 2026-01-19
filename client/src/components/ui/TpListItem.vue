<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    clickable?: boolean;
    active?: boolean;
    disabled?: boolean;
    href?: string;
    target?: string;
  }>(),
  {
    clickable: false,
    active: false,
    disabled: false,
  },
);

const emit = defineEmits<{
  click: [event: MouseEvent];
}>();

const tag = computed(() => {
  if (props.href) return "a";
  if (props.clickable) return "button";
  return "li";
});

const handleClick = (event: MouseEvent) => {
  if (!props.disabled) {
    emit("click", event);
  }
};
</script>

<template>
  <component
    :is="tag"
    :class="[
      'tp-list-item',
      {
        'tp-list-item--clickable': clickable || href,
        'tp-list-item--active': active,
        'tp-list-item--disabled': disabled,
      },
    ]"
    :href="href"
    :target="target"
    :disabled="disabled && tag === 'button'"
    :role="clickable || href ? 'menuitem' : undefined"
    :tabindex="clickable || href ? 0 : undefined"
    @click="handleClick"
  >
    <span v-if="$slots.prepend" class="tp-list-item__prepend">
      <slot name="prepend" />
    </span>

    <span class="tp-list-item__content">
      <slot />
    </span>

    <span v-if="$slots.append" class="tp-list-item__append">
      <slot name="append" />
    </span>
  </component>
</template>

<style>
.tp-list-item {
  display: flex;
  align-items: center;
  gap: var(--tp-space-3);
  padding: var(--tp-space-2) var(--tp-space-3);
  color: var(--tp-text-primary);
  text-decoration: none;
  background: transparent;
  border: none;
  width: 100%;
  text-align: left;
  font-family: var(--tp-font-sans);
  font-size: var(--tp-text-base);
  transition: background-color var(--tp-transition-fast);
}

.tp-list-item--clickable {
  cursor: pointer;
}

.tp-list-item--clickable:hover:not(.tp-list-item--disabled) {
  background: var(--tp-bg-secondary);
}

.tp-list-item--active {
  background: var(--tp-bg-secondary);
  color: var(--tp-accent);
}

.tp-list-item--active::before {
  content: "> ";
  color: var(--tp-accent);
  font-family: var(--tp-font-mono);
}

.tp-list-item--disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.tp-list-item__prepend {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  color: var(--tp-text-muted);
}

.tp-list-item__content {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tp-list-item__append {
  display: flex;
  align-items: center;
  flex-shrink: 0;
  color: var(--tp-text-muted);
}

/* Focus styles */
.tp-list-item--clickable:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: calc(-1 * var(--tp-focus-offset));
  background: var(--tp-bg-secondary);
}
</style>

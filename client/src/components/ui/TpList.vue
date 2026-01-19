<script setup lang="ts">
withDefaults(
  defineProps<{
    dense?: boolean;
    dividers?: boolean;
  }>(),
  {
    dense: false,
    dividers: false,
  },
);
</script>

<template>
  <ul
    :class="[
      'tp-list',
      {
        'tp-list--dense': dense,
        'tp-list--dividers': dividers,
      },
    ]"
    role="list"
  >
    <slot />
  </ul>
</template>

<style>
.tp-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.tp-list--dense .tp-list-item {
  padding: var(--tp-space-1) var(--tp-space-3);
}

.tp-list--dividers .tp-list-item:not(:last-child) {
  border-bottom: var(--tp-border-width) solid var(--tp-border);
}
</style>

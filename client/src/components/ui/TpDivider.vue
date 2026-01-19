<script setup lang="ts">
withDefaults(
  defineProps<{
    vertical?: boolean;
    strong?: boolean;
  }>(),
  {
    vertical: false,
    strong: false,
  },
);
</script>

<template>
  <hr
    :class="[
      'tp-divider',
      {
        'tp-divider--vertical': vertical,
        'tp-divider--strong': strong,
      },
    ]"
  />
</template>

<style>
.tp-divider {
  border: none;
  margin: 0;
  flex-shrink: 0;
}

.tp-divider:not(.tp-divider--vertical) {
  border-top: var(--tp-border-width) solid var(--tp-border);
  width: 100%;
}

.tp-divider--vertical {
  border-left: var(--tp-border-width) solid var(--tp-border);
  height: 100%;
  width: 0;
}

.tp-divider--strong:not(.tp-divider--vertical) {
  border-top-width: var(--tp-border-width-thick);
  border-color: var(--tp-rule);
}

.tp-divider--strong.tp-divider--vertical {
  border-left-width: var(--tp-border-width-thick);
  border-color: var(--tp-rule);
}
</style>

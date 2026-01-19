<script setup lang="ts">
import { computed } from "vue";

const props = withDefaults(
  defineProps<{
    name?: string;
    src?: string;
    size?: "sm" | "md" | "lg" | "xl";
    alt?: string;
  }>(),
  {
    size: "md",
  },
);

const initials = computed(() => {
  if (!props.name) return "?";
  const parts = props.name.trim().split(/\s+/);
  if (parts.length === 1) {
    return parts[0].charAt(0).toUpperCase();
  }
  return (parts[0].charAt(0) + parts[parts.length - 1].charAt(0)).toUpperCase();
});

const sizePixels = computed(() => {
  switch (props.size) {
    case "sm":
      return 24;
    case "md":
      return 32;
    case "lg":
      return 40;
    case "xl":
      return 56;
    default:
      return 32;
  }
});
</script>

<template>
  <div
    :class="['tp-avatar', `tp-avatar--${size}`]"
    :style="{
      width: `${sizePixels}px`,
      height: `${sizePixels}px`,
    }"
  >
    <img v-if="src" :src="src" :alt="alt || name || 'Avatar'" class="tp-avatar__image" />
    <span v-else class="tp-avatar__initials">
      {{ initials }}
    </span>
  </div>
</template>

<style>
.tp-avatar {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: var(--tp-bg-tertiary);
  border: var(--tp-border-width) solid var(--tp-border);
  overflow: hidden;
  flex-shrink: 0;
}

.tp-avatar__image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}

.tp-avatar__initials {
  font-family: var(--tp-font-mono);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  text-transform: uppercase;
  line-height: 1;
}

/* Size-specific font sizes */
.tp-avatar--sm .tp-avatar__initials {
  font-size: 10px;
}

.tp-avatar--md .tp-avatar__initials {
  font-size: 12px;
}

.tp-avatar--lg .tp-avatar__initials {
  font-size: 14px;
}

.tp-avatar--xl .tp-avatar__initials {
  font-size: 18px;
}
</style>

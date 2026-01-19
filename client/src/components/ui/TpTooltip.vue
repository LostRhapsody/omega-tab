<script setup lang="ts">
import { ref, onUnmounted, nextTick } from "vue";

const props = withDefaults(
  defineProps<{
    content: string;
    position?: "top" | "bottom" | "left" | "right";
    delay?: number;
    disabled?: boolean;
    maxWidth?: string;
  }>(),
  {
    position: "top",
    delay: 300,
    disabled: false,
    maxWidth: "280px",
  },
);

const isVisible = ref(false);
const triggerRef = ref<HTMLElement>();
const tooltipPosition = ref({ top: "0px", left: "0px" });
let showTimeout: ReturnType<typeof setTimeout> | null = null;
let hideTimeout: ReturnType<typeof setTimeout> | null = null;

const updatePosition = () => {
  if (!triggerRef.value) return;
  const rect = triggerRef.value.getBoundingClientRect();
  const scrollX = window.scrollX;
  const scrollY = window.scrollY;

  let top = 0;
  let left = 0;

  switch (props.position) {
    case "top":
      top = rect.top + scrollY - 8;
      left = rect.left + scrollX + rect.width / 2;
      break;
    case "bottom":
      top = rect.bottom + scrollY + 8;
      left = rect.left + scrollX + rect.width / 2;
      break;
    case "left":
      top = rect.top + scrollY + rect.height / 2;
      left = rect.left + scrollX - 8;
      break;
    case "right":
      top = rect.top + scrollY + rect.height / 2;
      left = rect.right + scrollX + 8;
      break;
  }

  tooltipPosition.value = {
    top: `${top}px`,
    left: `${left}px`,
  };
};

const show = () => {
  if (props.disabled) return;
  if (hideTimeout) {
    clearTimeout(hideTimeout);
    hideTimeout = null;
  }
  showTimeout = setTimeout(() => {
    isVisible.value = true;
    nextTick(updatePosition);
  }, props.delay);
};

const hide = () => {
  if (showTimeout) {
    clearTimeout(showTimeout);
    showTimeout = null;
  }
  hideTimeout = setTimeout(() => {
    isVisible.value = false;
  }, 100);
};

const hideImmediately = () => {
  if (showTimeout) {
    clearTimeout(showTimeout);
    showTimeout = null;
  }
  if (hideTimeout) {
    clearTimeout(hideTimeout);
    hideTimeout = null;
  }
  isVisible.value = false;
};

onUnmounted(() => {
  if (showTimeout) clearTimeout(showTimeout);
  if (hideTimeout) clearTimeout(hideTimeout);
});
</script>

<template>
  <div
    ref="triggerRef"
    class="tp-tooltip-wrapper"
    @mouseenter="show"
    @mouseleave="hide"
    @focus="show"
    @blur="hideImmediately"
  >
    <slot />

    <Teleport to="body">
      <Transition name="tp-tooltip">
        <div
          v-if="isVisible && content"
          :class="['tp-tooltip', `tp-tooltip--${position}`]"
          :style="{
            position: 'absolute',
            top: tooltipPosition.top,
            left: tooltipPosition.left,
            maxWidth: maxWidth,
          }"
          role="tooltip"
        >
          {{ content }}
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<style>
.tp-tooltip-wrapper {
  position: relative;
  display: inline-flex;
}

.tp-tooltip {
  z-index: var(--tp-z-tooltip);
  padding: var(--tp-space-2) var(--tp-space-3);
  font-size: var(--tp-text-sm);
  font-family: var(--tp-font-mono);
  color: var(--tp-bg-primary);
  background: var(--tp-text-primary);
  border-radius: var(--tp-radius-sm);
  white-space: normal;
  word-wrap: break-word;
  pointer-events: none;
  text-align: center;
  line-height: var(--tp-leading-normal);
}

/* Positions - transforms for centering */
.tp-tooltip--top {
  transform: translate(-50%, -100%);
}

.tp-tooltip--bottom {
  transform: translate(-50%, 0);
}

.tp-tooltip--left {
  transform: translate(-100%, -50%);
}

.tp-tooltip--right {
  transform: translate(0, -50%);
}

/* Transitions */
.tp-tooltip-enter-active,
.tp-tooltip-leave-active {
  transition:
    opacity var(--tp-transition-fast),
    transform var(--tp-transition-fast);
}

.tp-tooltip-enter-from,
.tp-tooltip-leave-to {
  opacity: 0;
}

.tp-tooltip--top.tp-tooltip-enter-from,
.tp-tooltip--top.tp-tooltip-leave-to {
  transform: translate(-50%, calc(-100% + 4px));
}

.tp-tooltip--bottom.tp-tooltip-enter-from,
.tp-tooltip--bottom.tp-tooltip-leave-to {
  transform: translate(-50%, -4px);
}

.tp-tooltip--left.tp-tooltip-enter-from,
.tp-tooltip--left.tp-tooltip-leave-to {
  transform: translate(calc(-100% + 4px), -50%);
}

.tp-tooltip--right.tp-tooltip-enter-from,
.tp-tooltip--right.tp-tooltip-leave-to {
  transform: translate(-4px, -50%);
}
</style>

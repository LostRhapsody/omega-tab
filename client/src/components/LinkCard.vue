<script setup lang="ts">
import { computed, ref } from "vue";
import { useUserSettingsStore } from "../stores/settings";
import { TpMenu, TpMenuItem, TpIcon, TpSnackbar } from "@/components/ui";

interface Props {
  icon?: string;
  title: string;
  description: string;
  link: string;
  index: number;
  shortcut: string;
  onDelete: () => void;
  onEdit: () => void;
  draggable?: boolean;
  onDragStart?: (index: number) => void;
  onDragOver?: (index: number) => void;
  onDragEnd?: () => void;
  isDragOver?: boolean;
}

const settingsStore = useUserSettingsStore();
const props = defineProps<Props>();
const isMdiIcon = computed(() => props.icon?.startsWith("mdi-"));
const snackbar = ref(false);
const iconBackground = ref("");

const copyToClipboard = (text: string) => {
  navigator.clipboard
    .writeText(text)
    .then(() => {
      snackbar.value = true;
    })
    .catch((err) => {
      console.error("Failed to copy: ", err);
    });
};

const isIconDark = (iconUrl: string): Promise<boolean> => {
  if (iconUrl.includes("svg+xml")) {
    return new Promise((resolve) => resolve(false));
  }
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = "Anonymous";
    img.src = iconUrl;
    img.onload = () => {
      const canvas = document.createElement("canvas");
      const ctx = canvas.getContext("2d");
      if (!ctx) return resolve(false);
      canvas.width = img.width;
      canvas.height = img.height;
      ctx.drawImage(img, 0, 0, img.width, img.height);
      const imageData = ctx.getImageData(0, 0, img.width, img.height);
      let totalBrightness = 0;
      for (let i = 0; i < imageData.data.length; i += 4) {
        const r = imageData.data[i];
        const g = imageData.data[i + 1];
        const b = imageData.data[i + 2];
        totalBrightness += r * 0.299 + g * 0.587 + b * 0.114;
      }
      const avgBrightness = totalBrightness / (imageData.data.length / 4);
      resolve(avgBrightness < 50);
    };
    img.onerror = () => resolve(false);
  });
};

if (props.icon && !isMdiIcon.value) {
  isIconDark(props.icon).then((isDark) => {
    iconBackground.value = isDark ? "var(--tp-bg-tertiary)" : "";
  });
}

// Drag handlers
const handleDragStart = (event: DragEvent) => {
  if (!props.draggable) return;
  event.dataTransfer?.setData("text/plain", props.index.toString());
  props.onDragStart?.(props.index);
};

const handleDragOver = () => {
  if (!props.draggable) return;
  props.onDragOver?.(props.index);
};

const handleDragEnd = () => {
  if (!props.draggable) return;
  props.onDragEnd?.();
};

// Map MDI icons to our icon names
const getMappedIcon = (mdiIcon: string) => {
  const iconMap: Record<string, string> = {
    "mdi-link": "link",
    "mdi-cog": "cog",
    "mdi-help": "help",
    "mdi-book": "book",
    "mdi-rocket": "rocket",
  };
  const iconName = mdiIcon.replace("mdi-", "");
  return iconMap[mdiIcon] || iconName;
};
</script>

<template>
  <div
    class="link-card"
    :class="{ 'link-card--drag-over': isDragOver }"
    :draggable="draggable"
    @dragstart="handleDragStart"
    @dragover.prevent="handleDragOver"
    @dragend="handleDragEnd"
    @drop.prevent="handleDragEnd"
  >
    <a
      :href="link"
      :target="settingsStore.settings.new_tabs ? '_blank' : '_self'"
      class="link-card__link"
    >
      <div
        class="link-card__icon-wrapper"
        :class="{ 'link-card__icon-wrapper--bg': iconBackground }"
      >
        <TpIcon v-if="isMdiIcon" :name="getMappedIcon(icon!)" size="lg" class="link-card__icon" />
        <img
          v-else-if="icon"
          :src="icon"
          :alt="title"
          class="link-card__custom-icon"
          :style="{ backgroundColor: iconBackground }"
        />
        <TpIcon v-else name="link" size="lg" class="link-card__icon" />
      </div>

      <div class="link-card__content">
        <div class="link-card__title">
          {{ title }}
          <div v-if="shortcut" class="link-card__shortcut">
            <kbd>{{ shortcut }}+{{ index + 1 }}</kbd>
          </div>
        </div>
        <div v-if="description" class="link-card__description">
          {{ description }}
        </div>
      </div>
    </a>

    <TpMenu position="bottom-end" class="link-card__menu">
      <template #trigger>
        <button class="link-card__menu-btn" @click.prevent aria-label="Link options">
          <TpIcon name="menu-dots" size="md" />
        </button>
      </template>

      <TpMenuItem icon="edit" @click="onEdit"> Edit </TpMenuItem>
      <TpMenuItem icon="copy" @click="copyToClipboard(link)"> Copy URL </TpMenuItem>
      <TpMenuItem icon="trash" danger @click="onDelete"> Delete </TpMenuItem>
    </TpMenu>

    <TpSnackbar
      v-model="snackbar"
      message="URL copied to clipboard"
      type="success"
      :duration="3000"
    />
  </div>
</template>

<style scoped>
.link-card {
  min-height: 4rem;
  position: relative;
  display: flex;
  align-items: center;
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  background: var(--tp-bg-secondary);
  transition:
    background-color var(--tp-transition-fast),
    border-color var(--tp-transition-fast);
}

.link-card:hover {
  background: var(--tp-bg-tertiary);
  border-color: var(--tp-border-strong);
}

.link-card--drag-over {
  border-color: var(--tp-accent);
  border-style: dashed;
  background: var(--tp-accent-glow);
}

.link-card[draggable="true"] {
  cursor: grab;
}

.link-card[draggable="true"]:active {
  cursor: grabbing;
}

.link-card__link {
  display: flex;
  align-items: center;
  flex: 1;
  padding: var(--tp-space-3) var(--tp-space-4);
  text-decoration: none;
  color: inherit;
  min-width: 0;
  gap: var(--tp-space-3);
}

.link-card__link:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: calc(-1 * var(--tp-focus-offset));
  border-radius: var(--tp-radius-sm);
}

.link-card__icon-wrapper {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  flex-shrink: 0;
  border-radius: var(--tp-radius-sm);
}

.link-card__icon-wrapper--bg {
  background: var(--tp-bg-tertiary);
}

.link-card__icon {
  color: var(--tp-accent);
}

.link-card__custom-icon {
  width: 36px;
  height: 36px;
  border-radius: var(--tp-radius-sm);
  object-fit: contain;
  transition: transform var(--tp-transition-fast);
}

.link-card:hover .link-card__custom-icon {
  transform: scale(1.05);
}

.link-card__content {
  flex: 1;
  min-width: 0;
}

.link-card__title {
  font-size: var(--tp-text-base);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
}

.link-card__description {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  margin-top: var(--tp-space-1);
}

.link-card__shortcut {
  margin-left: var(--tp-space-3);
  opacity: 0;
  transition: opacity var(--tp-transition-fast);
}

.link-card__shortcut kbd {
  font-size: var(--tp-text-xs);
  padding: 2px 4px;
}

.link-card:hover .link-card__shortcut {
  opacity: 1;
}

.link-card__menu {
  position: absolute;
  top: var(--tp-space-2);
  right: var(--tp-space-2);
  opacity: 0;
  transition: opacity var(--tp-transition-fast);
}

.link-card:hover .link-card__menu {
  opacity: 1;
}

.link-card__menu-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: var(--tp-radius-sm);
  color: var(--tp-text-muted);
  transition:
    background-color var(--tp-transition-fast),
    color var(--tp-transition-fast);
}

.link-card__menu-btn:hover {
  background: var(--tp-bg-secondary);
  color: var(--tp-text-primary);
}

.link-card__menu-btn:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .link-card,
  .link-card__custom-icon,
  .link-card__shortcut,
  .link-card__menu {
    transition: none;
  }
}
</style>

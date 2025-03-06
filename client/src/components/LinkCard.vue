<!-- ToolLink.vue -->
<script setup lang="ts">
  import { computed, defineProps, ref } from "vue";
  import { useUserSettingsStore } from "../stores/settings";

  interface Props {
    icon?: string;
    title: string;
    description: string;
    link: string;
    index: number;
    shortcut: string;
    onDelete: () => void;
    onEdit: () => void;
  }

  const settingsStore = useUserSettingsStore();
  const props = defineProps<Props>();
  const isMdiIcon = computed(() => props.icon?.startsWith("mdi-"));
  const snackbar = ref(false);
  const iconBackground = ref("");
  const iconLightBg = ref('rgba(255, 255, 255, 0.5)');
  const hoverBg = ref('rgba(64, 64, 64, 0.3)');
  const transitionSpeed = ref('0.2s');

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
          totalBrightness += (r * 0.299 + g * 0.587 + b * 0.114);
        }
        const avgBrightness = totalBrightness / (imageData.data.length / 4);
        resolve(avgBrightness < 50);
      };
      img.onerror = () => resolve(false);
    });
  };

if (props.icon && !isMdiIcon.value) {
  isIconDark(props.icon).then((isDark) => {
    iconBackground.value = isDark ? "var(--icon-light-bg)" : "";
  });
}
</script>

<template>
  <div>
  <v-card
      :href="link"
      :target="settingsStore.settings.new_tabs ? '_blank' : '_self'"
      variant="tonal"
      class="tool-link pa-4 mb-2 d-flex align-center"
      :ripple="false"
    >
      <div class="d-flex align-center flex-grow-1">
        <div v-if="icon" class="icon-wrapper" :class="{ 'with-background': iconBackground }">
        <v-icon
          v-if="isMdiIcon"
          :icon="icon"
          size="36"
          class="mr-4"
          color="primary"
        />
        <img
          v-else
          :src="icon"
          :alt="title"
          class="custom-icon mr-4"
          :style="{ backgroundColor: iconBackground }"
        />
        </div>
        <div class="content-wrapper">
        <div class="text-h6">{{ title }}</div>
        <div class="text-body-2 text-medium-emphasis description-text">
          {{ description }}
        </div>
        </div>
      </div>

      <v-menu>
        <template v-slot:activator="{ props }">
          <v-btn
            @click="(e: Event) => { e.preventDefault() }"
            v-bind="props"
            icon="mdi-dots-vertical"
            variant="text"
            size="small"
          />
        </template>
        <v-list>
          <v-list-item @click="props.onDelete()">
            <v-list-item-title>Delete</v-list-item-title>
          </v-list-item>
          <v-list-item @click="() => copyToClipboard(link)">
            <v-list-item-title>Copy URL</v-list-item-title>
          </v-list-item>
          <v-list-item @click="() => onEdit()">
            <v-list-item-title>Edit</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
      <v-icon icon="mdi-chevron-right" size="24" class="chevron" />
      <div v-if="shortcut" class="shortcut-text">{{ shortcut }}+{{ index + 1 }}</div>
    </v-card>
    <v-snackbar v-model="snackbar" :timeout="3000">
      URL Copied to clipboard
      <template v-slot:actions>
        <v-btn color="red" variant="text" @click="snackbar = false">
          Close
        </v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<style scoped>
.tool-link {
  text-decoration: none;
  background: rgba(0, 0, 0, 0.1);
  position: relative;
  transition: background-color v-bind(transitionSpeed) ease-in-out;
}

.icon-wrapper {
  border-radius: 8px;
  width: 36px;
  height: 36px;
  margin-right: 0.5rem;
  min-width: 36px;
}

.icon-wrapper.with-background {
  background: v-bind(iconLightBg);
}

.custom-icon {
  width: 36px;
  height: 36px;
  flex-shrink: 0;
  border-radius: 6px;
  transition: transform v-bind(transitionSpeed) ease-in-out;
}

.content-wrapper {
  flex-grow: 1;
  min-width: 0; /* Ensures text truncation works */
}

.description-text {
  white-space: pre-wrap;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
  text-overflow: ellipsis;
}

.shortcut-text {
  position: absolute;
  bottom: 8px;
  right: 8px;
  opacity: 0;
  transition: opacity v-bind(transitionSpeed) ease-in-out;
  font-size: 0.875rem;
  color: rgba(255, 255, 255, 0.7);
}

.chevron {
  transform: translateX(0);
  transition: transform v-bind(transitionSpeed) ease-in-out;
  opacity: 0.7;
}

/* Hover states */
.tool-link:hover {
  background-color: v-bind(hoverBg);
}

.tool-link:hover .chevron {
  transform: translateX(8px);
}

.tool-link:hover .shortcut-text {
  opacity: 1;
}

.tool-link:hover .custom-icon {
  transform: scale(1.05);
}

@media (prefers-reduced-motion: reduce) {
  .tool-link,
  .custom-icon,
  .chevron,
  .shortcut-text {
    transition: none;
  }
}
</style>
<template>
  <div class="icon-search-panel">
    <TpInput
      v-model="searchQuery"
      label="Search Dashboard Icons"
      placeholder="e.g., github, docker, slack..."
      @input="handleSearchInput"
    />

    <div v-if="isLoading" class="icon-search-panel__loading">
      <TpSpinner size="sm" />
      <span>Searching icons...</span>
    </div>

    <div v-else-if="error" class="icon-search-panel__error">
      {{ error }}
    </div>

    <div v-else-if="icons.length > 0" class="icon-search-panel__grid">
      <button
        v-for="icon in icons"
        :key="icon.name"
        type="button"
        class="icon-search-panel__item"
        :title="icon.name"
        @click="selectIcon(icon)"
      >
        <img :src="icon.png_url" :alt="icon.name" class="icon-search-panel__img" />
      </button>
    </div>

    <div v-else-if="searchQuery && hasSearched" class="icon-search-panel__empty">
      No icons found for "{{ searchQuery }}"
    </div>

    <div v-else class="icon-search-panel__hint">
      Search for application icons from dashboardicons.com
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { TpInput, TpSpinner } from "@/components/ui";
import { API } from "@/constants/api";
import api from "@/services/api";
import type { DashboardIcon, IconSearchResponse } from "@/types/DashboardIcon";

const emit = defineEmits<{
  (e: "select", url: string): void;
}>();

const searchQuery = ref("");
const icons = ref<DashboardIcon[]>([]);
const isLoading = ref(false);
const error = ref("");
const hasSearched = ref(false);

let debounceTimer: ReturnType<typeof setTimeout> | null = null;

const handleSearchInput = () => {
  if (debounceTimer) {
    clearTimeout(debounceTimer);
  }

  if (!searchQuery.value.trim()) {
    icons.value = [];
    hasSearched.value = false;
    error.value = "";
    return;
  }

  debounceTimer = setTimeout(() => {
    searchIcons();
  }, 500);
};

const searchIcons = async () => {
  const query = searchQuery.value.trim();
  if (!query) return;

  isLoading.value = true;
  error.value = "";
  hasSearched.value = true;

  try {
    const response = await api.get<IconSearchResponse>(API.SEARCH_ICONS(query));
    icons.value = response.data.icons;
  } catch (err) {
    console.error("Error searching icons:", err);
    error.value = "Failed to search icons. Please try again.";
    icons.value = [];
  } finally {
    isLoading.value = false;
  }
};

const selectIcon = (icon: DashboardIcon) => {
  emit("select", icon.png_url);
};
</script>

<style scoped>
.icon-search-panel {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.icon-search-panel__loading {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
  padding: var(--tp-space-2);
}

.icon-search-panel__error {
  color: var(--tp-error);
  font-size: var(--tp-text-sm);
  padding: var(--tp-space-2);
}

.icon-search-panel__grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(48px, 1fr));
  gap: var(--tp-space-2);
  max-height: 160px;
  overflow-y: auto;
  padding: var(--tp-space-2);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  background: var(--tp-bg-secondary);
}

.icon-search-panel__item {
  width: 48px;
  height: 48px;
  padding: var(--tp-space-1);
  border: var(--tp-border-width) solid transparent;
  border-radius: var(--tp-radius-sm);
  background: var(--tp-bg-primary);
  cursor: pointer;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
}

.icon-search-panel__item:hover {
  border-color: var(--tp-accent);
  background: var(--tp-accent-glow);
}

.icon-search-panel__img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.icon-search-panel__empty,
.icon-search-panel__hint {
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
  padding: var(--tp-space-2);
  text-align: center;
}
</style>

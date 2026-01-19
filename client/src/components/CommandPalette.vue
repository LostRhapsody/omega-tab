<template>
  <TpModal
    v-model="isOpen"
    title="Command Palette"
    size="md"
    initial-focus="#command-palette-input"
  >
    <div class="command-palette">
      <input
        id="command-palette-input"
        v-model="query"
        type="text"
        placeholder="Type a command or search..."
        @keydown="handleKeydown"
        class="command-palette__input"
        autocomplete="off"
      />

      <ul v-if="filteredResults.length > 0" ref="listRef" class="command-palette__results">
        <li
          v-for="(result, index) in filteredResults"
          :key="index"
          :class="[
            'command-palette__item',
            { 'command-palette__item--focused': focusedIndex === index },
          ]"
          :ref="(el) => setItemRef(el as HTMLLIElement, index)"
          @click="handleSelect(result)"
          @mouseenter="focusedIndex = index"
        >
          <span class="command-palette__title">{{ result.title }}</span>
          <span class="command-palette__subtitle">{{ result.subtitle }}</span>
        </li>
      </ul>

      <div v-else class="command-palette__empty">No results found</div>
    </div>
  </TpModal>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch, nextTick } from "vue";
import { useRouter } from "vue-router";
import { useLinksStore } from "../stores/links";
import { useSearchEngineStore } from "../stores/searchEngine";
import { storeToRefs } from "pinia";
import { openUrl } from "../utils/openUrl";
import { TpModal } from "@/components/ui";

type Result = {
  title: string;
  subtitle: string;
  action: () => void;
};

const isOpen = ref(false);
const query = ref("");
const focusedIndex = ref(0);
const listRef = ref<HTMLUListElement | null>(null);
const itemRefs = ref<Map<number, HTMLLIElement>>(new Map());

const router = useRouter();
const linksStore = useLinksStore();
const searchEngineStore = useSearchEngineStore();
const { links } = storeToRefs(linksStore);

const commands = [
  {
    title: "Navigate to Settings",
    subtitle: "Go to settings page",
    action: () => router.push("/settings"),
  },
  { title: "Add New Link", subtitle: "Add a new link", action: () => triggerAddLink() },
];

const filteredResults = computed(() => {
  const lowerQuery = query.value.toLowerCase();
  const linkResults = links.value
    .filter((link) => link.title.toLowerCase().includes(lowerQuery))
    .map((link) => ({
      title: link.title,
      subtitle: link.url,
      action: () => openUrl(link.url),
    }));

  const commandResults = commands.filter((command) =>
    command.title.toLowerCase().includes(lowerQuery),
  );

  const searchEngineResults = searchEngineStore.searchEngines
    .map((engine) => ({
      title: `Switch to ${engine.name}`,
      subtitle: `Change search engine to ${engine.name}`,
      action: () => searchEngineStore.setSearchEngine(engine.url),
    }))
    .filter(
      (engineResult) =>
        engineResult.title.toLowerCase().includes(lowerQuery) ||
        engineResult.subtitle.toLowerCase().includes(lowerQuery),
    );

  return [...linkResults, ...commandResults, ...searchEngineResults];
});

const setItemRef = (el: HTMLLIElement | null, index: number) => {
  if (el) {
    itemRefs.value.set(index, el);
  } else {
    itemRefs.value.delete(index);
  }
};

const scrollToFocused = () => {
  nextTick(() => {
    const item = itemRefs.value.get(focusedIndex.value);
    if (item) {
      item.scrollIntoView({ block: "nearest", behavior: "smooth" });
    }
  });
};

const handleKeydown = (event: KeyboardEvent) => {
  const totalItems = filteredResults.value.length;
  if (totalItems === 0) return;

  switch (event.key) {
    case "ArrowDown":
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value + 1) % totalItems;
      scrollToFocused();
      break;
    case "ArrowUp":
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value - 1 + totalItems) % totalItems;
      scrollToFocused();
      break;
    case "Enter":
      event.preventDefault();
      if (focusedIndex.value >= 0 && focusedIndex.value < totalItems) {
        filteredResults.value[focusedIndex.value].action();
      }
      closePalette();
      break;
    case "Escape":
      closePalette();
      break;
  }
};

const handleSelect = (result: Result) => {
  result.action();
  closePalette();
};

const openPalette = (event: KeyboardEvent) => {
  if (event.key === "k" && event.ctrlKey) {
    event.preventDefault();
    isOpen.value = true;
  }
};

const closePalette = () => {
  isOpen.value = false;
  query.value = "";
  focusedIndex.value = 0;
};

const triggerAddLink = () => {
  const addLinkButton = document.querySelector("#add-link-card");
  if (addLinkButton) {
    (addLinkButton as HTMLElement).click();
  }
};

const handleTriggerAddLink = (event: KeyboardEvent) => {
  if (event.key === "n" && event.altKey && !event.ctrlKey && !event.metaKey) {
    event.preventDefault();
    triggerAddLink();
  }
};

// Reset focus index when query changes
watch(query, () => {
  focusedIndex.value = 0;
});

// Reset state when modal opens
watch(isOpen, (newVal) => {
  if (newVal) {
    query.value = "";
    focusedIndex.value = 0;
    itemRefs.value.clear();
  }
});

onMounted(() => {
  window.addEventListener("keydown", openPalette);
  window.addEventListener("keydown", handleTriggerAddLink);
});

onUnmounted(() => {
  window.removeEventListener("keydown", openPalette);
  window.removeEventListener("keydown", handleTriggerAddLink);
});
</script>

<style scoped>
.command-palette {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.command-palette__input {
  width: 100%;
  padding: var(--tp-space-3) var(--tp-space-4);
  background: var(--tp-surface);
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-mono);
  outline: none;
  transition: border-color var(--tp-transition-fast);
}

.command-palette__input:focus {
  border-color: var(--tp-accent);
}

.command-palette__input::placeholder {
  color: var(--tp-text-muted);
}

.command-palette__results {
  list-style: none;
  margin: 0;
  padding: 0;
  max-height: 300px;
  overflow-y: auto;
  border: var(--tp-border-width) solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
}

.command-palette__item {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  padding: var(--tp-space-3) var(--tp-space-4);
  cursor: pointer;
  border-left: 2px solid transparent;
  transition:
    background-color var(--tp-transition-fast),
    border-color var(--tp-transition-fast);
}

.command-palette__item:not(:last-child) {
  border-bottom: var(--tp-border-width) solid var(--tp-border);
}

.command-palette__item:hover,
.command-palette__item--focused {
  background: var(--tp-accent-glow);
  border-left-color: var(--tp-accent);
}

.command-palette__title {
  font-size: var(--tp-text-base);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.command-palette__subtitle {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  font-family: var(--tp-font-mono);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.command-palette__empty {
  text-align: center;
  padding: var(--tp-space-8);
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
}
</style>

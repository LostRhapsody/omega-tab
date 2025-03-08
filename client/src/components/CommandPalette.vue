<template>
  <v-dialog v-model="isOpen" max-width="600px">
    <v-card>
      <v-card-title>
        <v-text-field v-model="query" placeholder="Type a command or search..." @keydown="handleKeydown" ref="input"
          autofocus />
      </v-card-title>
      <v-card-text>
        <v-list>
          <v-list-item v-for="(result, index) in filteredResults" :key="index"
            :class="{ focused: focusedIndex === index, 'rounded-lg': true }" @click="handleSelect(result)"
            @mouseover="focusedIndex = index" rounded-pill :title="result.title"
            :subtitle="result.subtitle"></v-list-item>
        </v-list>
      </v-card-text>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue';
import { useRouter } from 'vue-router';
import { useLinksStore } from '../stores/links';
import { useSearchEngineStore } from '../stores/searchEngine';
import { storeToRefs } from 'pinia';
import { openUrl } from '../utils/openUrl';

type Result = {
  title: string;
  subtitle: string;
  action: () => void;
}

const isOpen = ref(false);
const query = ref('');
const focusedIndex = ref(-1);
const input = ref<HTMLInputElement | null>(null);

const router = useRouter();
const linksStore = useLinksStore();
const searchEngineStore = useSearchEngineStore();
const { links } = storeToRefs(linksStore);

const commands = [
  { title: 'Navigate to Settings', subtitle: 'Go to settings page', action: () => router.push('/settings') },
  { title: 'Add New Link', subtitle: 'Add a new link', action: () => triggerAddLink() },
];

const filteredResults = computed(() => {
  const lowerQuery = query.value.toLowerCase();
  const linkResults = links.value.filter(link => link.title.toLowerCase().includes(lowerQuery)).map(link => ({
    title: link.title,
    subtitle: link.url,
    action: () => openUrl(link.url)
  }));

  const commandResults = commands.filter(command => command.title.toLowerCase().includes(lowerQuery));

  const searchEngineResults = searchEngineStore.searchEngines.map(engine => ({
    title: `Switch to ${engine.name}`,
    subtitle: `Change search engine to ${engine.name}`,
    action: () => searchEngineStore.setSearchEngine(engine.url)
  })).filter(engineResult =>
    engineResult.title.toLowerCase().includes(lowerQuery) ||
    engineResult.subtitle.toLowerCase().includes(lowerQuery)
  );

  return [...linkResults, ...commandResults, ...searchEngineResults];
});

const handleKeydown = (event: KeyboardEvent) => {
  const totalItems = filteredResults.value.length;

  switch (event.key) {
    case 'ArrowDown':
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value + 1) % totalItems;
      break;
    case 'ArrowUp':
      event.preventDefault();
      focusedIndex.value = (focusedIndex.value - 1 + totalItems) % totalItems;
      break;
    case 'Enter':
      event.preventDefault();
      if (focusedIndex.value >= 0) {
        filteredResults.value[focusedIndex.value].action();
      } else if (filteredResults.value.length > 0) {
        filteredResults.value[0].action();
      }
      closePalette();
      break;
    case 'Escape':
      closePalette();
      break;
  }
};

const handleSelect = (result: Result) => {
  result.action();
  closePalette();
};

const openPalette = (event: KeyboardEvent) => {
  if (event.key === 'k' && event.ctrlKey) {
    event.preventDefault();
    isOpen.value = true;
    nextTick(() => {
      input.value?.focus();
    });
  }
};

const closePalette = () => {
  isOpen.value = false;
  query.value = '';
  focusedIndex.value = -1;
};

const triggerAddLink = () => {
  // Trigger the Add Link routine
  const addLinkButton = document.querySelector('#add-link-card');
  if (addLinkButton) {
    (addLinkButton as HTMLElement).click();
  }
};

const handleTriggerAddLink = (event: KeyboardEvent) => {
  if (event.key === 'n' && event.ctrlKey && event.altKey) {
    triggerAddLink();
  }
}

onMounted(() => {
  // show command palette
  window.addEventListener('keydown', openPalette);
  // add a new link
  window.addEventListener('keydown', handleTriggerAddLink);
});

onUnmounted(() => {
  window.removeEventListener('keydown', openPalette);
  window.removeEventListener('keydown', handleTriggerAddLink);
});
</script>

<style scoped>
.focused {
  background-color: white;
  color: black !important;
  padding: 8px;
}

.v-list-item {
  padding: 8px;
}
</style>

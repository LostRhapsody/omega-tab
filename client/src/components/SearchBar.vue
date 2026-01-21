<template>
  <div class="search-bar-wrapper">
    <div class="search-bar-container">
      <div class="search-bar">
        <textarea
          v-model="searchQuery"
          :placeholder="placeholder"
          @keydown="handleKeydown"
          @keydown.shift.enter="addNewLine"
          @mouseover="focusedIndex = -1"
          ref="searchInput"
          @focus="handleFocus"
          @blur="handleBlur"
          class="search-bar__input"
          :style="{ height: textareaHeight + 'px' }"
          autofocus
        />
      </div>

      <div class="search-bar__controls">
        <div class="search-bar__engine-select">
          <TpSelect
            v-model="searchEngineStore.selectedEngine"
            :options="searchEngineOptions"
            @change="updateSelectedEngine"
          >
            <template #selected="{ selected }">
              <div class="search-bar__engine-icon">
                <TpIcon
                  v-if="getSelectedEngine?.icon?.startsWith('mdi-')"
                  :name="mapMdiIcon(getSelectedEngine.icon)"
                  size="lg"
                />
                <img
                  v-else-if="getSelectedEngine?.icon"
                  :src="getSelectedEngine.icon"
                  :alt="getSelectedEngine?.name"
                  class="search-bar__custom-icon"
                />
              </div>
            </template>
          </TpSelect>
        </div>

        <TpButton
          variant="primary"
          icon-only
          @click="performSearch"
          class="search-bar__submit"
          aria-label="Search"
        >
          <TpIcon name="arrow-right" />
        </TpButton>
      </div>

      <div
        v-if="
          fuzzyResults.length ||
          (getFilteredHistory.length && searchQuery) ||
          autoSuggestions.length
        "
        class="search-bar__dropdown"
      >
        <!-- My Links section -->
        <div v-if="fuzzyResults.length" class="search-bar__section">
          <em class="search-bar__section-title">My Links</em>
          <TpDivider />
          <div
            v-for="(result, index) in fuzzyResults"
            :key="result.item.title"
            class="search-bar__item"
            :class="{ 'search-bar__item--focused': focusedIndex === index }"
            @mouseover="focusedIndex = index"
          >
            <a :href="result.item.url" class="search-bar__item-link">
              <div class="search-bar__item-title">
                <TpIcon name="link" size="sm" />
                {{ result.item.title }}
              </div>
              <span v-if="result.item.description" class="search-bar__item-desc">
                {{ result.item.description }}
              </span>
            </a>
          </div>
        </div>

        <!-- Suggestions section -->
        <div v-else-if="autoSuggestions.length" class="search-bar__section">
          <em class="search-bar__section-title">Suggestions</em>
          <TpDivider />
          <div
            v-for="(suggestion, index) in autoSuggestions"
            :key="suggestion.query"
            class="search-bar__item"
            :class="{ 'search-bar__item--focused': focusedIndex === index }"
            @mouseover="focusedIndex = index"
          >
            <a class="search-bar__item-link" @click="() => suggestionHandler(suggestion.query)">
              <div v-if="suggestion.isHistory" class="search-bar__history-item">
                <div class="search-bar__history-query">
                  <TpIcon name="clock" size="sm" />
                  {{ suggestion.query }}
                </div>
                <TpIcon name="trash" size="sm" class="search-bar__trash" />
              </div>
              <div v-else>
                {{ suggestion.query }}
              </div>
            </a>
          </div>
          <div v-if="autoSuggestions.some((s) => !s.isHistory)" class="search-bar__powered">
            <em>Suggestions POWERED BY BRAVE</em>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Fuse from "fuse.js";
import type { FuseResult } from "fuse.js";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import type { Link } from "../types/Link";
import type { Suggestions, SuggestionsResponse } from "@/types/Suggestion";
import { debounce } from "lodash";
import { searchEngines } from "../data/SearchEngines";
import { useLinksStore } from "../stores/links";
import { useUserSettingsStore } from "../stores/settings";
import { storeToRefs } from "pinia";
import { API } from "../constants/api";
import { useSearchEngineStore } from "../stores/searchEngine";
import { useUserStore } from "@/stores/user";
import { openUrl } from "../utils/openUrl";
import { useBreakpoint } from "@/composables/useBreakpoint";
import api from "@/services/api";
import { AxiosError } from "axios";
import { CacheKeys, cache } from "@/utils/cache";
import { TpSelect, TpButton, TpIcon, TpDivider } from "@/components/ui";

const AUTO_SUGGEST_ON = import.meta.env.VITE_AUTO_SUGGEST_ON === "true";
const { smAndDown: mobile } = useBreakpoint();

interface HistoryItem {
  query: string;
  freq: string;
  timestamp: number;
}

interface ScoredHistoryItem {
  query: string;
  score: number;
  matchScore: number;
  freqScore: number;
  recencyScore: number;
}

const MAX_STORED_HISTORY = 500;
const MAX_DISPLAYED_HISTORY = 5;
const MAX_HISTORY_SUGGESTIONS = 5;
const STORAGE_KEY = "search_history";

interface EnhancedSuggestion extends Suggestions {
  isHistory?: boolean;
  score?: number;
}

const linksStore = useLinksStore();
const settingsStore = useUserSettingsStore();
const { links } = storeToRefs(linksStore);

const searchQuery = ref("");
const searchHistory = ref<string[]>([]);
const historyItems = ref<HistoryItem[]>([]);
const showHistory = ref(false);
const searchInput = ref<HTMLElement | null>(null);
const searchEngineStore = useSearchEngineStore();
const selectedEngine = computed(() => searchEngineStore.selectedEngine);
const focusedIndex = ref(-1);
const fuseInstance = ref<Fuse<Link> | null>(null);
const textareaHeight = ref(50);
const maxHeight = 300;
const MAX_HISTORY_ENTRIES = Number.parseInt(import.meta.env.VITE_MAX_HISTORY_ENTRIES || "500");

const fuzzyResults = ref<FuseResult<Link>[]>([]);
const autoSuggestions = ref<EnhancedSuggestion[]>([]);
const historySuggestions = ref<ScoredHistoryItem[]>([]);
const engineChangedViaKeyboard = ref(false);

const searchEngineOptions = computed(() =>
  searchEngines.map((engine) => ({
    value: engine.url,
    label: engine.name,
  })),
);

const getSelectedEngine = computed(() =>
  searchEngines.find((engine) => engine.url === selectedEngine.value),
);

const mapMdiIcon = (mdiIcon: string) => {
  const iconMap: Record<string, string> = {
    "mdi-magnify": "search",
    "mdi-google": "search",
    "mdi-duck": "search",
    "mdi-shield-search": "search",
  };
  return iconMap[mdiIcon] || "search";
};

const placeholder = computed(() => {
  const engineName = searchEngines.find((engine) => engine.url === selectedEngine.value)?.name;
  return `Search ${engineName}...`;
});

const initializeFuse = (data: Link[]) => {
  fuseInstance.value = new Fuse(data, {
    keys: ["title", "description", "url"],
    threshold: 0.1,
    findAllMatches: false,
  });
};

watch(
  () => links.value,
  (newData) => {
    if (newData?.length) {
      initializeFuse(newData);
    }
  },
  { immediate: true },
);

const adjustHeight = () => {
  const textarea = searchInput.value;
  if (!textarea) return;

  textarea.style.height = "auto";
  const newHeight = Math.min(maxHeight, Math.max(50, textarea.scrollHeight));
  textarea.style.height = `${newHeight}px`;
  textareaHeight.value = newHeight;
};

const getFilteredHistory = computed(() => {
  if (!searchQuery.value) {
    return searchHistory.value.slice(0, MAX_DISPLAYED_HISTORY).map((item) => ({
      item,
      score: 0,
    }));
  }

  return historyFuse.value.search(searchQuery.value).slice(0, MAX_DISPLAYED_HISTORY);
});

const isCompleteURI = computed(() => {
  if (!searchQuery.value) return false;
  // Must have protocol (http:// or https://) to be treated as direct URL navigation
  if (!searchQuery.value.match(/^https?:\/\//i)) return false;
  // Cannot contain spaces (indicates a search query, not a URL)
  if (searchQuery.value.includes(" ")) return false;
  // Must be valid URL format
  return linksStore.validateUrl(searchQuery.value) === true;
});

const jiraLink = computed(() => `https://atlassian.net/browse/${searchQuery.value}`);

const confluenceLink = computed(
  () => `https://atlassian.net/wiki/search?text="${searchQuery.value}"`,
);

const historyFuse = computed(
  () =>
    new Fuse(searchHistory.value, {
      threshold: 0.3,
      findAllMatches: true,
      includeScore: true,
      keys: [
        {
          name: "query",
          weight: 2,
        },
        {
          name: "queryLower",
          weight: 1,
        },
      ],
    }),
);

const loadSearchHistory = () => {
  try {
    const stored = cache.get_search_history(CacheKeys.SEARCH_HISTORY);
    if (stored) {
      const parsed: HistoryItem[] = JSON.parse(stored);
      historyItems.value = parsed.slice(0, MAX_STORED_HISTORY);
      searchHistory.value = parsed
        .sort((a, b) => b.timestamp - a.timestamp)
        .map((item) => item.query)
        .slice(0, MAX_STORED_HISTORY);
    }
  } catch (error) {
    console.error("Error loading search history:", error);
    historyItems.value = [];
    searchHistory.value = [];
  }
};

const calculateHistorySuggestions = (query: string): ScoredHistoryItem[] => {
  if (!query || !historyItems.value.length) return [];

  const now = Date.now();
  const maxAge = 30 * 24 * 60 * 60 * 1000;
  const queryLower = query.toLowerCase();

  const maxFreq = Math.max(...historyItems.value.map((item) => parseInt(item.freq || "1")));

  const scoredItems = historyItems.value
    .map((item) => {
      let matchScore = 0;
      if (item.query.toLowerCase() === queryLower) {
        matchScore = 1;
      } else if (item.query.toLowerCase().startsWith(queryLower)) {
        matchScore = 0.8;
      } else if (item.query.toLowerCase().includes(queryLower)) {
        matchScore = 0.6;
      } else {
        const matchLength = Math.min(queryLower.length, item.query.length);
        const maxLength = Math.max(queryLower.length, item.query.length);
        matchScore = (matchLength / maxLength) * 0.4;
      }

      if (matchScore < 0.3) return null;

      const freqScore = parseInt(item.freq || "1") / maxFreq;

      const age = now - item.timestamp;
      const recencyScore = Math.max(0, 1 - age / maxAge);

      const score = matchScore * 0.5 + freqScore * 0.3 + recencyScore * 0.2;

      return {
        query: item.query,
        score,
        matchScore,
        freqScore,
        recencyScore,
      };
    })
    .filter((item) => item !== null) as ScoredHistoryItem[];

  return scoredItems.sort((a, b) => b.score - a.score).slice(0, MAX_HISTORY_SUGGESTIONS);
};

const addToHistory = (query: string) => {
  if (!query || !query.trim()) return;

  try {
    const stored = cache.get_search_history(CacheKeys.SEARCH_HISTORY);
    const history: HistoryItem[] = stored ? JSON.parse(stored) : [];

    const existingIndex = history.findIndex(
      (item) => item.query.toLowerCase() === query.toLowerCase(),
    );

    if (existingIndex !== -1) {
      history[existingIndex].freq = (parseInt(history[existingIndex].freq || "0") + 1).toString();
      history[existingIndex].timestamp = Date.now();

      const item = history.splice(existingIndex, 1)[0];
      history.unshift(item);
    } else {
      history.unshift({
        query,
        freq: "1",
        timestamp: Date.now(),
      });

      if (history.length > MAX_HISTORY_ENTRIES) {
        history.pop();
      }
    }

    cache.set_search_history(CacheKeys.SEARCH_HISTORY, JSON.stringify(history));

    historyItems.value = history.slice(0, MAX_STORED_HISTORY);
    searchHistory.value = history.map((item) => item.query);
  } catch (error) {
    console.error("Error saving to search history:", error);
  }
};

const prepareUrl = (url: string) => {
  try {
    // First, ensure we have a protocol
    let processedUrl = url;
    if (!processedUrl.startsWith("http://") && !processedUrl.startsWith("https://")) {
      processedUrl = `https://${processedUrl}`;
    }

    // Try to parse the URL to validate it
    const parsedUrl = new URL(processedUrl);

    // Return the properly formatted URL
    return parsedUrl.href;
  } catch (error) {
    // If URL parsing fails, try basic protocol addition as fallback
    if (!url.startsWith("http://") && !url.startsWith("https://")) {
      return `https://${url}`;
    }
    // If it already has a protocol but still fails parsing, return as-is
    // The browser will handle the error gracefully
    return url;
  }
};

const performSearch = () => {
  if (searchQuery.value.trim()) {
    if (fuzzyResults.value.length > 0) {
      openUrl(fuzzyResults.value[0].item.url);
    } else if (isCompleteURI.value) {
      openUrl(prepareUrl(searchQuery.value));
    } else {
      const searchUrl = selectedEngine.value + encodeURIComponent(searchQuery.value);
      openUrl(searchUrl);
    }
    addToHistory(searchQuery.value);
    searchQuery.value = "";
  }
};

const handleFocus = () => {
  if (!searchQuery.value) {
    showHistory.value = true;
  }
};

const handleBlur = () => {
  setTimeout(() => {
    showHistory.value = false;
  }, 200);
};

const selectHistoryItem = (query: string) => {
  searchQuery.value = query;
  showHistory.value = false;
  performSearch();
};

const handleKeydown = (event: KeyboardEvent) => {
  // Let Ctrl+Arrow events pass through for search engine switching
  if (event.ctrlKey && (event.key === "ArrowUp" || event.key === "ArrowDown")) {
    return;
  }

  const historyLength = getFilteredHistory.value.length;
  const fuzzyLength = fuzzyResults.value.length;
  const suggestionsLength = autoSuggestions.value.length;
  const totalItems = historyLength + fuzzyLength + suggestionsLength;

  if (totalItems > 0) {
    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        event.stopPropagation();
        focusedIndex.value = (focusedIndex.value + 1) % totalItems;
        break;
      case "ArrowUp":
        event.preventDefault();
        event.stopPropagation();
        focusedIndex.value = (focusedIndex.value - 1 + totalItems) % totalItems;
        break;
      case "Enter":
        event.preventDefault();
        if (focusedIndex.value >= 0) {
          if (focusedIndex.value < historyLength) {
            selectHistoryItem(getFilteredHistory.value[focusedIndex.value].item);
          } else if (focusedIndex.value < historyLength + suggestionsLength) {
            suggestionHandler(autoSuggestions.value[focusedIndex.value - historyLength].query);
          } else {
            const fuzzyIndex = focusedIndex.value - historyLength;
            openUrl(fuzzyResults.value[fuzzyIndex].item.url);
            searchQuery.value = "";
          }
        } else if (!event.shiftKey) {
          performSearch();
        }
        return;
    }
  } else if (event.key === "ArrowDown") {
    // No dropdown items - focus first link card
    event.preventDefault();
    const firstLinkCard = document.querySelector(".link-columns__card a") as HTMLElement;
    if (firstLinkCard) {
      firstLinkCard.focus();
    }
    return;
  } else if (event.key === "Enter" && !event.shiftKey) {
    event.preventDefault();
    performSearch();
    return;
  }
};

const updateSelectedEngine = () => {
  searchEngineStore.setSearchEngine(selectedEngine.value);
};

const addNewLine = (event: KeyboardEvent) => {
  if (event.shiftKey && event.key === "Enter") {
    event.preventDefault();
    const textarea = searchInput.value as HTMLTextAreaElement;
    if (textarea) {
      const start = textarea.selectionStart;
      const end = textarea.selectionEnd;
      searchQuery.value = `${searchQuery.value.substring(0, start)}\n${searchQuery.value.substring(end)}`;
      textarea.selectionStart = textarea.selectionEnd = start + 1;
    }
  }
};

const debouncedFuzzySearch = debounce(async (query: string) => {
  if (!fuseInstance.value || !query.trim()) {
    fuzzyResults.value = [];
    return;
  }
  fuzzyResults.value = fuseInstance.value.search(query).slice(0, 3);
}, 10);

const handleSearchEngineHotkeys = (event: KeyboardEvent) => {
  if (!event.ctrlKey || (event.key !== "ArrowUp" && event.key !== "ArrowDown")) return;

  event.preventDefault();
  const currentIndex = searchEngines.findIndex((engine) => engine.url === selectedEngine.value);
  let newIndex: number;

  if (event.key === "ArrowUp") {
    newIndex = (currentIndex - 1 + searchEngines.length) % searchEngines.length;
  } else {
    newIndex = (currentIndex + 1) % searchEngines.length;
  }

  engineChangedViaKeyboard.value = true;
  searchEngineStore.setSearchEngine(searchEngines[newIndex].url);
  updateSelectedEngine();
};

const getSuggestions = async (query: string) => {
  const historyResults = calculateHistorySuggestions(query);

  const historySuggestionsFormatted = historyResults.map((item) => ({
    query: item.query,
    score: item.score,
    isHistory: true,
  }));

  let apiSuggestions: EnhancedSuggestion[] = [];

  if (AUTO_SUGGEST_ON && settingsStore.settings.autosuggest) {
    try {
      const userStore = useUserStore();
      const authToken = userStore.getAuthToken();

      if (authToken) {
        const response = await api.get(API.SUGGEST(query), {
          headers: {
            "X-User-Authorization": authToken,
          },
        });

        if (response.status === 200) {
          const suggestionResponse = response.data as SuggestionsResponse;
          apiSuggestions = suggestionResponse.suggestions;
        }
      }
    } catch (error) {
      if ((error as AxiosError).response?.status !== 429) {
        console.error("Error fetching suggestion:", error);
        if ((error as AxiosError).response?.status === 403) {
          alert(
            "Auto-suggestions are not available for your account. This feature has been disabled.",
          );
          settingsStore.updateSetting("autosuggest", false);
        }
      }
    }
  }

  const seenQueries = new Set<string>(
    historySuggestionsFormatted.map((s) => s.query.toLowerCase()),
  );
  const uniqueApiSuggestions = apiSuggestions.filter(
    (s) => !seenQueries.has(s.query.toLowerCase()),
  );

  autoSuggestions.value = [...historySuggestionsFormatted, ...uniqueApiSuggestions];
};

const suggestionHandler = (suggestion: string) => {
  searchQuery.value = suggestion;
  performSearch();
};

const lastQuery = ref("");
const lastQueryTime = ref(0);
const DEBOUNCE_TIME = 1000;
const REQUEST_TIME = 500;
const MIN_TOKEN_SIZE = 3;

watch(searchQuery, async (newQuery) => {
  focusedIndex.value = -1;

  adjustHeight();

  if (searchQuery.value.trim().length === 0) {
    autoSuggestions.value = [];
    fuzzyResults.value = [];
    return;
  }

  if (!isCompleteURI.value) {
    await debouncedFuzzySearch(newQuery);
    await new Promise((resolve) => setTimeout(resolve, 15));

    if (fuzzyResults.value.length > 0) {
      autoSuggestions.value = [];
      return;
    }

    const now = Date.now();
    const timeSinceLastQuery = now - lastQueryTime.value;
    const charDiff = Math.abs(newQuery.length - lastQuery.value.length);

    if (
      (charDiff >= MIN_TOKEN_SIZE && timeSinceLastQuery >= REQUEST_TIME) ||
      (timeSinceLastQuery >= DEBOUNCE_TIME && newQuery !== lastQuery.value)
    ) {
      lastQuery.value = newQuery;
      lastQueryTime.value = now;
      await getSuggestions(newQuery);
    }
  } else {
    autoSuggestions.value = [];
    fuzzyResults.value = [];
  }
});

watch(selectedEngine, () => {
  // Only auto-focus search if engine was changed via keyboard shortcut
  if (engineChangedViaKeyboard.value) {
    setTimeout(() => {
      if (searchInput.value) {
        (searchInput.value as HTMLElement).focus();
      }
      engineChangedViaKeyboard.value = false;
    }, 200);
  }
});

const focusSearchInput = () => {
  if (searchInput.value) {
    searchInput.value.focus();
  }
};

const handleVisibilityChange = () => {
  if (document.visibilityState === "visible") {
    // Re-focus when tab becomes visible (e.g., user switches to this tab)
    setTimeout(focusSearchInput, 50);
    setTimeout(focusSearchInput, 150);
  }
};

onMounted(() => {
  // Aggressive focus strategy to beat browser's URL bar focus
  // Try multiple times with increasing delays to ensure focus lands on search box
  focusSearchInput();
  requestAnimationFrame(focusSearchInput);
  setTimeout(focusSearchInput, 50);
  setTimeout(focusSearchInput, 100);
  setTimeout(focusSearchInput, 150);
  setTimeout(focusSearchInput, 200);

  loadSearchHistory();
  window.addEventListener("keydown", handleSearchEngineHotkeys);
  document.addEventListener("visibilitychange", handleVisibilityChange);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleSearchEngineHotkeys);
  document.removeEventListener("visibilitychange", handleVisibilityChange);
});
</script>

<style scoped>
.search-bar-wrapper {
  margin-top: var(--tp-space-16);
  padding: 0 var(--tp-space-4);
}

@media (min-width: 640px) {
  .search-bar-wrapper {
    padding: 0 var(--tp-space-16);
  }
}

.search-bar-container {
  border: 1px solid var(--tp-border);
  border-radius: var(--tp-radius-lg);
  background: var(--tp-bg-secondary);
  transition:
    border-color var(--tp-transition-fast),
    box-shadow var(--tp-transition-fast);
  padding: var(--tp-space-4);
}

.search-bar-container:focus-within {
  border-color: var(--tp-accent);
  box-shadow: 0 0 0 3px var(--tp-accent-glow);
}

.search-bar {
  display: flex;
  align-items: center;
}

.search-bar__input {
  width: 100%;
  background: transparent;
  border: none;
  color: var(--tp-text-primary);
  font-size: var(--tp-text-base);
  font-family: var(--tp-font-sans);
  resize: none;
  overflow: auto;
  padding: var(--tp-space-3);
}

.search-bar__input:focus {
  outline: none;
}

.search-bar__input::placeholder {
  color: var(--tp-text-muted);
}

.search-bar__controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: var(--tp-space-3);
}

.search-bar__engine-select {
  width: 180px;
}

@media (max-width: 640px) {
  .search-bar__engine-select {
    width: 140px;
  }
}

.search-bar__engine-icon {
  display: flex;
  align-items: center;
}

.search-bar__custom-icon {
  width: 24px;
  height: 24px;
  object-fit: contain;
}

.search-bar__submit {
  flex-shrink: 0;
}

.search-bar__dropdown {
  border-top: 1px solid var(--tp-border);
  margin-top: var(--tp-space-4);
  padding-top: var(--tp-space-4);
}

.search-bar__section {
  margin-bottom: var(--tp-space-4);
}

.search-bar__section-title {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  font-style: normal;
}

.search-bar__item {
  padding: var(--tp-space-2);
  border-radius: var(--tp-radius-sm);
  transition: background-color var(--tp-transition-fast);
}

.search-bar__item--focused {
  background: var(--tp-bg-tertiary);
}

.search-bar__item--focused .search-bar__item-link {
  color: var(--tp-text-primary);
}

.search-bar__item-link {
  display: block;
  color: var(--tp-text-secondary);
  text-decoration: none;
  cursor: pointer;
}

.search-bar__item-title {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.search-bar__item-desc {
  display: block;
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  margin-top: var(--tp-space-1);
}

.search-bar__history-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.search-bar__history-query {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
}

.search-bar__trash {
  opacity: 0;
  transition: opacity var(--tp-transition-fast);
  color: var(--tp-text-muted);
}

.search-bar__item:hover .search-bar__trash {
  opacity: 1;
}

.search-bar__powered {
  font-size: var(--tp-text-xs);
  color: var(--tp-text-muted);
  margin-top: var(--tp-space-4);
  text-align: center;
}
</style>

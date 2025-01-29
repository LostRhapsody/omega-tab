<template>
  <div class="mt-16 mx-16 search-bar">
    <v-row>
      <v-col cols="9" class="cols">
        <v-text-field v-model="searchQuery" :placeholder="placeholder" variant="plain" hide-details
          @keyup.enter="performSearch" @keydown="handleKeydown" @mouseover="focusedIndex = -1"
          prepend-inner-icon="mdi-magnify" ref="searchInput" @focus="handleFocus" @blur="handleBlur" />
        <div v-if="isCSQuery || fuzzyResults.length || (getFilteredHistory.length && searchQuery)"
          class="dropdown-menu">
          <!-- History Section -->
          <div v-if="getFilteredHistory.length" class="section-container">
            <div class="section-header">
              <span>Recent Searches</span>
              <v-btn variant="text" density="compact" size="small" @click="clearHistory">
                Clear
              </v-btn>
            </div>
            <div v-for="(result, index) in getFilteredHistory" :key="result.item" class="dropdown-item history-item"
              :class="{ focused: focusedIndex === index }" @mouseover="focusedIndex = index"
              @click="selectHistoryItem(result.item)">
              <v-icon icon="mdi-history" size="small" class="mr-2" />
              {{ result.item }}
            </div>
            <v-divider v-if="fuzzyResults.length || isCSQuery" class="my-2"></v-divider>
          </div>
          <!-- CSQuery Section -->
          <div v-if="isCSQuery">
            <div class="dropdown-item" :class="{ focused: focusedIndex === 0 }" @mouseover="focusedIndex = 0">
              <div>Jira Link</div>
              <a :href="jiraLink" target="_blank">{{ searchQuery }} <v-icon icon="mdi-link" /></a>
            </div>
            <v-divider></v-divider>
            <div class="dropdown-item">
              <div>Other</div>
              <p>Not yet supported</p>
              <div class="pill-links">
                <a v-for="(pill, index) in pillLinks" :key="pill.text" :href="pill.link" target="_blank" class="pill"
                  :class="{ focused: focusedIndex === index + 2 }" @mouseover="focusedIndex = index + 2">{{ pill.text
                  }}</a>
              </div>
            </div>
            <v-divider></v-divider>
            <!-- Confluence Link Section -->
            <div class="dropdown-item" :class="{ focused: focusedIndex === pillLinks.length + 2 }"
              @mouseover="focusedIndex = pillLinks.length + 2">
              <div>Search Confluence</div>
              <a :href="confluenceLink" target="_blank"><v-icon icon="mdi-magnify" />"{{ searchQuery }}"</a>
            </div>
          </div>
          <div v-else>
            <!-- Tool section -->
            <div v-for="(result, index) in fuzzyResults" :key="result.item.title" class="dropdown-item"
              :class="{ focused: focusedIndex === index }" @mouseover="focusedIndex = index">
              <div>{{ result.item.title }}</div>
              <a :href="result.item.url" target="_blank">{{ result.item.description }} <v-icon icon="mdi-link" /></a>
            </div>
          </div>
        </div>
      </v-col>
      <v-col cols="3" class="cols">
        <v-select v-model="selectedEngine" :items="searchEngines" item-title="name" item-value="url" variant="plain"
          hide-details @update:modelValue="updateSelectedEngine">
          <!-- For the selected value display -->
          <template v-slot:selection="{ item }">
            {{ item.raw.name }}
          </template>

          <!-- For each item in the dropdown -->
          <template v-slot:item="{ props, item }">
            <v-list-item v-bind="props">
              <template v-slot:prepend>
                <div class="d-flex align-center">
                  <v-icon v-if="item.raw.icon.startsWith('mdi-')" :icon="item.raw.icon" size="36" class="mr-2" />
                  <img v-else :src="item.raw.icon" :alt="item.raw.name" class="custom-icon" />
                </div>
              </template>
            </v-list-item>
          </template>
        </v-select>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
import Fuse from "fuse.js";
import type { FuseResult } from "fuse.js";
import { computed, defineProps, onMounted, ref, watch } from "vue";
import { useTheme } from "vuetify";
import type { Tables } from "../types/Database";
type Link = Tables<"links">;

import { debounce } from "lodash";

interface Props {
	tools: Link[];
	docs: Link[];
}

interface HistoryItem {
	query: string;
	timestamp: number;
	engine: string;
}

const MAX_STORED_HISTORY = 100; // Maximum number of items to store
const MAX_DISPLAYED_HISTORY = 5; // Maximum number of items to display
const STORAGE_KEY = "search_history";

const props = defineProps<Props>();

const searchQuery = ref("");

// Add these to your existing refs
const searchHistory = ref<string[]>([]);
const showHistory = ref(false);

const searchInput = ref<HTMLElement | null>(null);
const searchEngines = [
	{
		icon: "mdi-google",
		name: "Google",
		url: "https://www.google.com/search?q=",
	},
	{
		icon: "mdi-microsoft-bing",
		name: "Bing",
		url: "https://www.bing.com/search?q=",
	},
	{
		icon: "icons/perplexity.png",
		name: "Perplexity",
		url: "https://www.perplexity.ai/search?q=",
	},
];

const selectedEngine = ref(
	localStorage.getItem("defaultSearchEngine") || searchEngines[0].url,
);

const placeholder = computed(() => {
	const engineName = searchEngines.find(
		(engine) => engine.url === selectedEngine.value,
	)?.name;
	return `Search ${engineName}...`;
});

// Update the loadSearchHistory function
const loadSearchHistory = () => {
	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		if (stored) {
			const parsed: HistoryItem[] = JSON.parse(stored);
			// Sort by timestamp and get just the queries
			searchHistory.value = parsed
				.sort((a, b) => b.timestamp - a.timestamp)
				.map((item) => item.query)
				.slice(0, MAX_STORED_HISTORY);
		}
	} catch (error) {
		console.error("Error loading search history:", error);
		searchHistory.value = [];
	}
};

const addToHistory = (query: string) => {
	if (!query || !query.trim() || isCSQuery.value) return;

	try {
		const stored = localStorage.getItem(STORAGE_KEY);
		const history: HistoryItem[] = stored ? JSON.parse(stored) : [];

		// Remove any existing entry with the same query
		const filtered = history.filter(
			(item) => item.query.toLowerCase() !== query.toLowerCase(),
		);

		// Add new entry
		filtered.unshift({
			query,
			timestamp: Date.now(),
			engine: selectedEngine.value,
		});

		// Keep only MAX_STORED_HISTORY items
		const trimmed = filtered.slice(0, MAX_STORED_HISTORY);

		// Save to localStorage
		localStorage.setItem(STORAGE_KEY, JSON.stringify(trimmed));

		// Update the reactive history
		searchHistory.value = trimmed.map((item) => item.query);
	} catch (error) {
		console.error("Error saving to search history:", error);
	}
};

// Add function to get filtered history results
const getFilteredHistory = computed(() => {
	if (!searchQuery.value) {
		// When empty, show most recent items
		return searchHistory.value.slice(0, MAX_DISPLAYED_HISTORY).map((item) => ({
			item,
			score: 0,
		}));
	}

	// Use fuzzy search when there's input
	return historyFuse.value
		.search(searchQuery.value)
		.slice(0, MAX_DISPLAYED_HISTORY);
});

const clearHistory = (query: string) => {
	searchHistory.value = [];
	localStorage.removeItem(STORAGE_KEY);
};

// Modify your existing performSearch function
const performSearch = () => {
	if (searchQuery.value.trim()) {
		const searchUrl =
			selectedEngine.value + encodeURIComponent(searchQuery.value);
		window.open(searchUrl, "_blank");
		addToHistory(searchQuery.value); // Add this line
		searchQuery.value = "";
	}
};

// Add these event handlers
const handleFocus = () => {
	if (!searchQuery.value) {
		showHistory.value = true;
	}
};

const handleBlur = () => {
	// Small delay to allow for clicking history items
	setTimeout(() => {
		showHistory.value = false;
	}, 200);
};

const selectHistoryItem = (query: string) => {
	searchQuery.value = query;
	showHistory.value = false;
	performSearch();
};

const isCSQuery = computed(() => {
	const result = /^(CS|ERP|WMS|RD)-\d{3,6}$/i.test(searchQuery.value);
	console.log("isCSQuery:", result);
	return result;
});

// Replace the isCompleteURI computed property with this optimized version
const isCompleteURI = computed(() => {
	// Early return for empty strings or strings without dots
	if (!searchQuery.value || !searchQuery.value.includes(".")) {
		return false;
	}

	try {
		// Use URL constructor for validation instead of regex
		new URL(
			searchQuery.value.startsWith("http")
				? searchQuery.value
				: `https://${searchQuery.value}`,
		);
		return true;
	} catch {
		return false;
	}
});

const jiraLink = computed(
	() => `https://atlassian.net/browse/${searchQuery.value}`,
);

const confluenceLink = computed(
	() => `https://atlassian.net/wiki/search?text="${searchQuery.value}"`,
);

const pillLinks = [
	{ text: "ERP", link: "https://erp.com" },
	{ text: "WMS", link: "https://wms.com" },
	{ text: "B2B", link: "https://b2b.com" },
	{ text: "B2C", link: "https://b2c.com" },
	{ text: "Compiler", link: "https://compiler.com" },
];

const focusedIndex = ref(-1);

const handleKeydown = (event: KeyboardEvent) => {
	const historyLength = getFilteredHistory.value.length;
	const fuzzyLength = fuzzyResults.value.length;
	const totalItems = historyLength + fuzzyLength;

	if (totalItems > 0) {
		switch (event.key) {
			case "ArrowDown":
				event.preventDefault();
				focusedIndex.value = (focusedIndex.value + 1) % totalItems;
				break;
			case "ArrowUp":
				event.preventDefault();
				focusedIndex.value = (focusedIndex.value - 1 + totalItems) % totalItems;
				break;
			case "Enter":
				event.preventDefault();
				if (focusedIndex.value >= 0) {
					if (focusedIndex.value < historyLength) {
						// Handle history item selection
						selectHistoryItem(
							getFilteredHistory.value[focusedIndex.value].item,
						);
					} else {
						// Handle fuzzy result selection
						const fuzzyIndex = focusedIndex.value - historyLength;
						window.open(fuzzyResults.value[fuzzyIndex].item.url, "_blank");
						searchQuery.value = "";
					}
				}
				return;
		}
	}
	// Handle CS ticket queries
	else if (isCSQuery.value) {
		switch (event.key) {
			case "ArrowDown":
				event.preventDefault();
				focusedIndex.value = (focusedIndex.value + 1) % (pillLinks.length + 3);
				break;
			case "ArrowUp":
				event.preventDefault();
				focusedIndex.value =
					(focusedIndex.value - 1 + (pillLinks.length + 3)) %
					(pillLinks.length + 3);
				break;
			case "Enter":
				event.preventDefault();
				if (focusedIndex.value === 0) {
					window.open(jiraLink.value, "_blank");
				} else if (
					focusedIndex.value >= 2 &&
					focusedIndex.value < pillLinks.length + 2
				) {
					window.open(pillLinks[focusedIndex.value - 2].link, "_blank");
				} else if (focusedIndex.value === pillLinks.length + 2) {
					window.open(confluenceLink.value, "_blank");
				}
				searchQuery.value = ""; // Clear the search after opening
				return; // Prevent further processing
		}
	}
	// Handle complete URI
	else if (isCompleteURI.value && event.key === "Enter") {
		event.preventDefault();
		window.open(
			searchQuery.value.startsWith("http")
				? searchQuery.value
				: `https://${searchQuery.value}`,
			"_blank",
		);
		searchQuery.value = "";
		return;
	}
};

const updateSelectedEngine = () => {
	localStorage.setItem("defaultSearchEngine", selectedEngine.value);
};

const theme = useTheme();

onMounted(() => {
	if (searchInput.value) {
		searchInput.value.focus();
	}
	loadSearchHistory();
});

// Fuzzy search setup
const fuse = new Fuse<Link>([...props.tools, ...props.docs], {
	keys: ["title", "description"],
	threshold: 0.3,
	findAllMatches: false,
});

// A new Fuse instance for history search
const historyFuse = computed(
	() =>
		new Fuse(searchHistory.value, {
			threshold: 0.3,
			findAllMatches: true,
			// Including fields that help match both start of string and anywhere in string
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

const fuzzyResults = ref<FuseResult<Link>[]>([]);

// Create a debounced search function
const debouncedFuzzySearch = debounce((query: string) => {
	if (query.trim()) {
		fuzzyResults.value = fuse.search(query).slice(0, 3);
	} else {
		fuzzyResults.value = [];
	}
}, 100); // 100ms delay

watch(searchQuery, (newQuery) => {
	if (!isCSQuery.value && !isCompleteURI.value) {
		debouncedFuzzySearch(newQuery);
	} else {
		fuzzyResults.value = [];
	}
});
</script>

<style>
  .cols {
    padding: 1rem;
  }

  .search-bar {
    display: flex;
    align-items: center;
    border: 1px solid dimgray;
    border-radius: 12px;
    margin: 0rem 8rem;
  }

  .search-bar:focus-within {
    border: 1px solid rgb(170, 170, 170) !important;
  }

  .v-field__input,
  .v-field__prepend-inner,
  .v-field__append-inner {
    padding: 0px !important;
    display: flex;
    align-items: center !important;
    padding-right: 4px !important;
  }

  .dropdown-menu {
    border-radius: 8px;
    padding: 1rem;
    margin-top: 0.5rem;
  }

  .dropdown-item {
    margin-bottom: 1rem;
    color: var(--color-text);
    padding: 4px;
  }

  .dropdown-item a {
    color: var(--color-text);
    text-decoration: underline;
  }

  .pill-links {
    display: flex;
    gap: 0.5rem;
  }

  .pill {
    background: black;
    border-radius: 16px;
    padding: 0.25rem 0.5rem;
    color: var(--color-text);
    text-decoration: none;
  }

  .pill:hover {
    background: #d0d0d0;
  }

  .focused,
  .focused a {
    background-color: whitesmoke;
    color: black !important;
    border-radius: 12px;
  }

  .custom-icon {
    width: 36px;
    height: 36px;
    margin-right: 8px;
    /* Add right margin to match mr-2 */
    vertical-align: middle;
    /* Ensure vertical alignment with text */

  }

  .history-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    color: var(--color-text);
    opacity: 0.7;
  }

  .history-item {
    display: flex;
    align-items: center;
    padding: 0.5rem;
    cursor: pointer;
    transition: background-color 0.2s;
  }

  .history-item:hover {
    background-color: rgba(128, 128, 128, 0.1);
  }
</style>

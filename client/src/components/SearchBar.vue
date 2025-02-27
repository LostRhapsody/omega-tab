<template>
	<div class="mt-16 sm:mx-16">
		<div class="searchBarContainer">
			<v-container>
				<v-row>
					<v-col>
						<textarea v-model="searchQuery" :placeholder="placeholder"
							@keydown="handleKeydown" @keydown.shift.enter="addNewLine" @mouseover="focusedIndex = -1" ref="searchInput"
							@focus="handleFocus" @blur="handleBlur"
							class="overflow-auto focus:outline-none focus:ring-0 focus-visible:ring-0 focus-visible:outline-none searchBar mt-0 resize-none"
							:style="{ height: textareaHeight + 'px' }" />
					</v-col>
				</v-row>
				<v-row>
					<v-col>
						<v-row class="flex justify-between">
							<v-col :cols="mobile ? 6: 2" class="select-container">
								<v-select v-model="searchEngineStore.selectedEngine" :items="searchEngines" item-title="name"
									item-value="url" variant="solo-inverted" hide-details
									>
									<template v-slot:selection="{ item }">
										<div class="d-flex align-center">
											<v-icon v-if="item.raw.icon.startsWith('mdi-')" :icon="item.raw.icon" size="36" class="mr-2" />
											<img v-else :src="item.raw.icon" :alt="item.raw.name" class="custom-icon" />
										</div>
									</template>

									<template v-slot:item="{ props, item }">
										<v-list-item v-bind="props" @click="updateSelectedEngine">
											<template v-slot:prepend>
												<div class="d-flex align-center">
													<v-icon v-if="item.raw.icon.startsWith('mdi-')"
														:icon="item.raw.icon" size="36" class="mr-2" />
													<img v-else :src="item.raw.icon" :alt="item.raw.name"
														class="custom-icon" />
												</div>
											</template>
										</v-list-item>
									</template>
								</v-select>
							</v-col>
							<v-col :cols="mobile ? 6: 2" class="flex justify-end items-end">
								<v-btn icon="mdi-arrow-right" @click="performSearch" class="search-btn">
								</v-btn>
							</v-col>
						</v-row>
					</v-col>
				</v-row>
				<div v-if="fuzzyResults.length || (getFilteredHistory.length && searchQuery) || autoSuggestions.length" class="dropdown-menu">
					<div>

						<!-- Tool section -->
						<div v-for="(result, index) in fuzzyResults" :key="result.item.title" class="dropdown-item"
							:class="{ focused: focusedIndex === index }" @mouseover="focusedIndex = index">
							<div>
								<a :href="result.item.url">
									<div> <v-icon icon="mdi-link" /> {{ result.item.title }}</div>
									<span v-if="result.item.description">{{ result.item.description }}</span>
								</a>
							</div>
						</div>

						<!-- Suggestions -->
						<div v-for="(suggestion, index) in autoSuggestions" :key="suggestion.query" class="dropdown-item"
							:class="{ focused: focusedIndex === index }"
							@mouseover="focusedIndex = index">
							<a variant="plain" @click="() => suggestionHandler(suggestion.query)">{{ suggestion.query }}</a>
						</div>
						<div v-if="autoSuggestions.length">
							<v-divider class="mb-2" />
							<em>Suggestions POWERED BY BRAVE</em>
						</div>
					</div>
				</div>
			</v-container>
		</div>
	</div>
</template>

<script setup lang="ts">
// imports
import Fuse from "fuse.js";
import type { FuseResult } from "fuse.js";
import { computed, defineProps, onMounted, onUnmounted, ref, watch } from "vue";
import type { Link } from "../types/Link";
import type { Suggestions, SuggestionsResponse } from "@/types/Suggestion";
import { debounce } from "lodash";
import { searchEngines } from "../data/SearchEngines";
import { useLinksStore } from "../stores/links";
import { useUserSettingsStore } from "../stores/settings";
import { storeToRefs } from "pinia";
import { API } from "../constants/api";
import { useSearchEngineStore } from '../stores/searchEngine';
import { openUrl } from '../utils/openUrl';
import { useDisplay } from 'vuetify';
import api from "@/services/api";

const AUTO_SUGGEST_ON = import.meta.env.VITE_AUTO_SUGGEST_ON === 'true';
const mobile = useDisplay().smAndDown;

interface HistoryItem {
	query: string;
	timestamp: number;
	engine: string;
}

// consts and refs
const MAX_STORED_HISTORY = 100; // Maximum number of items to store
const MAX_DISPLAYED_HISTORY = 5; // Maximum number of items to display
const STORAGE_KEY = "search_history";

const linksStore = useLinksStore();
const settingsStore = useUserSettingsStore();
const { links } = storeToRefs(linksStore)

const searchQuery = ref("");
const searchHistory = ref<string[]>([]);
const showHistory = ref(false);
const searchInput = ref<HTMLElement | null>(null);
const searchEngineStore = useSearchEngineStore();
const selectedEngine = computed(() => searchEngineStore.selectedEngine);
const focusedIndex = ref(-1);
const fuseInstance = ref<Fuse<Link> | null>(null);
const textareaHeight = ref(50);
const maxHeight = 300;

// Fuzzy search setup
const fuzzyResults = ref<FuseResult<Link>[]>([]);
const autoSuggestions = ref<Suggestions[]>([]);

// computed properties
const placeholder = computed(() => {
	const engineName = searchEngines.find(
		(engine) => engine.url === selectedEngine.value,
	)?.name;
	return `Search ${engineName}...`;
});

const initializeFuse = (data: Link[]) => {
	fuseInstance.value = new Fuse(data, {
		keys: ["title", "description", "url"],
		threshold: 0.1,
		findAllMatches: false,
	});
}

watch(links, (newData) => {
	if (newData?.length) {
		initializeFuse(newData);
	}
}, { immediate: true });

// Method to adjust height
const adjustHeight = () => {
	const textarea = searchInput.value;
	if (!textarea) return;

	textarea.style.height = 'auto';
	const newHeight = Math.min(maxHeight, Math.max(50, textarea.scrollHeight));
	textarea.style.height = `${newHeight}px`;
	textareaHeight.value = newHeight;
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

// Replace the isCompleteURI computed property with this optimized version
const isCompleteURI = computed(() => {
	// Early return for empty strings or strings without dots
	if (!searchQuery.value || !searchQuery.value.includes(".")) {
		return false;
	}

	if (linksStore.validateUrl(searchQuery.value)) {
		return true;
	}

	return false;

});

const jiraLink = computed(
	() => `https://atlassian.net/browse/${searchQuery.value}`,
);

const confluenceLink = computed(
	() => `https://atlassian.net/wiki/search?text="${searchQuery.value}"`,
);

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

// functions
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
	if (!query || !query.trim()) return;

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

const clearHistory = (query: string) => {
	searchHistory.value = [];
	localStorage.removeItem(STORAGE_KEY);
};

// Modify your existing performSearch function
const performSearch = () => {
	if (searchQuery.value.trim()) {
		// If there are fuzzy results, open the first result's URL
		if (fuzzyResults.value.length > 0) {
			openUrl(fuzzyResults.value[0].item.url);
		} else {
			// Otherwise perform normal search
			const searchUrl = selectedEngine.value + encodeURIComponent(searchQuery.value);
			openUrl(searchUrl);
		}
		addToHistory(searchQuery.value);
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

const handleKeydown = (event: KeyboardEvent) => {
	const historyLength = getFilteredHistory.value.length;
	const fuzzyLength = fuzzyResults.value.length;
	const suggestionsLength = autoSuggestions.value.length;
	const totalItems = historyLength + fuzzyLength + suggestionsLength;

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
					}
					// handle the auto suggestions
					else if (focusedIndex.value < historyLength + suggestionsLength) {
						suggestionHandler(autoSuggestions.value[focusedIndex.value - historyLength].query);
					}
					else {
						// Handle fuzzy result selection
						const fuzzyIndex = focusedIndex.value - historyLength;
						openUrl(fuzzyResults.value[fuzzyIndex].item.url);
						searchQuery.value = "";
					}
				}
				if(!event.shiftKey){
					performSearch();
				}
				return;
		}
	}
	if(event.key === "Enter" && !event.shiftKey){
		performSearch();
	}
	// Handle CS ticket queries
	// else if (isCSQuery.value) {
	// 	switch (event.key) {
	// 		case "ArrowDown":
	// 			event.preventDefault();
	// 			focusedIndex.value = (focusedIndex.value + 1) % (pillLinks.length + 3);
	// 			break;
	// 		case "ArrowUp":
	// 			event.preventDefault();
	// 			focusedIndex.value =
	// 				(focusedIndex.value - 1 + (pillLinks.length + 3)) %
	// 				(pillLinks.length + 3);
	// 			break;
	// 		case "Enter":
	// 			event.preventDefault();
	// 			if (focusedIndex.value === 0) {
	// 				window.open(jiraLink.value, "_blank");
	// 			} else if (
	// 				focusedIndex.value >= 2 &&
	// 				focusedIndex.value < pillLinks.length + 2
	// 			) {
	// 				window.open(pillLinks[focusedIndex.value - 2].link, "_blank");
	// 			} else if (focusedIndex.value === pillLinks.length + 2) {
	// 				window.open(confluenceLink.value, "_blank");
	// 			}
	// 			searchQuery.value = ""; // Clear the search after opening
	// 			return; // Prevent further processing
	// 	}
	// }
	// Handle complete URI
	else if (isCompleteURI.value && event.key === "Enter") {
		event.preventDefault();
		openUrl(
			searchQuery.value.startsWith("http")
				? searchQuery.value
				: `https://${searchQuery.value}`
		);
		searchQuery.value = "";
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

// Create a debounced search function
const debouncedFuzzySearch = debounce(async (query: string) => {
	if (!fuseInstance.value || !query.trim()) {
		fuzzyResults.value = [];
		return;
	}
	fuzzyResults.value = fuseInstance.value.search(query).slice(0, 3);
}, 10); // 100ms delay

// Add event listener for ctrl+arrow keys to cycle through search engines
const handleSearchEngineHotkeys = (event: KeyboardEvent) => {
	if (!event.ctrlKey || (event.key !== 'ArrowUp' && event.key !== 'ArrowDown')) return;

	event.preventDefault();
	const currentIndex = searchEngines.findIndex(engine => engine.url === selectedEngine.value);
	let newIndex: number;

	if (event.key === 'ArrowUp') {
		newIndex = (currentIndex - 1 + searchEngines.length) % searchEngines.length;
	} else {
		newIndex = (currentIndex + 1) % searchEngines.length;
	}

	searchEngineStore.setSearchEngine(searchEngines[newIndex].url);
	updateSelectedEngine();
};

const getSuggestions = async (query: string) => {
	if(!AUTO_SUGGEST_ON) return;
	try {
		const response = await api.get(API.SUGGEST(query));
		const suggestionResponse = response.data as SuggestionsResponse;
		// todo make this type safer
		autoSuggestions.value = suggestionResponse.suggestions;
	} catch (error) {
		console.error("Error fetching suggestion:", error);
	}
};

const suggestionHandler = (suggestion: string) => {
	searchQuery.value = suggestion;
	performSearch();
};

// watch, mount, and unmount
watch(searchQuery, async (newQuery) => {
	// if you type more stuff, reset the focused index,
	// so we don't have the wrong thing selected by accident
	focusedIndex.value = -1;

	// Run height adjustment when text changes
	adjustHeight();

    // Clear results if the search query is empty
    if (searchQuery.value.trim().length === 0) {
        autoSuggestions.value = [];
        fuzzyResults.value = [];
        return;
    }

    // if is not a complete URI, perform fuzzy search for links
    if (!isCompleteURI.value) {

		await debouncedFuzzySearch(newQuery);
		// this is here on purpose. The debounced search has a 10 ms delay
		// so we need to wait for it to finish before fetching suggestions
		await new Promise(resolve => setTimeout(resolve, 15));

        // if there are fuzzy results, only display fuzzy results
        if (fuzzyResults.value.length > 0) {
            autoSuggestions.value = [];
            return;
        }

        // if no fuzzy results, get search suggestions
        await getSuggestions(newQuery);
    } else {
        // If it's a complete URI, clear the fuzzy results and suggestions
        autoSuggestions.value = [];
        fuzzyResults.value = [];
    }
});

// Add new watcher for selectedEngine
watch(selectedEngine, () => {
	setTimeout(() => {
    if (searchInput.value) {
      (searchInput.value as HTMLElement).focus();
    }
  }, 200);
});

onMounted(() => {
	if (searchInput.value) {
		searchInput.value.focus();
	}
	loadSearchHistory();
	window.addEventListener('keydown', handleSearchEngineHotkeys);
});

onUnmounted(() => {
	window.removeEventListener('keydown', handleSearchEngineHotkeys);
});
</script>

<style scoped>
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
	vertical-align: middle;
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

.searchBar {
	display: flex;
	align-items: center;
	padding: 0.75rem;
	background-color: var(--color-background);
	width: 100%;
	margin-top: 1rem;
}

.searchBar input {
	flex: 1;
	border: none;
	outline: none;
	padding: 0.75rem;
	background-color: transparent;
	color: var(--color-text);
}

.searchBar button {
	background: none;
	border: none;
	cursor: pointer;
	padding: 0.75rem;
	color: var(--color-text);
}

.searchBar button:hover {
	color: var(--color-text-hover);
}

.searchBarContainer {
	border: #ffffff1e 1px solid;
	border-radius: 1em;
	transition: border-color 0.3s ease, box-shadow 0.3s ease;
}

.searchBarContainer:focus-within {
	border: #ffffff1e 1px solid;
	box-shadow: 0 2px 10px 1px rgba(255, 255, 255, 0.1);
}

/* Add responsive styles */
@media (max-width: 600px) {

}
</style>

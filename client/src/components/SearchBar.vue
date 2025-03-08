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
						<!-- my links section -->
						<div v-if="fuzzyResults.length">
							<em>My Links</em>
							<v-divider class="mb-2" />						
							<div v-for="(result, index) in fuzzyResults" :key="result.item.title" class="dropdown-item"
								:class="{ focused: focusedIndex === index}" @mouseover="focusedIndex = index">
								<div>
									<a :href="result.item.url">
										<div> <v-icon icon="mdi-link" /> {{ result.item.title }}</div>
										<span v-if="result.item.description">{{ result.item.description }}</span>
									</a>
								</div>
							</div>
						</div>
						<!-- Suggestions -->
						<div v-else-if="autoSuggestions.length">
							<em>Suggestions</em>
							<v-divider class="mb-2" />
							<div v-for="(suggestion, index) in autoSuggestions" :key="suggestion.query" class="dropdown-item"
								:class="{ focused: focusedIndex === index }"
								@mouseover="focusedIndex = index">
								<a variant="plain" @click="() => suggestionHandler(suggestion.query)" class="d-flex align-center">
									<div v-if="suggestion.isHistory" class="flex justify-between w-full">
										<div>
											<v-icon v-if="suggestion.isHistory" icon="mdi-history" size="small" class="mr-1" />
											{{ suggestion.query }}
										</div>
										<v-icon 
											icon="mdi-trash-can" 
											size="small" 
											class="mr-1 trash-can"
										/>
									</div>
									<div v-else>
										{{ suggestion.query }}
									</div>
								</a>
							</div>
							<div v-if="autoSuggestions.some(s => !s.isHistory)">
								<em>Suggestions POWERED BY BRAVE</em>
							</div>
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
import { useUserStore } from "@/stores/user";
import { openUrl } from '../utils/openUrl';
import { useDisplay } from 'vuetify';
import api from "@/services/api";
import { AxiosError } from "axios";
import { CacheKeys, cache } from "@/utils/cache";

const AUTO_SUGGEST_ON = import.meta.env.VITE_AUTO_SUGGEST_ON === 'true';
const mobile = useDisplay().smAndDown;

interface HistoryItem {
	query: string;
	freq: string;
	timestamp: number;
}

interface ScoredHistoryItem {
	query: string;
	score: number; // Combined score for ranking
	matchScore: number; // How well it matches the search query
	freqScore: number; // Normalized frequency score
	recencyScore: number; // Normalized recency score
}

// consts and refs
const MAX_STORED_HISTORY = 500; // Maximum number of items to store
const MAX_DISPLAYED_HISTORY = 5; // Maximum number of history items to display in suggestions
const MAX_HISTORY_SUGGESTIONS = 5; // Maximum number of history suggestions to mix with API suggestions
const STORAGE_KEY = "search_history";

// Define a more complete suggestion type that can handle both history and API suggestions
interface EnhancedSuggestion extends Suggestions {
  isHistory?: boolean;
  score?: number;
}

const linksStore = useLinksStore();
const settingsStore = useUserSettingsStore();
const { links } = storeToRefs(linksStore)

const searchQuery = ref("");
const searchHistory = ref<string[]>([]); // For backward compatibility
const historyItems = ref<HistoryItem[]>([]); // Complete history items with metadata
const showHistory = ref(false);
const searchInput = ref<HTMLElement | null>(null);
const searchEngineStore = useSearchEngineStore();
const selectedEngine = computed(() => searchEngineStore.selectedEngine);
const focusedIndex = ref(-1);
const fuseInstance = ref<Fuse<Link> | null>(null);
const textareaHeight = ref(50);
const maxHeight = 300;
const MAX_HISTORY_ENTRIES = Number.parseInt(import.meta.env.VITE_MAX_HISTORY_ENTRIES || '500');

// Fuzzy search setup
const fuzzyResults = ref<FuseResult<Link>[]>([]);
const autoSuggestions = ref<EnhancedSuggestion[]>([]); // Updated to use our enhanced type
const historySuggestions = ref<ScoredHistoryItem[]>([]);

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
		const stored = cache.get_search_history(CacheKeys.SEARCH_HISTORY);
		if (stored) {
			const parsed: HistoryItem[] = JSON.parse(stored);
				// Store complete history items
				historyItems.value = parsed.slice(0, MAX_STORED_HISTORY);
				// For backward compatibility, still keep searchHistory
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

// Calculate history suggestions based on query matching, frequency and recency
const calculateHistorySuggestions = (query: string): ScoredHistoryItem[] => {
	if (!query || !historyItems.value.length) return [];
	
	const now = Date.now();
	const maxAge = 30 * 24 * 60 * 60 * 1000; // 30 days in milliseconds
	const queryLower = query.toLowerCase();
	
	// Get max frequency for normalization
	const maxFreq = Math.max(...historyItems.value.map(item => parseInt(item.freq || '1')));
	
	// Score each history item
	const scoredItems = historyItems.value.map(item => {
		// Calculate match score (exact match gets highest score)
		let matchScore = 0;
		if (item.query.toLowerCase() === queryLower) {
			matchScore = 1;
		} else if (item.query.toLowerCase().startsWith(queryLower)) {
			matchScore = 0.8;
		} else if (item.query.toLowerCase().includes(queryLower)) {
			matchScore = 0.6;
		} else {
			// Fuzzy match based on query length vs item length (simple approximation)
			const matchLength = Math.min(queryLower.length, item.query.length);
			const maxLength = Math.max(queryLower.length, item.query.length);
			matchScore = matchLength / maxLength * 0.4; // max 0.4 for fuzzy matches
		}
		
		// Skip items with very low match score
		if (matchScore < 0.3) return null;
		
		// Calculate frequency score (normalized)
		const freqScore = parseInt(item.freq || '1') / maxFreq;
		
		// Calculate recency score (newer gets higher score)
		const age = now - item.timestamp;
		const recencyScore = Math.max(0, 1 - (age / maxAge));
		
		// Combined score with weights: match (0.5) + frequency (0.3) + recency (0.2)
		const score = (matchScore * 0.5) + (freqScore * 0.3) + (recencyScore * 0.2);
		
		return {
			query: item.query,
			score,
			matchScore,
			freqScore,
			recencyScore
		};
	}).filter(item => item !== null) as ScoredHistoryItem[];
	
	// Sort by combined score and return top results
	return scoredItems.sort((a, b) => b.score - a.score)
		.slice(0, MAX_HISTORY_SUGGESTIONS);
};

const addToHistory = (query: string) => {
	if (!query || !query.trim()) return;

	try {
		const stored = cache.get_search_history(CacheKeys.SEARCH_HISTORY);
		const history: HistoryItem[] = stored ? JSON.parse(stored) : [];

		// Check if query already exists
		const existingIndex = history.findIndex(
			(item) => item.query.toLowerCase() === query.toLowerCase()
		);

		if (existingIndex !== -1) {
			// Increment frequency count
			history[existingIndex].freq = (parseInt(history[existingIndex].freq || '0') + 1).toString();			
			history[existingIndex].timestamp = Date.now(); // Update timestamp to now
			
			// Move this item to the beginning of the array
			const item = history.splice(existingIndex, 1)[0];
			history.unshift(item);
		} else {
			// Add new entry
			history.unshift({
				query,
				freq: '1',
				timestamp: Date.now()
			});

			// If we've exceeded the maximum, remove the oldest entry
			if (history.length > MAX_HISTORY_ENTRIES) {
				history.pop();
			}
		}

		// Save to local Storage
		cache.set_search_history(CacheKeys.SEARCH_HISTORY, JSON.stringify(history));

		// Update the reactive history arrays
		historyItems.value = history.slice(0, MAX_STORED_HISTORY);
		searchHistory.value = history.map((item) => item.query);
	} catch (error) {
		console.error("Error saving to search history:", error);
	}
};

// Add a function to prepare URL (add protocol if needed)
const prepareUrl = (url: string) => {
  if (!url.startsWith('http://') && !url.startsWith('https://')) {
    return `https://${url}`;
  }
  return url;
};

// Modify your existing performSearch function
const performSearch = () => {
	if (searchQuery.value.trim()) {
		// If there are fuzzy results, open the first result's URL
		if (fuzzyResults.value.length > 0) {
			openUrl(fuzzyResults.value[0].item.url);
			// If it's a valid URL, open it directly
		} else if (isCompleteURI.value) {
			openUrl(prepareUrl(searchQuery.value));
			// Otherwise perform normal search
		} else {
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
				} else if(!event.shiftKey) {
					performSearch();
				}
				return;
		}
	} else if(event.key === "Enter" && !event.shiftKey) {
		// Handle direct URL input or search
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
	// Calculate history suggestions first
	const historyResults = calculateHistorySuggestions(query);
	
	// Convert history suggestions to the same format as API suggestions
	const historySuggestionsFormatted = historyResults.map(item => ({
		query: item.query,
		score: item.score,
		isHistory: true // Add flag to identify history suggestions
	}));
	
	let apiSuggestions: EnhancedSuggestion[] = []; // Updated to use our enhanced type
	
	// Get API suggestions if enabled
	if(AUTO_SUGGEST_ON && settingsStore.settings.autosuggest){
		try {
			const userStore = useUserStore();
			const authToken = userStore.getAuthToken();
			
			// Only proceed if we have an auth token
			if (authToken) {
				const response = await api.get(API.SUGGEST(query), {
					headers: {
						'X-User-Authorization': authToken
					}
				});
				
				// Check for successful response
				if (response.status === 200) {
					const suggestionResponse = response.data as SuggestionsResponse;
					apiSuggestions = suggestionResponse.suggestions;
				}
			}
		} catch (error) {
			// Silently handle 429 errors (too many requests)
			if ((error as AxiosError).response?.status !== 429) {
				console.error("Error fetching suggestion:", error);
				if ((error as AxiosError).response?.status === 403) {
					alert('Auto-suggestions are not available for your account. This feature has been disabled.');
					settingsStore.updateSetting('autosuggest', false);
				}
			}
		}
	}
	
	// Combine history and API suggestions, giving preference to history suggestions
	// We'll show all history suggestions first (up to MAX_HISTORY_SUGGESTIONS),
	// then fill the rest with API suggestions
	
	// Get unique suggestions (avoid duplicates between history and API)
	const seenQueries = new Set<string>(historySuggestionsFormatted.map(s => s.query.toLowerCase()));
	const uniqueApiSuggestions = apiSuggestions.filter(s => !seenQueries.has(s.query.toLowerCase()));
	
	// Combine the suggestions
	autoSuggestions.value = [
		...historySuggestionsFormatted,
		...uniqueApiSuggestions
	];
};

const suggestionHandler = (suggestion: string) => {
	searchQuery.value = suggestion;
	performSearch();
};

// Add these variables near the other refs and constants
const lastQuery = ref(""); // Keep track of the last query we sent to the API
const lastQueryTime = ref(0); // Keep track of when we last sent a request
const DEBOUNCE_TIME = 1000; // milliseconds to wait for user typing to settle
const REQUEST_TIME = 500; // milliseconds to wait for user typing to settle
const MIN_TOKEN_SIZE = 3; // minimum characters difference to trigger a new request

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
		// Always perform the local fuzzy search (it's fast and doesn't hit APIs)
		await debouncedFuzzySearch(newQuery);
		// this is here on purpose. The debounced search has a 10 ms delay
		// so we need to wait for it to finish before fetching suggestions
		await new Promise(resolve => setTimeout(resolve, 15));

        // if there are fuzzy results, only display fuzzy results
        if (fuzzyResults.value.length > 0) {
            autoSuggestions.value = [];
            return;
        }

        // Rate limit checks before getting search suggestions
        const now = Date.now();
        const timeSinceLastQuery = now - lastQueryTime.value;
        const charDiff = Math.abs(newQuery.length - lastQuery.value.length);
        
        // Get suggestions if:
        // 1. We have enough new characters (tokenization) OR
        // 2. Enough time has passed since last request AND the query is different
        if (
            (charDiff >= MIN_TOKEN_SIZE && timeSinceLastQuery >= REQUEST_TIME) || 
            (timeSinceLastQuery >= DEBOUNCE_TIME && newQuery !== lastQuery.value)
        ) {
            lastQuery.value = newQuery;
            lastQueryTime.value = now;
            await getSuggestions(newQuery);
        }
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

.trash-can {
	display: none;
}

:hover.trash-can {
	display: block;
}

/* Add responsive styles */
@media (max-width: 600px) {

}
</style>

import { CacheKeys, cache } from "@/utils/cache";
import { defineStore } from "pinia";

export const useSearchEngineStore = defineStore("searchEngine", {
  state: () => {
    const cachedEngine = cache.get<string>(CacheKeys.SEARCH_ENGINE);
    return {
      searchEngines: [
        { name: "Brave", url: "https://search.brave.com/search?q=" },
        { name: "Perplexity", url: "https://www.perplexity.ai/search?q=" },
        { name: "Google", url: "https://www.google.com/search?q=" },
        { name: "Bing", url: "https://www.bing.com/search?q=" },
      ],
      selectedEngine: cachedEngine || "https://www.google.com/search?q=",
    };
  },
  actions: {
    setSearchEngine(url: string) {
      this.selectedEngine = url;
      cache.set(CacheKeys.SEARCH_ENGINE, url);
    },
  },
});

const CACHE_PREFIX = "betternewtab_";
const CACHE_VERSION = "v1_";

export const CacheKeys = {
  USER: `${CACHE_PREFIX}${CACHE_VERSION}user`,
  LINKS: `${CACHE_PREFIX}${CACHE_VERSION}links`,
  SETTINGS: `${CACHE_PREFIX}${CACHE_VERSION}settings`,
  SEARCH_ENGINE: `${CACHE_PREFIX}${CACHE_VERSION}search_engine`,
  STAGING_LOGGED_IN: `${CACHE_PREFIX}${CACHE_VERSION}staging_logged_in`,
  SEARCH_HISTORY: `${CACHE_PREFIX}${CACHE_VERSION}search_history`,
} as const;

export const cache = {
  set: <T>(key: string, data: T): void => {
    try {
      localStorage.setItem(
        key,
        JSON.stringify({
          timestamp: Date.now(),
          data,
        }),
      );
    } catch (error) {
      console.error("Cache write failed:", error);
    }
  },

  get: <T>(key: string): T | null => {
    try {
      const item = localStorage.getItem(key);
      if (!item) return null;

      const parsed = JSON.parse(item);
      return parsed.data as T;
    } catch (error) {
      console.error("Cache read failed:", error);
      return null;
    }
  },

  // search history uses it's own timestamp
  get_search_history: <T>(key: string): T | null => {
    try {
      const item = localStorage.getItem(key);
      if (!item) return null;
      return item;
    } catch (error) {
      console.error("Cache read failed:", error);
      return null;
    }
  },

  // search history is stored unqiuely
  set_search_history: <T>(key: string, data: T): void => {
    try {
      localStorage.setItem(key, data);
    } catch (error) {
      console.error("Cache write failed:", error);
    }
  },

  clear: (key: string): void => {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      console.error("Cache clear failed:", error);
    }
  },

  clearAll: (): void => {
    try {
      // Clear all cache keys used by the application
      // Object.values(CacheKeys).forEach((key) => {
      //   localStorage.removeItem(key);
      // });
      for (const key of Object.values(CacheKeys)) {
        localStorage.removeItem(key);
      }
    } catch (error) {
      console.error("Cache clear all failed:", error);
    }
  },
};

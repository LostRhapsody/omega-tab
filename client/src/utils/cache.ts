const CACHE_PREFIX = "betternewtab_";
const CACHE_VERSION = "v1_";

export const CacheKeys = {
  USER: `${CACHE_PREFIX}${CACHE_VERSION}user`,
  LINKS: `${CACHE_PREFIX}${CACHE_VERSION}links`,
  SETTINGS: `${CACHE_PREFIX}${CACHE_VERSION}settings`,
  SEARCH_ENGINE: `${CACHE_PREFIX}${CACHE_VERSION}search_engine`,
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

  clear: (key: string): void => {
    try {
      localStorage.removeItem(key);
    } catch (error) {
      console.error("Cache clear failed:", error);
    }
  },
};

import { onMounted, onUnmounted, ref, watch } from "vue";

export type Theme = "light" | "dark" | "system";
export type ResolvedTheme = "light" | "dark";

const STORAGE_KEY = "tp-theme-preference";

// Shared state across all instances
const theme = ref<Theme>("system");
const resolvedTheme = ref<ResolvedTheme>("light");
const isInitialized = ref(false);

/**
 * Composable for managing theme state
 * Respects system preference by default, allows manual override
 */
export function useTheme() {
  let mediaQuery: MediaQueryList | null = null;

  const getSystemTheme = (): ResolvedTheme => {
    if (typeof window === "undefined") return "light";
    return window.matchMedia("(prefers-color-scheme: dark)").matches
      ? "dark"
      : "light";
  };

  const updateResolvedTheme = () => {
    resolvedTheme.value =
      theme.value === "system" ? getSystemTheme() : theme.value;
  };

  const applyTheme = () => {
    const html = document.documentElement;

    // Remove existing theme classes
    html.classList.remove("tp-theme-light", "tp-theme-dark");

    // Add new theme class
    html.classList.add(`tp-theme-${resolvedTheme.value}`);

    // Update meta theme-color for mobile browsers
    const metaThemeColor = document.querySelector('meta[name="theme-color"]');
    if (metaThemeColor) {
      metaThemeColor.setAttribute(
        "content",
        resolvedTheme.value === "dark" ? "#111111" : "#f8f7f4",
      );
    }
  };

  const setTheme = (newTheme: Theme) => {
    theme.value = newTheme;
    localStorage.setItem(STORAGE_KEY, newTheme);
    updateResolvedTheme();
    applyTheme();
  };

  const toggleTheme = () => {
    // Cycle: system -> light -> dark -> system
    if (theme.value === "system") {
      setTheme("light");
    } else if (theme.value === "light") {
      setTheme("dark");
    } else {
      setTheme("system");
    }
  };

  const handleSystemThemeChange = () => {
    if (theme.value === "system") {
      updateResolvedTheme();
      applyTheme();
    }
  };

  const initialize = () => {
    if (isInitialized.value) return;

    // Load saved preference
    const saved = localStorage.getItem(STORAGE_KEY) as Theme | null;
    if (saved && ["light", "dark", "system"].includes(saved)) {
      theme.value = saved;
    }

    updateResolvedTheme();
    applyTheme();

    // Listen for system theme changes
    mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    mediaQuery.addEventListener("change", handleSystemThemeChange);

    isInitialized.value = true;
  };

  onMounted(() => {
    initialize();
  });

  onUnmounted(() => {
    // Note: We don't remove the listener since state is shared
    // and other components may still be using it
  });

  // Watch for external changes to theme
  watch(theme, () => {
    updateResolvedTheme();
    applyTheme();
  });

  return {
    /** Current theme setting: 'light', 'dark', or 'system' */
    theme,
    /** Resolved theme after applying system preference: 'light' or 'dark' */
    resolvedTheme,
    /** Set theme explicitly */
    setTheme,
    /** Cycle through themes: system -> light -> dark -> system */
    toggleTheme,
    /** Whether the current resolved theme is dark */
    isDark: () => resolvedTheme.value === "dark",
    /** Initialize theme (called automatically on mount) */
    initialize,
  };
}

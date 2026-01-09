import { computed, onMounted, onUnmounted, ref } from "vue";

// Breakpoint values matching common conventions
const BREAKPOINTS = {
  xs: 0,
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  "2xl": 1536,
} as const;

export type Breakpoint = keyof typeof BREAKPOINTS;

// Shared state across all instances
const width = ref(typeof window !== "undefined" ? window.innerWidth : 1024);
const height = ref(typeof window !== "undefined" ? window.innerHeight : 768);
let resizeHandler: (() => void) | null = null;
let listenerCount = 0;

/**
 * Composable for responsive breakpoint detection
 * Replaces Vuetify's useDisplay()
 */
export function useBreakpoint() {
  const handleResize = () => {
    width.value = window.innerWidth;
    height.value = window.innerHeight;
  };

  onMounted(() => {
    if (listenerCount === 0) {
      resizeHandler = handleResize;
      window.addEventListener("resize", resizeHandler, { passive: true });
    }
    listenerCount++;
    // Initial measurement
    handleResize();
  });

  onUnmounted(() => {
    listenerCount--;
    if (listenerCount === 0 && resizeHandler) {
      window.removeEventListener("resize", resizeHandler);
      resizeHandler = null;
    }
  });

  // Current breakpoint name
  const current = computed<Breakpoint>(() => {
    if (width.value >= BREAKPOINTS["2xl"]) return "2xl";
    if (width.value >= BREAKPOINTS.xl) return "xl";
    if (width.value >= BREAKPOINTS.lg) return "lg";
    if (width.value >= BREAKPOINTS.md) return "md";
    if (width.value >= BREAKPOINTS.sm) return "sm";
    return "xs";
  });

  // Boolean breakpoint checks (matching Vuetify's useDisplay API)
  const xs = computed(() => current.value === "xs");
  const sm = computed(() => current.value === "sm");
  const md = computed(() => current.value === "md");
  const lg = computed(() => current.value === "lg");
  const xl = computed(() => current.value === "xl");
  const xxl = computed(() => current.value === "2xl");

  // "And down" breakpoints (width <= breakpoint max)
  const xsOnly = computed(() => width.value < BREAKPOINTS.sm);
  const smAndDown = computed(() => width.value < BREAKPOINTS.md);
  const mdAndDown = computed(() => width.value < BREAKPOINTS.lg);
  const lgAndDown = computed(() => width.value < BREAKPOINTS.xl);
  const xlAndDown = computed(() => width.value < BREAKPOINTS["2xl"]);

  // "And up" breakpoints (width >= breakpoint min)
  const smAndUp = computed(() => width.value >= BREAKPOINTS.sm);
  const mdAndUp = computed(() => width.value >= BREAKPOINTS.md);
  const lgAndUp = computed(() => width.value >= BREAKPOINTS.lg);
  const xlAndUp = computed(() => width.value >= BREAKPOINTS.xl);
  const xxlAndUp = computed(() => width.value >= BREAKPOINTS["2xl"]);

  // Mobile detection (commonly used)
  const mobile = computed(() => width.value < BREAKPOINTS.md);
  const tablet = computed(
    () => width.value >= BREAKPOINTS.md && width.value < BREAKPOINTS.lg,
  );
  const desktop = computed(() => width.value >= BREAKPOINTS.lg);

  return {
    // Raw dimensions
    width,
    height,

    // Current breakpoint
    current,

    // Exact breakpoint matches
    xs,
    sm,
    md,
    lg,
    xl,
    xxl,

    // "And down" (less than next breakpoint)
    xsOnly,
    smAndDown,
    mdAndDown,
    lgAndDown,
    xlAndDown,

    // "And up" (greater than or equal to breakpoint)
    smAndUp,
    mdAndUp,
    lgAndUp,
    xlAndUp,
    xxlAndUp,

    // Device type helpers
    mobile,
    tablet,
    desktop,

    // Breakpoint values for reference
    breakpoints: BREAKPOINTS,
  };
}

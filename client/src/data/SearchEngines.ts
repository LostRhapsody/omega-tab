import type { SearchEngine } from "../types/SearchEngine";

export const searchEngines: SearchEngine[] = [
  {
    icon: "icons/google.svg",
    name: "Google",
    url: "https://www.google.com/search?q=",
  },
  {
    icon: "icons/bing.svg",
    name: "Bing",
    url: "https://www.bing.com/search?q=",
  },
  {
    icon: "icons/perplexity.svg",
    name: "Perplexity",
    url: "https://www.perplexity.ai/search?q=",
  },
  {
    icon: "icons/brave.svg",
    name: "Brave",
    url: "https://search.brave.com/search?q=",
  },
];

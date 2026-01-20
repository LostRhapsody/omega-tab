<template>
  <nav class="header">
    <div class="tp-container">
      <div class="header__inner">
        <!-- Logo -->
        <a href="/" class="header__logo tp-mono">OmegaTab_</a>
        <!-- Desktop CTA -->
        <div class="header__cta">
          <a
            href="https://github.com/LostRhapsody/omega-tab"
            target="_blank"
            class="tp-btn tp-btn-ghost"
          >
            <svg class="icon" viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
              />
            </svg>
            GitHub
          </a>
          <a :href="docsUrl + '/getting-started'" class="tp-btn tp-btn-primary">Get Started</a>
        </div>

        <!-- Mobile Menu Toggle -->
        <button class="header__toggle" @click="isOpen = !isOpen" aria-label="Toggle menu">
          <svg
            v-if="!isOpen"
            class="icon"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M3 12h18M3 6h18M3 18h18" />
          </svg>
          <svg
            v-else
            class="icon"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M18 6L6 18M6 6l12 12" />
          </svg>
        </button>
      </div>

      <!-- Mobile Navigation -->
      <div v-if="isOpen" class="header__mobile">
        <a :href="docsUrl + '/guides'" @click="isOpen = false">Guides</a>
        <hr />
        <a
          href="https://github.com/LostRhapsody/omega-tab"
          target="_blank"
          class="tp-btn tp-btn-secondary"
          @click="isOpen = false"
        >
          View on GitHub
        </a>
        <a
          :href="docsUrl + '/getting-started'"
          class="tp-btn tp-btn-primary"
          @click="isOpen = false"
        >
          Get Started
        </a>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref } from "vue";

const isOpen = ref(false);
const docsUrl = import.meta.env.VITE_DOCS_URL || "/docs";
</script>

<style scoped>
.header {
  position: sticky;
  top: 0;
  z-index: var(--tp-z-sticky);
  background: var(--tp-bg-primary);
  border-bottom: var(--tp-border-width) solid var(--tp-border);
  padding: var(--tp-space-4) 0;
}

.header__inner {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--tp-space-6);
}

.header__logo {
  font-size: var(--tp-text-lg);
  font-weight: var(--tp-font-bold);
  text-decoration: none;
}

.header__nav {
  display: none;
  align-items: center;
  gap: var(--tp-space-6);
}

.header__nav a {
  color: var(--tp-text-secondary);
  text-decoration: none;
  font-size: var(--tp-text-sm);
  transition: color var(--tp-transition-fast);
}

.header__nav a:hover {
  color: var(--tp-text-primary);
}

.header__cta {
  display: none;
  align-items: center;
  gap: var(--tp-space-3);
}

.header__toggle {
  display: flex;
  padding: var(--tp-space-2);
}

.icon {
  width: 20px;
  height: 20px;
}

.header__mobile {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
  padding-top: var(--tp-space-6);
  border-top: var(--tp-border-width) solid var(--tp-border);
  margin-top: var(--tp-space-4);
}

.header__mobile a:not(.tp-btn) {
  color: var(--tp-text-secondary);
  text-decoration: none;
  padding: var(--tp-space-2) 0;
}

.header__mobile a:not(.tp-btn):hover {
  color: var(--tp-text-primary);
}

.header__mobile hr {
  margin: var(--tp-space-2) 0;
}

@media (min-width: 768px) {
  .header__nav {
    display: flex;
  }

  .header__cta {
    display: flex;
  }

  .header__toggle {
    display: none;
  }

  .header__mobile {
    display: none;
  }
}
</style>

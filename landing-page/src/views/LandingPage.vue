<template>
  <div class="min-h-screen">
    <AppHeader />

    <!-- Hero Section -->
    <section class="tp-section">
      <div class="tp-container text-center">
        <!-- Badge -->
        <div class="flex justify-center mb-8">
          <span
            class="inline-flex items-center px-4 py-2 text-sm tp-secondary border border-tp-border rounded"
          >
            Free &amp; Open Source
          </span>
        </div>

        <!-- Hero Content -->
        <div class="max-w-3xl mx-auto">
          <h1 class="text-4xl md:text-5xl font-bold mb-6 leading-tight">
            The <span class="tp-accent">ultimate &#937; new tab</span> for your browser
          </h1>
          <p class="text-lg tp-secondary mb-12 max-w-2xl mx-auto">
            Create a personalized launch pad for all your tools and resources. Access everything you
            need in one place, with powerful search and integrations.
          </p>

          <!-- Homepage Image -->
          <div class="mb-8">
            <img
              src="/copy/homepage.png"
              alt="OmegaTab Homepage"
              class="w-full rounded shadow-lg border border-tp-border"
            />
          </div>

          <!-- CTA -->
          <div class="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a :href="docsUrl + '/getting-started'" class="tp-btn tp-btn-primary">
              Get Started
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-4 w-4"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              >
                <path d="M5 12h14M12 5l7 7-7 7" />
              </svg>
            </a>
            <a
              href="https://github.com/LostRhapsody/omega-tab"
              target="_blank"
              class="tp-btn tp-btn-secondary"
            >
              View on GitHub
            </a>
          </div>
        </div>
      </div>
    </section>

    <!-- Build your own new tab -->
    <section class="tp-section">
      <div class="tp-container max-w-3xl text-center">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">Build your own new tab landing page</h2>
        <p class="text-lg tp-secondary mb-8">
          Customize your new tab page with your favorite tools and resources.
        </p>
        <div>
          <img
            src="/copy/add_link.png"
            alt="Add Link Feature"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>
      </div>
    </section>

    <!-- Keyboard shortcuts -->
    <section class="tp-section">
      <div class="tp-container max-w-3xl text-center">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">
          Keyboard oriented control with intuitive shortcuts
        </h2>
        <p class="text-lg tp-secondary mb-8">
          Navigate and control your new tab page with ease using keyboard shortcuts.
        </p>
        <div>
          <img
            src="/copy/shortcuts.png"
            alt="Keyboard Shortcuts"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>
      </div>
    </section>

    <!-- Search bar -->
    <section class="tp-section">
      <div class="tp-container max-w-3xl text-center">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">A super-powered search bar</h2>
        <p class="text-lg tp-secondary mb-8">
          Search across all your tools and resources from one place.
        </p>

        <p class="tp-muted mb-4">Quickly find your pinned links</p>
        <div class="mb-8">
          <img
            src="/copy/my_link.png"
            alt="Find Links"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>

        <p class="tp-muted mb-4">
          Get search suggestions based on local history and your search terms
        </p>
        <div>
          <img
            src="/copy/suggestions.png"
            alt="Search Suggestions"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>
      </div>
    </section>

    <!-- Command Palette -->
    <section class="tp-section">
      <div class="tp-container max-w-3xl text-center">
        <h2 class="text-2xl md:text-3xl font-bold mb-4">The power user's favorite</h2>
        <p class="text-lg tp-secondary mb-8">
          Navigate quickly to your links, settings, or actions with the command palette.
        </p>

        <p class="tp-muted mb-4">Open with <kbd>Ctrl+K</kbd></p>
        <div class="mb-8">
          <img
            src="/copy/command_palette.png"
            alt="Command Palette"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>

        <p class="tp-muted mb-4">Type to perform actions, open links, or navigate to settings</p>
        <div>
          <img
            src="/copy/command_new_link.png"
            alt="Command Palette Actions"
            class="w-full rounded shadow-lg border border-tp-border"
          />
        </div>
      </div>
    </section>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppFooter from "@/components/AppFooter.vue";
import AppHeader from "@/components/AppHeader.vue";

const docsUrl = "https://omega-tab.evanrobertson.dev/docs";
</script>

<style scoped>
.border-tp-border {
  border-color: var(--tp-border);
}
</style>

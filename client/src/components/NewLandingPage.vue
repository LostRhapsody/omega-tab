<!-- LandingPage.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    
    <AppHeader />

    <!-- Hero Section -->
    <div class="container mx-auto px-4 py-24">
      <!-- Update Badge -->
      <div class="flex justify-center mb-8">
        <a href="#"
          class="inline-flex items-center px-4 py-2 rounded-full bg-gray-800 text-gray-300 hover:text-white transition-colors">
          <span>v0.1.0 Soon</span>
          <v-icon icon="mdi-chevron-right" size="small" />
        </a>
      </div>

      <!-- Hero Content -->
      <div class="text-center max-w-4xl mx-auto">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight">
          A
          <span class="text-blue-400">better new tab</span>
          for your browser
        </h1>
        <p class="text-xl text-gray-300 mb-12">
          Create a personalized launch pad for all your tools and resources.
          Access everything you need in one place, with powerful search and integrations.
        </p>

        <!-- Homepage Image -->
        <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/homepage.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
        </div>

        <!-- Signup CTA -->
        <div class="max-w-lg mx-auto">
          <div class="flex justify-center">
            <button @click="handleShowSignUp"
              class="px-8 py-4 rounded-full bg-white/5 backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2 shadow-inner shadow-white/20">
              <span class="text-lg">Get Started</span>
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14M12 5l7 7-7 7" />
              </svg>
            </button>
          </div>

          <!-- Trial Info -->
          <div class="flex justify-center items-center space-x-6 mt-6 text-gray-400">
            <div class="flex items-center">
              <span>Free to try</span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <!-- Build your own new tab landing page -->
    <div class="container max-w-4xl mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Build your own new tab landing page</h2>
      <p class="text-xl text-gray-300 mb-12">
        Customize your new tab page with your favorite tools and resources.
      </p>
      <!-- Add Link Image -->
      <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/add_link.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
    </div>

    <!-- Keyboard oriented control with intuitive shortcuts -->
    <div class="container max-w-4xl mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Keyboard oriented control with intuitive shortcuts</h2>
      <p class="text-xl text-gray-300 mb-12">
        Navigate and control your new tab page with ease using keyboard shortcuts.
      </p>
      <!-- Shortcuts Image -->
      <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/shortcuts.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
    </div>

    <!-- A super-powered search bar -->
    <div class="container max-w-4xl mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">A super-powered search bar</h2>
      <p class="text-xl text-gray-300 mb-12">
        Search across all your tools and resources from one place.
      </p>
      <em class="text-lg text-gray-300">Quickly find your pinned links</em>
      <!-- My link Image -->
      <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/my_link.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
      <!-- Suggestions Image -->
      <em class="text-lg text-gray-300">Get search suggestions based on a local search history and your search terms</em>
      <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/suggestions.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
    </div>

    
    <!-- Command Palette -->
    <div class="container max-w-4xl mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">The power users favorite</h2>
      <p class="text-xl text-gray-300 mb-12">
        Navigate quickly to your links, settings, or actions with the command palette.
      </p>
      <em class="text-lg text-gray-300">Open with easily <kbd class="kbd">Ctrl+K</kbd></em>
      <!-- My link Image -->
      <div class="flex justify-center mt-2 mb-8 z-10 relative">
          <img src="/copy/command_palette.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
      <!-- Suggestions Image -->
      <em class="text-lg text-gray-300">Type to perform actions, open links, or navigate to settings.</em>
      <div class="flex justify-center mb-8 z-10 relative">
          <img src="/copy/command_new_link.png" alt="Hero Image" class="w-full rounded-lg shadow-lg" />
      </div>
    </div>

    <!-- Integrations for the best tools -->
    <div class="container max-w-4xl mx-auto px-4 py-12 text-center">
      <a href="#"
          class="inline-flex items-center px-4 py-2 mb-2 rounded-full bg-gray-800 text-gray-300 hover:text-white transition-colors">
          <span>Coming Soon!</span>          
        </a>
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Integrations for the best tools</h2>
      <p class="text-xl text-gray-300 mb-12">
        Connect with your favorite tools and services for a seamless experience with support for Jira, Confluence, Linear, and more.
      </p>
    </div>

    <!-- Pricing -->
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 id="pricing" class="text-3xl md:text-4xl font-bold text-white mb-6">Pricing</h2>
      <p class="text-xl text-gray-300 mb-12">
        Choose a plan that fits your needs. Free and premium options available.
      </p>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <!-- Free Plan -->
        <v-card class="!bg-white/5 !shadow-lg !rounded-lg !p-6 !shadow-white/10 plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Free</v-card-title>
            <div class="text-h3 mb-1">$0</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list class="!bg-transparent">
              <v-list-item class="text-start" v-for="feature in FreePlanFeatures" :key="feature">
                <v-list-item-title><v-icon icon="mdi-check" class="mr-2 text-blue-400"></v-icon>{{ feature
                  }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <button @click="handleShowSignUp"
              class="px-8 py-4 rounded-full bg-blue-500/70 text-white hover:bg-blue-600 transition-colors w-full">
              Get Started
            </button>
          </v-card-actions>
        </v-card>

        <!-- Plus Plan -->
        <v-card class="!bg-white/5 !shadow-lg !rounded-lg !p-6 !shadow-white/10 plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Plus</v-card-title>
            <div class="text-h3 mb-1">$5</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list class="!bg-transparent">
              <v-list-item class="text-start" v-for="feature in PlusPlanFeatures" :key="feature">
                <v-list-item-title><v-icon icon="mdi-check" class="mr-2 text-blue-400"></v-icon>{{ feature
                  }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <a class="w-full" :href=PlusPlanUrl>
              <button
                class="px-8 py-4 rounded-full bg-blue-500/70 text-white hover:bg-blue-600 transition-colors w-full">
                Get BetterNewTab+!
              </button>
            </a>
          </v-card-actions>
        </v-card>

        <!-- Pro Plan -->
        <v-card class="!bg-white/5 !shadow-lg !rounded-lg !p-6 !shadow-white/10 plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Pro</v-card-title>
            <div class="text-h3 mb-1">$10</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list class="!bg-transparent">
              <v-list-item class="text-start" v-for="feature in ProPlanFeatures" :key="feature">
                <v-list-item-title><v-icon icon="mdi-check" class="mr-2 text-blue-400"></v-icon>{{ feature
                  }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <!-- :href=ProPlanUrl -->
            <button disabled
              class="px-8 py-4 rounded-full bg-blue-500/70 text-white transition-colors w-full">
              <!-- Get BetterNewTab Pro! -->
                Coming Soon!
            </button>
          </v-card-actions>
        </v-card>
      </div>
    </div>

    <AppFooter />

    <v-dialog v-model="showSignIn" max-width="600px">
      <div class="m-auto">
        <div id="sign-in"></div>
      </div>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from "vue";
import { Clerk } from "@clerk/clerk-js";
import { FreePlanFeatures, PlusPlanFeatures, ProPlanFeatures } from "../data/plans";
import { cache } from "@/utils/cache";
import AppHeader from '@/components/AppHeader.vue';
import AppFooter from '@/components/AppFooter.vue';

const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);
const showSignIn = ref(false);
const PlusPlanUrl = import.meta.env.VITE_PLUS_PLAN_URL;
const ProPlanUrl = import.meta.env.VITE_PRO_PLAN_URL;

const handleShowSignIn = () => {
  // Clear all cache before signing in to ensure a fresh state
  cache.clearAll();
  
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignIn(signInDiv as HTMLDivElement);
    }
  });
};

const handleShowSignUp = () => {
  // Clear all cache before signing up to ensure a fresh state
  cache.clearAll();
  
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignUp(signInDiv as HTMLDivElement);
    }
  });
};

onMounted(async () => {
  await clerk.load();
});
</script>

<style scoped>
.v-text-field :deep(.v-field__input) {
  color: white !important;
}

.v-text-field :deep(.v-field__outline) {
  --v-field-border-opacity: 0.2;
}

.animated-gradient {
  animation: animateBg 12s ease infinite;
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.6), rgb(141, 145, 255));
  background-size: 400% 400%;
  opacity: 0.5;
}

@keyframes animateBg {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

.plan-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s;
  justify-content: space-between;
}

.plan-card:hover {
  transform: translateY(-5px);
}

div {
  white-space: normal !important;
  overflow: visible !important;
  text-overflow: clip !important;
}
</style>
<!-- LandingPage.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    <!-- Navigation -->
    <nav
      class="px-4 py-4 min-h-24 flex align-center border-b border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
      <div class="container mx-auto">
        <div class="flex justify-between items-center">
          <!-- Logo -->
          <div class="flex items-center space-x-2">
            <!-- <img src="/api/placeholder/32/32" alt="Logo" class="w-8 h-8" /> -->
            <span class="text-white text-xl font-semibold">BetterNewTab_</span>
          </div>

          <!-- Navigation Links -->
          <div class="hidden md:flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
            <a href="#" class="text-gray-300 hover:text-white transition-colors">About</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="#" class="text-gray-300 hover:text-white transition-colors">Integrations</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="#" class="text-gray-300 hover:text-white transition-colors">Pricing</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="#" class="text-gray-300 hover:text-white transition-colors">Features</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="#" class="text-gray-300 hover:text-white transition-colors">Contact</a>
          </div>

          <!-- Get Started Button -->
          <div class="flex items-center space-x-4">
            <button @click="handleShowSignIn"
              class="px-8 py-4 rounded-full backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2  shadow-inner shadow-white/20">
              <span class="text-lg">Get Started</span>
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14M12 5l7 7-7 7" />
              </svg>
            </button>
          </div>
        </div>
      </div>
    </nav>

    <!-- Hero Section -->
    <div class="container mx-auto px-4 py-24">
      <!-- Update Badge -->
      <div class="flex justify-center mb-8">
        <a href="#"
          class="inline-flex items-center px-4 py-2 rounded-full bg-gray-800 text-gray-300 hover:text-white transition-colors">
          <span>Launching Soon</span>
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

        <!-- Email Signup -->
        <div class="max-w-lg mx-auto">
          <div class="flex justify-center">
            <button @click="handleShowSignIn"
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
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Build your own new tab landing page</h2>
      <p class="text-xl text-gray-300 mb-12">
        Customize your new tab page with widgets, shortcuts, and more. Make it truly yours.
      </p>
    </div>

    <!-- Keyboard oriented control with intuitive shortcuts -->
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Keyboard oriented control with intuitive shortcuts</h2>
      <p class="text-xl text-gray-300 mb-12">
        Navigate and control your new tab page with ease using keyboard shortcuts.
      </p>
    </div>

    <!-- A super-powered search bar -->
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">A super-powered search bar</h2>
      <p class="text-xl text-gray-300 mb-12">
        Search across all your tools and resources from one place.
      </p>
    </div>

    <!-- Integrations for the best tools -->
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Integrations for the best tools</h2>
      <p class="text-xl text-gray-300 mb-12">
        Connect with your favorite tools and services for a seamless experience.
      </p>
    </div>

    <!-- Pricing -->
    <div class="container mx-auto px-4 py-24 text-center">
      <h2 class="text-3xl md:text-4xl font-bold text-white mb-6">Pricing</h2>
      <p class="text-xl text-gray-300 mb-12">
        Choose a plan that fits your needs. Free and premium options available.
      </p>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-2 gap-8">
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
            <button @click="handleShowSignIn"
              class="px-8 py-4 rounded-full bg-blue-500/70 text-white hover:bg-blue-600 transition-colors w-full">
              Get Started
            </button>
          </v-card-actions>
        </v-card>

        <!-- Plus Plan -->
        <v-card class="!bg-white/5 !shadow-lg !rounded-lg !p-6 !shadow-white/10 plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Plus</v-card-title>
            <div class="text-h3 mb-1">$8</div>
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
      </div>
    </div>
    <!-- Footer -->
    <div class="pt-24 text-center bottom-0 w-full">
      <footer
        class="px-4 py-4 min-h-24 flex align-center border-t border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
        <div class="container mx-auto">
          <div class="flex justify-between items-center">
            <!-- Company Name -->
            <div class="flex items-center space-x-2">
              <span class="text-white text-xl font-semibold">BetterNewTab_</span>
            </div>

            <!-- Footer Links -->
            <div class="hidden md:flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
              <a href="#" class="text-gray-300 hover:text-white transition-colors">About</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">Privacy Policy</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">Terms of Service</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">Contact</a>
            </div>

            <!-- Social Media Links -->
            <div class="flex items-center space-x-4">
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-facebook" />
              </a>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-twitter" />
              </a>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-linkedin" />
              </a>
            </div>
          </div>
        </div>
      </footer>
    </div>
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
import { FreePlanFeatures, PlusPlanFeatures } from "../data/plans";

const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);
const showSignIn = ref(false);
const PlusPlanUrl = import.meta.env.VITE_PLUS_PLAN_URL;

const handleShowSignIn = () => {
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignIn(signInDiv as HTMLDivElement);
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
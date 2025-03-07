<template>
  <!-- Navigation -->
  <nav
    class="px-4 py-4 min-h-24 flex align-center border-b border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
    <div class="container mx-auto">
      <div class="flex justify-between items-center">
        <!-- Logo -->
        <div class="flex items-center space-x-2">
          <span class="text-white text-xl font-semibold">
            <a href="/">BetterNewTab_</a>
          </span>
        </div>

        <!-- Navigation Links -->
        <div class="hidden md:flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
          <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors">Getting Started</a>
          <span class="text-gray-300 cursor-default">•</span>
          <a href="/docs/guides/confluence-integration" class="text-gray-300 hover:text-white transition-colors">Integrations</a>
          <span class="text-gray-300 cursor-default">•</span>
          <a href="/#pricing" class="text-gray-300 hover:text-white transition-colors">Pricing</a>
          <span class="text-gray-300 cursor-default">•</span>
          <a href="/docs/guides" class="text-gray-300 hover:text-white transition-colors">Guides</a>
          <span class="text-gray-300 cursor-default">•</span>
          <a href="/contact" class="text-gray-300 hover:text-white transition-colors">Contact</a>
        </div>

        <!-- Get Started and Sign In Buttons -->
        <div class="flex items-center space-x-4">
          <button @click="handleShowSignUp"
            class="px-4 py-4 rounded-l-full backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2 shadow-inner shadow-white/20">
            <span class="text-lg">Get Started</span>
            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none"
              stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M5 12h14M12 5l7 7-7 7" />
            </svg>
          </button>
          <button @click="handleShowSignIn"
            class="px-4 py-4 rounded-r-full backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2 shadow-inner shadow-white/20">
            <span class="text-lg">Sign In</span>
          </button>
        </div>
      </div>
    </div>
  </nav>

  <v-dialog v-model="showSignIn" max-width="600px">
    <div class="m-auto">
      <div id="sign-in"></div>
    </div>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from "vue";
import { Clerk } from "@clerk/clerk-js";
import { cache } from "@/utils/cache";

const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);
const showSignIn = ref(false);

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
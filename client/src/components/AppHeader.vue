<template>
    <!-- Navigation -->
    <nav
        class="px-4 py-6 min-h-24 flex items-center border-b border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
        <div class="container mx-auto">
            <div class="grid grid-cols-12 gap-4 items-center w-full">
                <!-- Logo (3 columns) -->
                <div class="col-span-7 md:col-span-3 flex items-center space-x-2">
                    <span class="text-white text-xl font-semibold">
                        <a href="/">BetterNewTab_</a>
                    </span>
                </div>

                <!-- Mobile Menu Toggle -->
                <div class="col-span-5 md:hidden flex justify-end">
                    <v-btn icon @click="drawer = !drawer" color="white">
                        <v-icon>mdi-menu</v-icon>
                    </v-btn>
                </div>

                <!-- Desktop Navigation Links (6 columns) - Hidden on mobile -->
                <div class="hidden md:flex col-span-12 md:col-span-6 justify-around space-x-4 border border-gray-300 rounded-full px-6 py-4 overflow-x-auto">
                    <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors whitespace-nowrap">Getting Started</a>
                    <span class="text-gray-300">•</span>
                    <a href="/docs/guides/confluence-integration" class="text-gray-300 hover:text-white transition-colors whitespace-nowrap">Integrations</a>
                    <span class="text-gray-300">•</span>
                    <a href="/#pricing" class="text-gray-300 hover:text-white transition-colors whitespace-nowrap">Pricing</a>
                    <span class="text-gray-300">•</span>
                    <a href="/docs/guides" class="text-gray-300 hover:text-white transition-colors whitespace-nowrap">Guides</a>
                    <span class="text-gray-300">•</span>
                    <a href="/contact" class="text-gray-300 hover:text-white transition-colors whitespace-nowrap">Contact</a>
                </div>

                <!-- Unified Pill Button (3 columns) - Hidden on mobile -->
                <div class="hidden md:flex col-span-12 md:col-span-3 justify-center md:justify-end">
                    <div class="pill-button border border-gray-300 rounded-full">
                        <button @click="handleShowSignUp" class="pill-half">
                            <span class="text-sm">Get Started</span>
                        </button>
                        <button @click="handleShowSignIn" class="pill-half">
                            <span class="text-sm">Sign In</span>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </nav>

    <!-- Mobile Navigation Drawer -->
    <v-layout>

    <v-navigation-drawer
        v-model="drawer"
        temporary
        location="right"
        class="nav-drawer"
        width="250"
    >
        <div class="px-4 py-6">
            <!-- Close Button -->
            <div class="flex justify-end mb-6">
                <v-btn icon @click="drawer = false" color="white">
                    <v-icon>mdi-close</v-icon>
                </v-btn>
            </div>
            
            <!-- Mobile Navigation Links -->
            <div class="flex flex-col space-y-4 text-center">
                <a href="/docs/getting-started" class="text-xl text-white border-b border-gray-300 py-3">
                    Getting Started
                </a>
                <a href="/docs/guides/confluence-integration" class="text-xl text-white border-b border-gray-300 py-3">
                    Integrations
                </a>
                <a href="/#pricing" class="text-xl text-white border-b border-gray-300 py-3">
                    Pricing
                </a>
                <a href="/docs/guides" class="text-xl text-white border-b border-gray-300 py-3">
                    Guides
                </a>
                <a href="/contact" class="text-xl text-white border-b border-gray-300 py-3">
                    Contact
                </a>
            </div>
            
            <!-- Mobile Authentication Buttons -->
            <div class="mt-6 flex flex-col space-y-4">
                <button block @click="handleShowSignUp" 
                class=" px-8 py-4 rounded-full bg-white/5 backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors space-x-2 shadow-inner shadow-white/20 text-center"
                >Get Started
                </button>
                <button block @click="handleShowSignIn" variant="outlined"
                class=" px-8 py-4 rounded-full bg-white/5 backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors space-x-2 shadow-inner shadow-white/20 text-center"
                >
                    Sign In
                </button>
            </div>
        </div>
    </v-navigation-drawer>
</v-layout>


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
const drawer = ref(false);

const handleShowSignIn = () => {
    cache.clearAll();
    showSignIn.value = true;
    drawer.value = false; // Close drawer when signing in
    nextTick(() => {
        const signInDiv = document.getElementById("sign-in");
        if (signInDiv) {
            clerk.mountSignIn(signInDiv as HTMLDivElement);
        }
    });
};

const handleShowSignUp = () => {
    cache.clearAll();
    showSignIn.value = true;
    drawer.value = false; // Close drawer when signing up
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
/* Pill Button Container */
.pill-button {
    display: flex;
    border-radius: 9999px;
    overflow: hidden;
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    width: 16em;
}

/* Half Pill Buttons */
.pill-half {
    flex: 1;
    padding-left: 24px;
    padding-right: 24px;
    padding-top: 16px;
    padding-bottom: 16px;
    font-weight: 500;
    color: #ffffff;
    background: none;
    border: none;
    cursor: pointer;
    transition: 0.3s ease, transform 0.1s ease;
}

.pill-half:hover {
    background: rgba(255, 255, 255, 0.1);
}

.pill-half:active {
    background: rgba(255, 255, 255, 0.2);
}

.nav-drawer {
    background-color: #b1bdffe1;
}
</style>
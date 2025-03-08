<template>
  <div>
    <div v-if="isLoading" class="h-screen flex items-center justify-center">
      <v-progress-circular indeterminate />
    </div>
    <div v-else-if="isLoggedIn && !isLoading">
      <header class="border-b border-gray-700 bg-white/5">
        <v-container>
          <v-row class="items-center">
            <v-col>
              <h1 class="text-xl">
                <a href="#">BetterNewTab_</a>
              </h1>
            </v-col>
            <v-col class="flex justify-end">
              <div class="flex rounded-full items-center">
                <button id="user-button" aria-label="User account"></button>
                <v-btn icon="mdi-cog" @click="router.push('/settings');" class="!w-[42px] !h-[42px] ms-8"
                  aria-label="Settings" />
              </div>
            </v-col>
          </v-row>
        </v-container>
      </header>
      <main>
        <v-container>
          <section aria-label="Search">
            <SearchBar />
          </section>
          <section aria-label="Link columns">
            <LinkColumns :userId="userId"
              :maxPins="userStore.userPlan?.max_pins || 6" :canAddLinks="canShowAddLink" :isPlanFree="userStore.userPlan?.name === 'free'" />
          </section>
          <v-dialog v-model="showHelpDialog" max-width="900px">
            <v-card>
              <v-card-title class="headline">Keyboard Shortcuts</v-card-title>
              <v-card-text>
                <h4 class="text-xl mb-4">Open Links</h4>
                <div v-if="uniqueColumnTypes.length" class="border p-4 rounded-lg mb-4">
                  <v-row>
                    <v-col>
                      <div v-for="(columnType, colIndex) in uniqueColumnTypes" :key="columnType">
                        <ul>
                          <li v-for="(link, index) in getLinksByColumnType(columnType)" :key="link.order_index">
                            <div v-if="colIndex < 2">
                              <div class="grid grid-cols-3 gap-2">
                                <div class="col-span-2">
                                  {{ link.title }}
                                </div>
                                <div class="col-span-1">
                                  <span v-if="getShortcut(columnType).includes('+')" class="mr-2">
                                    <span class="kbd">{{ getShortcut(columnType).split('+')[0] }}</span>
                                    +
                                    <span class="kbd">{{ getShortcut(columnType).split('+')[1] }}</span>
                                  </span>
                                  <span v-else class="kbd">{{ getShortcut(columnType) }}</span>
                                  +
                                  <span class="kbd">{{ index + 1 }}</span>
                                </div>
                              </div>
                              <v-divider v-if="showShortcutDivider(index,getLinksByColumnType(columnType).length,colIndex)" class="my-4"></v-divider>
                            </div>
                          </li>
                        </ul>
                      </div>
                    </v-col>
                  </v-row>
                </div>
                <div v-else class="border p-4 rounded-lg mb-4">
                  No links added
                </div>
                <h4 class="text-xl mb-4 mt-8">Change Search Engine</h4>
                <div class="border p-4 rounded-lg mb-4">
                  <p class="text-lg mb-4">
                    Use
                    <span class="kbd !text-sm">Ctrl</span> +
                    <span class="kbd !text-sm">
                      <v-icon icon="mdi-arrow-up"></v-icon>
                      up arrow
                    </span>
                    or
                    <span class="kbd !text-sm">Ctrl</span> +
                    <span class="kbd !text-sm">
                      <v-icon icon="mdi-arrow-down"></v-icon>
                      down arrow
                    </span>
                    to cycle through search engines.
                  </p>
                  <v-row>
                    <v-col>
                      <ul>
                        <li v-for="(engine, index) in searchEngines" :key="engine.name">
                          <div class="grid grid-cols-3 gap-2">
                            <div class="col-span-2">
                              {{ engine.name }}
                            </div>
                            <div class="col-span-1">
                              Search Engine {{ index + 1 }}
                            </div>
                          </div>
                          <v-divider v-if="index + 1 !== searchEngines.length" class="my-4"></v-divider>
                        </li>
                      </ul>
                    </v-col>
                  </v-row>
                </div>
                <h4 class="text-xl mb-4 mt-8">Navigate links</h4>
                <div class="border p-4 rounded-lg mb-4">
                  <p class="text-lg mb-4">
                    Use
                    <span class="kbd !text-sm">
                      <v-icon icon="mdi-arrow-up"></v-icon>
                      up arrow
                    </span>
                    or
                    <span class="kbd !text-sm">
                      <v-icon icon="mdi-arrow-down"></v-icon>
                      down arrow
                    </span>
                    to jump between links when you are not focused on the search bar.
                  </p>                  
                </div>
                <h4 class="text-xl mb-4">Other Shortcuts</h4>
                <div class="border p-4 rounded-lg mb-4">
                  <v-row>
                    <v-col>
                      <ul>
                        <li>
                          <div class="grid grid-cols-3 gap-2">
                            <div class="col-span-2">
                              Show Keyboard Shortcuts
                            </div>
                            <div class="col-span-1">
                              <span class="kbd">?</span> (<span class="kbd">shift + /</span>)
                            </div>
                          </div>
                          <v-divider class="my-4"></v-divider>
                        </li>
                        <li>
                          <div class="grid grid-cols-3 gap-2">
                            <div class="col-span-2">
                              Show Command Palette
                            </div>
                            <div class="col-span-1">
                              <span class="kbd">ctrl</span>
                              +
                              <span class="kbd">k</span>
                            </div>
                          </div>
                          <v-divider class="my-4"></v-divider>
                        </li>
                        <li>
                          <div class="grid grid-cols-3 gap-2">
                            <div class="col-span-2">
                              Add A New Link
                            </div>
                            <div class="col-span-1">
                              <span class="kbd">ctrl</span>
                              +
                              <span class="kbd">alt</span>
                              +
                              <span class="kbd">n</span>
                            </div>
                          </div>
                        </li>
                      </ul>
                    </v-col>
                  </v-row>
                </div>
              </v-card-text>
              <v-card-actions>
                <v-btn variant="tonal" @click="showHelpDialog = false">Close</v-btn>
              </v-card-actions>
            </v-card>
          </v-dialog>
        </v-container>
      </main>
      <div class="fixed bottom-4 right-4">
        <v-menu location="top">
          <template v-slot:activator="{ props }">
            <v-btn v-bind="props" class="!w-[42px] !h-[42px] bg-white" icon="mdi-help" variant="tonal"
              aria-label="Help menu" />
          </template>
          <v-list class="w-64" lines="two">
            <v-list-item @click="showFeedbackDialog = false">
              <a href="/docs/getting-started">
                <v-list-item-title>
                  <v-icon icon="mdi-rocket-launch" />
                  Getting Started
                </v-list-item-title>
              </a>
            </v-list-item>
            <v-list-item @click="showHelpDialog = true">
              <v-list-item-title>
                <v-icon icon="mdi-keyboard" />
                Keyboard Shortcuts
              </v-list-item-title>
            </v-list-item>
            <v-list-item @click="router.push('/plans')">
              <v-list-item-title>
                <v-icon icon="mdi-plus" />
                Better New Tab Plus & Pro
              </v-list-item-title>
            </v-list-item>
            <v-list-item @click="showFeedbackDialog = false">
              <a href="/docs/">
                <v-list-item-title>
                  <v-icon icon="mdi-book" />
                  Guides
                </v-list-item-title>
              </a>
            </v-list-item>
            <v-list-item @click="showFeedbackDialog = true">
              <v-list-item-title>
                <v-icon icon="mdi-comment-quote-outline" />
                Send Feedback
              </v-list-item-title>
            </v-list-item>
          </v-list>
        </v-menu>
      </div>
    </div>
    <div v-else>
      <NewLandingPage />
      <v-dialog v-model="showSignIn" max-width="600px">
        <div class="m-auto">
          <div id="sign-in"></div>
        </div>
      </v-dialog>
    </div>

    <Feedback v-model="showFeedbackDialog" @update:modelValue="handleFeedbackDialogClose" :cancelSubscription=false />

    <v-dialog v-model="showFeedbackMessageDialog" max-width="500px">
      <v-card>
        <v-card-title class="headline">{{ feedbackMessageTitle }}</v-card-title>
        <v-card-text>{{ feedbackMessage }}</v-card-text>
        <v-card-actions>
          <v-btn variant="tonal" @click="showFeedbackMessageDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <CommandPalette />
  </div>
</template>
<script setup lang="ts">
import CommandPalette from '../components/CommandPalette.vue';
import { Clerk } from "@clerk/clerk-js";
import { computed, nextTick, onMounted, ref, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import NewLandingPage from "../components/NewLandingPage.vue";
import LinkColumns from "../components/LinkColumns.vue";
import SearchBar from "../components/SearchBar.vue";
import Feedback from "../components/Feedback.vue";
import { useUserStore } from "../stores/user";
import { useLinksStore, SHORTCUT_MAPPINGS } from "../stores/links";
import { useFeedbackStore } from "../stores/feedback";
import { useUserSettingsStore } from "../stores/settings";
import { searchEngines } from "../data/SearchEngines";
import { API } from "../constants/api";
import api from "../services/api";
// Import useHead from Unhead
import { useHead } from '@unhead/vue';

// Set SEO metadata using Unhead
useHead({
  // Title tag - crucial for SEO
  title: 'BetterNewTab - The Ultimate New Tab',
  // Meta tags
  meta: [
    {
      name: 'description',
      content: 'Create the ultimate new tab landing page.'
    },
    {
      name: 'keywords',
      content: 'new tab, browser extension, productivity, keyboard shortcuts, Jira, Linear, command palette, browser landing page'
    },
    // Open Graph tags for social media sharing
    {
      property: 'og:title',
      content: 'BetterNewTab - The Ultimate New Tab'
    },
    {
      property: 'og:description',
      content: 'Create the ultimate new tab landing page.'
    },
    {
      property: 'og:type',
      content: 'website'
    },
    {
      property: 'og:url',
      content: 'https://betternewtab.com'
    },
    // Twitter card tags
    {
      name: 'twitter:card',
      content: 'summary_large_image'
    },
    {
      name: 'twitter:title',
      content: 'BetterNewTab - The Ultimate New Tab'
    },
    {
      name: 'twitter:description',
      content: 'Create the ultimate new tab landing page.'
    }
  ],
  // Schema.org JSON-LD structured data
  // This is properly handled by Unhead with automatic stringification
  script: [
    {
      type: 'application/ld+json',
      children: JSON.stringify({
        "@context": "https://schema.org",
        "@type": "WebApplication",
        "name": "BetterNewTab",
        "description": "Create the ultimate new tab landing page.",
        "applicationCategory": "ProductivityApplication",
        "operatingSystem": "Any",
        "offers": {
          "@type": "Offer",
          "price": "0",
          "priceCurrency": "USD"
        },
        "featureList": "Keyboard shortcuts, Command palette, Tool integrations with Jira and Linear"
      })
    }
  ],
  // Link tags
  link: [
    {
      rel: 'canonical',
      href: 'https://betternewtab.com'
    }
  ]
});

const userStore = useUserStore();
const linksStore = useLinksStore();
const feedbackStore = useFeedbackStore();
const userSettingsStore = useUserSettingsStore();

// Initialize services
const router = useRouter();
const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);

// State management
const isLoggedIn = ref(false);
const isLoading = ref(true);
const showSignIn = ref(false);
const showHelpDialog = ref(false);
const showFeedbackDialog = ref(false);
const showFeedbackMessageDialog = ref(false);
const feedbackMessageTitle = ref("");
const feedbackMessage = ref("");

// Token refresh interval
let tokenRefreshInterval: number | undefined;
let lastActivityTimestamp: number = Date.now();

// User and data state
const userId = ref<string | null>(null);
const currentRole = ref("member");
const uniqueColumnTypes = computed(() => linksStore.uniqueColumnTypes);

const getShortcut = (columnType: string) => {
  const columnIndex = uniqueColumnTypes.value.indexOf(columnType);
  if (columnIndex >= 0 && columnIndex < SHORTCUT_MAPPINGS.length) {
    return SHORTCUT_MAPPINGS[columnIndex].label;
  }
  return '';
};

const getLinksByColumnType = (columnType: string) => {
  return linksStore.links.filter(link => link.column_type === columnType);
};

const canShowAddLink = computed(() => {
  if (userStore.userPlan?.name === "free" || userStore.userPlan?.name === "plus") {
    return true;
  }

  if (
    userStore.userPlan?.name === "team" &&
    (currentRole.value === "admin" || currentRole.value === "owner")
  ) {
    return true;
  }

  if (
    userStore.userPlan?.name === "enterprise" &&
    (currentRole.value === "admin" || currentRole.value === "owner")
  ) {
    return true;
  }

  return false;
});

const handleShowKeyboardShortcuts = (event: KeyboardEvent) => {
  if (event.key === "?") {
    showHelpDialog.value = true;
  }
};

const handleFeedbackDialogClose = async (value: boolean) => {
  // only want to run this on close (meaning value is false)
  if (value) return;

  showFeedbackDialog.value = value;
  if (!userStore.userId) return;
  if (!userStore.email) return;
  if (!feedbackStore.reasons) return;

  try {
    const response = await api.post(API.FEEDBACK, {
      reasons: feedbackStore.reasons,
      feedback_comment: feedbackStore.feedbackComment,
    });

    switch (response.status) {
      case 200:
        feedbackMessageTitle.value = "Thank You!";
        feedbackMessage.value = "Your feedback has been submitted successfully.";
        break;
      case 429:
        feedbackMessageTitle.value = "Too Many Requests";
        feedbackMessage.value = "Feedback already submitted today, please wait 24 hours before submitting again.";
        break;
      default:
        feedbackMessageTitle.value = "Error";
        feedbackMessage.value = "An unknown error occurred.";
        break;
    }
  } catch (error) {
    feedbackMessageTitle.value = "Error";
    feedbackMessage.value = "An unknown error occurred.";
  } finally {
    showFeedbackMessageDialog.value = true;
    feedbackStore.clearFeedback();
  }
};

const refreshToken = async () => {
  try {
    // Check if user is still active
    const inactiveTime = Date.now() - lastActivityTimestamp;
    const inactiveThreshold = 5 * 60 * 1000; // 5 minutes
    const session = await clerk.session;

    if (!session) {
      console.warn("No active session found during token refresh");
      return false;
    }

    // If inactive for too long, force a more thorough session check
    if (inactiveTime > inactiveThreshold) {
      await session.getToken(); // Force a check of the session
    }

    // This will trigger a token refresh if needed
    const token = await session.getToken({ leewayInSeconds: 30 }); // 30 seconds leeway to handle clock skew
    if (token) {
      localStorage.setItem("token", token);
    } else {
      console.warn("Failed to get token during refresh");
      // Try a more direct approach to refresh if the token wasn't returned
      await session.touch();
    }
  } catch (error) {
    console.error("Error refreshing JWT token:", error);
    // If refreshing fails, try a forced reload of the Clerk client
    try {
      await clerk.load();
      const newSession = await clerk.session;
      if (newSession) {
        const token = await newSession.getToken();
        if (token) {
          localStorage.setItem("token", token);
        }
      }
    } catch (reloadError) {
      console.error("Failed to reload clerk after refresh error:", reloadError);
    }
  }
};

const startTokenRefreshInterval = () => {
  // Refresh token every 4 minutes (Clerk tokens typically expire after 5 minutes of inactivity)
  tokenRefreshInterval = window.setInterval(refreshToken, 4 * 60 * 1000);

  // Setup activity tracking to detect user presence
  const trackUserActivity = () => {
    lastActivityTimestamp = Date.now();
  };

  // Track various user activities
  window.addEventListener('mousemove', trackUserActivity);
  window.addEventListener('keydown', trackUserActivity);
  window.addEventListener('click', trackUserActivity);
  window.addEventListener('scroll', trackUserActivity);
  window.addEventListener('focus', () => {
    trackUserActivity();
    // When tab regains focus, immediately refresh token
    refreshToken();
  });
};

const stopTokenRefreshInterval = () => {
  if (tokenRefreshInterval) {
    clearInterval(tokenRefreshInterval);
  }

  // Remove activity tracking
  window.removeEventListener('mousemove', () => { });
  window.removeEventListener('keydown', () => { });
  window.removeEventListener('click', () => { });
  window.removeEventListener('scroll', () => { });
  window.removeEventListener('focus', () => { });
};

const showShortcutDivider = (index:number, linksInColumn:number, currentColumn:number) => {
  // if there is only 1 column, then we don't need a divider for the last index of this column.
  // if there is more than 1 column, then we don't need a divider for the last index of the last column.
  const numberOfColumns = linksStore.uniqueColumnTypes.length;
  if(numberOfColumns === 1){
    return index !== linksInColumn - 1;
  } else {
    const isLastLink = index === linksInColumn - 1;
    const isLastColumn = currentColumn === numberOfColumns - 1;
    return  !(isLastLink && isLastColumn);
  }

}

// Lifecycle hooks
onMounted(async () => {
  isLoading.value = true;

  try {
    await clerk.load();
    isLoggedIn.value = !!clerk.user;

    if (isLoggedIn.value && clerk.user) {

      try {
        // Retrieve JWT token and store it in local storage
        const session = await clerk.session;
        const token = await session?.getToken();
        if (token) {
          localStorage.setItem("token", token);
        }
      } catch (error) {
        console.error("Error fetching JWT token:", error);
      }

      // Start token refresh interval
      startTokenRefreshInterval();

      let gotUser = false;
      try {
        // if user store is already initialized, no need to fetch user data
        if (!userStore.userId) {
          // Fetch user data and wait for it to complete
          gotUser = await userStore.fetchUserData({
            id: clerk.user.id,
            firstName: clerk.user.firstName || "",
            lastName: clerk.user.lastName || "",
            email: clerk.user.emailAddresses[0].emailAddress,
          });
        } else {
          gotUser = true;
        }
      } catch (error) {
        console.error("Error fetching user data:", error);
      }

      if (!gotUser) {
        throw new Error("Error fetching user data");
      }

      // always fetch settings with User
      userSettingsStore.fetchSettings();

      // this is def not gonna happen but for type errors
      if (!userStore.userId) {
        throw new Error("User ID not found");
      }
      linksStore.fetchLinks(userStore.userId);
    }
  } catch (error) {
    console.error("Error during initialization:", error);
    // Handle error appropriately
  } finally {
    isLoading.value = false;
  }

  // Mount Clerk user button if logged in (has nothing to do with user data above)
  if (isLoggedIn.value) {
    nextTick(() => {
      const userButtonDiv = document.getElementById("user-button");
      if (userButtonDiv) {
        clerk.mountUserButton(userButtonDiv as HTMLDivElement, {
          appearance: {
            elements: {
              rootBox: "scale-150 items-center",
            },
          },
        });
      }
    });
  }

  // mount event listenrs
  window.addEventListener('keydown', handleShowKeyboardShortcuts);
});

onUnmounted(() => {
  stopTokenRefreshInterval();
  window.removeEventListener('keydown', handleShowKeyboardShortcuts);
});
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
}

.logo {
  display: block;
  margin: 0 auto 2rem;
}

img {
  width: 100%;
  height: auto;
  border: 1px solid transparent;
  border-radius: 12px;
}

.WeatherAndTime {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
}

ul {
  list-style-type: none;
  padding: 0;
}

li {
  margin-bottom: 0.5rem;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a.router-link-exact-active {
  color: var(--color-text);
}

nav a.router-link-exact-active:hover {
  background-color: transparent;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}

nav a:first-of-type {
  border: 0;
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  nav {
    text-align: left;
    margin-left: -1rem;
    font-size: 1rem;
    padding: 1rem 0;
    margin-top: 1rem;
  }
}
</style>
<script setup lang="ts">
import { ref, nextTick, onMounted } from 'vue';
import SearchBar from './components/SearchBar.vue';
import LinkColumns from './components/LinkColumns.vue';
import LandingPage from './components/LandingPage.vue';
import { useApi } from './composables/useApi';
import { Clerk } from "@clerk/clerk-js";
import { linkUtils, subscriptionUtils } from './composables/useDatabase';
import type { Tables, Json, Plan } from './types/Database';
type Link = Tables<'links'>;

const { api } = useApi()
const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);

const isLoggedIn = ref(false);
const showSignIn = ref(false);
const isLoading = ref(true);

const userId = ref<string | null>(null);
  const userPlan = ref<Tables<'plans'> | null>(null);

const tools = ref<Link[]>([]);
const docs = ref<Link[]>([]);
const handleToolAdded = (tool: Link) => {
  tools.value.push(tool);
};

const handleDocAdded = (doc: Link) => {
  docs.value.push(doc);
};

const showHelpDialog = ref(false);

const toolShortcuts = tools.value.map((tool, index) => ({
  shortcut: `Ctrl+${index + 1}`,
  description: `Open ${tool.title}`
}));

const docShortcuts = docs.value.map((doc, index) => ({
  shortcut: `Alt+${index + 1}`,
  description: `Open ${doc.title}`
}));

function handleShowSignIn() {
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById('sign-in');
    if (signInDiv) {
      clerk.mountSignIn(signInDiv as HTMLDivElement);
    }
  });
}

onMounted(async () => {
  isLoading.value = true;
  await clerk.load();
  isLoggedIn.value = !!clerk.user;

  if (isLoggedIn.value) {
    // mostly for type checks
    if(!clerk.user) return;
    userId.value = clerk.user.id;
    const links = await linkUtils.getUserLinks(clerk.user.id);
    for(const link of links) {
      if (link.column_type === "tools") handleToolAdded(link); else handleDocAdded(link);
    }

    userPlan.value = await subscriptionUtils.getUserPlan(userId.value);
    if(!userPlan.value) {
      console.error('User plan not found');
    } else if(!userPlan.value.max_pins) {
      userPlan.value.max_pins = 6;
    }

  } // end if is logged in

  isLoading.value = false;
  if(isLoggedIn.value) {
    nextTick(() => {
      // mount the 'user edit' button
      const userButtonDiv = document.getElementById('user-button');
      console.log(userButtonDiv);
      if (userButtonDiv) {
        clerk.mountUserButton(userButtonDiv as HTMLDivElement);
      }
    });
  }

});
</script>

<template>
  <v-theme-provider theme="dark">
    <div v-if="isLoading" class="h-screen flex items-center justify-center">
      <v-progress-circular indeterminate />
    </div>
    <div v-else>
      <div v-if="isLoggedIn" class="mt-16">
        <v-container class="bg-primary">
          <v-row class="items-center">
            <v-col>
              Plan:{{ userPlan?.name }} <br/> Max pins:{{ userPlan?.max_pins }} <br /> Features: {{ userPlan?.features }}
            </v-col>
            <v-col class="text-end">
              <v-btn id="user-button">User</v-btn>
            </v-col>
          </v-row>
        </v-container>
        <div class="header">
          <h1 class="mt-16 text-3xl">
            <v-icon icon="mdi-rocket" size="24" />
            Better New Tab
          </h1>
          <v-btn icon="mdi-help" @click="showHelpDialog = true"></v-btn>
        </div>
        <SearchBar :tools="tools" :docs="docs" />
        <LinkColumns v-if="userPlan" :tools="tools" :docs="docs" :userId="userId" :maxPins="userPlan.max_pins" :can-add-links="true" @tool-added="handleToolAdded"
        @doc-added="handleDocAdded" />
        <v-dialog v-model="showHelpDialog" max-width="900px">
          <v-card>
            <v-card-title class="headline">Help</v-card-title>
            <v-card-text>
              <h3 class="text-xl">Search Bar Controls</h3>
              <p>While in the search bar, type in a Jira Ticket number for relevant links, then use arrow keys or your mouse to navigate</p>
              <br />
              <h3 class="text-xl">Keyboard Shortcuts</h3>
              <br />
              <h4 class="text-lg"><v-icon icon="mdi-chevron-right" />Tools and Docs</h4>
              <v-row>
                <v-col>
                  <ul>
                    <li v-for="shortcut in toolShortcuts" :key="shortcut.shortcut">
                      <strong>{{ shortcut.shortcut }}</strong>: {{ shortcut.description }}
                    </li>
                  </ul>
                </v-col>
                <v-col>
                  <ul>
                    <li v-for="shortcut in docShortcuts" :key="shortcut.shortcut">
                      <strong>{{ shortcut.shortcut }}</strong>: {{ shortcut.description }}
                    </li>
                  </ul>
                </v-col>
              </v-row>
            </v-card-text>
            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn variant="tonal" @click="showHelpDialog = false">Close</v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
      </div>
      <div v-else class="mt-16">
        <v-container class="bg-primary text-center">
          <v-row align="center" justify="end" class="text-end">
            <v-col>
              <v-btn @click="handleShowSignIn" color="primary">Login</v-btn>
            </v-col>
          </v-row>
        </v-container>
        <LandingPage />
        <v-dialog v-model="showSignIn" max-width="600px">
          <div class="m-auto">
            <div id="sign-in"></div>
          </div>
          </v-dialog>
      </div>
    </div>
  </v-theme-provider>
</template>

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
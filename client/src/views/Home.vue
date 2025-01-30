<template>
  <div v-if="isLoading" class="h-screen flex items-center justify-center">
    <v-progress-circular indeterminate />
  </div>
  <v-container v-else>
    <div v-if="isLoggedIn">
      <v-container class="bg-primary">
        <v-row class="items-center">
          <v-col>
            <h1 class="text-3xl">
              Better New Tab
            </h1>
          </v-col>
          <v-col class="flex justify-end">
            <div class="flex justify-evenly w-1/2">
              <button id="user-button"></button>
              <v-btn icon="mdi-cog" @click="router.push('/settings');" class="!w-[42px] !h-[42px]" />
              <v-btn icon="mdi-help" @click="showHelpDialog = true" class="!w-[42px] !h-[42px]"></v-btn>
            </div>
          </v-col>
        </v-row>
      </v-container>

      <!-- Rest of the template remains unchanged -->

      <SearchBar :tools="tools" :docs="docs" />
      <LinkColumns :tools="toolLinks" :docs="docLinks" :userId="userId"
        :maxPins="userStore.userPlan?.max_pins || 6" :canAddLinks="canShowAddLink"
        @link-deleted="handleDeleteLink" :isPlanFree="userStore.userPlan?.name === 'free'" />
      <v-dialog v-model="showHelpDialog" max-width="900px">
        <v-card>
          <v-card-title class="headline">Help</v-card-title>
          <v-card-text>
            <h3 class="text-xl">Search Bar Controls</h3>
            <p>While in the search bar, type in a Jira Ticket number for relevant links, then use arrow keys or your
              mouse to
              navigate</p>
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
  </v-container>
  <div class="fixed bottom-4 right-4 bg-gray-800 p-4 rounded-lg shadow-lg z-50">
    <div class="mb-4">
      <h3 class="text-sm font-semibold mb-2">Plans:</h3>
      <div class="space-x-2">
        <v-btn v-for="plan in plans" :key="plan.name" :color="userPlan?.name === plan.name ? 'primary' : ''"
          @click="switchPlan(plan)" size="small">
          {{ plan.name }}
        </v-btn>
      </div>
    </div>

    <div>
      <h3 class="text-sm font-semibold mb-2">Roles:</h3>
      <div class="space-x-2">
        <v-btn v-for="role in roles" :key="role" @click="currentRole = role"
          :color="currentRole === role ? 'primary' : ''" size="small">
          {{ role }}
        </v-btn>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import type { Link } from "@/types/Link";
  import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
  import type { ClerkUser } from "@/types/User";
  import { Clerk } from "@clerk/clerk-js";
  import { computed, nextTick, onMounted, ref } from "vue";
  import { useRouter } from "vue-router";
  import LandingPage from "../components/LandingPage.vue";
  import LinkColumns from "../components/LinkColumns.vue";
  import SearchBar from "../components/SearchBar.vue";
  import { useUserStore } from "../stores/user";
  import { useLinksStore } from "../stores/links";
  import { storeToRefs } from "pinia";
  const userStore = useUserStore();
  const linksStore = useLinksStore();
  // Convert store properties to refs for reactivity
  const { toolLinks, docLinks } = storeToRefs(linksStore)

  // Initialize services
  const router = useRouter();
  const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
  const clerk = new Clerk(clerkPubKey);
  let clerkUser: ClerkUser;

  // State management
  const isLoggedIn = ref(false);
  const isLoading = ref(true);
  const showSignIn = ref(false);
  const showHelpDialog = ref(false);

  // User and data state
  const userId = ref<string | null>(null);
  const userPlan = ref<Subscription | null>(null);
  const currentRole = ref("member");
  const tools = ref<Link[]>([]);
  const docs = ref<Link[]>([]);

  // Computed properties
  const toolShortcuts = computed(() =>
    tools.value.map((tool, index) => ({
      shortcut: `Ctrl+${index + 1}`,
      description: `Open ${tool.title}`,
    })),
  );

  const docShortcuts = computed(() =>
    docs.value.map((doc, index) => ({
      shortcut: `Alt+${index + 1}`,
      description: `Open ${doc.title}`,
    })),
  );

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

  const handleDeleteLink = (type: string, index: number) => {
    console.log("Deleting link", type, index);
    if (type === "tool") {
      tools.value.splice(index, 1);
      // Reorder remaining tools
      tools.value.forEach((tool, idx) => {
        tool.order_index = idx;
      });
    } else {
      docs.value.splice(index, 1);
      // Reorder remaining docs
      docs.value.forEach((doc, idx) => {
        doc.order_index = idx;
      });
    }
  };

  const handleShowSignIn = () => {
    showSignIn.value = true;
    nextTick(() => {
      const signInDiv = document.getElementById("sign-in");
      if (signInDiv) {
        clerk.mountSignIn(signInDiv as HTMLDivElement);
      }
    });
  };

  // Lifecycle hooks
  onMounted(async () => {
    isLoading.value = true;

    try {
      await clerk.load();
      isLoggedIn.value = !!clerk.user;

      if (isLoggedIn.value && clerk.user) {
        let gotUser = false;
        try {

          // pass clerk data to fetch user data
          gotUser = await userStore.fetchUserData({
            id: clerk.user.id,
            firstName: clerk.user.firstName || "",
            lastName: clerk.user.lastName || "",
            email: clerk.user.emailAddresses[0].emailAddress,
          });

        } catch (error) {
          console.error("Error fetching user data:", error);
        }
        if (!gotUser) {
          throw new Error("Error fetching user data");
        }

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
  });

  const plans: Subscription[] = [
    {
      name: "free",
      max_pins: 6,
      features: { custom_domains: false, analytics: false, team_features: false },
      created_at: null,
      stripe_id: "",
      id: "",
    },
    {
      name: "plus",
      max_pins: 20,
      features: { custom_domains: false, analytics: false, team_features: false },
      created_at: null,
      stripe_id: "prod_RedoeQpFeq9qCd",
      id: "5eb628db-35df-4c0d-80b8-2a609aa8bddd",
    },
    {
      name: "team",
      max_pins: 50,
      features: { custom_domains: true, analytics: true, team_features: true },
      created_at: null,
      stripe_id: "",
      id: "48c706b0-6da9-439a-8ce5-916544130a70",
    },
    {
      name: "enterprise",
      max_pins: 100,
      features: { custom_domains: true, analytics: true, team_features: true },
      created_at: null,
      stripe_id: "",
      id: "f5dfd34a-62a0-4963-8b82-097a06baf99f",
    },
  ];

  const roles = ["member", "admin", "owner"];

  function switchPlan(plan: (typeof plans)[0]) {
    userPlan.value = plan;
  }
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
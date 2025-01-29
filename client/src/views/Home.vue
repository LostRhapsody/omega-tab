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
      <LinkColumns v-if="userPlan" :tools="tools" :docs="docs" :userId="userId" :maxPins="userPlan.max_pins"
        :canAddLinks="canShowAddLink" @tool-added="handleToolAdded" @doc-added="handleDocAdded"
        @link-deleted="handleDeleteLink" :isPlanFree="userPlan.name === 'free'" />
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
import { Clerk } from "@clerk/clerk-js";
import { computed, nextTick, onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import LandingPage from "../components/LandingPage.vue";
import LinkColumns from "../components/LinkColumns.vue";
import SearchBar from "../components/SearchBar.vue";
import { useApi } from "../composables/useApi";
import { useUserStore } from "../stores/user";
import type { Tables } from "../types/Database";
const userStore = useUserStore();

// Initialize services
const { api } = useApi();
const router = useRouter();
const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);

// State management
const isLoggedIn = ref(false);
const isLoading = ref(true);
const showSignIn = ref(false);
const showHelpDialog = ref(false);
const selectedTeamId = ref<string>("");
const isOrganization = ref(false);

// User and data state
const userId = ref<string | null>(null);
const userPlan = ref<Subscription | null>(null);
const currentRole = ref("member");
const userTeams = ref<
	Array<{ id: string; name: string; role: string; organization_id: string }>
>([]);
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
	if (userPlan.value?.name === "free" || userPlan.value?.name === "plus") {
		return true;
	}

	if (
		userPlan.value?.name === "team" &&
		(currentRole.value === "admin" || currentRole.value === "owner")
	) {
		return true;
	}

	if (
		userPlan.value?.name === "enterprise" &&
		(currentRole.value === "admin" || currentRole.value === "owner")
	) {
		return true;
	}

	return false;
});

// Event handlers
const handleToolAdded = (tool: Link) => {
	tools.value.push(tool);
	tools.value.sort((a, b) => (a.order_index || 0) - (b.order_index || 0));
};

const handleDocAdded = (doc: Link) => {
	docs.value.push(doc);
	docs.value.sort((a, b) => (a.order_index || 0) - (b.order_index || 0));
};

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

// API interaction methods
const loadUserData = async () => {
	try {
		if (!clerk.user?.emailAddresses[0]) {
			throw new Error("No user email found");
		}

		const email = clerk.user.emailAddresses[0].emailAddress;

		const createUserResponse: { message: string } = await api("/create_user", {
			method: "POST",
			body: JSON.stringify({
				user_id: clerk.user.id,
				email: email,
			}),
		});

		if (createUserResponse.message === "User created successfully") {
			console.log("User created successfully");
		} else if (createUserResponse.message === "User already exists") {
			console.log("User already exists");
		} else {
			throw new Error("Unexpected response from create_user endpoint");
		}

		// Set user in store
		userStore.setUserId(clerk.user.id);
		userStore.setFirstName(clerk.user.firstName || "");
		userStore.setLastName(clerk.user.lastName || "");
		userStore.setEmail(email);

		// Get subscription status from backend
		const subscriptionData = (await api("/confirm", {
			method: "POST",
			body: JSON.stringify({
				email: email,
			}),
		})) as SubscriptionResponse;

		// Load user plan data
		const userPlanData: Subscription = await api(
			`/plan/${subscriptionData.plan_id}`,
		);
		userPlan.value = userPlanData;

		// Set user plan in store
		userStore.setPlan(userPlan.value);

		// Load user links
		const linksData: Link[] = await api(`/user/${clerk.user.id}/links`);
		if (linksData !== undefined)
			for (const link of linksData) {
				if (link.column_type === "tools") {
					handleToolAdded(link);
				} else {
					handleDocAdded(link);
				}
			}

		// Load user teams
		// const { data: teamsData } = await api(`/teams/${clerk.user.id}`);
		// userTeams.value = teamsData.map((t: any) => ({
		//   id: t.entity_id,
		//   name: t.teams?.name || '',
		//   role: t.role,
		//   organization_id: t.teams?.organization_id || ''
		// })).filter((t: any) => t.role === 'admin' || t.role === 'owner');

		// Update organization status
		// isOrganization.value = userTeams.value.some(team => team.organization_id.length > 0);

		// Set current role if teams exist
		if (userTeams.value[0]) {
			currentRole.value = userTeams.value[0].role;
		}
	} catch (error) {
		console.error("Error loading user data:", error);
		// Handle error appropriately
	}
};

// Lifecycle hooks
onMounted(async () => {
	isLoading.value = true;

	try {
		await clerk.load();
		isLoggedIn.value = !!clerk.user;

		if (isLoggedIn.value && clerk.user) {
			userId.value = clerk.user.id;
			await loadUserData();
		}
	} catch (error) {
		console.error("Error during initialization:", error);
		// Handle error appropriately
	} finally {
		isLoading.value = false;
	}

	// Mount Clerk user button if logged in
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
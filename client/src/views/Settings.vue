<template>
  <div class="settings-container h-screen">
    <v-card>
      <v-layout>
        <v-navigation-drawer permanent location="left" width="256" class="h-full">
          <v-list>
            <v-list-item prepend-icon="mdi-arrow-left-circle" title="Back" value="back"
              @click="router.push('/')" />
            <v-list-item prepend-icon="mdi-account-cog" title="User Preferences" value="preferences"
              :active="activeTab === 'preferences'" @click="activeTab = 'preferences'" />
            <v-list-item prepend-icon="mdi-account-group" title="Manage Team" value="team"
              :active="activeTab === 'team'" @click="activeTab = 'team'" />
            <v-list-item prepend-icon="mdi-domain" title="Organization" value="organization"
              :active="activeTab === 'organization'" @click="activeTab = 'organization'" />
            <v-list-item prepend-icon="mdi-credit-card" title="Billing" value="billing"
              :active="activeTab === 'billing'" @click="activeTab = 'billing'" />
          </v-list>
        </v-navigation-drawer>
      </v-layout>
    </v-card>


    <div class="content-area ml-64 p-8">
      <!-- User Preferences -->
      <div v-if="activeTab === 'preferences'" class="max-w-2xl">
        <h2 class="text-2xl font-bold mb-6">User Preferences</h2>
        <v-form>
          <v-text-field v-model="fullName" label="Name" disabled class="mb-4" />
          <v-text-field v-model="email" label="Email" disabled class="mb-4" />
        </v-form>
        <v-alert type="info" class="my-8">
          More options and themes coming soon
        </v-alert>
        <v-switch v-model="settings.searchHistory" label="Enable Search History" class="mb-4" color="primary" :disabled="userPlan?.name === 'free'" />
        <v-switch v-model="settings.autocompleteSuggestions" label="Enable Autocomplete Suggestions Powered By Google" class="mb-4" color="primary" :disabled="userPlan?.name === 'free'" />
        <v-switch v-model="settings.jiraIntegration" label="Enable Jira Integration" class="mb-4" color="primary" :disabled="userPlan?.name === 'free'" />
        <v-switch v-model="settings.confluenceIntegration" label="Enable Confluence Integration" class="mb-4" color="primary" :disabled="userPlan?.name === 'free'" />
      </div>

      <!-- Team Management -->
      <div v-else-if="activeTab === 'team'" class="max-w-3xl">
        <template v-if="!isTeamPlan">
          <div class="text-center py-12">
            <v-icon icon="mdi-account-group" size="64" class="mb-4" />
            <h3 class="text-2xl font-bold mb-4">Team Plan Coming Soon</h3>
            <!-- <h3 class="text-2xl font-bold mb-4">Upgrade to Team Plan</h3>
            <p class="mb-6">Access team features by upgrading to our team plan</p>
            <v-btn color="primary" @click="router.push('/plans')">View Plans</v-btn> -->

          </div>
        </template>
        <template v-else-if="!hasTeam">
          <div class="text-center py-12">
            <v-icon icon="mdi-account-group-outline" size="64" class="mb-4" />
            <h3 class="text-2xl font-bold mb-4">Create Your Team</h3>
            <p class="mb-6">Get started by creating your first team</p>
            <v-btn color="primary" @click="showTeamModal = true">Create Team</v-btn>
          </div>
        </template>
        <template v-else>
          <div class="flex justify-between items-center mb-6">
            <h2 class="text-2xl font-bold">Team Management</h2>
            <v-btn color="primary" prepend-icon="mdi-plus" @click="showInviteModal = true">
              Invite Member
            </v-btn>
          </div>

          <v-table>
            <thead>
              <tr>
                <th>Name</th>
                <th>Email</th>
                <th>Role</th>
                <th>Actions</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="member in teamMembers" :key="member.user_id">
                <td>{{ member.name }}</td>
                <td>{{ member.email }}</td>
                <td>
                  <v-select v-if="member.role !== 'owner'" v-model="member.role" :items="['admin', 'member']"
                    density="compact" variant="underlined" hide-details
                    @update:model-value="updateMemberRole(member.user_id, $event)" />
                  <span v-else class="font-medium">Owner</span>
                </td>
                <td>
                  <v-btn v-if="member.role !== 'owner'" icon="mdi-delete" variant="text" color="error"
                    density="comfortable" @click="removeMember(member.user_id)" />
                </td>
              </tr>
            </tbody>
          </v-table>
        </template>
      </div>

      <!-- Organization -->
      <div v-else-if="activeTab === 'organization'" class="max-w-3xl">
        <template v-if="!isEnterprisePlan">
          <div class="text-center py-12">
            <v-icon icon="mdi-domain" size="64" class="mb-4" />
            <h3 class="text-2xl font-bold mb-4">Enterprise Plan Coming Soon</h3>
            <!-- <p class="mb-6">Contact sales to discuss an enterprise plan for organizations with multiple teams</p>
            <v-btn color="primary" @click="contactSales">Contact Sales</v-btn> -->
          </div>
        </template>
        <template v-else>
          <div class="mb-8">
            <h2 class="text-2xl font-bold mb-6">Organization Settings</h2>
            <v-text-field v-model="orgName" label="Organization Name" class="mb-4" />
            <v-select v-model="selectedTeam" :items="teams" label="Current Team" item-title="name" item-value="id"
              class="mb-4" />
            <v-btn color="primary" prepend-icon="mdi-plus" @click="showTeamModal = true">
              Add Team
            </v-btn>
          </div>

          <div class="mt-12">
            <div class="flex justify-between items-center mb-4">
              <h3 class="text-xl font-bold">Organization Members</h3>
              <v-text-field v-model="memberSearch" label="Search members" density="compact" hide-details
                class="max-w-xs" prepend-inner-icon="mdi-magnify" />
            </div>

            <v-table>
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Email</th>
                  <th>Team</th>
                  <th>Role</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="member in filteredOrgMembers" :key="member.id">
                  <td>{{ member.name }}</td>
                  <td>{{ member.email }}</td>
                  <td>{{ member.team }}</td>
                  <td>{{ member.role }}</td>
                </tr>
              </tbody>
            </v-table>
          </div>
        </template>
      </div>

      <!-- Billing -->
      <div v-else-if="activeTab === 'billing'" class="max-w-3xl">
        <h2 class="text-2xl font-bold mb-6">Billing & Subscription</h2>
        <v-card class="mb-6">
          <v-card-item>
            <v-card-title>Current Plan</v-card-title>
            <v-card-text>
              <div class="text-h4 mb-2">{{ userPlan?.name || 'Free' }}</div>
              <div class="text-body-1">{{ userPlan?.max_pins || 6 }} pins included</div>
            </v-card-text>
            <v-card-actions>
              <v-btn variant="elevated" color="primary" @click="router.push('/plans')">Change Plan</v-btn>
              <v-btn variant="elevated" color="red" @click="router.push('/plans')">Cancel Plan</v-btn>
            </v-card-actions>
          </v-card-item>
        </v-card>
      </div>
    </div>

    <!-- Create/Edit Team Modal -->
    <v-dialog v-model="showTeamModal" max-width="500">
      <v-card>
        <v-card-title>{{ selectedTeamId ? 'Edit Team' : 'Create Team' }}</v-card-title>
        <v-card-text>
          <v-form @submit.prevent="handleTeamSubmit">
            <v-text-field v-model="teamForm.name" label="Team Name" required />
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="grey" variant="text" @click="showTeamModal = false">Cancel</v-btn>
          <v-btn color="primary" @click="handleTeamSubmit">
            {{ selectedTeamId ? 'Save Changes' : 'Create Team' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Invite Member Modal -->
    <v-dialog v-model="showInviteModal" max-width="500">
      <v-card>
        <v-card-title>Invite Team Member</v-card-title>
        <v-card-text>
          <v-form @submit.prevent="handleInvite">
            <v-text-field v-model="inviteEmail" label="Email Address" type="email" required />
          </v-form>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn color="grey" variant="text" @click="showInviteModal = false">Cancel</v-btn>
          <v-btn color="primary" @click="handleInvite">Send Invite</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { type ComputedRef, computed, ref } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "../stores/user";
import type { Features } from "../types/Features";

// In Settings.vue setup
const userStore = useUserStore();

const userId = computed(() => userStore.userId);
const firstName = computed(() => userStore.firstName);
const lastName = computed(() => userStore.lastName);
const fullName = computed(() => `${firstName.value} ${lastName.value}`);
const email = computed(() => userStore.email);
const userPlan = computed(() => userStore.userPlan) as ComputedRef<{
	created_at: string | null;
	features: Features;
	id: string;
	max_pins: number;
	name: string;
	stripe_id: string | null;
} | null>;

const router = useRouter();
const activeTab = ref("preferences");
const showTeamModal = ref(false);
const showInviteModal = ref(false);
const selectedTeamId = ref("");
const memberSearch = ref("");

// Team management data
const teamForm = ref({
	name: "",
});

const inviteEmail = ref("");
const teamMembers = ref<TeamMember[]>([]);
type TeamMember = {
	user_id: string;
	name: string;
	email: string;
	role: string;
};

// Organization data
const orgName = ref("");
const selectedTeam = ref<string>("");
const teams = ref<string[]>([]);
const orgMembers = ref<OrgMember[]>([]);
type OrgMember = {
	user_id: string;
	name: string;
	email: string;
	team: string;
	role: string;
	id: string;
};

// Computed properties
const isTeamPlan = computed(
	() =>
		userPlan.value?.name === "team" || userPlan.value?.name === "enterprise",
);
const isEnterprisePlan = computed(() => userPlan.value?.name === "enterprise");
const showTeamTab = computed(() => isTeamPlan.value);
const hasTeam = computed(() => teamMembers.value.length > 0);

const settings = ref({
	searchHistory: false,
	autocompleteSuggestions: false,
	jiraIntegration: false,
	confluenceIntegration: false,
});

const filteredOrgMembers = computed(() => {
	if (!memberSearch.value) return orgMembers.value;
	const search = memberSearch.value.toLowerCase();
	return orgMembers.value.filter(
		(member) =>
			member.name.toLowerCase().includes(search) ||
			member.email.toLowerCase().includes(search) ||
			member.team.toLowerCase().includes(search),
	);
});

// Methods
const handleTeamSubmit = async () => {
	try {
		if (selectedTeamId.value) {
			// Update existing team
			// todo add team creation/update logic to backend and call here
			// await teamUtils.updateTeam(selectedTeamId.value, { name: teamForm.value.name });
		} else {
			// todo add team creation/update logic to backend and call here
			// Create new team
			// await teamUtils.createTeam(
			//   userId,
			//   userPlan.id,
			//   teamForm.value.name
			// );
		}
		showTeamModal.value = false;
		// loadTeamData();
	} catch (error) {
		console.error("Error managing team:", error);
	}
};

const handleInvite = async () => {
	try {
		// todo add membership creation/update logic to backend and call here
		// await membershipUtils.addMember(inviteEmail.value, selectedTeamId.value, 'team', 'member');
		showInviteModal.value = false;
		// loadTeamData();
	} catch (error) {
		console.error("Error inviting member:", error);
	}
};

const updateMemberRole = async (userId: string, newRole: string) => {
	// try {
	//   // todo add membership creation/update logic to backend and call here
	//   // await membershipUtils.updateMemberRole(userId, selectedTeamId.value, newRole);
	//   // loadTeamData();
	// } catch (error) {
	//   console.error('Error updating member role:', error);
	// }
};

const removeMember = async (userId: string) => {
	// try {
	//   // todo add membership creation/update logic to backend and call here
	//   // await membershipUtils.removeMember(userId, selectedTeamId.value);
	//   // loadTeamData();
	// } catch (error) {
	//   console.error('Error removing member:', error);
	// }
};

const loadTeamData = async () => {
	try {
		if (!userId.value) return;
		// todo add team creation/update logic to backend and call here
		// const userTeams = await teamUtils.getUserTeams(userId.value);
		// if (userTeams.length > 0) {
		// selectedTeamId.value = userTeams[0].teams?.id || "";
		// todo add team creation/update logic to backend and call here
		// const members = await teamUtils.getTeamMembers(selectedTeamId.value);
		// teamMembers.value = members;
		// }
	} catch (error) {
		console.error("Error loading team data:", error);
	}
};

const contactSales = () => {
	window.location.href = "mailto:sales@example.com";
};
</script>

<style scoped>
.settings-container {
  display: flex;
  min-height: 100vh;
}

.content-area {
  flex: 1;
  background-color: var(--v-background);
}
</style>
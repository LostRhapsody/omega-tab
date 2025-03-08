<template>
  <div class="settings-container h-screen">
    <v-card>
      <v-layout>
        <v-navigation-drawer :rail="mobile" permanent location="left" width="256" class="h-full">
          <v-list>
            <v-list-item prepend-icon="mdi-arrow-left-circle" title="Back" value="back" @click="router.push('/')" />
            <v-list-item prepend-icon="mdi-account-cog" title="User Preferences" value="preferences"
              :active="activeTab === 'preferences'" @click="activeTab = 'preferences'" />
            <v-list-item prepend-icon="mdi-credit-card" title="Billing" value="billing"
              :active="activeTab === 'billing'" @click="activeTab = 'billing'" />
            <v-list-item prepend-icon="mdi-account-group" title="Manage Team" value="team"
              :active="activeTab === 'team'" @click="activeTab = 'team'" />
            <v-list-item prepend-icon="mdi-domain" title="Organization" value="organization"
              :active="activeTab === 'organization'" @click="activeTab = 'organization'" />            
          </v-list>
        </v-navigation-drawer>
      </v-layout>
    </v-card>


    <div :class="mobile ? 'content-area ml-16 p-8' : 'content-area ml-64 p-8'">
      <!-- User Preferences -->
      <div v-if="activeTab === 'preferences'" class="max-w-2xl">
        <h2 class="text-2xl font-bold mb-6">User Preferences</h2>
        <v-form>
          <v-text-field v-model="fullName" label="Name" disabled class="mb-4" />
          <v-text-field v-model="email" label="Email" disabled class="mb-4" />
        </v-form>
        <div v-for="setting in UserSettingsLabels" :key="setting.key">
          <!-- Todo, when hover over a setting, provide more details -->
            <v-switch
            v-if="setting.active"
            color="primary"
            v-model="settingsStore.settings[setting.key as keyof UserSettings]"
            :label="setting.label"
            @change="settingsStore.updateSetting(setting.key as keyof UserSettings, settingsStore.settings[setting.key as keyof UserSettings])"
            :disabled="setting.plan === 'plus' && userPlan?.name !== 'plus'"
            >
            <template v-if="setting.plan === 'plus'" v-slot:label>
              {{ setting.label }}&nbsp;<span class="kbd">+Plus Feature</span>
            </template>
            </v-switch>
        </div>
        <v-btn @click="clearSearchHistory">Clear Search History</v-btn>

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
            <v-card-title class="mb-2">Current Plan</v-card-title>
            <v-card-text class="border border-gray-200 rounded-lg !p-2">
                <div class="text-h4 mb-2">
                {{ userPlan?.name === 'plus' ? 'Plus+' : userPlan?.name === 'pro' ? 'Pro' : 'Free' }}
                </div>
              <div class="text-body-1">{{ userPlan?.max_pins || 6 }} pins included</div>
            </v-card-text>
            <v-card-actions>
              <v-btn v-if="userPlan?.name === 'free'" variant="elevated" color="primary"
                @click="router.push('/plans')">Upgrade Plan</v-btn>
              <v-btn v-if="userPlan?.name !== 'free'" variant="elevated" color="red"
                @click="showCancelDialog = true">Cancel
                Plan</v-btn>
              <a :href="stripe_manage_url" target="_blank" rel="noopener noreferrer">
                <v-btn v-if="userPlan?.name !== 'free'" variant="elevated">Manage Subscription</v-btn>
              </a>
            </v-card-actions>
          </v-card-item>
        </v-card>
        <v-dialog v-model="showCancelDialog" max-width="400">
          <v-card>
            <v-card-title>Cancel Subscription</v-card-title>
            <v-card-text>
              Are you sure you want to cancel your subscription?
              <br/><br/>You will have access to your subscription features until the
              end of
              the current billing period.
            </v-card-text>
            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn color="grey" variant="text" @click="showCancelDialog = false">No, Keep It</v-btn>
              <v-btn color="error" @click="collectedFeedback">Yes, Cancel</v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>

        <!-- Subscription Canceled Modal -->
        <v-dialog v-model="showSubscriptionCanceledDialog" max-width="400">
          <v-card>
            <v-card-title>Subscription Canceled</v-card-title>
            <v-card-text>
              <template v-if="subscriptionCanceledSuccess">
                <div class="text-center">
                  <v-icon color="green" size="64">mdi-check-circle</v-icon>
                  <br />
                  <p>Your subscription has been successfully canceled.</p>
                </div>
              </template>
              <template v-else>
                <div class="text-center">
                  <v-icon color="red" size="64">mdi-close-circle</v-icon>
                  <p>An error occurred while canceling your subscription:</p>
                  <v-divider class="my-4" />
                  <p>{{ subscriptionCanceledError }}</p>
                  <v-divider class="my-4" />
                  <v-btn color="error" @click="reportError">Report Error</v-btn>
                </div>
              </template>
            </v-card-text>
            <v-card-actions>
              <v-spacer></v-spacer>
              <v-btn color="primary" @click="showSubscriptionCanceledDialog = false">Close</v-btn>
            </v-card-actions>
          </v-card>
        </v-dialog>
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

    <!-- Feedback Form -->
    <Feedback v-model="showFeedbackDialog" @update:modelValue="handleFeedbackDialogClose" :cancelSubscription=true />
  </div>
</template>

<script setup lang="ts">
  import { type ComputedRef, computed, ref, onMounted, onUnmounted } from "vue";
  import { useRouter } from "vue-router";
  import { useUserStore } from "../stores/user";
  import type { Features } from "../types/Features";
  import { API } from "../constants/api";
  import Feedback from "../components/Feedback.vue";
  import { useFeedbackStore } from "../stores/feedback";
  import type { CancellationReason } from "../types/CancellationReasons";
  import { type UserSettings, UserSettingsLabels } from "../types/UserSettings";
  import { useUserSettingsStore } from "../stores/settings";
  import { useDisplay } from 'vuetify';
  import api from "@/services/api";
  import { Clerk } from "@clerk/clerk-js";
  import { cache, CacheKeys } from "@/utils/cache";
  

  const userStore = useUserStore();
  const feedbackStore = useFeedbackStore();
  const settingsStore = useUserSettingsStore();
  const mobile = useDisplay().smAndDown;

  const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
  const clerk = new Clerk(clerkPubKey);

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
  const showPaymentTooltip = ref(false);
  const showFeedbackDialog = ref(false);
  const showSubscriptionCanceledDialog = ref(false);
  const subscriptionCanceledSuccess = ref(false);
  const subscriptionCanceledError = ref("");
  const stripe_manage_url = import.meta.env.VITE_STRIPE_MANAGE_URL;

  // Token refresh interval
  let tokenRefreshInterval: number | undefined;

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

  const showCancelDialog = ref(false);

  const refreshToken = async () => {
    try {
      const session = await clerk.session;
      const token = await session?.getToken();
      if (token) {
        localStorage.setItem("token", token);
      }
    } catch (error) {
      console.error("Error refreshing JWT token:", error);
    }
  };

  const startTokenRefreshInterval = () => {
    // Refresh token every 15 minutes
    tokenRefreshInterval = window.setInterval(refreshToken, 1 * 60 * 1000);
  };

  const stopTokenRefreshInterval = () => {
    if (tokenRefreshInterval) {
      clearInterval(tokenRefreshInterval);
    }
  };

  // Methods
  const handleTeamSubmit = async () => {
    try {
      if (selectedTeamId.value) {
        // Update existing team
        // todo add team creation/update logic to backend and call here
        // await api.put(`/teams/${selectedTeamId.value}`, { name: teamForm.value.name });
      } else {
        // todo add team creation/update logic to backend and call here
        // Create new team
        // await api.post('/teams', { userId: userId.value, planId: userPlan.value.id, name: teamForm.value.name });
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
      // await api.post('/memberships', { email: inviteEmail.value, teamId: selectedTeamId.value, role: 'member' });
      showInviteModal.value = false;
      // loadTeamData();
    } catch (error) {
      console.error("Error inviting member:", error);
    }
  };

  const updateMemberRole = async (userId: string, newRole: string) => {
    // try {
    //   // todo add membership creation/update logic to backend and call here
    //   // await api.put(`/memberships/${userId}`, { teamId: selectedTeamId.value, role: newRole });
    //   // loadTeamData();
    // } catch (error) {
    //   console.error('Error updating member role:', error);
    // }
  };

  const removeMember = async (userId: string) => {
    // try {
    //   // todo add membership creation/update logic to backend and call here
    //   // await api.delete(`/memberships/${userId}`, { data: { teamId: selectedTeamId.value } });
    //   // loadTeamData();
    // } catch (error) {
    //   console.error('Error removing member:', error);
    // }
  };

  const loadTeamData = async () => {
    try {
      if (!userId.value) return;
      // todo add team creation/update logic to backend and call here
      // const userTeams = await api.get(`/users/${userId.value}/teams`);
      // if (userTeams.length > 0) {
      // selectedTeamId.value = userTeams[0].teams?.id || "";
      // todo add team creation/update logic to backend and call here
      // const members = await api.get(`/teams/${selectedTeamId.value}/members`);
      // teamMembers.value = members;
      // }
    } catch (error) {
      console.error("Error loading team data:", error);
    }
  };

  const contactSales = () => {
    window.location.href = "mailto:sales@example.com";
  };

  const collectedFeedback = async () => {
    showFeedbackDialog.value = true;
  }

  const handleFeedbackDialogClose = (value: boolean) => {
    if(!value) {
      showCancelDialog.value = false;
      cancelSubHandler();
    }
  }

  const cancelSubHandler = async () => {
    showFeedbackDialog.value = true;
    showCancelDialog.value = false;
    if (!userStore.userId) {
      throw new Error("User ID not found");
    }
    if (!userStore.email) {
      throw new Error("User email not found");
    }
    try {
      let reasons: CancellationReason | null = feedbackStore.reasons as CancellationReason;
      let feedback_comment: String | null = feedbackStore.feedbackComment;
      console.log("reasons", reasons);
      console.log("feedback_comment", feedback_comment);      
      if (reasons === "") {
        reasons = null;
      }
      if (feedback_comment === "") {
        feedback_comment = null;
      }
      const response = await api.post(API.CANCEL_SUBSCRIPTION, {
        reasons: feedbackStore.reasons as CancellationReason,
        feedback_comment: feedbackStore.feedbackComment,
      });
      /*
        200 If unsubscribed successfully
        400 if request is incorrect
        401 if the user is not subscribed at all yet we're here somehow
        404 if the user or sub is not found (how are you here)
        500/default some unknown error
      */
      switch (response.status) {
        case 200:
          subscriptionCanceledSuccess.value = true;
          // this updates the user store with the new data so the billing details are up to date
          userStore.fetchUserData({
            id: userStore.userId,
            email: userStore.email,
            firstName: userStore.firstName || "",
            lastName: userStore.lastName || "",
          });
          break;
        case 400:
          throw new Error("Invalid request: The cancel subscription request was incorrect.");
        case 401:
          throw new Error("Unauthorized: This user does not have an active subscription, using this button is a bug.");
        case 404:
          throw new Error("User not found: This user was not found, using this button is a bug.");
        default:
          throw new Error("An error occurred: An unknown error occurred while cancelling your subscription.");
      }
    } catch (err) {
      console.error("Error cancelling subscription:", err);
      subscriptionCanceledSuccess.value = false;
      subscriptionCanceledError.value = err as string;
    } finally {
      showSubscriptionCanceledDialog.value = true;
    }

  }

  const reportError = () => {
    window.location.href = "mailto:evan.robertson77@gmail.com";
  };

  const clearSearchHistory = () => {
    cache.clear(CacheKeys.SEARCH_HISTORY);
  };

  onMounted(() => {
    // Start token refresh interval
    startTokenRefreshInterval();
  });

  onUnmounted(() => {
    stopTokenRefreshInterval();
  });
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
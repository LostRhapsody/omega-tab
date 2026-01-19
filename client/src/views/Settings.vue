<template>
  <div class="settings">
    <!-- Sidebar Navigation -->
    <aside class="settings__sidebar" :class="{ 'settings__sidebar--collapsed': mobile }">
      <nav class="settings__nav">
        <button class="settings__nav-item" @click="router.push('/')">
          <TpIcon name="arrow-left" size="sm" />
          <span v-if="!mobile">Back</span>
        </button>
        <button
          class="settings__nav-item"
          :class="{ 'settings__nav-item--active': activeTab === 'preferences' }"
          @click="activeTab = 'preferences'"
        >
          <TpIcon name="user" size="sm" />
          <span v-if="!mobile">User Preferences</span>
        </button>
        <button
          class="settings__nav-item"
          :class="{ 'settings__nav-item--active': activeTab === 'team' }"
          @click="activeTab = 'team'"
        >
          <TpIcon name="user" size="sm" />
          <span v-if="!mobile">Manage Team</span>
        </button>
        <button
          class="settings__nav-item"
          :class="{ 'settings__nav-item--active': activeTab === 'organization' }"
          @click="activeTab = 'organization'"
        >
          <TpIcon name="cog" size="sm" />
          <span v-if="!mobile">Organization</span>
        </button>
      </nav>
    </aside>

    <!-- Content Area -->
    <main class="settings__content">
      <!-- User Preferences -->
      <div v-if="activeTab === 'preferences'" class="settings__panel">
        <h2 class="settings__title">User Preferences</h2>

        <div class="settings__form">
          <TpInput v-model="emailValue" label="Email" disabled />

          <div v-for="setting in UserSettingsLabels" :key="setting.key">
            <TpSwitch
              v-if="setting.active"
              v-model="settingsStore.settings[setting.key as keyof UserSettings]"
              :label="setting.label"
              @update:modelValue="
                settingsStore.updateSetting(
                  setting.key as keyof UserSettings,
                  settingsStore.settings[setting.key as keyof UserSettings],
                )
              "
            />
          </div>

          <TpButton variant="secondary" @click="clearSearchHistory">
            Clear Search History
          </TpButton>
        </div>
      </div>
    </main>

    <!-- Create/Edit Team Modal -->
    <TpModal
      v-model="showTeamModal"
      :title="selectedTeamId ? 'Edit Team' : 'Create Team'"
      size="sm"
    >
      <form @submit.prevent="handleTeamSubmit" class="settings__modal-form">
        <TpInput v-model="teamForm.name" label="Team Name" required />
      </form>
      <template #actions>
        <TpButton variant="ghost" @click="showTeamModal = false">Cancel</TpButton>
        <TpButton variant="primary" @click="handleTeamSubmit">
          {{ selectedTeamId ? "Save Changes" : "Create Team" }}
        </TpButton>
      </template>
    </TpModal>

    <!-- Invite Member Modal -->
    <TpModal v-model="showInviteModal" title="Invite Team Member" size="sm">
      <form @submit.prevent="handleInvite" class="settings__modal-form">
        <TpInput v-model="inviteEmail" label="Email Address" type="email" required />
      </form>
      <template #actions>
        <TpButton variant="ghost" @click="showInviteModal = false">Cancel</TpButton>
        <TpButton variant="primary" @click="handleInvite">Send Invite</TpButton>
      </template>
    </TpModal>
  </div>
</template>

<script setup lang="ts">
import { type ComputedRef, computed, ref } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "../stores/user";
import type { Features } from "../types/Features";
import { type UserSettings, UserSettingsLabels } from "../types/UserSettings";
import { useUserSettingsStore } from "../stores/settings";
import { useBreakpoint } from "@/composables/useBreakpoint";
import { cache, CacheKeys } from "@/utils/cache";
import { TpIcon, TpInput, TpSwitch, TpSelect, TpButton, TpModal } from "@/components/ui";

const userStore = useUserStore();
const settingsStore = useUserSettingsStore();
const { smAndDown: mobile } = useBreakpoint();

const userId = computed(() => userStore.userId);
const email = computed(() => userStore.email);
const emailValue = ref(email.value || "");

const router = useRouter();
const activeTab = ref("preferences");
const showTeamModal = ref(false);
const showInviteModal = ref(false);
const selectedTeamId = ref("");
const memberSearch = ref("");

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

const roleOptions = [
  { value: "admin", label: "Admin" },
  { value: "member", label: "Member" },
];

const teamOptions = computed(() =>
  teams.value.map((team) => ({
    value: team,
    label: team,
  })),
);

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

const handleTeamSubmit = async () => {
  try {
    if (selectedTeamId.value) {
      // Update existing team
    } else {
      // Create new team
    }
    showTeamModal.value = false;
  } catch (error) {
    console.error("Error managing team:", error);
  }
};

const handleInvite = async () => {
  try {
    showInviteModal.value = false;
  } catch (error) {
    console.error("Error inviting member:", error);
  }
};

const updateMemberRole = async (userId: string, newRole: string) => {
  // Implementation pending
};

const removeMember = async (userId: string) => {
  // Implementation pending
};

const clearSearchHistory = () => {
  cache.clear(CacheKeys.SEARCH_HISTORY);
};
</script>

<style scoped>
.settings {
  display: flex;
  min-height: 100vh;
  background: var(--tp-bg-primary);
}

.settings__sidebar {
  width: 256px;
  border-right: 1px solid var(--tp-border);
  background: var(--tp-bg-secondary);
  flex-shrink: 0;
}

.settings__sidebar--collapsed {
  width: 64px;
}

.settings__nav {
  display: flex;
  flex-direction: column;
  padding: var(--tp-space-4);
  gap: var(--tp-space-1);
}

.settings__nav-item {
  display: flex;
  align-items: center;
  gap: var(--tp-space-3);
  padding: var(--tp-space-3) var(--tp-space-4);
  border-radius: var(--tp-radius-sm);
  color: var(--tp-text-secondary);
  text-align: left;
  transition:
    background-color var(--tp-transition-fast),
    color var(--tp-transition-fast);
}

.settings__nav-item:hover {
  background: var(--tp-bg-tertiary);
  color: var(--tp-text-primary);
}

.settings__nav-item--active {
  background: var(--tp-accent-glow);
  color: var(--tp-accent);
  border-left: 2px solid var(--tp-accent);
}

.settings__nav-item:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

.settings__content {
  flex: 1;
  padding: var(--tp-space-8);
  overflow-y: auto;
}

.settings__panel {
  max-width: 768px;
}

.settings__title {
  font-size: var(--tp-text-2xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-6);
}

.settings__subtitle {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
}

.settings__form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.settings__header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: var(--tp-space-6);
  flex-wrap: wrap;
  gap: var(--tp-space-4);
}

.settings__section {
  margin-bottom: var(--tp-space-12);
}

.settings__search {
  width: 256px;
}

.settings__empty {
  text-align: center;
  padding: var(--tp-space-12);
}

.settings__empty-title {
  font-size: var(--tp-text-2xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin: var(--tp-space-4) 0;
}

.settings__empty-text {
  color: var(--tp-text-muted);
  margin-bottom: var(--tp-space-6);
}

.settings__table {
  width: 100%;
  border-collapse: collapse;
}

.settings__table th,
.settings__table td {
  padding: var(--tp-space-3) var(--tp-space-4);
  text-align: left;
  border-bottom: 1px solid var(--tp-border);
}

.settings__table th {
  font-weight: var(--tp-font-semibold);
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
  text-transform: uppercase;
  letter-spacing: var(--tp-tracking-wide);
}

.settings__table td {
  color: var(--tp-text-primary);
}

.settings__owner-badge {
  font-weight: var(--tp-font-medium);
  color: var(--tp-accent);
}

.settings__modal-form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

@media (max-width: 768px) {
  .settings__content {
    padding: var(--tp-space-4);
  }

  .settings__search {
    width: 100%;
  }
}
</style>

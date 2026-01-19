<template>
  <TpMenu position="bottom-end">
    <template #trigger="{ isOpen }">
      <button class="user-menu-trigger" :aria-expanded="isOpen" aria-haspopup="menu">
        <TpAvatar :name="userEmail" size="md" />
      </button>
    </template>

    <template #default="{ close }">
      <div class="user-menu__header">
        <span class="user-menu__email">{{ userEmail }}</span>
      </div>

      <TpDivider />

      <TpMenuItem icon="cog" @click="goToSettings(close)"> Settings </TpMenuItem>

      <TpMenuItem icon="logout" @click="logout"> Sign Out </TpMenuItem>
    </template>
  </TpMenu>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRouter } from "vue-router";
import { useUserStore } from "@/stores/user";
import { authService } from "@/services/auth";
import { TpMenu, TpMenuItem, TpAvatar, TpDivider } from "@/components/ui";

const router = useRouter();
const userStore = useUserStore();

const userEmail = computed(() => userStore.email || "");

const goToSettings = (close: () => void) => {
  close();
  router.push("/settings");
};

const logout = () => {
  authService.logout();
  window.location.href = "/";
};
</script>

<style scoped>
.user-menu-trigger {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--tp-space-1);
  border-radius: 50%;
  transition: background-color var(--tp-transition-fast);
}

.user-menu-trigger:hover {
  background: var(--tp-bg-secondary);
}

.user-menu-trigger:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

.user-menu__header {
  padding: var(--tp-space-2) var(--tp-space-3);
}

.user-menu__email {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
  font-family: var(--tp-font-mono);
}
</style>

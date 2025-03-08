<template>
  <div class="min-h-screen flex flex-col items-center justify-center p-4">
    <!-- Loading State -->
    <div v-if="isLoading" class="text-center">
      <v-progress-circular indeterminate size="64" class="mb-4"></v-progress-circular>
      <p class="text-h6">Confirming your subscription...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="text-center">
      <v-icon icon="mdi-alert-circle" color="error" size="64" class="mb-4"></v-icon>
      <p class="text-h6 text-error mb-4">Something went wrong</p>
      <p class="text-h6 text-error mb-4">{{ error_message }}</p>
      <v-btn color="primary" @click="retryConfirmation">
        Retry
      </v-btn>
    </div>

    <!-- Success State -->
    <div v-else class="text-center">
      <v-icon icon="mdi-check-circle" color="success" size="64" class="mb-4"></v-icon>
      <h1 class="text-h4 mb-4">Welcome to Better New Tab!</h1>
      <p class="text-h6 mb-8">Your subscription has been confirmed.</p>
      <v-btn color="primary" size="x-large" @click="goToHome">
        Start using Better New Tab
      </v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { onMounted, ref } from "vue";
  import { useRouter } from "vue-router";
  import { useUserStore } from "@/stores/user";
  import { useUserSettingsStore } from "@/stores/settings";
  import { Clerk } from "@clerk/clerk-js";

  const router = useRouter();
  const userStore = useUserStore();
  const userSettingsStore = useUserSettingsStore();

  const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
  const clerk = new Clerk(clerkPubKey);

  const isLoading = ref(true);
  const isLoggedIn = ref(false);
  const error = ref(false);
  const error_message = ref("");

  // Refresh the token to ensure we have a fresh one before API calls
  const refreshToken = async () => {
    try {
      await clerk.load();
      const session = await clerk.session;
      
      if (!session) {
        console.warn("No active session found during token refresh");
        return false;
      }
      
      // Get token with leeway to handle clock skew
      const token = await session.getToken({ leewayInSeconds: 30 });
      if (token) {
        localStorage.setItem("token", token);
        return true;
      } else {
        console.warn("Failed to get token during refresh");
        // Try more direct approach
        await session.touch();
        const retryToken = await session.getToken();
        if (retryToken) {
          localStorage.setItem("token", retryToken);
          return true;
        }
      }
      return false;
    } catch (err) {
      console.error("Error refreshing token:", err);
      // Last resort: try to force a new clerk session
      try {
        await clerk.load();
        const newSession = await clerk.session;
        if (newSession) {
          const token = await newSession.getToken();
          if (token) {
            localStorage.setItem("token", token);
            return true;
          }
        }
      } catch (reloadErr) {
        console.error("Failed final token refresh attempt:", reloadErr);
      }
      return false;
    }
  };

  const confirmSubscription = async () => {
    try {
      await clerk.load();
      isLoggedIn.value = !!clerk.user;
      isLoading.value = true;
      error.value = false;
      let gotUser = false;

      if (!isLoggedIn.value || !clerk.user) {
        throw new Error("User not logged in");
      }

      // Refresh token before making any API calls
      const tokenRefreshed = await refreshToken();
      if (!tokenRefreshed) {
        throw new Error("Failed to refresh authentication token");
      }

      gotUser = await userStore.fetchUserData({
        id: clerk.user.id,
        firstName: clerk.user.firstName || "",
        lastName: clerk.user.lastName || "",
        email: clerk.user.emailAddresses[0].emailAddress,
      });

      if (!gotUser) {
        throw new Error("Failed to fetch user data");
      }

      // now confirm the subscription
      const confirmed = await userStore.confirmSubscription();
      if (!confirmed) {
        throw new Error("Failed to confirm subscription");
      }

      isLoading.value = false;
    } catch (err) {
      console.error("Error confirming subscription:", err);
      error_message.value = err instanceof Error ? err.message : String(err);
      error.value = true;
      isLoading.value = false;
    }
  };

  const retryConfirmation = () => {
    confirmSubscription();
  };

  const goToHome = () => {
    router.push("/");
  };

  onMounted(() => {
    confirmSubscription();
  });
</script>
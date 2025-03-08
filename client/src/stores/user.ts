import { API } from "@/constants/api";
import api from "@/services/api";
import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
import type {
  ClerkUser,
  User,
  UserDataResponse,
  UserState,
} from "@/types/User";
import { CacheKeys, cache } from "@/utils/cache";
import { defineStore } from "pinia";
import { useLinksStore } from "./links";
import { useUserSettingsStore } from "./settings";

export const useUserStore = defineStore("user", {
  state: (): UserState => ({
    userId: null,
    firstName: null,
    lastName: null,
    email: null,
    userPlan: null,
    isLoading: false,
    error: null,
    auth_token: null,
  }),

  actions: {
    /**
     * Fetches user data from the API using the Clerk user information and updates the store state.
     * This includes the user record, subscription record, and plan record.
     * @param clerk_user - The Clerk user object containing authentication details.
     * @returns true if the user data was successfully fetched, false otherwise.
     * @throws Error if the user data could not be fetched.
     */
    async fetchUserData(clerk_user: ClerkUser): Promise<boolean> {
      this.isLoading = true;

      // set ID and Email from Clerk user initially to use in middleware
      this.setEmail(clerk_user.email);
      this.setUserId(clerk_user.id);

      // Load from cache initially for fast page load
      const cachedData = cache.get<UserState>(CacheKeys.USER);
      if (cachedData) {
        Object.assign(this.$state, cachedData);
      }

      try {
        const response = await api.get<UserDataResponse>(API.GET_USER_DATA);

        if (response.status !== 200) {
          throw new Error(
            `Failed to fetch user data, status: ${response.status}`,
          );
        }

        const data = response.data;

        // Reset all stores to ensure clean state
        this.$reset();
        const linksStore = useLinksStore();
        linksStore.$reset();
        const settingsStore = useUserSettingsStore();
        settingsStore.$reset();

        // Update stores with fresh data from DB
        if (data.user) {
          this.setEmail(data.user.email);
          this.setUserId(data.user.id);
          this.setFirstName(clerk_user.firstName);
          this.setLastName(clerk_user.lastName);

          // Store the auth token
          if (data.user.auth_token) {
            this.setAuthToken(data.user.auth_token);
          }
        }

        if (data.plan) {
          this.setPlan(data.plan);
        }

        if (data.links) {
          linksStore.$patch({ links: data.links });
          cache.set(CacheKeys.LINKS, data.links);
        } else {
          cache.clear(CacheKeys.LINKS);
        }

        if (data.settings?.settings_blob) {
          settingsStore.$patch({ settings: data.settings.settings_blob });
          cache.set(CacheKeys.SETTINGS, data.settings.settings_blob);
        } else {
          cache.clear(CacheKeys.SETTINGS);
        }

        // Update user cache with latest state
        cache.set(CacheKeys.USER, this.$state);
        return true;
      } catch (error) {
        // On error, clear all caches and reset stores
        this.$reset();
        cache.clear(CacheKeys.USER);
        cache.clear(CacheKeys.LINKS);
        cache.clear(CacheKeys.SETTINGS);
        this.error = error as string;
        return false;
      } finally {
        this.isLoading = false;
      }
    },

    async confirmSubscription(): Promise<boolean> {
      if (!this.userId || !this.email) {
        throw new Error("User ID or email not found");
      }

      const response = await api.get(API.CONFIRM_SUBSCRIPTION);
      if (response.status !== 200) {
        throw new Error(
          `Failed to confirm subscription, status: ${response.status}`,
        );
      }

      return true;
    },

    setUserId(userId: string) {
      this.userId = userId;
    },

    setFirstName(firstname: string) {
      this.firstName = firstname;
    },

    setLastName(lastName: string) {
      this.lastName = lastName;
    },

    setEmail(email: string) {
      this.email = email;
    },

    setPlan(plan: Subscription) {
      this.userPlan = plan;
    },

    setAuthToken(token: string) {
      this.auth_token = token;
    },

    getAuthToken(): string | null {
      return this.auth_token;
    },
  },
});

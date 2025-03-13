import { API } from "@/constants/api";
import api from "@/services/api";
import type { Link } from "@/types/Link";
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
     * Fetches user data from the cache if available
     * @param clerk_user - The Clerk user object containing authentication details.
     * @returns true if data was retrieved from cache, false otherwise
     */
    fetchUserDataFromCache(clerk_user: ClerkUser): boolean {
      // set ID and Email from Clerk user initially to use in middleware
      this.setEmail(clerk_user.email);
      this.setUserId(clerk_user.id);

      // Load from cache for fast page load
      const cachedData = cache.get<UserState>(CacheKeys.USER);
      if (cachedData) {
        Object.assign(this.$state, cachedData);

        // Also load settings and links from cache if available
        const linksStore = useLinksStore();
        const settingsStore = useUserSettingsStore();

        const cachedLinks: Link[] | null = cache.get(CacheKeys.LINKS);
        if (cachedLinks) {
          linksStore.$patch({ links: cachedLinks });
        }

        const cachedSettings = cache.get(CacheKeys.SETTINGS);
        if (cachedSettings) {
          settingsStore.$patch({ settings: cachedSettings });
        }

        return true;
      }

      return false;
    },

    /**
     * Fetches user data from the API and updates the store state.
     * This includes the user record, subscription record, and plan record.
     * @param clerk_user - The Clerk user object containing authentication details.
     * @returns true if the user data was successfully fetched, false otherwise.
     * @throws Error if the user data could not be fetched.
     */
    async fetchUserDataFromServer(clerk_user: ClerkUser): Promise<boolean> {
      this.isLoading = true;

      try {
        const response = await api.get<UserDataResponse>(API.GET_USER_DATA);

        if (response.status !== 200) {
          throw new Error(
            `Failed to fetch user data, status: ${response.status}`,
          );
        }

        const data = response.data;
        const linksStore = useLinksStore();
        const settingsStore = useUserSettingsStore();

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
        }

        if (data.settings?.settings_blob) {
          settingsStore.$patch({ settings: data.settings.settings_blob });
          cache.set(CacheKeys.SETTINGS, data.settings.settings_blob);
        }

        // Update user cache with latest state
        cache.set(CacheKeys.USER, this.$state);
        return true;
      } catch (error) {
        this.error = error as string;
        return false;
      } finally {
        this.isLoading = false;
      }
    },

    /**
     * Legacy method that combines cache and server operations
     * @param clerk_user - The Clerk user object
     * @returns true if the data was successfully fetched
     */
    async fetchUserData(clerk_user: ClerkUser): Promise<boolean> {
      const gotCachedData = this.fetchUserDataFromCache(clerk_user);

      if (!gotCachedData) {
        // If no cache data, we must wait for the server data
        return await this.fetchUserDataFromServer(clerk_user);
      }

      // If we had cache data, still fetch from server but don't wait
      this.fetchUserDataFromServer(clerk_user).catch((error) => {
        console.error("Error fetching user data from server:", error);
      });

      return true;
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

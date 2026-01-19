import { API } from "@/constants/api";
import api from "@/services/api";
import type { Link } from "@/types/Link";
import type { AuthUser, UserDataResponse, UserState } from "@/types/User";
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
    isLoading: false,
    error: null,
    auth_token: null,
  }),

  actions: {
    /**
     * Fetches user data from the cache if available
     * @param authUser - The authenticated user object containing id and email.
     * @returns true if data was retrieved from cache, false otherwise
     */
    fetchUserDataFromCache(authUser: AuthUser): boolean {
      // set ID and Email from auth user initially to use in middleware
      this.setEmail(authUser.email);
      this.setUserId(authUser.id);

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
          const settings =
            typeof cachedSettings === "string" ? JSON.parse(cachedSettings) : cachedSettings;
          settingsStore.$patch({ settings });
        }

        return true;
      }

      return false;
    },

    /**
     * Fetches user data from the API and updates the store state.
     * This includes the user record and user settings record.
     * @param authUser - The authenticated user object containing id and email.
     * @returns true if the user data was successfully fetched, false otherwise.
     * @throws Error if the user data could not be fetched.
     */
    async fetchUserDataFromServer(_authUser: AuthUser): Promise<boolean> {
      this.isLoading = true;

      try {
        const response = await api.get<UserDataResponse>(API.GET_USER_DATA);

        if (response.status !== 200) {
          throw new Error(`Failed to fetch user data, status: ${response.status}`);
        }

        const data = response.data;
        const linksStore = useLinksStore();
        const settingsStore = useUserSettingsStore();

        // Update stores with fresh data from DB
        if (data.user) {
          this.setEmail(data.user.email);
          this.setUserId(data.user.id);

          // Store the auth token
          if (data.user.auth_token) {
            this.setAuthToken(data.user.auth_token);
          }
        }

        if (data.links) {
          linksStore.$patch({ links: data.links });
          cache.set(CacheKeys.LINKS, data.links);
        }

        if (data.settings?.settings_blob) {
          const settings =
            typeof data.settings.settings_blob === "string"
              ? JSON.parse(data.settings.settings_blob)
              : data.settings.settings_blob;
          settingsStore.$patch({ settings });
          cache.set(CacheKeys.SETTINGS, settings);
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
     * Combines cache and server operations for fetching user data
     * @param authUser - The authenticated user object
     * @returns true if the data was successfully fetched
     */
    async fetchUserData(authUser: AuthUser): Promise<boolean> {
      const gotCachedData = this.fetchUserDataFromCache(authUser);

      if (!gotCachedData) {
        // If no cache data, we must wait for the server data
        return await this.fetchUserDataFromServer(authUser);
      }

      // If we had cache data, still fetch from server but don't wait
      this.fetchUserDataFromServer(authUser).catch((error) => {
        console.error("Error fetching user data from server:", error);
      });

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

    setAuthToken(token: string) {
      this.auth_token = token;
    },

    getAuthToken(): string | null {
      return this.auth_token;
    },

    /**
     * Clears all user state on logout
     */
    clearUser(): void {
      this.userId = null;
      this.firstName = null;
      this.lastName = null;
      this.email = null;
      this.isLoading = false;
      this.error = null;
      this.auth_token = null;

      // Clear all cached data
      cache.clear(CacheKeys.USER);
      cache.clear(CacheKeys.LINKS);
      cache.clear(CacheKeys.SETTINGS);
    },
  },
});

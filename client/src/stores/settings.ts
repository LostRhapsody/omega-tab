import { API } from "@/constants/api";
import api from "@/services/api";
import type { UserSettings } from "@/types/UserSettings";
import { CacheKeys, cache } from "@/utils/cache";
import { defineStore } from "pinia";
import { useUserStore } from "./user";

export const useUserSettingsStore = defineStore("userSettings", {
  state: () => ({
    settings: {
      search_history: false,
      autosuggest: false,
      jira_api: false,
      confluence_api: false,
      linear_api: false,
      new_tabs: false,
      metadata: false,
    } as UserSettings,
  }),
  actions: {
    async updateSetting(key: keyof UserSettings, value: boolean) {
      console.log("Updating setting:", key, value);
      this.settings[key] = value;
      console.log("Settings:", this.settings);
      try {
        const userStore = useUserStore();
        if (!userStore.userId) return;
        console.log("User ID:", userStore.userId);
        await api.put(API.UPDATE_SETTINGS, this.settings);
        console.log("Settings updated:", this.settings);
        cache.set(CacheKeys.SETTINGS, this.settings);
        console.log("Settings cached:", cache.get(CacheKeys.SETTINGS));
      } catch (error) {
        console.error("Failed to update settings:", error);
      }
    },

    async fetchSettings() {
      // Only fetch if not already populated by user store
      if (Object.values(this.settings).every((val) => val === false)) {
        const cachedSettings = cache.get<UserSettings>(CacheKeys.SETTINGS);
        if (cachedSettings) {
          this.settings =
            typeof cachedSettings === "string" ? JSON.parse(cachedSettings) : cachedSettings;
          return;
        }

        try {
          const userStore = useUserStore();
          if (!userStore.userId) return;
          const response = await api.get(API.GET_SETTINGS);
          const settingsBlob = response.data.settings_blob;
          this.settings =
            typeof settingsBlob === "string" ? JSON.parse(settingsBlob) : settingsBlob;
          cache.set(CacheKeys.SETTINGS, this.settings);
        } catch (error) {
          console.error("Failed to fetch settings:", error);
        }
      }
    },

    async createSettings() {
      try {
        const userStore = useUserStore();
        if (!userStore.userId) return;
        await api.post(API.CREATE_SETTINGS, this.settings);
        cache.set(CacheKeys.SETTINGS, this.settings);
      } catch (error) {
        console.error("Failed to create settings:", error);
      }
    },
  },
});

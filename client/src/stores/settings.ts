import { API } from "@/constants/api";
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
    } as UserSettings,
  }),
  actions: {
    async updateSetting(key: keyof UserSettings, value: boolean) {
      this.settings[key] = value;
      console.log("Updated setting:", key, value);
      try {
        const userStore = useUserStore();
        if (!userStore.userId) return;
        await fetch(API.UPDATE_SETTINGS(userStore.userId), {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.settings),
        });
        cache.set(CacheKeys.SETTINGS, this.settings);
      } catch (error) {
        console.error("Failed to update settings:", error);
      }
    },

    async fetchSettings() {
      const cachedSettings = cache.get<UserSettings>(CacheKeys.SETTINGS);
      if (cachedSettings) {
        this.settings = cachedSettings;
      }

      try {
        const userStore = useUserStore();
        if (!userStore.userId) return;
        const response = await fetch(API.GET_SETTINGS(userStore.userId));
        const settings = await response.json();
        this.settings = settings.settings_blob;
        cache.set(CacheKeys.SETTINGS, settings.settings_blob);
      } catch (error) {
        console.error("Failed to fetch settings:", error);
      }
    },

    async createSettings() {
      try {
        const userStore = useUserStore();
        if (!userStore.userId) return;
        await fetch(API.CREATE_SETTINGS(userStore.userId), {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(this.settings),
        });
        cache.set(CacheKeys.SETTINGS, this.settings);
      } catch (error) {
        console.error("Failed to create settings:", error);
      }
    },
  },
});

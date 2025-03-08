import { API } from "@/constants/api";
import api from "@/services/api";
import { useUserSettingsStore } from "@/stores/settings";
import { useUserStore } from "@/stores/user";
import type { CreateLinkRequest, Link, UpdateLinkRequest } from "@/types/Link";
import { CacheKeys, cache } from "@/utils/cache";
import type { AxiosError } from "axios";
import { defineStore } from "pinia";

// Define shortcut mapping for columns
export const SHORTCUT_MAPPINGS = [
  { key: "ctrl", label: "Ctrl" }, // First column
  { key: "ctrlalt", label: "Ctrl+Alt" }, // Second column
];

interface LinksState {
  links: Link[];
  isLoading: boolean;
  error: string | null;
}

export const useLinksStore = defineStore("links", {
  state: (): LinksState => ({
    links: [],
    isLoading: false,
    error: null,
  }),

  getters: {
    uniqueColumnTypes: (state) => {
      const columnTypes = new Set(state.links.map((link) => link.column_type));
      return Array.from(columnTypes);
    },
    getColumnShortcut: (state) => (columnType: string) => {
      const columnIndex = state.uniqueColumnTypes.indexOf(columnType);
      if (columnIndex >= 0 && columnIndex < SHORTCUT_MAPPINGS.length) {
        return SHORTCUT_MAPPINGS[columnIndex].label;
      }
      return "";
    },
  },

  actions: {
    async fetchLinks(userId: string) {
      // Only fetch if we don't have data already
      if (this.links.length === 0) {
        // Load from cache first
        const cachedLinks = cache.get<Link[]>(CacheKeys.LINKS);
        if (cachedLinks) {
          this.links = cachedLinks;
          this.isLoading = false;
          return;
        }

        // Then fetch fresh data
        this.isLoading = true;
        try {
          const response = await api.get(API.GET_USER_LINKS);
          switch (response.status) {
            case 200: {
              const links = response.data;
              this.links = links;
              cache.set(CacheKeys.LINKS, links);
              break;
            }
            default: {
              throw new Error("Failed to fetch user link data");
            }
          }
        } catch (error) {
          this.error = error as string;
          throw new Error("Failed to fetch user link data");
        } finally {
          this.isLoading = false;
        }
      }
    },

    async postLink(link: CreateLinkRequest) {
      this.isLoading = true;
      const userStore = useUserStore();
      const authToken = userStore.getAuthToken();
      const settingsStore = useUserSettingsStore();
      const metadata_on = settingsStore.settings.metadata;

      // Only proceed if we have an auth token
      if (!authToken) {
        console.warn("No auth token available for create link");
        return;
      }

      try {
        const response = await api.post(API.CREATE_LINK, link, {
          headers: {
            "X-User-Authorization": authToken,
            "X-Fetch-Metadata": metadata_on,
          },
        });
        if (response.status !== 201) {
          throw new Error(`Failed to create link ${response.status}`);
        }
        const newLink = response.data as Link;
        if (!this.isLink(newLink)) {
          throw new Error("Invalid link data");
        }
        this.addLink(newLink);
        cache.set(CacheKeys.LINKS, this.links);
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        if ((error as AxiosError).status === 502) {
          return 502;
        }
        return false;
      } finally {
        this.isLoading = false;
      }
      return true;
    },

    addLink(link: Link) {
      this.links = [...this.links, link];
    },

    async removeLink(linkId: string) {
      this.isLoading = true;
      this.links = this.links.filter((link) => link.id !== linkId);
      try {
        const response = await api.delete(API.DELETE_LINK(linkId));
        if (response.status !== 200) {
          throw new Error(`Failed to delete link ${response.status}`);
        }
        cache.set(CacheKeys.LINKS, this.links);
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        return false;
      } finally {
        this.isLoading = false;
      }

      return true;
    },

    async updateLink(link: Link) {
      this.isLoading = true;
      this.links = this.links.map((l) => (l.id === link.id ? link : l));
      // because all the fields are technically optional on the backend,
      // use a new type to keep it simple
      const updateLink: UpdateLinkRequest = {
        id: link.id,
        url: link.url,
        description: link.description,
        title: link.title,
        icon: link.icon,
        column_type: link.column_type,
      };
      /*
        if 200, link was updated, nothing else to do
        if not 200, something went wrong
      */
      try {
        const response = await api.put(API.UPDATE_LINK, updateLink);
        if (response.status !== 200) {
          throw new Error(`Failed to update link ${response.status}`);
        }
        cache.set(CacheKeys.LINKS, this.links);
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        return false;
      } finally {
        this.isLoading = false;
      }
      return true;
    },

    isLink(obj: Link): obj is Link {
      return (
        typeof obj === "object" &&
        obj !== null &&
        typeof obj.id === "string" &&
        typeof obj.title === "string" &&
        typeof obj.url === "string" &&
        (typeof obj.icon === "string" || obj.icon === null) &&
        typeof obj.order_index === "number" &&
        typeof obj.owner_type === "string" &&
        typeof obj.owner_id === "string" &&
        typeof obj.created_at === "string" &&
        (typeof obj.description === "string" || obj.description === null) &&
        typeof obj.column_type === "string"
      );
    },

    validateUrl(url: string): boolean | string {
      const urlPattern =
        /^(https?:\/\/)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*\/?(\?.*)?$/;
      return urlPattern.test(url) ? true : "Please enter a valid URL";
    },
  },
});

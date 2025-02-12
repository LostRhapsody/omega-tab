import { API } from "@/constants/api";
import type { CreateLinkRequest, Link, UpdateLinkRequest } from "@/types/Link";
import { CacheKeys, cache } from "@/utils/cache";
import { defineStore } from "pinia";

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
    toolLinks: (state) =>
      state.links.filter((link) => link.column_type === "tools"),
    docLinks: (state) =>
      state.links.filter((link) => link.column_type === "docs"),
    uniqueColumnTypes: (state) => {
      const columnTypes = new Set(state.links.map((link) => link.column_type));
      return Array.from(columnTypes);
    },
  },

  actions: {
    async fetchLinks(userId: string) {
      // Load from cache first
      const cachedLinks = cache.get<Link[]>(CacheKeys.LINKS);
      if (cachedLinks) {
        this.links = cachedLinks;
        this.isLoading = false;
      }

      // Then fetch fresh data
      this.isLoading = true;
      try {
        const response = await fetch(API.GET_USER_LINKS(userId));
        /*
                if 200, set user link data
                    200 will return empty array if no links, so no other real errors
                Any other error is a 500 basically
                */
        switch (response.status) {
          case 200: {
            const links = await response.json();
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
    },

    async postLink(link: CreateLinkRequest) {
      this.isLoading = true;
      try {
        const response = await fetch(API.CREATE_LINK, {
          headers: {
            "Content-Type": "application/json",
          },
          method: "POST",
          body: JSON.stringify(link),
        });
        if (response.status !== 201) {
          throw new Error(`Failed to create link ${response.status}`);
        }
        const newLink = (await response.json()) as Link;
        if (!this.isLink(newLink)) {
          throw new Error("Invalid link data");
        }
        this.addLink(newLink);
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

    addLink(link: Link) {
      this.links = [...this.links, link];
    },

    async removeLink(linkId: string) {
      this.isLoading = true;
      this.links = this.links.filter((link) => link.id !== linkId);
      try {
        const response = await fetch(API.DELETE_LINK(linkId), {
          method: "DELETE",
        });
        if (!response.ok) {
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
        const response = await fetch(API.UPDATE_LINK, {
          headers: {
            "Content-Type": "application/json",
          },
          method: "PUT",
          body: JSON.stringify(updateLink),
        });
        if (!response.ok) {
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

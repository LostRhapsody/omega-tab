import { defineStore } from "pinia";
import type { Link, CreateLinkRequest } from "@/types/Link";
import { API } from "@/constants/api";

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
	},

	actions: {
		async fetchLinks(userId: string) {
			this.isLoading = true;
			// load link data
			try {
				const response = await fetch(API.GET_USER_LINKS(userId));
				/*
                if 200, set user link data
                    200 will return empty array if no links, so no other real errors
                Any other error is a 500 basically
                */
				switch (response.status) {
					case 200: {
                        this.links = await response.json();
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
                const newLink = await response.json() as Link;
                if(!this.isLink(newLink)) {
                    throw new Error("Invalid link data");
                }
                this.addLink(newLink);
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

		removeLink(linkId: string) {
			this.links = this.links.filter((link) => link.id !== linkId);
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
        }
	},
});

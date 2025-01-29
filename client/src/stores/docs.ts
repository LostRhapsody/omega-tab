import { defineStore } from "pinia";
import type { Tables } from "../types/Database";

interface LinkState {
	links: Tables<"links">[];
}

export const useDocStore = defineStore("link", {
	state: (): LinkState => ({
		links: [],
	}),

	actions: {
		setLinks(links: Tables<"links">[]) {
			this.links = links;
		},

		addLink(link: Tables<"links">) {
			this.links.push(link);
		},

		removeLink(linkId: string) {
			this.links = this.links.filter((link) => link.id !== linkId);
		},

		updateLink(updatedLink: Tables<"links">) {
			const index = this.links.findIndex((link) => link.id === updatedLink.id);
			if (index !== -1) {
				this.links[index] = updatedLink;
			}
		},
	},
});

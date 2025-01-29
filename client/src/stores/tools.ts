import { defineStore } from "pinia";
import type { Tables } from "../types/Database";

interface LinkState {
	links: Tables<"links">[];
}

export const useToolStore = defineStore("link", {
	state: (): LinkState => ({
		links: [],
	}),

	actions: {
		sortLinks() {
			this.links.sort((a, b) => a.order_index - b.order_index);
		},

		setLinks(links: Tables<"links">[]) {
			this.links = links;
			this.sortLinks();
		},

		addLink(link: Tables<"links">) {
			this.links.push(link);
			this.sortLinks();
		},

		removeLink(linkId: string) {
			this.links = this.links.filter((link) => link.id !== linkId);
			this.sortLinks();
		},

		updateLink(updatedLink: Tables<"links">) {
			const index = this.links.findIndex((link) => link.id === updatedLink.id);
			if (index !== -1) {
				this.links[index] = updatedLink;
				this.sortLinks();
			}
		},
	},
});

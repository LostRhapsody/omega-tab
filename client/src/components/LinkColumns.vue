<template>
	<div class="link-card-grid">
		<div>
			<h2 class="text-xl">Tools</h2>
			<LinkCard v-for="(tool, index) in tools" :key="tool.order_index" :icon="tool.icon ?? ''" :title="tool.title"
				:description="tool.description ?? ''" :link="tool.url" :index="index" :shortcut="ctrl" class="mb-2"
				:onDelete="() => handleDeleteLink(tools[index])" :onEdit="() => handleEditLink(tools[index])" />
			<AddLinkCard v-if="canAddLinks" :columnType="'tools'" :tools="props.tools" :docs="props.docs"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<div>
			<h2 class="text-xl">Docs</h2>
			<LinkCard v-for="(doc, index) in docs" :key="doc.order_index" :icon="doc.icon ?? ''" :title="doc.title"
				:description="doc.description ?? ''" :link="doc.url" :index="index" :shortcut="alt" class="mb-2"
				:onDelete="() => handleDeleteLink(docs[index])" :onEdit="() => handleEditLink(docs[index])" />
			<AddLinkCard v-if="canAddLinks" :columnType="'docs'" :tools="props.tools" :docs="props.docs"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<EditLinkModal v-model="showEditModal" :link="editingLink" />
	</div>
</template>

<script setup lang="ts">
	import { defineProps, onMounted, onUnmounted, ref } from "vue";
	import AddLinkCard from "./AddLinkCard.vue";
	import EditLinkModal from "./EditLinkModal.vue";
	import LinkCard from "./LinkCard.vue";
	import type { Link } from "../types/Link";
	import { useLinksStore } from "../stores/links";
	const linkStore = useLinksStore();

	const ctrl = "ctrl";
	const alt = "alt";
	const showEditModal = ref(false);
	const editingLink = ref<Link | undefined>();

	const props = defineProps<{
		tools: Link[];
		docs: Link[];
		canAddLinks?: boolean;
		userId: string | null;
		maxPins: number;
		isPlanFree: boolean;
	}>();

	const handleDeleteLink = async (link: Link) => linkStore.removeLink(link.id);

	const handleEditLink = (link: Link) => {
		editingLink.value = link;
		showEditModal.value = true;
	};

	const handleKeydown = (event: KeyboardEvent) => {
		if (event.ctrlKey) {
			const index = Number.parseInt(event.key) - 1;
			if (index >= 0 && index < props.tools.length) {
				window.open(props.tools[index].url, "_blank");
			}
		} else if (event.altKey) {
			const index = Number.parseInt(event.key) - 1;
			if (index >= 0 && index < props.docs.length) {
				window.open(props.docs[index].url, "_blank");
			}
		}
	};

	onMounted(() => {
		window.addEventListener("keydown", handleKeydown);
	});

	onUnmounted(() => {
		window.removeEventListener("keydown", handleKeydown);
	});
</script>

<style scoped>
	.link-card-grid {
		display: grid;
		grid-template-columns: 1fr 1fr;
		margin-top: 3rem;
		gap: 2rem;
	}
</style>
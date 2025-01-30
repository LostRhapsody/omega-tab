<template>
	<div class="link-card-grid">
		<div>
			<h2 class="text-xl">Tools</h2>
			<LinkCard v-for="(tool, index) in tools" :key="tool.order_index" :icon="tool.icon ?? ''" :title="tool.title"
				:description="tool.description ?? ''" :link="tool.url" :index="index" :shortcut="ctrl" class="mb-2"
				:onDelete="() => handleDeleteLink('tool', index)" :onEdit="() => handleEditLink(tools[index])" />
			<AddLinkCard v-if="canAddLinks" :columnType="'tools'" :tools="props.tools" :docs="props.docs"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<div>
			<h2 class="text-xl">Docs</h2>
			<LinkCard v-for="(doc, index) in docs" :key="doc.order_index" :icon="doc.icon ?? ''" :title="doc.title"
				:description="doc.description ?? ''" :link="doc.url" :index="index" :shortcut="alt" class="mb-2"
				:onDelete="() => handleDeleteLink('doc', index)" :onEdit="() => handleEditLink(docs[index])" />
			<AddLinkCard v-if="canAddLinks" :columnType="'docs'" :tools="props.tools" :docs="props.docs"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
	</div>
	<EditLinkModal v-model="showEditModal" :link="editingLink" @linkUpdated="handleLinkUpdated" />
</template>

<script setup lang="ts">
	import { defineProps, onMounted, onUnmounted, ref } from "vue";
	import { useApi } from "../composables/useApi";
	import type { Tables } from "../types/Database";
	import AddLinkCard from "./AddLinkCard.vue";
	import EditLinkModal from "./EditLinkModal.vue";
	import LinkCard from "./LinkCard.vue";
	type Link = Tables<"links">;
	const { api } = useApi();

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

	const emit = defineEmits<(e: "linkDeleted", type: string, index: number) => void>();

	const handleDeleteLink = async (type: string, index: number) => {
		if (type === "tool") {
			if (
				confirm(
					`Are you sure you want to delete the link "${props.tools[index].title}"?`,
				)
			) {
				await api(`/link/${props.tools[index].id}`, { method: "DELETE" });
				emit("linkDeleted", "tool", index); // Emit event
			}
		} else {
			if (
				confirm(
					`Are you sure you want to delete the link "${props.docs[index].title}"?`,
				)
			) {
				await api(`/link/${props.docs[index].id}`, { method: "DELETE" });
				emit("linkDeleted", "doc", index); // Emit event
			}
		}
	};

	const handleEditLink = (link: Link) => {
		editingLink.value = link;
		showEditModal.value = true;
	};

	const handleLinkUpdated = async (updatedLink: Link) => {
		const index =
			updatedLink.column_type === "tools"
				? props.tools.findIndex((t) => t.id === updatedLink.id)
				: props.docs.findIndex((d) => d.id === updatedLink.id);

		if (index !== -1) {
			if (updatedLink.column_type === "tools") {
				// Preserve order_index from original array position
				props.tools[index] = {
					...updatedLink,
					order_index: props.tools[index].order_index,
				};
			} else {
				props.docs[index] = {
					...updatedLink,
					order_index: props.docs[index].order_index,
				};
			}
			showEditModal.value = false;
		}
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
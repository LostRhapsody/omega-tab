<template>
	<div class="link-card-grid">
		<div v-if="uniqueColumnTypes.length === 0" class="single-column">
			<AddLinkCard :columnType="'default'" :tools="props.tools" :docs="props.docs" :userId="props.userId"
				:maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<div v-else v-for="columnType in uniqueColumnTypes" :key="columnType" :class="columnClass">
			<h2 class="text-xl">{{ columnType.charAt(0).toUpperCase() + columnType.slice(1) }}</h2>
			<LinkCard v-for="(link, index) in getLinksByColumnType(columnType)" :key="link.order_index"
				:icon="link.icon ?? ''" :title="link.title" :description="link.description ?? ''" :link="link.url"
				:index="index" :shortcut="getShortcut(columnType)" class="mb-2" :onDelete="() => handleDeleteLink(link)"
				:onEdit="() => handleEditLink(link)" />
			<AddLinkCard v-if="canAddLinks" :columnType="columnType" :tools="props.tools" :docs="props.docs"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<EditLinkModal v-model="showEditModal" :link="editingLink" />
	</div>
</template>

<script setup lang="ts">
import { defineProps, onMounted, onUnmounted, ref, computed } from "vue";
import AddLinkCard from "./AddLinkCard.vue";
import EditLinkModal from "./EditLinkModal.vue";
import LinkCard from "./LinkCard.vue";
import type { Link } from "../types/Link";
import { useLinksStore, SHORTCUT_MAPPINGS } from "../stores/links";
const linkStore = useLinksStore();

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

const uniqueColumnTypes = computed(() => linkStore.uniqueColumnTypes);

const getLinksByColumnType = (columnType: string) => {
	return linkStore.links.filter(link => link.column_type === columnType);
};

const getShortcut = (columnType: string) => {
	const columnIndex = uniqueColumnTypes.value.indexOf(columnType);
	if (columnIndex >= 0 && columnIndex < SHORTCUT_MAPPINGS.length) {
		return SHORTCUT_MAPPINGS[columnIndex].label;
	}
	return '';
};

const handleDeleteLink = async (link: Link) => linkStore.removeLink(link.id);

const handleEditLink = (link: Link) => {
	editingLink.value = link;
	showEditModal.value = true;
};

const handleKeydown = (event: KeyboardEvent) => {
	// Only process numeric keys 1-9
	if (!/^[1-9]$/.test(event.key)) return;

	const numKey = Number.parseInt(event.key) - 1;

	// Check which shortcut combination is pressed
	let columnIndex = -1;
	if (event.ctrlKey && event.altKey) {
		columnIndex = 1; // Ctrl+Alt (second column)	
	} else if (event.ctrlKey) {
		columnIndex = 0; // Ctrl (first column)
	}

	// If we have a valid column, try to open the corresponding link
	if (columnIndex >= 0 && columnIndex < uniqueColumnTypes.value.length) {
		const columnType = uniqueColumnTypes.value[columnIndex];
		const links = getLinksByColumnType(columnType);

		if (numKey >= 0 && numKey < links.length) {
			window.open(links[numKey].url, "_blank");
		}
	}
};

onMounted(() => {
	window.addEventListener("keydown", handleKeydown);
});

onUnmounted(() => {
	window.removeEventListener("keydown", handleKeydown);
});

const columnClass = computed(() => {
	const columnCount = uniqueColumnTypes.value.length;
	if (columnCount === 1) return 'single-column';
	if (columnCount === 2) return 'two-columns';
	if (columnCount === 3) return 'three-columns';
	if (columnCount === 4) return 'four-columns';
	return 'grid-column';
});
</script>

<style scoped>
.link-card-grid {
	display: flex;
	flex-wrap: wrap;
	margin-top: 3rem;
	gap: 2rem;
	padding-bottom: 2rem;
	justify-content: center;
}

/* Stylish and modern scroll bar */
::-webkit-scrollbar {
	width: 8px;
	height: 8px;
}

::-webkit-scrollbar-track {
	background: #181818;
	border-radius: 10px;
}

::-webkit-scrollbar-thumb {
	background: rgba(0, 0, 0, 0.3);
	border-radius: 10px;
}

::-webkit-scrollbar-thumb:hover {
	background: rgba(0, 0, 0, 0.2);
}

.single-column {
	flex: 0 0 65%;
	margin: 0 auto;
}

.two-columns {
	flex: 0 0 45%;
}

.three-columns {
	flex: 0 0 30%;
}

.four-columns {
	flex: 0 0 45%;
	min-width: 20rem;
	max-width: 30rem;
}

.grid-column {
	flex: 0 0 30%;
	min-width: 20rem;
	max-width: 30rem;
}

/* For small screens, stack all columns */
@media (max-width: 768px) {
	.link-card-grid {
		flex-direction: column;
	}

	.link-card-grid>div {
		flex: 0 0 100%;
		width: 90%;
		margin: 0 auto;
	}
}

/* For medium screens, show 2 columns max per row */
@media (min-width: 769px) and (max-width: 1200px) {

	.grid-column,
	.four-columns {
		flex: 0 0 45%;
	}
}
</style>
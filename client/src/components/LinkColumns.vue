<template>
	<div class="link-card-grid">
		<div v-if="uniqueColumnTypes.length === 0" class="single-column">
			<AddLinkCard :columnType="'default'" :userId="props.userId"
				:maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<div v-else v-for="columnType in uniqueColumnTypes" :key="columnType" :class="columnClass">
			<h2 class="text-xl">{{ columnType.charAt(0).toUpperCase() + columnType.slice(1) }}</h2>
			<LinkCard v-for="(link, index) in getLinksByColumnType(columnType)" :key="link.order_index"
				:icon="link.icon ?? ''" :title="link.title" :description="link.description ?? ''" :link="link.url"
				:index="index" :shortcut="getShortcut(columnType)" class="mb-2" :onDelete="() => handleDeleteLink(link)"
				:onEdit="() => handleEditLink(link)" :ref="el => { if (el) linkRefs.push(el) }" 
				:tabindex="getFocusableIndex(columnType, index)" 
				:data-column="columnType" 
				:data-link-index="index" />
			<AddLinkCard v-if="canAddLinks" :columnType="columnType"
				:userId="props.userId" :maxPins="props.maxPins" :isPlanFree="isPlanFree" />
		</div>
		<EditLinkModal v-model="showEditModal" :link="editingLink" />
	</div>
</template>

<script setup lang="ts">
import { defineProps, onMounted, onUnmounted, ref, computed, nextTick } from "vue";
import AddLinkCard from "./AddLinkCard.vue";
import EditLinkModal from "./EditLinkModal.vue";
import LinkCard from "./LinkCard.vue";
import type { Link } from "../types/Link";
import { useLinksStore, SHORTCUT_MAPPINGS } from "../stores/links";
const linkStore = useLinksStore();

const showEditModal = ref(false);
const editingLink = ref<Link | undefined>();
const linkRefs = ref<any[]>([]);
const currentFocus = ref<{ columnType: string, index: number } | null>(null);

const props = defineProps<{
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

const getFocusableIndex = (columnType: string, index: number) => {
	// Make link cards focusable for keyboard navigation
	return 0;
};

const handleDeleteLink = async (link: Link) => linkStore.removeLink(link.id);

const handleEditLink = (link: Link) => {
	editingLink.value = link;
	showEditModal.value = true;
};

const isSearchInputFocused = () => {
	const activeElement = document.activeElement;
	return activeElement && (
		activeElement.tagName === 'TEXTAREA' ||
		activeElement.classList.contains('searchBar')
	);
};

const focusLinkCard = (columnType: string, index: number) => {
	nextTick(() => {
		// Reset refs array before getting fresh references
		linkRefs.value = [];
		nextTick(() => {
			const targetLink = linkRefs.value.find(
				ref => ref.$el.dataset.column === columnType && 
				parseInt(ref.$el.dataset.linkIndex) === index
			);
			
			if (targetLink && targetLink.$el) {
				const anchorElement = targetLink.$el.querySelector('a');
				if (anchorElement) {
					anchorElement.focus();
				} else {
					targetLink.$el.focus(); // Fallback to div if anchor not found
				}
				currentFocus.value = { columnType, index };
			}
		});
	});
};

const handleArrowKeys = (event: KeyboardEvent) => {
	// Skip if search input is focused or if modifiers are pressed
	if (isSearchInputFocused() || event.ctrlKey || event.altKey || event.metaKey) {
		return;
	}
	
	if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') {
		return;
	}
	
	event.preventDefault();
	
	// If no current focus, focus the first link in the first column
	if (!currentFocus.value) {
		if (uniqueColumnTypes.value.length > 0) {
			const firstColumnType = uniqueColumnTypes.value[0];
			const columnLinks = getLinksByColumnType(firstColumnType);
			if (columnLinks.length > 0) {
				focusLinkCard(firstColumnType, 0);
			}
		}
		return;
	}
	
	const { columnType, index } = currentFocus.value;
	const columnLinks = getLinksByColumnType(columnType);
	
	if (event.key === 'ArrowDown') {
		// Move down to next link in same column
		if (index < columnLinks.length - 1) {
			focusLinkCard(columnType, index + 1);
		}
	} else if (event.key === 'ArrowUp') {
		// Move up to previous link in same column
		if (index > 0) {
			focusLinkCard(columnType, index - 1);
		}
	}
};

onMounted(() => {
	window.addEventListener("keydown", handleKeydown);
	window.addEventListener("keydown", handleArrowKeys);
	
	// Reset link refs whenever links change
	linkRefs.value = [];
});

onUnmounted(() => {
	window.removeEventListener("keydown", handleKeydown);
	window.removeEventListener("keydown", handleArrowKeys);
});

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

/* Style for focused link cards */
:deep(.link-card:focus) {
  outline: 2px solid #4a9df8;
  outline-offset: 2px;
}
</style>
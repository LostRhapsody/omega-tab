<!-- AddLinkCard.vue -->
<template>
	<div v-if="!isAtMaxPins || (isAtMaxPins && isPlanFree)" @click="handleClick"
		class="group cursor-pointer border-2 rounded-lg p-4 transition-all duration-300 flex flex-col items-center justify-center space-y-2"
		:class="[
			isAtMaxPins
				? 'border-amber-500 bg-amber-50 hover:bg-amber-100'
				: 'border-dashed hover:border-primary border-gray-300'
		]" @mouseenter="hover = true" @mouseleave="hover = false">
		<v-icon :color="isAtMaxPins ? 'amber-darken-2' : (hover ? 'primary' : 'grey')" size="24">
			{{ isAtMaxPins ? 'mdi-arrow-up-circle' : 'mdi-plus' }}
		</v-icon>
		<span :class="isAtMaxPins ? 'text-amber-700' : (hover ? 'text-primary' : 'text-grey')">
			{{ isAtMaxPins ? 'Upgrade for more pins' : 'Add new link' }}
		</span>
	</div>

	<v-dialog v-model="isModalOpen" width="500">
		<v-card>
			<v-card-title>Add New Link</v-card-title>

			<v-card-text>
				<v-form @submit.prevent="handleSubmit" ref="form">
					<v-text-field v-model="formData.url" :rules="[v => !!v || 'URL is required', validateUrl]"
						label="URL" required type="url" @keyup.enter="handleSubmit"></v-text-field>

					<v-text-field @keyup.enter="(e: Event) => { e.preventDefault(); handleSubmit() }"
						v-model="formData.title" label="Title"></v-text-field>

					<v-textarea @keydown.enter.prevent="(e: KeyboardEvent) => {
						if (e.shiftKey && e.target !== null) {
							(e.target as HTMLTextAreaElement).value += '\n'
						} else {
							handleSubmit()
						}
					}" v-model="formData.description" label="Description" rows="3"></v-textarea>
				</v-form>
				<p></p>
			</v-card-text>


			<v-card-actions>
				<div class="text-xs text-gray-500 grid grid-cols-3 gap-2 ps-4 pb-4">
					<span class="col-span-1 text-start">Submit:</span>
					<span class="col-span-2">
						<span class="kbd">enter<v-icon size="18">mdi-keyboard-return</v-icon></span>
					</span>
					<span class="col-span-1 text-start">New line:</span>
					<span class="col-span-2">
						<span class="kbd">shift<v-icon size="18">mdi-arrow-up</v-icon></span> + <span
							class="kbd">enter<v-icon size="18">mdi-keyboard-return</v-icon></span>
					</span>
				</div>
				<v-spacer></v-spacer>
				<v-btn color="grey-darken-1" variant="text" @click="closeModal">
					Cancel
				</v-btn>
				<v-btn color="primary" variant="text" :loading="isLoading" @click="handleSubmit">
					Add Link
				</v-btn>
			</v-card-actions>
		</v-card>
	</v-dialog>
</template>

<script setup lang="ts">
	import { computed, ref } from "vue";
	import { useRouter } from "vue-router";
	import { useLinksStore } from "../stores/links";
	import { useUserStore } from "../stores/user";
	import type {CreateLinkRequest, Link} from "../types/Link";
	const linksStore = useLinksStore();
	const userStore = useUserStore();

	type formData = {
		url: string;
		title: string;
		description: string;
	};

	const props = defineProps<{
		columnType: "tools" | "docs";
		tools: Link[];
		docs: Link[];
		userId: string | null;
		maxPins: number;
		isPlanFree: boolean;
	}>();

	const router = useRouter();
	const isModalOpen = ref(false);
	const isLoading = ref(false);
	const hover = ref(false);
	const form = ref<HTMLFormElement | null>(null);

	const formData = ref({
		url: "",
		title: "",
		description: "",
	});

	const isAtMaxPins = computed(() => {
		return props.tools.length + props.docs.length >= props.maxPins;
	});

	const handleClick = () => {
		if (isAtMaxPins.value) {
			router.push("/plans");
			return;
		}
		openModal();
	};

	const openModal = () => {
		const totalPins = props.tools.length + props.docs.length;
		if (totalPins >= props.maxPins) {
			// Use Vuetify's built-in snackbar or alert system
			alert(
				`You've reached your maximum number of pins (${props.maxPins}). Please upgrade your plan for more.`,
			);
			return;
		}
		isModalOpen.value = true;
	};

	const closeModal = () => {
		isModalOpen.value = false;
		resetForm();
	};

	const resetForm = () => {
		formData.value = {
			url: "",
			title: "",
			description: "",
		};
		if (form.value) {
			form.value.resetValidation();
		}
	};

	const validateUrl = (url: string): boolean | string => {
		const urlPattern = /^(https?:\/\/)?([\da-z.-]+)\.([a-z.]{2,6})([/\w .-]*)*\/?$/;
		return urlPattern.test(url) ? true : "Please enter a valid URL";
	};

	const handleSubmit = async () => {
		if (!form.value) return;

		const { valid } = await form.value.validate();
		if (!valid) return;

		try {
			isLoading.value = true;

			if(!userStore.userId) {
				console.error("User not logged in");
				return;
			}
			
			const linkData: CreateLinkRequest = {
				title: formData.value.title,
				description: formData.value.description,
				url: formData.value.url,
				next_order_index: 
					props.columnType === "tools" ? linksStore.toolLinks.length + 1 : linksStore.docLinks.length + 1,
				owner_id: userStore.userId,
				owner_type: "user",
				column_type: props.columnType,
			};

			const savedLink = await linksStore.postLink(linkData);			
			if(!savedLink) console.error("Error saving link");
			closeModal();
		} catch (error) {
			console.error("Error saving link:", error);
		} finally {
			isLoading.value = false;
		}
	};
</script>
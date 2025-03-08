<!-- AddLinkCard.vue -->
<template>
	<div>
		<div id="add-link-card" v-if="!isAtMaxPins || (isAtMaxPins && isPlanFree)" @click="handleClick"
			class="group cursor-pointer border-2 rounded-lg p-4 transition-all duration-300 flex flex-col items-center justify-center space-y-2"
			:class="[
				isAtMaxPins
					? 'border-amber-500/50 bg-amber-50/50 hover:bg-amber-100/50'
					: 'border-dashed hover:border-primary border-gray-300'
			]" @mouseenter="hover = true" @mouseleave="hover = false">
			<v-icon :color="isAtMaxPins ? 'amber-darken-2' : (hover ? 'primary' : 'grey')" size="24">
				{{ isAtMaxPins ? 'mdi-arrow-up-circle' : 'mdi-plus' }}
			</v-icon>
			<span :class="isAtMaxPins ? 'text-amber-800' : (hover ? 'text-primary' : 'text-grey')">
				{{ isAtMaxPins ? 'Upgrade for more pins' : 'Add new link' }}
			</span>
		</div>

		<v-dialog v-model="isModalOpen" width="500" :fullscreen="mobile">
			<v-card class="px-8 py-8 sm:py-0 sm:px-0">
				<v-card-title>Add New Link </v-card-title>

				<v-card-text>
					<v-form @submit.prevent="handleSubmit" ref="form">
						<v-text-field v-model="formData.url" :rules="[v => !!v || 'URL is required', linksStore.validateUrl]"
							label="URL" required type="url" @keyup.enter="handleSubmit"></v-text-field>
						<p v-if="!validLink" class="text-error my-2 text-sm">Invalid URL: Failed to connect with this URL's server, is it spelled correctly?</p>

						<v-text-field @keyup.enter="(e: Event) => { e.preventDefault(); handleSubmit() }"
							v-model="formData.title" label="Title"></v-text-field>

						<v-textarea @keydown.enter.prevent="(e: KeyboardEvent) => {
							if (e.shiftKey && e.target !== null) {
								(e.target as HTMLTextAreaElement).value += '\n'
							} else {
								handleSubmit()
							}
						}" v-model="formData.description" label="Description" rows="3"></v-textarea>

						<v-select v-model="formData.columnType" :items="columnTypes" label="Column Label" required>
							<template v-slot:append-item>
								<v-divider class="my-4"></v-divider>
								<v-list-item>
									<v-text-field
										v-model="newColumnType"
										label="New Column Type"
										dense
										hide-details
										@keyup.enter="addNewColumnType"
									></v-text-field>
									<v-btn @click="addNewColumnType" color="primary" class="w-full">
										Add
									</v-btn>
								</v-list-item>
							</template>
						</v-select>
					</v-form>
					<p></p>
				</v-card-text>


				<v-card-actions>
							<div :class="mobile ? 'hidden' : 'text-xs text-gray-500 grid grid-cols-3 gap-2 ps-4 pb-4'">
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
							<v-spacer v-if="!mobile"></v-spacer>
							<div :class="mobile ? 'flex flex-col gap-4 w-full' : 'grid grid-rows-2 gap-4'">
								<div :class="mobile ? 'flex justify-around' : ''">
									<v-btn color="grey-darken-1" :variant="mobile ? 'elevated' : 'text' " :size="mobile ? 'x-large' : 'default'" @click="closeModal">
										Cancel
									</v-btn>
									<v-btn color="primary" :variant="mobile ? 'elevated' : 'text'" :size="mobile ? 'x-large' : 'default'" :loading="isLoading" @click="handleSubmit">
										Add Link
									</v-btn>
								</div>
								<div class="flex justify-end">
									<v-tooltip location="left" :z-index="1000" max-width="300" open-on-click>
										<template v-slot:activator="{ props }">
											<v-btn v-bind="props">
												<v-icon size="x-large" icon="mdi-help-circle-outline" class="text-gray-500" />
											</v-btn>
										</template>
										<span>
											<span class="kbd">+Plus Feature</span><br/>
											If title and description are left blank, <strong>Better New Tab</strong> attempts to get them from the website, along
											with an icon, from the URL's website.
										</span>
									</v-tooltip>
							</div>
						</div>
					</v-card-actions>
			</v-card>
		</v-dialog>
	</div>
</template>

<script setup lang="ts">
	import { computed, ref, watch, nextTick } from "vue";
	import { useRouter } from "vue-router";
	import { useLinksStore } from "../stores/links";
	import { useUserStore } from "../stores/user";
	import type { CreateLinkRequest, Link } from "../types/Link";
	import { useDisplay } from 'vuetify';

	const linksStore = useLinksStore();
	const userStore = useUserStore();
	const mobile = useDisplay().smAndDown;
	const validLink = ref(true);

	type formData = {
		url: string;
		title: string;
		description: string;
		columnType: string;
	};

	const props = defineProps<{
		columnType: string;
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
		columnType: props.columnType,
	});

	const newColumnType = ref("");

	const columnTypes = computed(() => {
		return linksStore.uniqueColumnTypes;
	});

	const isAtMaxPins = computed(() => {
		return linksStore.links.length >= props.maxPins;
	});

	const handleClick = () => {
		if (isAtMaxPins.value) {
			router.push("/plans");
			return;
		}
		openModal();
	};

	const openModal = () => {
		if (linksStore.links.length >= props.maxPins) {
			alert(
				`You've reached your maximum number of pins (${props.maxPins}). Please upgrade your plan for more.`,
			);
			return;
		}
		isModalOpen.value = true;
		nextTick(() => {
			const urlField = document.querySelector('input[type="url"]');
			if (urlField) {
				(urlField as HTMLInputElement).focus();
			}
		});
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
			columnType: props.columnType,
		};
		newColumnType.value = "";
		if (form.value) {
			form.value.resetValidation();
		}
	};

	const addNewColumnType = () => {
		if (newColumnType.value && !columnTypes.value.includes(newColumnType.value)) {
			formData.value.columnType = newColumnType.value;
			newColumnType.value = "";
		}
	};

	const handleSubmit = async () => {
		if (!form.value) return;

		const { valid } = await form.value.validate();
		if (!valid) return;

		try {
			isLoading.value = true;

			if (!userStore.userId) {
				console.error("User not logged in");
				return;
			}

			const linkData: CreateLinkRequest = {
				title: formData.value.title,
				description: formData.value.description,
				url: formData.value.url,
				next_order_index: linksStore.links.length + 1,
				owner_id: userStore.userId,
				owner_type: "user",
				column_type: formData.value.columnType,
			};

			const savedLink = await linksStore.postLink(linkData);
			if (savedLink === 502) {
				validLink.value = false;
				return;
			}
			validLink.value = true;
			if (!savedLink) console.error("Error saving link");
			closeModal();
		} catch (error) {
			console.error("Error saving link:", error);
		} finally {
			isLoading.value = false;
		}
	};

	watch(isModalOpen, (newVal) => {
		if (!newVal) {
			// Modal is closed
			if (newColumnType.value) {
				// Clear the new column type if it exists
				newColumnType.value = "";
			}
			
			// Reset the form data if URL is not provided (form not completed)
			if (!formData.value.url) {
				resetForm();
			}
		}
	});
</script>
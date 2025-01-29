# EditLinkModal.vue
<template>
    <v-dialog v-model="isModalOpen" width="500">
        <v-card>
            <v-card-title>Edit Link</v-card-title>

            <v-card-text>
                <v-form @submit.prevent="handleSubmit" ref="form">
                    <v-text-field
                        v-model="formData.url"
                        :rules="[v => !!v || 'URL is required', validateUrl]"
                        label="URL"
                        required
                        type="url"
                    ></v-text-field>

                    <v-text-field
                        v-model="formData.title"
                        label="Title"
                    ></v-text-field>

                    <v-textarea
                        v-model="formData.description"
                        label="Description"
                        rows="3"
                    ></v-textarea>
                </v-form>
            </v-card-text>

            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn
                    color="grey-darken-1"
                    variant="text"
                    @click="closeModal"
                >
                    Cancel
                </v-btn>
                <v-btn
                    color="primary"
                    variant="text"
                    :loading="isLoading"
                    @click="handleSubmit"
                >
                    Save Changes
                </v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { useApi } from "@/composables/useApi";
import { ref, watch } from "vue";
import type { Tables } from "../types/Database";
type Link = Tables<"links">;

const props = defineProps<{
	modelValue: boolean;
	link?: Link;
}>();

const emit = defineEmits<{
	(e: "update:modelValue", value: boolean): void;
	(e: "linkUpdated", link: Link): void;
}>();

const isModalOpen = ref(props.modelValue);
const isLoading = ref(false);
const form = ref<HTMLFormElement | null>(null);

const formData = ref({
	url: "",
	title: "",
	description: "",
});

watch(
	() => props.modelValue,
	(newValue) => {
		isModalOpen.value = newValue;
		if (newValue && props.link) {
			formData.value = {
				url: props.link.url,
				title: props.link.title,
				description: props.link.description || "",
			};
		}
	},
);

watch(
	() => isModalOpen.value,
	(newValue) => {
		emit("update:modelValue", newValue);
	},
);

const validateUrl = (url: string): boolean | string => {
	try {
		new URL(url);
		return true;
	} catch {
		return "Please enter a valid URL";
	}
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

const handleSubmit = async () => {
	if (!form.value || !props.link) return;

	const { valid } = await form.value.validate();
	if (!valid) return;

	try {
		isLoading.value = true;
		const { api } = useApi();
		const response = await api("/link", {
			method: "PUT",
			body: JSON.stringify({
				id: props.link.id,
				url: formData.value.url,
				title: formData.value.title || new URL(formData.value.url).hostname,
				description: formData.value.description,
			}),
		});
		const updatedLink = {
			...props.link,
			url: formData.value.url,
			title: formData.value.title || new URL(formData.value.url).hostname,
			description: formData.value.description,
		};

		if (updatedLink) {
			emit("linkUpdated", updatedLink as Link);
		}
		closeModal();
	} catch (error) {
		console.error("Error updating link:", error);
	} finally {
		isLoading.value = false;
	}
};
</script>
<!-- AddLinkCard.vue -->
<template>
   <div @click="handleClick"
       class="group cursor-pointer border-2 rounded-lg p-4 transition-all duration-300 flex flex-col items-center justify-center space-y-2"
       :class="[
           isAtMaxPins
               ? 'border-amber-500 bg-amber-50 hover:bg-amber-100'
               : 'border-dashed hover:border-primary border-gray-300'
       ]"
       @mouseenter="hover = true"
       @mouseleave="hover = false">
       <v-icon
           :color="isAtMaxPins ? 'amber-darken-2' : (hover ? 'primary' : 'grey')"
           size="24"
       >
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
                        label="URL" required type="url"></v-text-field>

                    <v-text-field v-model="formData.title" label="Title"></v-text-field>

                    <v-textarea v-model="formData.description" label="Description" rows="3"></v-textarea>
                </v-form>
            </v-card-text>

            <v-card-actions>
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
    import { ref,computed } from 'vue';
    import { linkUtils } from '@/composables/useDatabase';
    import type { Tables } from '../types/Database';
    type Link = Tables<'links'>;

    type formData = {
        url: string;
        title: string;
        description: string;
    };

    const props = defineProps<{
        columnType: 'tools' | 'docs';
        tools: Link[];
        docs: Link[];
        userId: string | null;
        maxPins: number;
    }>();

    const emit = defineEmits<(e: 'linkAdded', link: Link) => void>();

    const isModalOpen = ref(false);
    const isLoading = ref(false);
    const hover = ref(false);
    const form = ref<HTMLFormElement | null>(null);

    const formData = ref({
        url: '',
        title: '',
        description: ''
    });

    const isAtMaxPins = computed(() => {
       return props.tools.length + props.docs.length >= props.maxPins;
    });

    const handleClick = () => {
    if (isAtMaxPins.value) {
        // TODO: Navigate to upgrade page or open upgrade modal
        return;
    }
    openModal();
    };


    const openModal = () => {
        const totalPins = props.tools.length + props.docs.length;
        if (totalPins >= props.maxPins) {
            // Use Vuetify's built-in snackbar or alert system
            alert(`You've reached your maximum number of pins (${props.maxPins}). Please upgrade your plan for more.`);
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
            url: '',
            title: '',
            description: ''
        };
        if (form.value) {
            form.value.resetValidation();
        }
    };

    const validateUrl = (url: string): boolean | string => {
        try {
            new URL(url);
            return true;
        } catch {
            return 'Please enter a valid URL';
        }
    };

    const fetchMetadata = async (url: string) => {
        return {favicon:null};
        // biome-ignore lint/correctness/noUnreachable: <explanation>
        try {
            const response = await fetch(`/api/metadata?url=${encodeURIComponent(url)}`);
            const data = await response.json();

            if (!formData.value.title && data.title) {
                formData.value.title = data.title;
            }
            if (!formData.value.description && data.description) {
                formData.value.description = data.description;
            }

            return {
                favicon: data.favicon || null
            };
        } catch (error) {
            console.error('Error fetching metadata:', error);
            return { favicon: null };
        }
    };

    const saveToSupabase = async (linkData: Omit<Link, 'id' | 'created_at'>) => {
        if(!props.userId) return;
        const link = {
            ...linkData,
            owner_id: props.userId,
            owner_type: 'user',
            order_index: props.columnType === 'tools' ? props.tools.length : props.docs.length
        };

        return await linkUtils.createLink(link);
    };

    const handleSubmit = async () => {
        if (!form.value) return;

        const { valid } = await form.value.validate();
        if (!valid) return;

        try {
            isLoading.value = true;
            const metadata = await fetchMetadata(formData.value.url);
            console.log(metadata);

            const linkData: Omit<Link, 'id' | 'created_at'> = {
                title: formData.value.title || new URL(formData.value.url).hostname,
                description: formData.value.description,
                url: formData.value.url,
                icon: metadata.favicon,
                order_index: 0,
                owner_id: '',
                owner_type: '',
                column_type: props.columnType
            };

            const savedLink = await saveToSupabase(linkData);
            if (savedLink) emit('linkAdded', savedLink);
            closeModal();
        } catch (error) {
            console.error('Error saving link:', error);
        } finally {
            isLoading.value = false;
        }
    };
</script>
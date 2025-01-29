<!-- ToolLink.vue -->
<script setup lang="ts">
import { computed, defineProps, ref } from "vue";

interface Props {
	icon?: string;
	title: string;
	description: string;
	link: string;
	index: number;
	shortcut: string;
	onDelete: () => void;
	onEdit: () => void;
}

const props = defineProps<Props>();
const isMdiIcon = computed(() => props.icon?.startsWith("mdi-"));
const snackbar = ref(false);

const copyToClipboard = (text: string) => {
	navigator.clipboard
		.writeText(text)
		.then(() => {
			snackbar.value = true;
		})
		.catch((err) => {
			console.error("Failed to copy: ", err);
		});
};
</script>

<template>
  <v-card
    :href="link"
    target="_blank"
    variant="tonal"
    class="tool-link pa-4 mb-2 d-flex align-center"
    :ripple="false"
  >
    <div class="d-flex align-center flex-grow-1">
      <div v-if="icon !== ''">
        <v-icon
            v-if="isMdiIcon"
            :icon="icon"
            size="36"
            class="mr-4"
            color="primary"
          />
          <img
            v-else
            :src="icon"
            alt=""
            class="custom-icon mr-4"
          />
      </div>
      <div>
        <div class="text-h6">{{ title }}</div>
        <div class="text-body-2 text-medium-emphasis">{{ description }}</div>
      </div>
    </div>
    <v-menu>
      <template v-slot:activator="{ props }">
        <v-btn @click="(e:Event) => {e.preventDefault()}" v-bind="props" icon="mdi-dots-vertical" variant="text" size="small"></v-btn>
      </template>
      <v-list>
        <v-list-item @click="props.onDelete()">
          <v-list-item-title>Delete</v-list-item-title>
        </v-list-item>
        <v-list-item @click="() => copyToClipboard(link)">
          <v-list-item-title>Copy URL</v-list-item-title>
        </v-list-item>
        <v-list-item @click="() => onEdit()">
          <v-list-item-title>Edit</v-list-item-title>
        </v-list-item>
      </v-list>
    </v-menu>
    <v-icon
      icon="mdi-chevron-right"
      size="24"
      class="chevron"
    />
    <div class="shortcut-text">{{ shortcut }}+{{ index + 1 }}</div>
  </v-card>
  <!-- Snackbar -->
  <v-snackbar
      v-model="snackbar"
      :timeout="3000"
    >
      URL Copied to clipboard
      <template v-slot:actions>
        <v-btn
          color="red"
          variant="text"
          @click="snackbar = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>
</template>

<style scoped>
.tool-link {
  text-decoration: none;
  /* border: 1px solid dimgray; */
  transition: border-color 0.2s ease-in-out;
  background: rgba(0, 0, 0, 0.1);
  position: relative;
}

.shortcut-text {
  position: absolute;
  bottom: 8px;
  right: 8px;
  display: none;
}

.tool-link:hover .shortcut-text {
  display: block;
}

.delete-text {
  position: absolute;
  top: 8px;
  right: 8px;
  border: 1px solid rgba(252, 131, 131, 0.5);
  background-color: rgba(252, 131, 131, 0.5);
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  color: white;
  display: none;
}

.tool-link:hover .delete-text {
  display: flex;
}

.tool-link:hover {
  border-color: rgb(var(--v-theme-primary));
  background-color: rgba(64, 64, 64, 0.3);
}

.chevron {
  transform: translateX(0);
  transition: transform 0.2s ease-in-out;
}

.tool-link:hover .chevron {
  transform: translateX(8px);
}

a:hover {
    background-color: DimGrey;
}

.custom-icon {
  width: 36px;
  height: 36px;
  flex-shrink: 0;
}
</style>
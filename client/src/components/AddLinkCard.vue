<template>
  <div>
    <div
      id="add-link-card"
      class="add-link-card"
      @click="handleClick"
      @mouseenter="hover = true"
      @mouseleave="hover = false"
    >
      <TpIcon
        name="plus"
        :class="['add-link-card__icon', { 'add-link-card__icon--active': hover }]"
      />
      <span class="add-link-card__text"> Add new link </span>
    </div>

    <TpModal
      v-model="isModalOpen"
      title="Add New Link"
      :size="mobile ? 'full' : 'md'"
      initial-focus="#add-link-card-url"
    >
      <form @submit.prevent="handleSubmit" ref="formRef" class="add-link-form">
        <TpInput
          input-id="add-link-card-url"
          v-model="formData.url"
          label="URL"
          type="url"
          placeholder="https://example.com"
          required
          :error="urlError"
          @enter="handleSubmit"
        />

        <p v-if="!validLink" class="add-link-form__error">
          Invalid URL: Failed to connect with this URL's server, is it spelled correctly?
        </p>

        <TpInput
          input-id="add-link-card-title"
          v-model="formData.title"
          label="Title"
          placeholder="My Link"
          @enter="handleSubmit"
        />

        <TpTextarea
          input-id="add-link-card-description"
          v-model="formData.description"
          label="Description"
          placeholder="Optional description"
          :rows="3"
        />

        <TpCombobox
          input-id="add-link-card-column-type"
          v-model="formData.columnType"
          :options="columnTypeOptions"
          label="Column Label"
          placeholder="Select or create column..."
          creatable
          create-label="Create '{input}'"
        />

        <div class="add-link-form__icon-section">
          <label class="add-link-form__label">Custom Icon (optional)</label>
          <div class="add-link-form__icon-row">
            <div class="add-link-form__icon-preview">
              <img
                v-if="formData.icon"
                :src="formData.icon"
                alt="Icon preview"
                class="add-link-form__icon-img"
              />
              <TpIcon v-else name="link" size="lg" />
            </div>
            <div class="add-link-form__icon-inputs">
              <TpInput
                v-model="formData.icon"
                label="Icon URL"
                placeholder="https://example.com/icon.png"
              />
              <div class="add-link-form__upload">
                <input
                  type="file"
                  accept="image/*"
                  @change="handleFileUpload"
                  ref="fileInputRef"
                  class="add-link-form__file-input"
                />
                <TpButton variant="ghost" size="sm" @click="triggerFileInput">
                  <TpIcon name="plus" size="sm" />
                  Upload Image
                </TpButton>
                <TpButton v-if="formData.icon" variant="ghost" size="sm" @click="clearIcon">
                  Clear
                </TpButton>
              </div>
            </div>
          </div>
        </div>
      </form>

      <template #actions>
        <div class="add-link-form__hints" v-if="!mobile">
          <span>Submit: <kbd>Enter</kbd></span>
          <span>New line: <kbd>Shift</kbd> + <kbd>Enter</kbd></span>
        </div>

        <div class="add-link-form__actions">
          <TpTooltip
            content="If title and description are left blank, Omega Tab attempts to fetch them from the website automatically."
            position="left"
          >
            <TpButton variant="ghost" icon-only>
              <TpIcon name="help" />
            </TpButton>
          </TpTooltip>

          <TpButton variant="ghost" @click="closeModal"> Cancel </TpButton>
          <TpButton variant="primary" :loading="isLoading" @click="handleSubmit">
            Add Link
          </TpButton>
        </div>
      </template>
    </TpModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { useLinksStore } from "../stores/links";
import { useUserStore } from "../stores/user";
import { useBreakpoint } from "@/composables/useBreakpoint";
import type { CreateLinkRequest } from "../types/Link";
import {
  TpIcon,
  TpModal,
  TpInput,
  TpTextarea,
  TpCombobox,
  TpButton,
  TpTooltip,
} from "@/components/ui";

const linksStore = useLinksStore();
const userStore = useUserStore();
const { smAndDown: mobile } = useBreakpoint();
const validLink = ref(true);
const urlError = ref("");
const props = defineProps<{ columnType: string }>();

const isModalOpen = ref(false);
const isLoading = ref(false);
const hover = ref(false);
const formRef = ref<HTMLFormElement | null>(null);

const formData = ref({
  url: "",
  title: "",
  description: "",
  columnType: props.columnType,
  icon: "",
});

const fileInputRef = ref<HTMLInputElement | null>(null);

const columnTypes = computed(() => linksStore.uniqueColumnTypes);

const columnTypeOptions = computed(() =>
  columnTypes.value.map((type) => ({
    value: type,
    label: type.charAt(0).toUpperCase() + type.slice(1),
  })),
);

const handleClick = () => {
  openModal();
};

const openModal = () => {
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
    columnType: props.columnType,
    icon: "",
  };
  urlError.value = "";
  validLink.value = true;
};

const triggerFileInput = () => {
  fileInputRef.value?.click();
};

const handleFileUpload = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  // Check file size (max 500KB)
  if (file.size > 500 * 1024) {
    alert("Image size must be less than 500KB");
    return;
  }

  const reader = new FileReader();
  reader.onload = (e) => {
    formData.value.icon = e.target?.result as string;
  };
  reader.readAsDataURL(file);
};

const clearIcon = () => {
  formData.value.icon = "";
  if (fileInputRef.value) {
    fileInputRef.value.value = "";
  }
};

const validateForm = (): boolean => {
  if (!formData.value.url) {
    urlError.value = "URL is required";
    return false;
  }

  const urlValidation = linksStore.validateUrl(formData.value.url);
  if (urlValidation !== true) {
    urlError.value = urlValidation as string;
    return false;
  }

  urlError.value = "";
  return true;
};

const handleSubmit = async () => {
  if (!validateForm()) return;

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
      icon: formData.value.icon || null,
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
    if (!formData.value.url) {
      resetForm();
    }
  }
});
</script>

<style scoped>
.add-link-card {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--tp-space-2);
  padding: var(--tp-space-4);
  border: var(--tp-border-width-thick) dashed var(--tp-border);
  border-radius: var(--tp-radius-sm);
  cursor: pointer;
  transition:
    border-color var(--tp-transition-fast),
    background-color var(--tp-transition-fast);
}

.add-link-card:hover {
  border-color: var(--tp-accent);
  background: var(--tp-accent-glow);
}

.add-link-card__icon {
  color: var(--tp-text-muted);
  transition: color var(--tp-transition-fast);
}

.add-link-card__icon--active {
  color: var(--tp-accent);
}

.add-link-card__text {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}

/* Form styles */
.add-link-form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
  min-height: 360px;
}

.add-link-form__error {
  color: var(--tp-error);
  font-size: var(--tp-text-sm);
  margin-top: calc(-1 * var(--tp-space-2));
}

.add-link-form__hints {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  font-size: var(--tp-text-xs);
  color: var(--tp-text-muted);
}

.add-link-form__hints kbd {
  font-size: var(--tp-text-xs);
}

.add-link-form__actions {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  margin-left: auto;
}

.add-link-form__icon-section {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.add-link-form__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.add-link-form__icon-row {
  display: flex;
  gap: var(--tp-space-4);
  align-items: flex-start;
}

.add-link-form__icon-preview {
  width: 48px;
  height: 48px;
  border-radius: var(--tp-radius-sm);
  border: var(--tp-border-width) solid var(--tp-border);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  background: var(--tp-bg-secondary);
  overflow: hidden;
}

.add-link-form__icon-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.add-link-form__icon-inputs {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.add-link-form__upload {
  display: flex;
  gap: var(--tp-space-2);
  align-items: center;
}

.add-link-form__file-input {
  display: none;
}
</style>

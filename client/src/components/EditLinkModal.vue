<template>
  <TpModal
    v-model="isModalOpen"
    title="Edit Link"
    :size="mobile ? 'full' : 'md'"
    initial-focus="#edit-link-url"
  >
    <form @submit.prevent="handleSubmit" ref="formRef" class="edit-link-form">
      <TpInput
        input-id="edit-link-url"
        v-model="formData.url"
        label="URL"
        type="url"
        placeholder="https://example.com"
        required
        :error="urlError"
        @enter="handleSubmit"
      />

      <TpInput v-model="formData.title" label="Title" placeholder="My Link" @enter="handleSubmit" />

      <TpTextarea
        v-model="formData.description"
        label="Description"
        placeholder="Optional description"
        :rows="3"
      />

      <TpCombobox
        v-model="formData.columnType"
        :options="columnTypeOptions"
        label="Column Label"
        placeholder="Select or create column..."
        creatable
        create-label="Create '{input}'"
      />

      <div class="edit-link-form__icon-section">
        <label class="edit-link-form__label">Custom Icon (optional)</label>
        <div class="edit-link-form__icon-row">
          <div class="edit-link-form__icon-preview">
            <img
              v-if="iconPreview"
              :src="iconPreview"
              alt="Icon preview"
              class="edit-link-form__icon-img"
            />
            <TpIcon v-else name="link" size="lg" />
          </div>
          <div class="edit-link-form__icon-inputs">
            <TpInput
              v-model="formData.icon"
              label="Icon URL"
              placeholder="https://example.com/icon.png"
              @input="handleIconUrlChange"
            />
            <div class="edit-link-form__upload">
              <input
                type="file"
                accept="image/*"
                @change="handleFileUpload"
                ref="fileInputRef"
                class="edit-link-form__file-input"
              />
              <TpButton variant="ghost" size="sm" @click="triggerFileInput">
                <TpIcon name="plus" size="sm" />
                Upload Image
              </TpButton>
              <TpButton v-if="formData.icon" variant="ghost" size="sm" @click="clearIcon">
                Clear
              </TpButton>
            </div>
            <IconSearchPanel @select="handleIconSelect" />
          </div>
        </div>
      </div>
    </form>

    <template #actions>
      <div class="edit-link-form__hints" v-if="!mobile">
        <span>Submit: <kbd>Enter</kbd></span>
        <span>New line: <kbd>Shift</kbd> + <kbd>Enter</kbd></span>
      </div>

      <div class="edit-link-form__actions">
        <TpTooltip
          content="Omega Tab will not attempt to fetch the title or description from the URL when editing a link. Add a new link to see this feature in action."
          position="left"
        >
          <TpButton variant="ghost" icon-only>
            <TpIcon name="help" />
          </TpButton>
        </TpTooltip>

        <TpButton variant="danger" icon-only @click="handleDelete">
          <TpIcon name="trash" />
        </TpButton>

        <TpButton variant="ghost" @click="closeModal"> Cancel </TpButton>
        <TpButton variant="primary" :loading="isLoading" @click="handleSubmit">
          Save Link
        </TpButton>
      </div>
    </template>
  </TpModal>
</template>

<script setup lang="ts">
import { ref, watch, computed } from "vue";
import type { Link } from "@/types/Link";
import { useLinksStore } from "../stores/links";
import { useBreakpoint } from "@/composables/useBreakpoint";
import {
  TpModal,
  TpInput,
  TpTextarea,
  TpCombobox,
  TpButton,
  TpTooltip,
  TpIcon,
} from "@/components/ui";
import IconSearchPanel from "@/components/IconSearchPanel.vue";

const linksStore = useLinksStore();
const { smAndDown: mobile } = useBreakpoint();

const props = defineProps<{
  modelValue: boolean;
  link?: Link;
}>();

const emit = defineEmits<(e: "update:modelValue", value: boolean) => void>();

const isModalOpen = ref(props.modelValue);
const isLoading = ref(false);
const formRef = ref<HTMLFormElement | null>(null);
const urlError = ref("");

const formData = ref({
  url: "",
  title: "",
  description: "",
  columnType: "",
  icon: "",
});

const fileInputRef = ref<HTMLInputElement | null>(null);
const iconPreview = computed(() => formData.value.icon || props.link?.icon || "");

const columnTypes = computed(() => linksStore.uniqueColumnTypes);

const columnTypeOptions = computed(() =>
  columnTypes.value.map((type) => ({
    value: type,
    label: type.charAt(0).toUpperCase() + type.slice(1),
  })),
);

watch(
  () => props.modelValue,
  (newValue) => {
    isModalOpen.value = newValue;
    if (newValue && props.link) {
      formData.value = {
        url: props.link.url,
        title: props.link.title,
        description: props.link.description || "",
        columnType: props.link.column_type || "",
        icon: props.link.icon || "",
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

const closeModal = () => {
  isModalOpen.value = false;
  resetForm();
};

const resetForm = () => {
  formData.value = {
    url: "",
    title: "",
    description: "",
    columnType: "",
    icon: "",
  };
  urlError.value = "";
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

const handleIconUrlChange = () => {
  // Icon URL is already bound via v-model, nothing extra needed
};

const clearIcon = () => {
  formData.value.icon = "";
  if (fileInputRef.value) {
    fileInputRef.value.value = "";
  }
};

const handleIconSelect = (url: string) => {
  formData.value.icon = url;
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
  if (!props.link) return;
  if (!validateForm()) return;

  try {
    isLoading.value = true;
    props.link.url = formData.value.url;

    // Safe URL parsing with fallback for special characters
    let hostname = "";
    try {
      const parsedUrl = new URL(formData.value.url);
      hostname = parsedUrl.hostname;
    } catch (urlError) {
      // Fallback: try to extract domain from the URL string
      try {
        // For URLs like "example.com/path", extract "example.com"
        const urlParts = formData.value.url.split("/")[0].split(".");
        if (urlParts.length >= 2) {
          hostname = urlParts[urlParts.length - 2] + "." + urlParts[urlParts.length - 1];
        }
      } catch (fallbackError) {
        // If all else fails, use the URL as-is
        hostname = formData.value.url;
      }
    }

    props.link.title = formData.value.title || hostname;
    props.link.description = formData.value.description;
    props.link.column_type = formData.value.columnType;
    props.link.icon = formData.value.icon || null;

    await linksStore.updateLink(props.link);
    closeModal();
  } catch (error) {
    console.error("Error updating link:", error);
  } finally {
    isLoading.value = false;
  }
};

const handleDelete = async () => {
  if (!props.link) return;

  try {
    await linksStore.removeLink(props.link.id);
    closeModal();
  } catch (error) {
    console.error("Error deleting link:", error);
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
.edit-link-form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.edit-link-form__hints {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-1);
  font-size: var(--tp-text-xs);
  color: var(--tp-text-muted);
}

.edit-link-form__hints kbd {
  font-size: var(--tp-text-xs);
}

.edit-link-form__actions {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  margin-left: auto;
}

.edit-link-form__icon-section {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.edit-link-form__label {
  font-size: var(--tp-text-sm);
  font-weight: var(--tp-font-medium);
  color: var(--tp-text-primary);
}

.edit-link-form__icon-row {
  display: flex;
  gap: var(--tp-space-4);
  align-items: flex-start;
}

.edit-link-form__icon-preview {
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

.edit-link-form__icon-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.edit-link-form__icon-inputs {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-2);
}

.edit-link-form__upload {
  display: flex;
  gap: var(--tp-space-2);
  align-items: center;
}

.edit-link-form__file-input {
  display: none;
}
</style>

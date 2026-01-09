<template>
  <div>
    <div
      class="add-link-card"
      @click="handleClick"
      @mouseenter="hover = true"
      @mouseleave="hover = false"
    >
      <TpIcon
        name="plus"
        :class="[
          'add-link-card__icon',
          { 'add-link-card__icon--active': hover }
        ]"
      />
      <span class="add-link-card__text">
        Add new link
      </span>
    </div>

    <TpModal v-model="isModalOpen" title="Add New Link" :size="mobile ? 'full' : 'md'">
      <form @submit.prevent="handleSubmit" ref="formRef" class="add-link-form">
        <TpInput
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
          v-model="formData.title"
          label="Title"
          placeholder="My Link"
          @enter="handleSubmit"
        />

        <TpTextarea
          v-model="formData.description"
          label="Description"
          placeholder="Optional description"
          :rows="3"
        />

        <div class="add-link-form__column-select">
          <TpSelect
            v-model="formData.columnType"
            :options="columnTypeOptions"
            label="Column Label"
          />

          <div class="add-link-form__new-column">
            <TpInput
              v-model="newColumnType"
              placeholder="New column name"
              @enter="addNewColumnType"
            />
            <TpButton variant="secondary" size="sm" @click="addNewColumnType">
              Add
            </TpButton>
          </div>
        </div>
      </form>

      <template #actions>
        <div class="add-link-form__hints" v-if="!mobile">
          <span>Submit: <kbd>Enter</kbd></span>
          <span>New line: <kbd>Shift</kbd> + <kbd>Enter</kbd></span>
        </div>

        <div class="add-link-form__actions">
          <TpTooltip content="If title and description are left blank, Better New Tab attempts to fetch them from the website automatically." position="left">
            <TpButton variant="ghost" icon-only>
              <TpIcon name="help" />
            </TpButton>
          </TpTooltip>

          <TpButton variant="ghost" @click="closeModal">
            Cancel
          </TpButton>
          <TpButton variant="primary" :loading="isLoading" @click="handleSubmit">
            Add Link
          </TpButton>
        </div>
      </template>
    </TpModal>
  </div>
</template>

<script setup lang="ts">
import { computed, ref, watch, nextTick } from 'vue'
import { useLinksStore } from '../stores/links'
import { useUserStore } from '../stores/user'
import { useBreakpoint } from '@/composables/useBreakpoint'
import type { CreateLinkRequest } from '../types/Link'
import {
  TpIcon,
  TpModal,
  TpInput,
  TpTextarea,
  TpSelect,
  TpButton,
  TpTooltip
} from '@/components/ui'

const linksStore = useLinksStore()
const userStore = useUserStore()
const { smAndDown: mobile } = useBreakpoint()
const validLink = ref(true)
const urlError = ref('')
const props = defineProps<{ columnType: string }>()

const isModalOpen = ref(false)
const isLoading = ref(false)
const hover = ref(false)
const formRef = ref<HTMLFormElement | null>(null)

const formData = ref({
  url: '',
  title: '',
  description: '',
  columnType: props.columnType
})

const newColumnType = ref('')

const columnTypes = computed(() => linksStore.uniqueColumnTypes)

const columnTypeOptions = computed(() =>
  columnTypes.value.map((type) => ({
    value: type,
    label: type.charAt(0).toUpperCase() + type.slice(1)
  }))
)

const handleClick = () => {
  openModal()
}

const openModal = () => {
  isModalOpen.value = true
  nextTick(() => {
    const urlField = document.querySelector('input[type="url"]')
    if (urlField) {
      ;(urlField as HTMLInputElement).focus()
    }
  })
}

const closeModal = () => {
  isModalOpen.value = false
  resetForm()
}

const resetForm = () => {
  formData.value = {
    url: '',
    title: '',
    description: '',
    columnType: props.columnType
  }
  newColumnType.value = ''
  urlError.value = ''
  validLink.value = true
}

const addNewColumnType = () => {
  if (newColumnType.value && !columnTypes.value.includes(newColumnType.value)) {
    formData.value.columnType = newColumnType.value
    newColumnType.value = ''
  }
}

const validateForm = (): boolean => {
  if (!formData.value.url) {
    urlError.value = 'URL is required'
    return false
  }

  const urlValidation = linksStore.validateUrl(formData.value.url)
  if (urlValidation !== true) {
    urlError.value = urlValidation as string
    return false
  }

  urlError.value = ''
  return true
}

const handleSubmit = async () => {
  if (!validateForm()) return

  try {
    isLoading.value = true

    if (!userStore.userId) {
      console.error('User not logged in')
      return
    }

    const linkData: CreateLinkRequest = {
      title: formData.value.title,
      description: formData.value.description,
      url: formData.value.url,
      next_order_index: linksStore.links.length + 1,
      owner_id: userStore.userId,
      owner_type: 'user',
      column_type: formData.value.columnType
    }

    const savedLink = await linksStore.postLink(linkData)
    if (savedLink === 502) {
      validLink.value = false
      return
    }
    validLink.value = true
    if (!savedLink) console.error('Error saving link')
    closeModal()
  } catch (error) {
    console.error('Error saving link:', error)
  } finally {
    isLoading.value = false
  }
}

watch(isModalOpen, (newVal) => {
  if (!newVal) {
    if (newColumnType.value) {
      newColumnType.value = ''
    }
    if (!formData.value.url) {
      resetForm()
    }
  }
})
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
}

.add-link-form__error {
  color: var(--tp-error);
  font-size: var(--tp-text-sm);
  margin-top: calc(-1 * var(--tp-space-2));
}

.add-link-form__column-select {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-3);
}

.add-link-form__new-column {
  display: flex;
  gap: var(--tp-space-2);
  align-items: flex-end;
}

.add-link-form__new-column > :first-child {
  flex: 1;
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
</style>

<template>
  <div class="link-columns">
    <div v-if="uniqueColumnTypes.length === 0" class="link-columns__single">
      <AddLinkCard
        :columnType="'default'"
        :userId="props.userId"
      />
    </div>

    <div
      v-else
      v-for="columnType in uniqueColumnTypes"
      :key="columnType"
      :class="columnClass"
    >
      <h2 class="link-columns__title">
        {{ columnType.charAt(0).toUpperCase() + columnType.slice(1) }}
      </h2>

      <LinkCard
        v-for="(link, index) in getLinksByColumnType(columnType)"
        :key="link.id"
        :icon="link.icon ?? ''"
        :title="link.title"
        :description="link.description ?? ''"
        :link="link.url"
        :index="index"
        :shortcut="getShortcut(columnType)"
        class="link-columns__card"
        :onDelete="() => handleDeleteLink(link)"
        :onEdit="() => handleEditLink(link)"
        :ref="el => { if (el) linkRefs.push(el) }"
        :tabindex="getFocusableIndex(columnType, index)"
        :data-column="columnType"
        :data-link-index="index"
        :draggable="true"
        :onDragStart="(idx: number) => handleDragStart(columnType, idx)"
        :onDragOver="(idx: number) => handleDragOver(columnType, idx)"
        :onDragEnd="handleDragEnd"
        :isDragOver="dragOverState?.columnType === columnType && dragOverState?.index === index"
      />

      <AddLinkCard
        :columnType="columnType"
        :userId="props.userId"
      />
    </div>

    <EditLinkModal v-model="showEditModal" :link="editingLink" />
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, computed, nextTick, watch } from 'vue'
import AddLinkCard from './AddLinkCard.vue'
import EditLinkModal from './EditLinkModal.vue'
import LinkCard from './LinkCard.vue'
import type { Link } from '../types/Link'
import { useLinksStore, SHORTCUT_MAPPINGS } from '../stores/links'
import { openUrl } from '../utils/openUrl'

const linkStore = useLinksStore()

const showEditModal = ref(false)
const editingLink = ref<Link | undefined>()
const linkRefs = ref<any[]>([])
const currentFocus = ref<{ columnType: string; index: number } | null>(null)

// Drag state
const dragStartState = ref<{ columnType: string; index: number } | null>(null)
const dragOverState = ref<{ columnType: string; index: number } | null>(null)

const props = defineProps<{
  userId: string | null
}>()

const uniqueColumnTypes = computed(() => linkStore.uniqueColumnTypes)

// Reset refs when links change to keep them in sync
watch(
  () => linkStore.links,
  () => {
    linkRefs.value = []
    currentFocus.value = null
  }
)

const getLinksByColumnType = (columnType: string) => {
  return linkStore.links.filter((link) => link.column_type === columnType)
}

const getShortcut = (columnType: string) => {
  const columnIndex = uniqueColumnTypes.value.indexOf(columnType)
  if (columnIndex >= 0 && columnIndex < SHORTCUT_MAPPINGS.length) {
    return SHORTCUT_MAPPINGS[columnIndex].label
  }
  return ''
}

const getFocusableIndex = (_columnType: string, _index: number) => {
  return 0
}

const handleDeleteLink = async (link: Link) => linkStore.removeLink(link.id)

const handleEditLink = (link: Link) => {
  editingLink.value = link
  showEditModal.value = true
}

// Drag handlers
const handleDragStart = (columnType: string, index: number) => {
  dragStartState.value = { columnType, index }
}

const handleDragOver = (columnType: string, index: number) => {
  if (!dragStartState.value) return
  // Only allow reordering within the same column
  if (dragStartState.value.columnType !== columnType) return
  dragOverState.value = { columnType, index }
}

const handleDragEnd = () => {
  if (dragStartState.value && dragOverState.value) {
    const { columnType: startColumn, index: startIndex } = dragStartState.value
    const { columnType: endColumn, index: endIndex } = dragOverState.value

    if (startColumn === endColumn && startIndex !== endIndex) {
      // Reorder links within the column
      const columnLinks = getLinksByColumnType(startColumn)
      const movedLink = columnLinks[startIndex]
      const targetLink = columnLinks[endIndex]

      if (movedLink && targetLink) {
        linkStore.reorderLinks(startColumn, startIndex, endIndex)
      }
    }
  }

  dragStartState.value = null
  dragOverState.value = null
}

const isSearchInputFocused = () => {
  const activeElement = document.activeElement
  return (
    activeElement &&
    (activeElement.tagName === 'TEXTAREA' ||
      activeElement.classList.contains('searchBar'))
  )
}

const isModalOpen = () => {
  return document.querySelector('.tp-modal-overlay') !== null
}

const isDropdownOpen = () => {
  // Check if any TpSelect dropdown is open (it uses --open class)
  return document.querySelector('.tp-select--open') !== null
}

const focusLinkCard = (columnType: string, index: number) => {
  nextTick(() => {
    const targetLink = linkRefs.value.find(
      (ref) =>
        ref.$el?.dataset.column === columnType &&
        parseInt(ref.$el?.dataset.linkIndex) === index
    )

    if (targetLink && targetLink.$el) {
      const anchorElement = targetLink.$el.querySelector('a')
      if (anchorElement) {
        anchorElement.focus()
      } else {
        targetLink.$el.focus()
      }
      currentFocus.value = { columnType, index }
    }
  })
}

const focusSearchBar = () => {
  const searchBar = document.querySelector('.search-bar__input') as HTMLInputElement
  if (searchBar) {
    searchBar.focus()
    currentFocus.value = null
  }
}

const handleArrowKeys = (event: KeyboardEvent) => {
  if (
    isSearchInputFocused() ||
    isModalOpen() ||
    isDropdownOpen() ||
    event.ctrlKey ||
    event.altKey ||
    event.metaKey
  ) {
    return
  }

  const arrowKeys = ['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight']
  if (!arrowKeys.includes(event.key)) {
    return
  }

  event.preventDefault()

  if (!currentFocus.value) {
    if (uniqueColumnTypes.value.length > 0) {
      const firstColumnType = uniqueColumnTypes.value[0]
      const columnLinks = getLinksByColumnType(firstColumnType)
      if (columnLinks.length > 0) {
        focusLinkCard(firstColumnType, 0)
      }
    }
    return
  }

  const { columnType, index } = currentFocus.value
  const columnLinks = getLinksByColumnType(columnType)
  const currentColumnIndex = uniqueColumnTypes.value.indexOf(columnType)

  if (event.key === 'ArrowDown') {
    if (index < columnLinks.length - 1) {
      focusLinkCard(columnType, index + 1)
    } else {
      focusSearchBar()
    }
  } else if (event.key === 'ArrowUp') {
    if (index > 0) {
      focusLinkCard(columnType, index - 1)
    } else {
      focusSearchBar()
    }
  } else if (event.key === 'ArrowLeft') {
    if (currentColumnIndex > 0) {
      const prevColumnType = uniqueColumnTypes.value[currentColumnIndex - 1]
      const prevColumnLinks = getLinksByColumnType(prevColumnType)
      if (prevColumnLinks.length > 0) {
        const targetIndex = Math.min(index, prevColumnLinks.length - 1)
        focusLinkCard(prevColumnType, targetIndex)
      }
    }
  } else if (event.key === 'ArrowRight') {
    if (currentColumnIndex < uniqueColumnTypes.value.length - 1) {
      const nextColumnType = uniqueColumnTypes.value[currentColumnIndex + 1]
      const nextColumnLinks = getLinksByColumnType(nextColumnType)
      if (nextColumnLinks.length > 0) {
        const targetIndex = Math.min(index, nextColumnLinks.length - 1)
        focusLinkCard(nextColumnType, targetIndex)
      }
    }
  }
}

const handleEditShortcut = (event: KeyboardEvent) => {
  if (event.key !== 'e') return
  if (isSearchInputFocused() || isModalOpen() || isDropdownOpen()) return
  if (event.ctrlKey || event.altKey || event.metaKey) return
  if (!currentFocus.value) return

  event.preventDefault()
  const { columnType, index } = currentFocus.value
  const links = getLinksByColumnType(columnType)
  if (index >= 0 && index < links.length) {
    handleEditLink(links[index])
  }
}

const handleDeleteShortcut = (event: KeyboardEvent) => {
  if (event.key !== 'd') return
  if (isSearchInputFocused() || isModalOpen() || isDropdownOpen()) return
  if (event.ctrlKey || event.altKey || event.metaKey) return
  if (!currentFocus.value) return

  event.preventDefault()
  const { columnType, index } = currentFocus.value
  const links = getLinksByColumnType(columnType)
  if (index >= 0 && index < links.length) {
    handleDeleteLink(links[index])
  }
}

const handleLinkFocus = (event: FocusEvent) => {
  const target = event.target as HTMLElement
  const linkCard = target.closest('.link-columns__card') as HTMLElement
  if (linkCard) {
    const columnType = linkCard.dataset.column
    const linkIndex = parseInt(linkCard.dataset.linkIndex || '0')
    if (columnType !== undefined) {
      currentFocus.value = { columnType, index: linkIndex }
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  window.addEventListener('keydown', handleArrowKeys)
  window.addEventListener('keydown', handleEditShortcut)
  window.addEventListener('keydown', handleDeleteShortcut)
  document.addEventListener('focusin', handleLinkFocus)
  linkRefs.value = []
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('keydown', handleArrowKeys)
  window.removeEventListener('keydown', handleEditShortcut)
  window.removeEventListener('keydown', handleDeleteShortcut)
  document.removeEventListener('focusin', handleLinkFocus)
})

const handleKeydown = (event: KeyboardEvent) => {
  if (!/^[1-9]$/.test(event.key)) return
  if (isModalOpen() || isDropdownOpen()) return

  const numKey = Number.parseInt(event.key) - 1

  let columnIndex = -1
  if (event.ctrlKey && event.altKey) {
    columnIndex = 1
  } else if (event.ctrlKey) {
    columnIndex = 0
  }

  if (columnIndex >= 0 && columnIndex < uniqueColumnTypes.value.length) {
    const columnType = uniqueColumnTypes.value[columnIndex]
    const links = getLinksByColumnType(columnType)

    if (numKey >= 0 && numKey < links.length) {
      openUrl(links[numKey].url)
    }
  }
}

const columnClass = computed(() => {
  const columnCount = uniqueColumnTypes.value.length
  if (columnCount === 1) return 'link-columns__single'
  if (columnCount === 2) return 'link-columns__two'
  if (columnCount === 3) return 'link-columns__three'
  if (columnCount === 4) return 'link-columns__four'
  return 'link-columns__grid'
})
</script>

<style scoped>
.link-columns {
  display: flex;
  flex-wrap: wrap;
  margin-top: var(--tp-space-12);
  gap: var(--tp-space-8);
  padding-bottom: var(--tp-space-8);
  justify-content: center;
}

.link-columns__title {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-4);
  font-family: var(--tp-font-mono);
  letter-spacing: var(--tp-tracking-wide);
  text-transform: uppercase;
}

.link-columns__title::before {
  content: '// ';
  color: var(--tp-accent);
}

.link-columns__card {
  margin-bottom: var(--tp-space-2);
}

.link-columns__single {
  flex: 0 0 65%;
  margin: 0 auto;
}

.link-columns__two {
  flex: 0 0 45%;
}

.link-columns__three {
  flex: 0 0 30%;
}

.link-columns__four {
  flex: 0 0 45%;
  min-width: 20rem;
  max-width: 30rem;
}

.link-columns__grid {
  flex: 0 0 30%;
  min-width: 20rem;
  max-width: 30rem;
}

/* Responsive */
@media (max-width: 768px) {
  .link-columns {
    flex-direction: column;
  }

  .link-columns > div {
    flex: 0 0 100%;
    width: 90%;
    margin: 0 auto;
  }
}

@media (min-width: 769px) and (max-width: 1200px) {
  .link-columns__grid,
  .link-columns__four {
    flex: 0 0 45%;
  }
}

/* Focus styles for link cards */
:deep(.link-card:focus-visible) {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>

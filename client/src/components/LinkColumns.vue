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
        :key="link.order_index"
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
import { onMounted, onUnmounted, ref, computed, nextTick } from 'vue'
import AddLinkCard from './AddLinkCard.vue'
import EditLinkModal from './EditLinkModal.vue'
import LinkCard from './LinkCard.vue'
import type { Link } from '../types/Link'
import { useLinksStore, SHORTCUT_MAPPINGS } from '../stores/links'

const linkStore = useLinksStore()

const showEditModal = ref(false)
const editingLink = ref<Link | undefined>()
const linkRefs = ref<any[]>([])
const currentFocus = ref<{ columnType: string; index: number } | null>(null)

const props = defineProps<{
  userId: string | null
}>()

const uniqueColumnTypes = computed(() => linkStore.uniqueColumnTypes)

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

const isSearchInputFocused = () => {
  const activeElement = document.activeElement
  return (
    activeElement &&
    (activeElement.tagName === 'TEXTAREA' ||
      activeElement.classList.contains('searchBar'))
  )
}

const focusLinkCard = (columnType: string, index: number) => {
  nextTick(() => {
    linkRefs.value = []
    nextTick(() => {
      const targetLink = linkRefs.value.find(
        (ref) =>
          ref.$el.dataset.column === columnType &&
          parseInt(ref.$el.dataset.linkIndex) === index
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
  })
}

const handleArrowKeys = (event: KeyboardEvent) => {
  if (
    isSearchInputFocused() ||
    event.ctrlKey ||
    event.altKey ||
    event.metaKey
  ) {
    return
  }

  if (event.key !== 'ArrowUp' && event.key !== 'ArrowDown') {
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

  if (event.key === 'ArrowDown') {
    if (index < columnLinks.length - 1) {
      focusLinkCard(columnType, index + 1)
    }
  } else if (event.key === 'ArrowUp') {
    if (index > 0) {
      focusLinkCard(columnType, index - 1)
    }
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
  window.addEventListener('keydown', handleArrowKeys)
  linkRefs.value = []
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
  window.removeEventListener('keydown', handleArrowKeys)
})

const handleKeydown = (event: KeyboardEvent) => {
  if (!/^[1-9]$/.test(event.key)) return

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
      window.open(links[numKey].url, '_blank')
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

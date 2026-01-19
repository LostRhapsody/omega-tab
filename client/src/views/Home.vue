<template>
  <div>
    <div v-if="isLoading" class="home-loading">
      <TpSpinner size="lg" />
    </div>
    <div v-else>
      <header class="home-header">
        <div class="home-header__container">
          <h1 class="home-header__logo">
            <a href="#">OmegaTab_</a>
          </h1>
          <UserMenu />
        </div>
      </header>

      <main class="home-main">
        <section aria-label="Search">
          <SearchBar />
        </section>
        <section aria-label="Link columns">
          <LinkColumns :userId="userId" />
        </section>

        <!-- Keyboard Shortcuts Modal -->
        <TpModal v-model="showHelpDialog" title="Keyboard Shortcuts" size="lg">
          <div class="shortcuts-modal">
            <h4 class="shortcuts-modal__section-title">Open Links</h4>
            <div v-if="uniqueColumnTypes.length" class="shortcuts-modal__section">
              <div v-for="(columnType, colIndex) in uniqueColumnTypes" :key="columnType">
                <div
                  v-for="(link, index) in getLinksByColumnType(columnType)"
                  :key="link.order_index"
                >
                  <div v-if="colIndex < 2" class="shortcuts-modal__row">
                    <span class="shortcuts-modal__label">{{ link.title }}</span>
                    <span class="shortcuts-modal__keys">
                      <template v-if="getShortcut(columnType).includes('+')">
                        <kbd>{{ getShortcut(columnType).split("+")[0] }}</kbd>
                        +
                        <kbd>{{ getShortcut(columnType).split("+")[1] }}</kbd>
                      </template>
                      <template v-else>
                        <kbd>{{ getShortcut(columnType) }}</kbd>
                      </template>
                      +
                      <kbd>{{ index + 1 }}</kbd>
                    </span>
                  </div>
                  <TpDivider
                    v-if="
                      showShortcutDivider(index, getLinksByColumnType(columnType).length, colIndex)
                    "
                  />
                </div>
              </div>
            </div>
            <div v-else class="shortcuts-modal__section shortcuts-modal__empty">No links added</div>

            <h4 class="shortcuts-modal__section-title">Change Search Engine</h4>
            <div class="shortcuts-modal__section">
              <p class="shortcuts-modal__description">
                Use
                <kbd>Ctrl</kbd> +
                <kbd><TpIcon name="arrow-up" size="sm" /> up</kbd>
                or
                <kbd>Ctrl</kbd> +
                <kbd><TpIcon name="arrow-down" size="sm" /> down</kbd>
                to cycle through search engines.
              </p>
              <div v-for="(engine, index) in searchEngines" :key="engine.name">
                <div class="shortcuts-modal__row">
                  <span class="shortcuts-modal__label">{{ engine.name }}</span>
                  <span class="shortcuts-modal__keys">Search Engine {{ index + 1 }}</span>
                </div>
                <TpDivider v-if="index + 1 !== searchEngines.length" />
              </div>
            </div>

            <h4 class="shortcuts-modal__section-title">Navigate Links</h4>
            <div class="shortcuts-modal__section">
              <p class="shortcuts-modal__description">
                Use
                <kbd><TpIcon name="arrow-up" size="sm" /> up</kbd>
                ,
                <kbd><TpIcon name="arrow-down" size="sm" /> down</kbd>
                ,
                <kbd><TpIcon name="arrow-left" size="sm" /> left</kbd>
                , or
                <kbd><TpIcon name="arrow-right" size="sm" /> right</kbd>
                to jump between links when you are not focused on the search bar.
              </p>
              <TpDivider />
              <div class="shortcuts-modal__row">
                <span class="shortcuts-modal__label">Edit Focused Link</span>
                <span class="shortcuts-modal__keys">
                  <kbd>E</kbd>
                </span>
              </div>
            </div>

            <h4 class="shortcuts-modal__section-title">Other Shortcuts</h4>
            <div class="shortcuts-modal__section">
              <div class="shortcuts-modal__row">
                <span class="shortcuts-modal__label">Show Keyboard Shortcuts</span>
                <span class="shortcuts-modal__keys"> <kbd>?</kbd> (<kbd>shift + /</kbd>) </span>
              </div>
              <TpDivider />
              <div class="shortcuts-modal__row">
                <span class="shortcuts-modal__label">Show Command Palette</span>
                <span class="shortcuts-modal__keys"> <kbd>Ctrl</kbd> + <kbd>K</kbd> </span>
              </div>
              <TpDivider />
              <div class="shortcuts-modal__row">
                <span class="shortcuts-modal__label">Add A New Link</span>
                <span class="shortcuts-modal__keys"> <kbd>Alt</kbd> + <kbd>N</kbd> </span>
              </div>
            </div>
          </div>

          <template #actions>
            <TpButton variant="secondary" @click="showHelpDialog = false"> Close </TpButton>
          </template>
        </TpModal>
      </main>

      <!-- Help Menu -->
      <div class="home-help-menu">
        <TpMenu position="top-end">
          <template #trigger>
            <button class="home-help-menu__trigger" aria-label="Help menu">
              <TpIcon name="help" />
            </button>
          </template>

          <TpMenuItem>
            <a href="/docs/getting-started" class="home-help-menu__link">
              <TpIcon name="rocket" size="sm" />
              Getting Started
            </a>
          </TpMenuItem>
          <TpMenuItem @click="showHelpDialog = true">
            <TpIcon name="cog" size="sm" />
            Keyboard Shortcuts
          </TpMenuItem>
          <TpMenuItem>
            <a href="/docs/" class="home-help-menu__link">
              <TpIcon name="book" size="sm" />
              Guides
            </a>
          </TpMenuItem>
          <TpMenuItem>
            <a href="https://github.com/LostRhapsody/omega-tab/issues" class="home-help-menu__link">
              <TpIcon name="help" size="sm" />
              Report Issues
            </a>
          </TpMenuItem>
        </TpMenu>
      </div>
    </div>

    <CommandPalette />
  </div>
</template>

<script setup lang="ts">
import CommandPalette from "../components/CommandPalette.vue";
import { computed, onMounted, ref, onUnmounted } from "vue";
import { useRouter } from "vue-router";
import LinkColumns from "../components/LinkColumns.vue";
import SearchBar from "../components/SearchBar.vue";
import UserMenu from "../components/UserMenu.vue";
import { useUserStore } from "../stores/user";
import { useLinksStore, SHORTCUT_MAPPINGS } from "../stores/links";
import { useFeedbackStore } from "../stores/feedback";
import { useUserSettingsStore } from "../stores/settings";
import { searchEngines } from "../data/SearchEngines";
import { API } from "../constants/api";
import api from "../services/api";
import { useHead } from "@unhead/vue";
import { authService } from "../services/auth";
import {
  TpSpinner,
  TpModal,
  TpButton,
  TpMenu,
  TpMenuItem,
  TpIcon,
  TpDivider,
} from "@/components/ui";

// Set SEO metadata using Unhead
useHead({
  title: "OmegaTab - The Ultimate New Tab",
  meta: [
    {
      name: "description",
      content: "Create the ultimate new tab landing page.",
    },
    {
      name: "keywords",
      content:
        "new tab, browser extension, productivity, keyboard shortcuts, Jira, Linear, command palette, browser landing page",
    },
    {
      property: "og:title",
      content: "OmegaTab - The Ultimate New Tab",
    },
    {
      property: "og:description",
      content: "Create the ultimate new tab landing page.",
    },
    {
      property: "og:type",
      content: "website",
    },
    {
      property: "og:url",
      content: "https://omega-tab.evanrobertson.dev",
    },
    {
      name: "twitter:card",
      content: "summary_large_image",
    },
    {
      name: "twitter:title",
      content: "OmegaTab - The Ultimate New Tab",
    },
    {
      name: "twitter:description",
      content: "Create the ultimate new tab landing page.",
    },
  ],
  script: [
    {
      type: "application/ld+json",
      children: JSON.stringify({
        "@context": "https://schema.org",
        "@type": "WebApplication",
        name: "OmegaTab",
        description: "Create the ultimate new tab landing page.",
        applicationCategory: "ProductivityApplication",
        operatingSystem: "Any",
        offers: {
          "@type": "Offer",
          price: "0",
          priceCurrency: "USD",
        },
        featureList: "Keyboard shortcuts, Command palette, Tool integrations with Jira and Linear",
      }),
    },
  ],
  link: [
    {
      rel: "canonical",
      href: "https://omega-tab.evanrobertson.dev",
    },
  ],
});

const userStore = useUserStore();
const linksStore = useLinksStore();
const feedbackStore = useFeedbackStore();
const userSettingsStore = useUserSettingsStore();

const router = useRouter();

const isLoading = ref(true);
const showHelpDialog = ref(false);
const showFeedbackDialog = ref(false);
const showFeedbackMessageDialog = ref(false);
const feedbackMessageTitle = ref("");
const feedbackMessage = ref("");

const userId = ref<string | null>(null);
const currentRole = ref("member");
const uniqueColumnTypes = computed(() => linksStore.uniqueColumnTypes);

const getShortcut = (columnType: string) => {
  const columnIndex = uniqueColumnTypes.value.indexOf(columnType);
  if (columnIndex >= 0 && columnIndex < SHORTCUT_MAPPINGS.length) {
    return SHORTCUT_MAPPINGS[columnIndex].label;
  }
  return "";
};

const getLinksByColumnType = (columnType: string) => {
  return linksStore.links.filter((link) => link.column_type === columnType);
};

const isModalOpen = () => {
  return document.querySelector(".tp-modal-overlay") !== null;
};

const handleShowKeyboardShortcuts = (event: KeyboardEvent) => {
  if (event.key === "?" && !isModalOpen()) {
    showHelpDialog.value = true;
  }
};

const handleFeedbackDialogClose = async (value: boolean) => {
  if (value) return;

  showFeedbackDialog.value = value;
  if (!userStore.userId) return;
  if (!userStore.email) return;
  if (!feedbackStore.reasons) return;

  try {
    const response = await api.post(API.FEEDBACK, {
      reasons: feedbackStore.reasons,
      feedback_comment: feedbackStore.feedbackComment,
    });

    switch (response.status) {
      case 200:
        feedbackMessageTitle.value = "Thank You!";
        feedbackMessage.value = "Your feedback has been submitted successfully.";
        break;
      case 429:
        feedbackMessageTitle.value = "Too Many Requests";
        feedbackMessage.value =
          "Feedback already submitted today, please wait 24 hours before submitting again.";
        break;
      default:
        feedbackMessageTitle.value = "Error";
        feedbackMessage.value = "An unknown error occurred.";
        break;
    }
  } catch (error) {
    feedbackMessageTitle.value = "Error";
    feedbackMessage.value = "An unknown error occurred.";
  } finally {
    showFeedbackMessageDialog.value = true;
    feedbackStore.clearFeedback();
  }
};

const showShortcutDivider = (index: number, linksInColumn: number, currentColumn: number) => {
  const numberOfColumns = linksStore.uniqueColumnTypes.length;
  if (numberOfColumns === 1) {
    return index !== linksInColumn - 1;
  }
  const isLastLink = index === linksInColumn - 1;
  const isLastColumn = currentColumn === numberOfColumns - 1;
  return !(isLastLink && isLastColumn);
};

onMounted(async () => {
  isLoading.value = true;

  try {
    if (userStore.userId) {
      await linksStore.fetchLinks(userStore.userId);
    } else {
      const response = await api.get<{ user: { id: string; email: string } }>(API.GET_USER_DATA);

      if (response.data.user) {
        const authUser = {
          id: response.data.user.id,
          email: response.data.user.email,
        };

        const gotCachedData = userStore.fetchUserDataFromCache(authUser);

        if (gotCachedData) {
          userStore.fetchUserDataFromServer(authUser).catch((error) => {
            console.error("Background refresh of user data failed:", error);
          });
        } else {
          const serverDataSuccess = await userStore.fetchUserDataFromServer(authUser);

          if (!serverDataSuccess) {
            throw new Error("Failed to fetch user data from server");
          }
        }

        userSettingsStore.fetchSettings();

        if (userStore.userId) {
          await linksStore.fetchLinks(userStore.userId);
        }
      } else {
        authService.logout();
        router.push("/login");
      }
    }
  } catch (error) {
    console.error("Error during initialization:", error);
    authService.logout();
    router.push("/login");
  } finally {
    isLoading.value = false;
  }

  window.addEventListener("keydown", handleShowKeyboardShortcuts);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleShowKeyboardShortcuts);
});
</script>

<style scoped>
.home-loading {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
}

.home-header {
  border-bottom: 1px solid var(--tp-border);
  background: var(--tp-bg-secondary);
  padding: var(--tp-space-4) var(--tp-space-6);
}

.home-header__container {
  display: flex;
  align-items: center;
  justify-content: space-between;
  max-width: 1400px;
  margin: 0 auto;
}

.home-header__logo {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  font-family: var(--tp-font-mono);
}

.home-header__logo a {
  color: var(--tp-text-primary);
  text-decoration: none;
}

.home-header__logo a::after {
  content: "";
  border-right: 2px solid var(--tp-accent);
  margin-left: 2px;
}

.home-main {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 var(--tp-space-6);
}

/* Shortcuts Modal */
.shortcuts-modal__section-title {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin: var(--tp-space-6) 0 var(--tp-space-4);
}

.shortcuts-modal__section-title:first-child {
  margin-top: 0;
}

.shortcuts-modal__section {
  border: 1px solid var(--tp-border);
  border-radius: var(--tp-radius-sm);
  padding: var(--tp-space-4);
}

.shortcuts-modal__empty {
  color: var(--tp-text-muted);
  text-align: center;
}

.shortcuts-modal__row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: var(--tp-space-2) 0;
}

.shortcuts-modal__label {
  color: var(--tp-text-primary);
}

.shortcuts-modal__keys {
  font-family: var(--tp-font-mono);
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}

.shortcuts-modal__keys kbd {
  display: inline-flex;
  align-items: center;
  gap: var(--tp-space-1);
}

.shortcuts-modal__description {
  color: var(--tp-text-secondary);
  margin-bottom: var(--tp-space-4);
}

/* Help Menu */
.home-help-menu {
  position: fixed;
  bottom: var(--tp-space-4);
  right: var(--tp-space-4);
}

.home-help-menu__trigger {
  width: 42px;
  height: 42px;
  border-radius: 50%;
  background: var(--tp-bg-secondary);
  border: 1px solid var(--tp-border);
  color: var(--tp-text-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition:
    background-color var(--tp-transition-fast),
    border-color var(--tp-transition-fast);
}

.home-help-menu__trigger:hover {
  background: var(--tp-bg-tertiary);
  border-color: var(--tp-accent);
}

.home-help-menu__trigger:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}

.home-help-menu__link {
  display: flex;
  align-items: center;
  gap: var(--tp-space-2);
  color: inherit;
  text-decoration: none;
}
</style>

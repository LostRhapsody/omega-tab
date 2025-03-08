import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "BetterNewTab | Guides",
  description: "Getting started and guides for BetterNewTab",
  base: '/docs/',
  themeConfig: {
    logo: '/favicon.svg',    
    search: {
      provider: 'local',
    },
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guides', link: '/guides' },
      { text: 'BetterNewTab', link: 'http://localhost:5173', target: '_self' },
    ],

    sidebar: [
      {
        text: 'Getting Started',
        link: '/getting-started',  
      },
      {
        text: 'Guides',
        items: [
          { text: 'Guides', link: '/guides' },
          { text: 'Creating a New Link', link: '/guides/creating-a-new-link' },
          { text: 'Editing a Link', link: '/guides/editing-a-link' },
          { text: 'User Settings', link: '/guides/user-settings' },
          { text: 'The Search Bar', link: '/guides/the-search-bar' },
          { text: 'Command Palette', link: '/guides/command-palette' },
          { text: 'Keyboard shortcuts', link: '/guides/keyboard-shortcuts' },
          { text: 'Metadata Fetching', link: '/guides/metadata-fetching' },
          { text: 'Search Suggestions', link: '/guides/search-suggestions' },
          { text: 'Manage Your Subscription', link: '/guides/manage-your-subscription' },
        ],
      },
      {
        text: 'Integrations',
        items: [
          { text: 'Confluence', link: '/guides/confluence-integration' },
          { text: 'Jira', link: '/guides/jira-integration' },
          { text: 'Linear', link: '/guides/linear-integration' },
        ],
      }
    ],
    footer: {
      message: 'BetterNewTab',
      copyright: 'Copyright © 2019-present Evan Robertson'
    },
    lastUpdated: {
      text: 'Updated at',
      formatOptions: {
        dateStyle: 'full',
        timeStyle: 'short'
      }
    },
  }
})
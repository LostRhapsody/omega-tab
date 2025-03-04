export type UserSettings = {
  search_history: boolean;
  autosuggest: boolean;
  jira_api: boolean;
  confluence_api: boolean;
  linear_api: boolean;
  new_tabs: boolean;
  metadata: boolean;
};

export const UserSettingsLabels = [
  {
    label: "Enable Search History",
    key: "search_history",
    active: false,
    plan: "free",
  },
  {
    label: "Enable Autosuggest (Powered By Brave)",
    key: "autosuggest",
    active: true,
    plan: "plus",
  },
  {
    label: "Fetch Metadata for new links",
    key: "metadata",
    active: true,
    plan: "plus",
  },
  { label: "Enable Jira API", key: "jira_api", active: false, plan: "pro" },
  {
    label: "Enable Confluence API",
    key: "confluence_api",
    active: false,
    plan: "pro",
  },
  {
    label: "Enable Linear API",
    key: "linear_api",
    active: false,
    plan: "pro",
  },
  {
    label: "Open Links In New Tabs",
    key: "new_tabs",
    active: true,
    plan: "free",
  },
];

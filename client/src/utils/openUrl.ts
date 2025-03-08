import { useUserSettingsStore } from "../stores/settings";

export const openUrl = (url: string) => {
  const settingsStore = useUserSettingsStore();
  if (settingsStore.settings.new_tabs) {
    window.open(url, "_blank");
  } else {
    window.location.href = url;
  }
};

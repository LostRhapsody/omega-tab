import { useUserSettingsStore } from "../stores/settings";

export const openUrl = (url: string) => {
  // Basic validation to prevent opening malformed URLs
  if (!url || typeof url !== "string") {
    console.error("Invalid URL provided to openUrl:", url);
    return;
  }

  // Trim whitespace
  const cleanUrl = url.trim();

  // Ensure URL has a protocol
  let finalUrl = cleanUrl;
  if (!cleanUrl.startsWith("http://") && !cleanUrl.startsWith("https://")) {
    finalUrl = `https://${cleanUrl}`;
  }

  try {
    // Validate URL format
    new URL(finalUrl);

    const settingsStore = useUserSettingsStore();
    if (settingsStore.settings.new_tabs) {
      window.open(finalUrl, "_blank");
    } else {
      window.location.href = finalUrl;
    }
  } catch (error) {
    console.error("Failed to open URL:", finalUrl, error);
    // Don't attempt to open invalid URLs
  }
};

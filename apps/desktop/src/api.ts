import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type {
  AdblockStatus,
  MediaSupport,
  Bookmark,
  Capabilities,
  Credential,
  Download,
  Preferences,
  Preview,
  UserAgentPreset,
  WrappedApp,
} from "./types";

export const listApps = () => invoke<WrappedApp[]>("list_apps");

export const listDownloads = () => invoke<Download[]>("list_downloads");

export const openDownload = (id: number) =>
  invoke<void>("open_download", { id });

export const revealDownload = (id: number) =>
  invoke<void>("reveal_download", { id });

export const clearDownloads = () => invoke<Download[]>("clear_downloads");

export const onDownload = (handler: (download: Download) => void) =>
  listen<Download>("zww://download", (event) => handler(event.payload));

export const previewUrl = (input: string) =>
  invoke<Preview>("preview_url", { input });

export const createApp = (input: string, title?: string, adblock = true) =>
  invoke<WrappedApp>("create_app", { input, title: title ?? null, adblock });

export const updateApp = (app: WrappedApp) =>
  invoke<void>("update_app", { app });

export const deleteApp = (id: string) => invoke<void>("delete_app", { id });

export const hasDesktopShortcut = (id: string) =>
  invoke<boolean>("has_desktop_shortcut", { id });

export const openApp = (id: string) => invoke<void>("open_app", { id });

export const iconDataUrl = (id: string) =>
  invoke<string | null>("icon_data_url", { id });

export const adblockStatus = () => invoke<AdblockStatus>("adblock_status");

export const mediaSupport = () => invoke<MediaSupport>("media_support");

export const getPreferences = () => invoke<Preferences>("get_preferences");

export const setPreferences = (prefs: Preferences) =>
  invoke<void>("set_preferences", { prefs });

export const autostartActive = () => invoke<boolean>("autostart_active");

export const validHotkey = (accelerator: string) =>
  invoke<boolean>("valid_hotkey", { accelerator });

export const platformReport = () => invoke<Capabilities>("platform_report");

export const discoverBookmarks = () => invoke<Bookmark[]>("discover_bookmarks");

export const importBookmarks = (urls: string[], titles: string[]) =>
  invoke<number>("import_bookmarks", { urls, titles });

export const setAppIcon = (id: string, dataUrl: string) =>
  invoke<string>("set_app_icon", { id, dataUrl });

export const resetAppIcon = (id: string) =>
  invoke<string | null>("reset_app_icon", { id });

export const userAgentPresets = () =>
  invoke<UserAgentPreset[]>("user_agent_presets");

export const refreshFilters = () => invoke<AdblockStatus>("refresh_filters");

export const revealDataDir = () => invoke<void>("reveal_data_dir");

export const dataLocation = () => invoke<string>("data_location");

export async function copyText(text: string): Promise<boolean> {
  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(text);
      return true;
    }
  } catch {}

  try {
    const area = document.createElement("textarea");
    area.value = text;
    area.setAttribute("readonly", "");
    area.style.position = "fixed";
    area.style.opacity = "0";
    document.body.appendChild(area);
    area.select();
    const copied = document.execCommand("copy");
    document.body.removeChild(area);
    return copied;
  } catch {
    return false;
  }
}

export function errorMessage(e: unknown): string {
  if (typeof e === "string") return e;
  if (e instanceof Error) return e.message;
  return String(e);
}

export function hostOf(url: string): string {
  try {
    return new URL(url).host.replace(/^www\./, "");
  } catch {
    return url;
  }
}

export function formatAge(seconds: number | null): string {
  if (seconds === null) return "never";
  if (seconds < 60) return "just now";
  const minutes = Math.floor(seconds / 60);
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  const days = Math.floor(hours / 24);
  return days === 1 ? "yesterday" : `${days}d ago`;
}

export function formatCount(n: number): string {
  return n.toLocaleString("en-US");
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  const units = ["KB", "MB", "GB", "TB"];
  let value = bytes / 1024;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit++;
  }

  return `${value < 10 ? value.toFixed(1) : Math.round(value)} ${units[unit]}`;
}

export const hiddenCounts = () =>
  invoke<{ total: number; apps: Record<string, number> }>("hidden_counts");

export const appCredential = (id: string) =>
  invoke<Credential | null>("app_credential", { id });

export const setAppCredential = (id: string, credential: Credential | null) =>
  invoke<void>("set_app_credential", { id, credential });

export const validProxy = (address: string) =>
  invoke<void>("valid_proxy", { address });

export const openLink = (url: string) => invoke<void>("open_link", { url });

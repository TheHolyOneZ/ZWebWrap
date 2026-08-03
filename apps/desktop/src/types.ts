export type Rule = "allow" | "deny" | "ask";

export type BlockMode = "full" | "network_only" | "off";

export type SessionMode = "shared" | "isolated";

export interface Permissions {
  notifications: Rule;
  microphone: Rule;
  camera: Rule;
  location: Rule;
}

export interface WrappedApp {
  id: string;
  url: string;
  title: string;
  icon: string | null;

  adblock: BlockMode;
  extra_scope: string[];
  offsite: "browser" | "in_window";

  group: string | null;

  session: SessionMode;
  window: {
    width: number;
    height: number;
    x: number | null;
    y: number | null;
    maximized: boolean;
  };
  user_agent: string | null;
  zoom: number;
  always_on_top: boolean;
  desktop_shortcut: boolean | null;
  user_css: string | null;
  user_js: string | null;
  hotkey: string | null;
  title_badge: boolean;
  permissions: Permissions;
  contain_fullscreen: boolean | null;

  proxy: string | null;

  has_auth: boolean;
  frame: "normal" | "no_border" | "frameless";
  created_at: number;
}

export interface Preview {
  url: string;
  title: string;
  icon_url: string | null;
}

export interface FilterSource {
  name: string;
  description: string;
  cached: boolean;
}

export interface AdblockStatus {
  ready: boolean;
  network_rules: number;
  scriptlets: number;
  lists_loaded: number;
  seed_only: boolean;
  age_secs: number | null;
  sources: FilterSource[];
}

export interface MediaSupport {
  ok: boolean;
  missing: string[];
  install_hint: string | null;
}

export interface UserAgentPreset {
  label: string;
  value: string;
  hint: string;
}

export interface Preferences {
  desktop_shortcut: boolean;
  adblock_by_default: boolean;
  ask_where_to_save: boolean;
  close_to_tray: boolean;
  start_with_os: boolean;
  contain_fullscreen: boolean;

  tour_done: boolean;
  language: string | null;

  theme: string | null;
}

export type DownloadState = "active" | "done" | "failed";

export interface Download {
  id: number;

  app: string;
  file_name: string;
  path: string;
  state: DownloadState;

  bytes: number;
}

export interface Bookmark {
  title: string;
  url: string;
  source: string;
}

export interface Capabilities {
  os: string;
  session: "x11" | "wayland" | "other";
  shortcuts: boolean;
  ad_blocking: boolean;
  downloads: boolean;
  permission_prompts: boolean;
  find_in_page: boolean;
  spell_check: boolean;
  window_identity: boolean;
  always_on_top: boolean;
  limitations: string[];
}

export interface Credential {
  username: string;
  password: string;
}

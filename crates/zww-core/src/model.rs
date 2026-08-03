use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OffsitePolicy {
    #[default]
    Browser,
    InWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FrameStyle {
    #[default]
    Normal,
    NoBorder,

    Frameless,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BlockMode {
    #[default]
    Full,

    NetworkOnly,
    Off,
}

impl BlockMode {
    pub fn blocks_network(self) -> bool {
        matches!(self, Self::Full | Self::NetworkOnly)
    }

    pub fn hides_elements(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SessionMode {
    #[default]
    Shared,
    Isolated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Rule {
    Allow,
    Deny,

    #[default]
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default = "allow")]
    pub notifications: Rule,

    #[serde(default)]
    pub microphone: Rule,
    #[serde(default)]
    pub camera: Rule,
    #[serde(default)]
    pub location: Rule,
}

fn allow() -> Rule {
    Rule::Allow
}

impl Default for Permissions {
    fn default() -> Self {
        Self {
            notifications: Rule::Allow,
            microphone: Rule::Ask,
            camera: Rule::Ask,
            location: Rule::Ask,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WindowState {
    pub width: f64,
    pub height: f64,
    pub x: Option<f64>,
    pub y: Option<f64>,
    #[serde(default)]
    pub maximized: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self { width: 1200.0, height: 800.0, x: None, y: None, maximized: false }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrappedApp {
    pub id: String,
    pub url: String,
    pub title: String,
    #[serde(default)]
    pub icon: Option<PathBuf>,

    #[serde(rename = "adblock", default, deserialize_with = "block_mode")]
    pub blocking: BlockMode,

    #[serde(default)]
    pub extra_scope: Vec<String>,

    #[serde(default)]
    pub offsite: OffsitePolicy,

    #[serde(default)]
    pub group: Option<String>,

    #[serde(default)]
    pub session: SessionMode,

    #[serde(default)]
    pub window: WindowState,

    #[serde(default)]
    pub user_agent: Option<String>,

    #[serde(default = "unit_zoom")]
    pub zoom: f64,

    #[serde(default)]
    pub always_on_top: bool,

    #[serde(default)]
    pub desktop_shortcut: Option<bool>,

    #[serde(default)]
    pub user_css: Option<String>,

    #[serde(default)]
    pub user_js: Option<String>,

    #[serde(default)]
    pub hotkey: Option<String>,

    #[serde(default)]
    pub title_badge: bool,

    #[serde(default)]
    pub permissions: Permissions,

    #[serde(default)]
    pub contain_fullscreen: Option<bool>,

    #[serde(default)]
    pub proxy: Option<String>,

    #[serde(default)]
    pub has_auth: bool,

    #[serde(default)]
    pub frame: FrameStyle,

    #[serde(default)]
    pub created_at: u64,
}

fn unit_zoom() -> f64 {
    1.0
}

fn block_mode<'de, D>(deserializer: D) -> Result<BlockMode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Stored {
        Mode(BlockMode),
        Legacy(bool),
    }

    Ok(match Stored::deserialize(deserializer)? {
        Stored::Mode(mode) => mode,
        Stored::Legacy(true) => BlockMode::Full,
        Stored::Legacy(false) => BlockMode::Off,
    })
}

impl WrappedApp {
    pub fn new(id: impl Into<String>, url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            title: title.into(),
            icon: None,
            blocking: BlockMode::Full,
            extra_scope: Vec::new(),
            offsite: OffsitePolicy::default(),
            group: None,
            session: SessionMode::default(),
            window: WindowState::default(),
            user_agent: None,
            zoom: 1.0,
            always_on_top: false,
            desktop_shortcut: None,
            user_css: None,
            user_js: None,
            hotkey: None,
            title_badge: false,
            permissions: Permissions::default(),
            contain_fullscreen: None,
            proxy: None,
            has_auth: false,
            frame: FrameStyle::default(),
            created_at: now_secs(),
        }
    }

    pub fn proxy_is_usable(&self) -> Option<&str> {
        let raw = self.proxy.as_deref().map(str::trim).filter(|p| !p.is_empty())?;
        (self.session == SessionMode::Isolated).then_some(raw)
    }
}

pub fn parse_proxy(raw: &str) -> Result<url::Url, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("a proxy needs an address".into());
    }

    let text = if raw.contains("://") { raw.to_string() } else { format!("http://{raw}") };
    let parsed: url::Url = text.parse().map_err(|_| format!("{raw} is not an address"))?;

    match parsed.scheme() {
        "http" | "socks5" => {}
        other => return Err(format!("{other} proxies are not supported — use http or socks5")),
    }
    if parsed.host_str().is_none_or(str::is_empty) {
        return Err("a proxy needs a host".into());
    }
    if parsed.port().is_none() {
        return Err("a proxy needs a port, e.g. 127.0.0.1:8080".into());
    }
    Ok(parsed)
}

pub fn group_of(app: &WrappedApp) -> Option<&str> {
    app.group.as_deref().map(str::trim).filter(|g| !g.is_empty())
}

pub fn groups_in_use(apps: &[WrappedApp]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for app in apps {
        let Some(group) = group_of(app) else { continue };
        if !seen.iter().any(|existing| existing.eq_ignore_ascii_case(group)) {
            seen.push(group.to_string());
        }
    }
    seen.sort_by_key(|g| g.to_lowercase());
    seen
}

pub fn resolved(per_app: Option<bool>, global: bool) -> bool {
    per_app.unwrap_or(global)
}

pub fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

pub fn slug_for_url(url: &url::Url) -> String {
    let host = url.host_str().unwrap_or("app");
    let host = host.strip_prefix("www.").unwrap_or(host);
    let slug: String = host
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' })
        .collect();
    let slug = slug.trim_matches('-').to_string();
    if slug.is_empty() {
        "app".to_string()
    } else {
        slug
    }
}

pub fn unique_id(url: &url::Url, taken: &[String]) -> String {
    let base = slug_for_url(url);
    if !taken.iter().any(|t| t == &base) {
        return base;
    }
    (2..)
        .map(|n| format!("{base}-{n}"))
        .find(|candidate| !taken.iter().any(|t| t == candidate))
        .expect("an unused suffix always exists")
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    fn u(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    #[test]
    fn slug_strips_www_and_punctuation() {
        assert_eq!(slug_for_url(&u("https://www.youtube.com/feed")), "youtube-com");
        assert_eq!(slug_for_url(&u("https://claude.ai")), "claude-ai");
        assert_eq!(slug_for_url(&u("https://sub.example.co.uk/x")), "sub-example-co-uk");
    }

    #[test]
    fn unique_id_suffixes_on_collision() {
        let taken = vec!["youtube-com".to_string(), "youtube-com-2".to_string()];
        assert_eq!(unique_id(&u("https://youtube.com"), &taken), "youtube-com-3");
        assert_eq!(unique_id(&u("https://claude.ai"), &taken), "claude-ai");
    }

    #[test]
    fn a_per_app_opinion_outranks_the_global_default() {
        assert!(resolved(Some(true), false), "the app asked for it");
        assert!(!resolved(Some(false), true), "the app asked against it");
        assert!(resolved(None, true), "no opinion, so follow the default");
        assert!(!resolved(None, false));
    }

    #[test]
    fn the_old_boolean_still_reads_as_a_mode() {
        let on: WrappedApp =
            serde_json::from_str(r#"{"id":"x","url":"https://x.com","title":"X","adblock":true}"#)
                .unwrap();
        assert_eq!(on.blocking, BlockMode::Full);

        let off: WrappedApp =
            serde_json::from_str(r#"{"id":"x","url":"https://x.com","title":"X","adblock":false}"#)
                .unwrap();
        assert_eq!(off.blocking, BlockMode::Off);

        let modern: WrappedApp = serde_json::from_str(
            r#"{"id":"x","url":"https://x.com","title":"X","adblock":"network_only"}"#,
        )
        .unwrap();
        assert_eq!(modern.blocking, BlockMode::NetworkOnly);
    }

    #[test]
    fn each_mode_applies_the_halves_it_promises() {
        assert!(BlockMode::Full.blocks_network() && BlockMode::Full.hides_elements());
        assert!(BlockMode::NetworkOnly.blocks_network());
        assert!(
            !BlockMode::NetworkOnly.hides_elements(),
            "network-only exists precisely so a page is left alone"
        );
        assert!(!BlockMode::Off.blocks_network() && !BlockMode::Off.hides_elements());
    }

    #[test]
    fn a_mode_is_written_under_the_adblock_key() {
        let mut app = WrappedApp::new("x", "https://x.com", "X");
        app.blocking = BlockMode::NetworkOnly;
        let json = serde_json::to_string(&app).unwrap();
        assert!(json.contains(r#""adblock":"network_only""#), "got {json}");
    }

    #[test]
    fn groups_come_from_the_apps_that_name_them() {
        let mut work = WrappedApp::new("a", "https://a.test", "A");
        work.group = Some("Work".into());
        let mut work2 = WrappedApp::new("b", "https://b.test", "B");

        work2.group = Some("work".into());
        let mut blank = WrappedApp::new("c", "https://c.test", "C");
        blank.group = Some("   ".into());
        let loose = WrappedApp::new("d", "https://d.test", "D");
        let mut personal = WrappedApp::new("e", "https://e.test", "E");
        personal.group = Some("Personal".into());

        let apps = vec![work, work2, blank, loose, personal];
        assert_eq!(groups_in_use(&apps), vec!["Personal".to_string(), "Work".to_string()]);

        assert_eq!(group_of(&apps[0]), Some("Work"));
        assert_eq!(group_of(&apps[2]), None, "a blank name is not a group");
        assert_eq!(group_of(&apps[3]), None);
    }

    #[test]
    fn defaults_fill_in_for_older_records() {
        let json = r#"{"id":"x","url":"https://x.com","title":"X"}"#;
        let app: WrappedApp = serde_json::from_str(json).unwrap();
        assert_eq!(app.blocking, BlockMode::Full);
        assert_eq!(app.zoom, 1.0);
        assert!(!app.always_on_top);
        assert_eq!(app.frame, FrameStyle::Normal, "an existing app keeps its frame");
        assert_eq!(
            app.desktop_shortcut, None,
            "an app from before the setting follows the global preference"
        );
        assert_eq!(app.contain_fullscreen, None);
        assert_eq!(
            app.permissions,
            Permissions::default(),
            "an app from before permissions existed gets the safe defaults"
        );
        assert_eq!(app.permissions.microphone, Rule::Ask, "never silently on");
        assert_eq!(app.permissions.notifications, Rule::Allow);
        assert_eq!(app.offsite, OffsitePolicy::Browser);
        assert_eq!(
            app.session,
            SessionMode::Shared,
            "an app from before sessions existed keeps the shared login"
        );
        assert_eq!(app.window.width, 1200.0);
    }
}

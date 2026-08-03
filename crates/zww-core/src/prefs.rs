use crate::paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Preferences {
    #[serde(default = "yes")]
    pub desktop_shortcut: bool,

    #[serde(default = "yes")]
    pub adblock_by_default: bool,

    #[serde(default = "yes")]
    pub ask_where_to_save: bool,

    #[serde(default)]
    pub close_to_tray: bool,

    #[serde(default = "yes")]
    pub contain_fullscreen: bool,

    #[serde(default)]
    pub language: Option<String>,

    #[serde(default)]
    pub start_with_os: bool,

    #[serde(default)]
    pub tour_done: bool,

    #[serde(default)]
    pub theme: Option<String>,
}

fn yes() -> bool {
    true
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            desktop_shortcut: true,
            adblock_by_default: true,
            ask_where_to_save: true,
            close_to_tray: false,
            start_with_os: false,
            contain_fullscreen: true,
            tour_done: false,
            language: None,
            theme: None,
        }
    }
}

fn file() -> Result<PathBuf> {
    Ok(paths::data_dir()?.join("preferences.json"))
}

pub fn load() -> Preferences {
    let Ok(path) = file() else { return Preferences::default() };
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|e| {
            tracing::warn!(error = %e, "preferences unreadable; using defaults");
            Preferences::default()
        }),
        Err(_) => Preferences::default(),
    }
}

pub fn save(prefs: &Preferences) -> Result<()> {
    let path = file()?;
    let json = serde_json::to_string_pretty(prefs)?;

    let parent = path.parent().context("preferences path has no parent")?;
    let tmp = parent.join(".preferences.json.tmp");
    std::fs::write(&tmp, json)?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_the_helpful_ones() {
        let prefs = Preferences::default();
        assert!(prefs.desktop_shortcut, "create app promises a desktop shortcut");
        assert!(prefs.adblock_by_default);
        assert!(!prefs.close_to_tray, "close should close until asked otherwise");
        assert!(!prefs.start_with_os, "nothing installs itself into login without consent");
        assert!(prefs.contain_fullscreen, "a wrapped window is its own screen");
        assert_eq!(prefs.language, None, "follow the system until told otherwise");
        assert_eq!(prefs.theme, None, "match the desktop until told otherwise");
    }

    #[test]
    fn missing_fields_fall_back_to_defaults() {
        let prefs: Preferences = serde_json::from_str("{}").unwrap();
        assert_eq!(prefs, Preferences::default());

        let partial: Preferences = serde_json::from_str(r#"{"desktop_shortcut":false}"#).unwrap();
        assert!(!partial.desktop_shortcut);
        assert!(partial.adblock_by_default);
    }
}

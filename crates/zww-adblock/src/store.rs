use crate::sources::{
    Source, DEFAULT_SOURCES, RESOURCES_URL, SCRIPTLETS_DIR_API, SCRIPTLETS_URL, SEED_LIST,
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub const DEFAULT_MAX_AGE_SECS: u64 = 4 * 24 * 60 * 60;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CacheMeta {
    pub fetched_at: u64,

    pub lists: Vec<String>,
}

pub struct Assets {
    pub lists: Vec<String>,

    pub resources_json: Option<String>,

    pub scriptlets_path: Option<PathBuf>,

    pub is_seed_only: bool,
}

pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn open() -> Result<Self> {
        let root = zww_core::paths::data_dir()?.join("adblock");
        std::fs::create_dir_all(root.join("lists"))
            .with_context(|| format!("creating {}", root.display()))?;
        Ok(Self { root })
    }

    pub fn at(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        std::fs::create_dir_all(root.join("lists"))?;
        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn meta_path(&self) -> PathBuf {
        self.root.join("meta.json")
    }

    fn list_path(&self, name: &str) -> PathBuf {
        self.root.join("lists").join(name)
    }

    fn resources_path(&self) -> PathBuf {
        self.root.join("resources.json")
    }

    fn scriptlets_path(&self) -> PathBuf {
        self.root.join("scriptlets.js")
    }

    pub fn meta(&self) -> CacheMeta {
        std::fs::read_to_string(self.meta_path())
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    pub fn age_secs(&self) -> Option<u64> {
        let fetched = self.meta().fetched_at;
        if fetched == 0 {
            return None;
        }
        Some(now_secs().saturating_sub(fetched))
    }

    pub fn is_stale(&self, max_age: u64) -> bool {
        self.age_secs().is_none_or(|age| age > max_age)
    }

    pub fn load_cached(&self) -> Assets {
        let mut lists: Vec<String> = Vec::new();
        for source in DEFAULT_SOURCES {
            if let Ok(text) = std::fs::read_to_string(self.list_path(source.name)) {
                if !text.trim().is_empty() {
                    lists.push(text);
                }
            }
        }

        let resources_json =
            std::fs::read_to_string(self.resources_path()).ok().filter(|s| !s.trim().is_empty());

        let scriptlets = self.scriptlets_path();
        let scriptlets_path =
            scriptlets.metadata().ok().filter(|m| m.len() > 0).map(|_| scriptlets);

        if lists.is_empty() {
            return Assets {
                lists: vec![SEED_LIST.to_string()],
                resources_json,
                scriptlets_path,
                is_seed_only: true,
            };
        }
        Assets { lists, resources_json, scriptlets_path, is_seed_only: false }
    }

    pub async fn refresh(&self, client: &reqwest::Client) -> Result<Assets> {
        let mut fetched_names = Vec::new();

        for source in DEFAULT_SOURCES {
            match fetch_text(client, source.url).await {
                Ok(text) if !text.trim().is_empty() => {
                    if let Err(e) = std::fs::write(self.list_path(source.name), &text) {
                        tracing::warn!(list = source.name, error = %e, "could not cache filter list");
                        continue;
                    }
                    tracing::info!(list = source.name, bytes = text.len(), "filter list updated");
                    fetched_names.push(source.name.to_string());
                }
                Ok(_) => tracing::warn!(list = source.name, "filter list was empty"),
                Err(e) => {
                    tracing::warn!(list = source.name, error = %e, "could not download filter list")
                }
            }
        }

        match fetch_scriptlet_library(client).await {
            Ok(text) if text.contains("builtinScriptlets.push") || text.contains("/// ") => {
                if let Err(e) = std::fs::write(self.scriptlets_path(), &text) {
                    tracing::warn!(error = %e, "could not cache scriptlets");
                } else {
                    tracing::info!(bytes = text.len(), "uBlock Origin scriptlets updated");
                }
            }
            Ok(_) => tracing::warn!("scriptlets file was not in the expected format"),
            Err(e) => tracing::warn!(error = %e, "could not download scriptlets"),
        }

        match fetch_text(client, RESOURCES_URL).await {
            Ok(text) if text.trim_start().starts_with('[') => {
                if let Err(e) = std::fs::write(self.resources_path(), &text) {
                    tracing::warn!(error = %e, "could not cache extra resources");
                } else {
                    tracing::info!(bytes = text.len(), "extra resources updated");
                }
            }
            Ok(_) => tracing::warn!("extra resources were not a JSON array"),
            Err(e) => tracing::warn!(error = %e, "could not download extra resources"),
        }

        if !fetched_names.is_empty() {
            let meta = CacheMeta { fetched_at: now_secs(), lists: fetched_names };
            if let Ok(json) = serde_json::to_string_pretty(&meta) {
                let _ = std::fs::write(self.meta_path(), json);
            }
        }

        Ok(self.load_cached())
    }

    pub async fn ensure(&self, client: &reqwest::Client, max_age: u64) -> Assets {
        if self.is_stale(max_age) {
            tracing::info!("filter lists are stale; refreshing");
            match self.refresh(client).await {
                Ok(assets) => return assets,
                Err(e) => tracing::warn!(error = %e, "refresh failed; using whatever is cached"),
            }
        }
        self.load_cached()
    }

    pub fn source_list() -> &'static [Source] {
        DEFAULT_SOURCES
    }
}

async fn fetch_text(client: &reqwest::Client, url: &str) -> Result<String> {
    let response = client.get(url).send().await?.error_for_status()?;
    Ok(response.text().await?)
}

async fn fetch_scriptlet_library(client: &reqwest::Client) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct Entry {
        name: String,
        download_url: Option<String>,
    }

    let listing: Vec<Entry> = match client
        .get(SCRIPTLETS_DIR_API)
        .header("User-Agent", "zwebwrap")
        .send()
        .await
        .and_then(|r| r.error_for_status())
    {
        Ok(response) => response.json().await.unwrap_or_default(),
        Err(e) => {
            tracing::warn!(error = %e, "could not list the scriptlet directory; using the main file only");
            return fetch_text(client, SCRIPTLETS_URL).await;
        }
    };

    let files: Vec<(String, String)> = listing
        .into_iter()
        .filter(|e| e.name.ends_with(".js"))
        .filter_map(|e| e.download_url.map(|url| (e.name, url)))
        .collect();

    if files.is_empty() {
        return fetch_text(client, SCRIPTLETS_URL).await;
    }

    let mut combined = String::new();
    let mut fetched = 0;
    for (name, url) in &files {
        match fetch_text(client, url).await {
            Ok(text) => {
                combined.push_str(&text);
                combined.push('\n');
                fetched += 1;
            }

            Err(e) => tracing::debug!(file = %name, error = %e, "scriptlet file skipped"),
        }
    }

    tracing::info!(files = fetched, total = files.len(), "scriptlet library downloaded");
    anyhow::ensure!(fetched > 0, "no scriptlet files could be downloaded");
    Ok(combined)
}

fn now_secs() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_cache_falls_back_to_the_seed_list() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();

        let assets = store.load_cached();
        assert!(assets.is_seed_only);
        assert_eq!(assets.lists.len(), 1);
        assert!(assets.lists[0].contains("doubleclick.net"));
    }

    #[test]
    fn cached_lists_are_preferred_over_the_seed() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        std::fs::write(store.list_path("easylist.txt"), "||ads.example.com^\n").unwrap();

        let assets = store.load_cached();
        assert!(!assets.is_seed_only);
        assert_eq!(assets.lists.len(), 1);
        assert!(assets.lists[0].contains("ads.example.com"));
    }

    #[test]
    fn a_never_refreshed_cache_is_stale() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        assert!(store.age_secs().is_none());
        assert!(store.is_stale(DEFAULT_MAX_AGE_SECS));
    }

    #[test]
    fn a_fresh_stamp_is_not_stale() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        let meta = CacheMeta { fetched_at: now_secs(), lists: vec!["easylist.txt".into()] };
        std::fs::write(store.meta_path(), serde_json::to_string(&meta).unwrap()).unwrap();

        assert!(!store.is_stale(DEFAULT_MAX_AGE_SECS));
        assert!(store.age_secs().unwrap() < 5);
    }

    #[test]
    fn blank_cached_files_are_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        std::fs::write(store.list_path("easylist.txt"), "   \n\n").unwrap();

        assert!(store.load_cached().is_seed_only);
    }
}

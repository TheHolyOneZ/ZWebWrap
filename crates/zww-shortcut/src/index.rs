use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const FORBIDDEN: [char; 9] = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];

pub fn safe_filename(title: &str) -> String {
    const RESERVED: [&str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    let cleaned: String = title
        .chars()
        .map(|c| if FORBIDDEN.contains(&c) || c.is_control() { '-' } else { c })
        .collect();

    let cleaned = cleaned.trim().trim_end_matches(['.', ' ']).trim().to_string();

    if cleaned.is_empty() {
        return "ZWebWrap app".to_string();
    }
    if RESERVED.iter().any(|r| cleaned.eq_ignore_ascii_case(r)) {
        return format!("{cleaned} app");
    }

    cleaned.chars().take(200).collect()
}

#[derive(Debug, Default, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Index(BTreeMap<String, Vec<PathBuf>>);

impl Index {
    pub fn load(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|e| {
                tracing::warn!(error = %e, "shortcut index unreadable; starting fresh");
                Self::default()
            }),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("creating {}", parent.display()))?;
        }
        let json = serde_json::to_string_pretty(self)?;

        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, json).with_context(|| format!("writing {}", tmp.display()))?;
        std::fs::rename(&tmp, path).with_context(|| format!("renaming into {}", path.display()))?;
        Ok(())
    }

    pub fn set(&mut self, id: &str, paths: Vec<PathBuf>) {
        self.0.insert(id.to_string(), paths);
    }

    pub fn get(&self, id: &str) -> &[PathBuf] {
        self.0.get(id).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn owner_of(&self, path: &Path) -> Option<&str> {
        self.0.iter().find(|(_, paths)| paths.iter().any(|p| p == path)).map(|(id, _)| id.as_str())
    }

    pub fn take(&mut self, id: &str) -> Vec<PathBuf> {
        self.0.remove(id).unwrap_or_default()
    }

    pub fn all_existing(&self) -> Vec<PathBuf> {
        let mut found: Vec<PathBuf> =
            self.0.values().flatten().filter(|p| p.is_file()).cloned().collect();
        found.sort();
        found
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forbidden_characters_are_replaced_not_dropped() {
        assert_eq!(safe_filename("AC/DC: Live?"), "AC-DC- Live-");
        assert_eq!(safe_filename(r"a\b|c*d"), "a-b-c-d");
    }

    #[test]
    fn ordinary_titles_are_left_alone() {
        assert_eq!(safe_filename("YouTube"), "YouTube");
        assert_eq!(safe_filename("Hacker News"), "Hacker News");

        assert_eq!(safe_filename("Übersicht — Nachrichten"), "Übersicht — Nachrichten");
    }

    #[test]
    fn reserved_device_names_get_out_of_the_way() {
        assert_eq!(safe_filename("CON"), "CON app");
        assert_eq!(safe_filename("nul"), "nul app");
        assert_eq!(safe_filename("COM1"), "COM1 app");

        assert_eq!(safe_filename("Console"), "Console");
    }

    #[test]
    fn trailing_dots_and_spaces_go_because_the_shell_strips_them() {
        assert_eq!(safe_filename("News. "), "News");
        assert_eq!(safe_filename("  Spaced  "), "Spaced");
    }

    #[test]
    fn a_title_with_nothing_usable_still_yields_a_filename() {
        assert_eq!(safe_filename(""), "ZWebWrap app");
        assert_eq!(safe_filename("   "), "ZWebWrap app");
        assert_eq!(safe_filename("..."), "ZWebWrap app");
    }

    #[test]
    fn very_long_titles_are_cut_to_fit_a_filename() {
        let long = "a".repeat(400);
        assert_eq!(safe_filename(&long).chars().count(), 200);
    }

    #[test]
    fn an_index_round_trips_through_disk() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("shortcuts.json");

        let mut index = Index::default();
        index.set("youtube-com", vec![PathBuf::from("/a/YouTube.lnk")]);
        index.save(&path).unwrap();

        let read = Index::load(&path);
        assert_eq!(read, index);
        assert_eq!(read.get("youtube-com"), [PathBuf::from("/a/YouTube.lnk")]);
        assert!(read.get("never-made").is_empty());
    }

    #[test]
    fn a_damaged_index_reads_as_empty_rather_than_failing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("shortcuts.json");
        std::fs::write(&path, "{ this is not json").unwrap();

        assert_eq!(Index::load(&path), Index::default());
    }

    #[test]
    fn taking_an_id_yields_its_paths_and_forgets_it() {
        let mut index = Index::default();
        index.set("x", vec![PathBuf::from("/a.lnk"), PathBuf::from("/b.lnk")]);

        assert_eq!(index.take("x").len(), 2);
        assert!(index.get("x").is_empty());

        assert!(index.take("x").is_empty());
    }

    #[test]
    fn listing_skips_files_that_are_no_longer_there() {
        let dir = tempfile::tempdir().unwrap();
        let real = dir.path().join("Real.lnk");
        std::fs::write(&real, "x").unwrap();

        let mut index = Index::default();
        index.set("a", vec![real.clone()]);
        index.set("b", vec![dir.path().join("DeletedByHand.lnk")]);

        assert_eq!(index.all_existing(), vec![real]);
    }

    #[test]
    fn no_temp_file_is_left_behind_after_a_save() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("shortcuts.json");
        Index::default().save(&path).unwrap();

        let leftovers: Vec<_> = std::fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "tmp"))
            .collect();
        assert!(leftovers.is_empty());
    }
}

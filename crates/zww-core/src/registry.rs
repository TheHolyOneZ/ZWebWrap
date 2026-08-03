use crate::model::{unique_id, WindowState, WrappedApp};
use crate::paths;
use anyhow::{Context, Result};
use fs2::FileExt;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Registry {
    path: PathBuf,
    lock_path: PathBuf,
}

impl Registry {
    pub fn open() -> Result<Self> {
        Ok(Self { path: paths::registry_file()?, lock_path: paths::registry_lock_file()? })
    }

    pub fn at(path: impl Into<PathBuf>, lock_path: impl Into<PathBuf>) -> Self {
        Self { path: path.into(), lock_path: lock_path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn lock(&self) -> Result<File> {
        if let Some(parent) = self.lock_path.parent() {
            fs::create_dir_all(parent).ok();
        }
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(&self.lock_path)
            .with_context(|| format!("opening lock file {}", self.lock_path.display()))?;
        file.lock_exclusive().context("acquiring registry lock")?;
        Ok(file)
    }

    fn read_unlocked(&self) -> Result<Vec<WrappedApp>> {
        let raw = match fs::read_to_string(&self.path) {
            Ok(raw) => raw,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e).with_context(|| format!("reading {}", self.path.display())),
        };
        if raw.trim().is_empty() {
            return Ok(Vec::new());
        }
        match serde_json::from_str(&raw) {
            Ok(apps) => Ok(apps),
            Err(e) => {
                let backup =
                    self.path.with_extension(format!("json.corrupt-{}", crate::model::now_secs()));
                let _ = fs::rename(&self.path, &backup);
                tracing::error!(
                    error = %e,
                    backup = %backup.display(),
                    "registry was unreadable; moved aside and starting empty"
                );
                Ok(Vec::new())
            }
        }
    }

    fn write_unlocked(&self, apps: &[WrappedApp]) -> Result<()> {
        let parent = self.path.parent().context("registry path has no parent directory")?;
        fs::create_dir_all(parent)?;

        let json = serde_json::to_vec_pretty(apps)?;
        let tmp = parent.join(format!(".apps.json.tmp.{}", std::process::id()));
        {
            let mut f =
                File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
            f.write_all(&json)?;

            f.sync_all()?;
        }
        fs::rename(&tmp, &self.path)
            .with_context(|| format!("replacing {}", self.path.display()))?;
        Ok(())
    }

    fn mutate<T>(&self, f: impl FnOnce(&mut Vec<WrappedApp>) -> Result<(T, bool)>) -> Result<T> {
        let _guard = self.lock()?;
        let mut apps = self.read_unlocked()?;
        let (out, changed) = f(&mut apps)?;
        if changed {
            self.write_unlocked(&apps)?;
        }
        Ok(out)
    }

    pub fn list(&self) -> Result<Vec<WrappedApp>> {
        let _guard = self.lock()?;
        self.read_unlocked()
    }

    pub fn get(&self, id: &str) -> Result<Option<WrappedApp>> {
        Ok(self.list()?.into_iter().find(|a| a.id == id))
    }

    pub fn upsert(&self, app: WrappedApp) -> Result<()> {
        self.mutate(|apps| {
            match apps.iter_mut().find(|a| a.id == app.id) {
                Some(slot) => *slot = app,
                None => apps.push(app),
            }
            Ok(((), true))
        })
    }

    pub fn remove(&self, id: &str) -> Result<Option<WrappedApp>> {
        self.mutate(|apps| match apps.iter().position(|a| a.id == id) {
            Some(i) => {
                let removed = apps.remove(i);
                Ok((Some(removed), true))
            }
            None => Ok((None, false)),
        })
    }

    pub fn allocate_id(&self, url: &url::Url) -> Result<String> {
        let taken: Vec<String> = self.list()?.into_iter().map(|a| a.id).collect();
        Ok(unique_id(url, &taken))
    }

    pub fn set_window(&self, id: &str, state: WindowState) -> Result<()> {
        self.mutate(|apps| match apps.iter_mut().find(|a| a.id == id) {
            Some(app) if app.window != state => {
                app.window = state;
                Ok(((), true))
            }
            _ => Ok(((), false)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    fn temp_registry() -> (tempfile::TempDir, Registry) {
        let dir = tempfile::tempdir().unwrap();
        let reg = Registry::at(dir.path().join("apps.json"), dir.path().join("apps.lock"));
        (dir, reg)
    }

    #[test]
    fn missing_file_reads_as_empty() {
        let (_d, reg) = temp_registry();
        assert!(reg.list().unwrap().is_empty());
    }

    #[test]
    fn upsert_then_get_roundtrips() {
        let (_d, reg) = temp_registry();
        reg.upsert(WrappedApp::new("youtube-com", "https://youtube.com", "YouTube")).unwrap();
        let got = reg.get("youtube-com").unwrap().unwrap();
        assert_eq!(got.title, "YouTube");
        assert_eq!(reg.list().unwrap().len(), 1);
    }

    #[test]
    fn upsert_replaces_rather_than_duplicating() {
        let (_d, reg) = temp_registry();
        reg.upsert(WrappedApp::new("x", "https://x.com", "Old")).unwrap();
        reg.upsert(WrappedApp::new("x", "https://x.com", "New")).unwrap();
        let all = reg.list().unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].title, "New");
    }

    #[test]
    fn remove_reports_what_it_removed() {
        let (_d, reg) = temp_registry();
        reg.upsert(WrappedApp::new("x", "https://x.com", "X")).unwrap();
        assert!(reg.remove("x").unwrap().is_some());
        assert!(reg.remove("x").unwrap().is_none());
        assert!(reg.list().unwrap().is_empty());
    }

    #[test]
    fn allocate_id_avoids_existing_ids() {
        let (_d, reg) = temp_registry();
        reg.upsert(WrappedApp::new("youtube-com", "https://youtube.com", "YT")).unwrap();
        let id = reg.allocate_id(&Url::parse("https://www.youtube.com/feed").unwrap()).unwrap();
        assert_eq!(id, "youtube-com-2");
    }

    #[test]
    fn corrupt_registry_is_moved_aside_not_fatal() {
        let (dir, reg) = temp_registry();
        fs::write(reg.path(), b"{ not json at all").unwrap();

        assert!(reg.list().unwrap().is_empty());

        let salvaged = fs::read_dir(dir.path())
            .unwrap()
            .filter_map(|e| e.ok())
            .any(|e| e.file_name().to_string_lossy().contains("corrupt-"));
        assert!(salvaged, "corrupt registry should be backed up, not deleted");
    }

    #[test]
    fn set_window_updates_geometry_only() {
        let (_d, reg) = temp_registry();
        reg.upsert(WrappedApp::new("x", "https://x.com", "X")).unwrap();
        let state = WindowState {
            width: 640.0,
            height: 480.0,
            x: Some(10.0),
            y: Some(20.0),
            maximized: false,
        };
        reg.set_window("x", state).unwrap();
        let got = reg.get("x").unwrap().unwrap();
        assert_eq!(got.window, state);
        assert_eq!(got.title, "X");
    }
}

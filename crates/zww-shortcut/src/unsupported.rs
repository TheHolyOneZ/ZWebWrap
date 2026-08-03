use crate::{ShortcutBackend, ShortcutSpec};
use anyhow::{bail, Result};
use std::path::PathBuf;

pub struct NoShortcuts;

impl ShortcutBackend for NoShortcuts {
    fn create(&self, _spec: &ShortcutSpec) -> Result<PathBuf> {
        bail!("ZWebWrap does not create shortcuts on this platform yet (Linux and Windows only)")
    }

    fn remove(&self, _id: &str) -> Result<()> {
        Ok(())
    }

    fn list(&self) -> Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }

    fn desktop_copy(&self, _id: &str) -> Option<PathBuf> {
        None
    }
}

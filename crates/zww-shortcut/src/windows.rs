use crate::index::{safe_filename, Index};
use crate::{ShortcutBackend, ShortcutSpec};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const MENU_FOLDER: &str = "ZWebWrap";

pub struct ShellLinks {
    index_path: PathBuf,
}

impl ShellLinks {
    pub fn new() -> Self {
        Self { index_path: index_path() }
    }

    pub fn with_index(path: impl Into<PathBuf>) -> Self {
        Self { index_path: path.into() }
    }
}

impl Default for ShellLinks {
    fn default() -> Self {
        Self::new()
    }
}

fn index_path() -> PathBuf {
    zww_core::paths::data_dir()
        .map(|d| d.join("shortcuts.json"))
        .unwrap_or_else(|_| PathBuf::from("shortcuts.json"))
}

pub const START_MENU_DIR_VAR: &str = "ZWEBWRAP_START_MENU_DIR";
pub const DESKTOP_DIR_VAR: &str = "ZWEBWRAP_DESKTOP_DIR";

fn dir_override(var: &str) -> Option<PathBuf> {
    std::env::var_os(var).filter(|v| !v.is_empty()).map(PathBuf::from)
}

fn start_menu_dir() -> Option<PathBuf> {
    if let Some(dir) = dir_override(START_MENU_DIR_VAR) {
        return Some(dir);
    }
    let appdata = std::env::var_os("APPDATA").filter(|v| !v.is_empty())?;
    Some(
        PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join(MENU_FOLDER),
    )
}

fn desktop_dir() -> Option<PathBuf> {
    if let Some(dir) = dir_override(DESKTOP_DIR_VAR) {
        return Some(dir);
    }
    if let Some(dir) =
        directories::UserDirs::new().and_then(|d| d.desktop_dir().map(Path::to_path_buf))
    {
        return Some(dir);
    }
    let profile = std::env::var_os("USERPROFILE").filter(|v| !v.is_empty())?;
    let fallback = PathBuf::from(profile).join("Desktop");
    fallback.is_dir().then_some(fallback)
}

fn icon_for(spec: &ShortcutSpec) -> Option<PathBuf> {
    let png = spec.icon.as_ref()?;
    match crate::ico::convert_file(png) {
        Ok(path) => Some(path),
        Err(e) => {
            tracing::warn!(
                error = %e,
                path = %png.display(),
                "could not build an .ico; using the default icon"
            );
            None
        }
    }
}

fn unique_filename(index: &Index, menu: &Path, title: &str) -> String {
    let base = safe_filename(title);
    for n in 1..100 {
        let candidate = if n == 1 { format!("{base}.lnk") } else { format!("{base} ({n}).lnk") };
        if index.owner_of(&menu.join(&candidate)).is_none() {
            return candidate;
        }
    }

    format!("{base}.lnk")
}

fn write_link(target: &Path, spec: &ShortcutSpec, icon: Option<&Path>) -> Result<()> {
    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }

    let mut link = mslnk::ShellLink::new(&spec.exec)
        .with_context(|| format!("preparing a shortcut to {}", spec.exec.display()))?;

    link.set_arguments(Some(format!("--app \"{}\"", spec.id)));
    link.set_name(Some(spec.name.clone()));
    if let Some(icon) = icon {
        link.set_icon_location(Some(icon.to_string_lossy().to_string()));
    }
    if let Some(dir) = spec.exec.parent() {
        link.set_working_dir(Some(dir.to_string_lossy().to_string()));
    }

    link.create_lnk(target).with_context(|| format!("writing {}", target.display()))?;
    stamp_app_user_model_id(target, &crate::wm_class(&spec.id));
    tracing::info!(path = %target.display(), "wrote shortcut");
    Ok(())
}

fn stamp_app_user_model_id(path: &Path, app_user_model_id: &str) {
    use windows::core::{Interface, HSTRING, PCWSTR};
    use windows::Win32::Foundation::PROPERTYKEY;
    use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, IPersistFile, CLSCTX_INPROC_SERVER,
        COINIT_APARTMENTTHREADED, STGM_READWRITE,
    };
    use windows::Win32::UI::Shell::PropertiesSystem::IPropertyStore;
    use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};

    const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
        fmtid: windows::core::GUID::from_u128(0x9F4C2855_9F79_4B39_A8D0_E1D42DE1D5F3),
        pid: 5,
    };

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let result = (|| -> windows::core::Result<()> {
            let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
            let file: IPersistFile = link.cast()?;

            let wide = HSTRING::from(path.to_string_lossy().as_ref());
            file.Load(PCWSTR(wide.as_ptr()), STGM_READWRITE)?;

            let store: IPropertyStore = link.cast()?;
            store.SetValue(&PKEY_APP_USER_MODEL_ID, &PROPVARIANT::from(app_user_model_id))?;
            store.Commit()?;

            file.Save(PCWSTR::null(), true)?;
            Ok(())
        })();

        match result {
            Ok(()) => {
                tracing::debug!(path = %path.display(), app_user_model_id, "stamped shortcut identity")
            }
            Err(e) => tracing::warn!(
                error = %e,
                path = %path.display(),
                "could not set the shortcut's AppUserModelID; the taskbar may group this app with the others"
            ),
        }
    }
}

impl ShortcutBackend for ShellLinks {
    fn create(&self, spec: &ShortcutSpec) -> Result<PathBuf> {
        let icon = icon_for(spec);

        let mut index = Index::load(&self.index_path);

        for stale in index.take(&spec.id) {
            let _ = std::fs::remove_file(stale);
        }

        let menu = start_menu_dir().context(
            "no Start Menu folder: APPDATA is not set, so this is not a normal user session",
        )?;
        let filename = unique_filename(&index, &menu, &spec.name);
        let menu_link = menu.join(&filename);
        write_link(&menu_link, spec, icon.as_deref())?;

        let mut written = vec![menu_link.clone()];

        if spec.on_desktop {
            match desktop_dir() {
                Some(desktop) => {
                    let desktop_link = desktop.join(&filename);
                    match write_link(&desktop_link, spec, icon.as_deref()) {
                        Ok(()) => written.push(desktop_link),

                        Err(e) => tracing::warn!(error = %e, "could not write the desktop copy"),
                    }
                }
                None => tracing::debug!("no Desktop folder found; Start Menu entry only"),
            }
        }

        index.set(&spec.id, written);
        index.save(&self.index_path)?;

        Ok(menu_link)
    }

    fn remove(&self, id: &str) -> Result<()> {
        let mut index = Index::load(&self.index_path);
        for path in index.take(id) {
            match std::fs::remove_file(&path) {
                Ok(()) => tracing::debug!(path = %path.display(), "removed shortcut"),

                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    tracing::warn!(error = %e, path = %path.display(), "could not remove shortcut")
                }
            }
        }
        index.save(&self.index_path)
    }

    fn list(&self) -> Result<Vec<PathBuf>> {
        Ok(Index::load(&self.index_path).all_existing())
    }

    fn desktop_copy(&self, id: &str) -> Option<PathBuf> {
        let desktop = desktop_dir()?;
        Index::load(&self.index_path)
            .get(id)
            .iter()
            .find(|p| p.parent() == Some(desktop.as_path()) && p.is_file())
            .cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_apps_with_the_same_title_get_different_files() {
        let menu = Path::new(r"C:\menu");
        let mut index = Index::default();

        let first = unique_filename(&index, menu, "Docs");
        assert_eq!(first, "Docs.lnk");
        index.set("docs-a", vec![menu.join(&first)]);

        let second = unique_filename(&index, menu, "Docs");
        assert_eq!(second, "Docs (2).lnk");
        index.set("docs-b", vec![menu.join(&second)]);

        assert_eq!(unique_filename(&index, menu, "Docs"), "Docs (3).lnk");
    }

    #[test]
    fn rewriting_the_same_app_keeps_its_filename() {
        let menu = Path::new(r"C:\menu");
        let index = Index::default();
        assert_eq!(unique_filename(&index, menu, "Docs"), "Docs.lnk");
    }

    #[test]
    fn a_collision_only_counts_in_the_same_folder() {
        let mut index = Index::default();
        index.set("docs-a", vec![Path::new(r"C:\desktop").join("Docs.lnk")]);

        assert_eq!(unique_filename(&index, Path::new(r"C:\menu"), "Docs"), "Docs.lnk");
    }

    #[test]
    fn an_unusable_title_still_produces_a_name() {
        let index = Index::default();
        assert_eq!(unique_filename(&index, Path::new(r"C:\menu"), "  "), "ZWebWrap app.lnk");
    }
}

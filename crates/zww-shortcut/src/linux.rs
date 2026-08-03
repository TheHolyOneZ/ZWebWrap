use crate::{wm_class, ShortcutBackend, ShortcutSpec};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

const PREFIX: &str = "zwebwrap-";

pub struct DesktopEntries {
    dir: PathBuf,

    desktop: Option<PathBuf>,
}

impl DesktopEntries {
    pub fn new() -> Self {
        Self { dir: applications_dir(), desktop: None }
    }

    pub fn in_dir(dir: impl Into<PathBuf>) -> Self {
        let dir = dir.into();

        let desktop = dir.join("desktop-copy");
        std::fs::create_dir_all(&desktop).ok();
        Self { dir, desktop: Some(desktop) }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn path_for(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{PREFIX}{id}.desktop"))
    }
}

impl Default for DesktopEntries {
    fn default() -> Self {
        Self::new()
    }
}

fn applications_dir() -> PathBuf {
    if let Some(xdg) = std::env::var_os("XDG_DATA_HOME").filter(|v| !v.is_empty()) {
        return PathBuf::from(xdg).join("applications");
    }
    directories::BaseDirs::new()
        .map(|d| d.data_dir().join("applications"))
        .unwrap_or_else(|| PathBuf::from("applications"))
}

pub fn desktop_dir() -> Option<PathBuf> {
    if let Some(dir) = std::env::var_os("XDG_DESKTOP_DIR").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(dir));
    }

    let home = directories::BaseDirs::new()?.home_dir().to_path_buf();
    let config = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|v| !v.is_empty())
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".config"));

    if let Ok(text) = std::fs::read_to_string(config.join("user-dirs.dirs")) {
        for line in text.lines() {
            let line = line.trim();
            let Some(value) = line.strip_prefix("XDG_DESKTOP_DIR=") else { continue };
            let value = value.trim().trim_matches('"');
            let expanded = match value.strip_prefix("$HOME/") {
                Some(rest) => home.join(rest),
                None if value == "$HOME" => home.clone(),
                None => PathBuf::from(value),
            };
            return expanded.is_dir().then_some(expanded);
        }
    }

    let fallback = home.join("Desktop");
    fallback.is_dir().then_some(fallback)
}

fn mark_trusted(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755));

    let _ = std::process::Command::new("gio")
        .args(["set", &path.to_string_lossy(), "metadata::trusted", "true"])
        .status();
}

fn escape_value(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '\\' => out.push_str(r"\\"),
            '\n' => out.push_str(r"\n"),
            '\r' => out.push_str(r"\r"),
            '\t' => out.push_str(r"\t"),
            _ => out.push(c),
        }
    }
    out
}

fn escape_exec_arg(arg: &str) -> String {
    let mut inner = String::with_capacity(arg.len() + 2);
    for c in arg.chars() {
        if matches!(c, '"' | '`' | '$' | '\\') {
            inner.push_str(r"\\");
        }
        inner.push(c);
    }
    format!("\"{inner}\"")
}

pub fn render_entry(spec: &ShortcutSpec) -> String {
    let mut out = String::new();
    out.push_str("[Desktop Entry]\n");
    out.push_str("Type=Application\n");
    out.push_str("Version=1.0\n");
    out.push_str(&format!("Name={}\n", escape_value(&spec.name)));
    out.push_str(&format!("Comment={}\n", escape_value(&spec.url)));
    out.push_str(&format!(
        "Exec={} --app {}\n",
        escape_exec_arg(&spec.exec.to_string_lossy()),
        escape_exec_arg(&spec.id)
    ));
    if let Some(icon) = &spec.icon {
        out.push_str(&format!("Icon={}\n", escape_value(&icon.to_string_lossy())));
    }
    out.push_str("Terminal=false\n");
    out.push_str("Categories=Network;\n");
    out.push_str(&format!("StartupWMClass={}\n", escape_value(&wm_class(&spec.id))));
    out.push_str("StartupNotify=true\n");

    out.push_str(&format!("X-ZWebWrap-Id={}\n", escape_value(&spec.id)));
    out
}

fn refresh_database(dir: &Path) {
    match std::process::Command::new("update-desktop-database").arg(dir).status() {
        Ok(status) if status.success() => {}
        Ok(status) => tracing::debug!(?status, "update-desktop-database returned non-zero"),
        Err(e) => tracing::debug!(error = %e, "update-desktop-database unavailable"),
    }
}

impl ShortcutBackend for DesktopEntries {
    fn create(&self, spec: &ShortcutSpec) -> Result<PathBuf> {
        std::fs::create_dir_all(&self.dir)
            .with_context(|| format!("creating {}", self.dir.display()))?;
        let path = self.path_for(&spec.id);
        let entry = render_entry(spec);
        std::fs::write(&path, &entry).with_context(|| format!("writing {}", path.display()))?;

        mark_trusted(&path);
        refresh_database(&self.dir);
        tracing::info!(path = %path.display(), "wrote desktop entry");

        if let Some(desktop) = self.desktop.clone().or_else(desktop_dir) {
            let on_desktop = desktop.join(format!("{PREFIX}{}.desktop", spec.id));

            if !spec.on_desktop {
                let _ = std::fs::remove_file(&on_desktop);
                return Ok(path);
            }

            match std::fs::write(&on_desktop, &entry) {
                Ok(()) => {
                    mark_trusted(&on_desktop);
                    tracing::info!(path = %on_desktop.display(), "placed shortcut on the desktop");
                }
                Err(e) => {
                    tracing::warn!(error = %e, path = %on_desktop.display(), "could not write the desktop copy");
                }
            }
        } else {
            tracing::debug!("no Desktop folder on this system; menu entry only");
        }

        Ok(path)
    }

    fn remove(&self, id: &str) -> Result<()> {
        if let Some(desktop) = self.desktop.clone().or_else(desktop_dir) {
            let _ = std::fs::remove_file(desktop.join(format!("{PREFIX}{id}.desktop")));
        }

        let path = self.path_for(id);
        match std::fs::remove_file(&path) {
            Ok(()) => {
                refresh_database(&self.dir);
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e).with_context(|| format!("removing {}", path.display())),
        }
    }

    fn desktop_copy(&self, id: &str) -> Option<PathBuf> {
        let desktop = self.desktop.clone().or_else(desktop_dir)?;
        let path = desktop.join(format!("{PREFIX}{id}.desktop"));
        path.is_file().then_some(path)
    }

    fn list(&self) -> Result<Vec<PathBuf>> {
        let entries = match std::fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(e).with_context(|| format!("reading {}", self.dir.display())),
        };
        let mut found: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().is_some_and(|ext| ext == "desktop")
                    && p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with(PREFIX))
            })
            .collect();
        found.sort();
        Ok(found)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zww_core::WrappedApp;

    #[test]
    #[ignore = "requires desktop-file-validate"]
    fn a_generated_entry_satisfies_the_freedesktop_spec() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("zwebwrap-probe.desktop");

        let spec = ShortcutSpec {
            id: "youtube-com".into(),
            name: "O'Brien's Übersicht".into(),
            url: "https://example.com/a?b=c&d=e".into(),
            exec: "/home/me/Apps/ZWebWrap-0.1.0 x86_64.AppImage".into(),
            icon: Some("/home/me/.local/share/zwebwrap/icons/youtube-com.png".into()),
            on_desktop: false,
        };
        std::fs::write(&path, render_entry(&spec)).unwrap();

        let out = std::process::Command::new("desktop-file-validate").arg(&path).output();
        let Ok(out) = out else { return };
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout) + String::from_utf8_lossy(&out.stderr)
        );
    }

    #[test]
    fn a_per_app_setting_adds_and_removes_the_desktop_copy() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());

        let mut app = WrappedApp::new("youtube-com", "https://youtube.com", "YouTube");

        app.desktop_shortcut = Some(false);
        backend.create(&ShortcutSpec::from_app(&app, "/usr/bin/zwebwrap".into())).unwrap();
        assert!(
            backend.desktop_copy("youtube-com").is_none(),
            "nothing asked for it, so nothing should be on the desktop"
        );
        assert_eq!(backend.list().unwrap().len(), 1, "the menu entry exists regardless");

        app.desktop_shortcut = Some(true);
        backend.create(&ShortcutSpec::from_app(&app, "/usr/bin/zwebwrap".into())).unwrap();
        let copy = backend.desktop_copy("youtube-com").expect("a desktop copy");
        assert!(copy.is_file());

        app.desktop_shortcut = Some(false);
        backend.create(&ShortcutSpec::from_app(&app, "/usr/bin/zwebwrap".into())).unwrap();
        assert!(backend.desktop_copy("youtube-com").is_none());
        assert!(!copy.exists());
    }

    #[test]
    fn a_shortcut_deleted_by_hand_reads_as_absent() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());

        let mut app = WrappedApp::new("x", "https://x.com", "X");
        app.desktop_shortcut = Some(true);
        backend.create(&ShortcutSpec::from_app(&app, "/usr/bin/zwebwrap".into())).unwrap();

        let copy = backend.desktop_copy("x").unwrap();
        std::fs::remove_file(&copy).unwrap();

        assert!(backend.desktop_copy("x").is_none());
    }

    fn spec() -> ShortcutSpec {
        ShortcutSpec {
            id: "youtube-com".into(),
            name: "YouTube".into(),
            url: "https://www.youtube.com/".into(),
            exec: PathBuf::from("/usr/bin/zwebwrap"),
            icon: Some(PathBuf::from("/home/u/.local/share/ZWebWrap/icons/youtube-com.png")),
            on_desktop: true,
        }
    }

    #[test]
    fn entry_has_the_keys_launchers_need() {
        let text = render_entry(&spec());
        assert!(text.starts_with("[Desktop Entry]\n"));
        assert!(text.contains("Name=YouTube\n"));
        assert!(text.contains("Exec=\"/usr/bin/zwebwrap\" --app \"youtube-com\"\n"));
        assert!(text.contains("Icon=/home/u/.local/share/ZWebWrap/icons/youtube-com.png\n"));
        assert!(text.contains("Type=Application\n"));
        assert!(text.contains("Terminal=false\n"));
    }

    #[test]
    fn wm_class_matches_the_window_identity() {
        let text = render_entry(&spec());
        assert!(text.contains(&format!("StartupWMClass={}\n", wm_class("youtube-com"))));
    }

    #[test]
    fn paths_with_spaces_stay_one_argument() {
        let mut s = spec();
        s.exec = PathBuf::from("/home/my user/Apps/ZWebWrap.AppImage");
        let text = render_entry(&s);
        assert!(
            text.contains("Exec=\"/home/my user/Apps/ZWebWrap.AppImage\" --app \"youtube-com\"")
        );
    }

    #[test]
    fn a_newline_in_the_title_cannot_inject_extra_keys() {
        let mut s = spec();
        s.name = "Evil\nExec=/bin/sh -c rm".into();
        let text = render_entry(&s);
        assert!(text.contains(r"Name=Evil\nExec=/bin/sh -c rm"));

        assert_eq!(text.lines().filter(|l| l.starts_with("Exec=")).count(), 1);
        assert!(text.contains("Exec=\"/usr/bin/zwebwrap\""));
    }

    #[test]
    fn shell_metacharacters_in_a_path_are_escaped() {
        let mut s = spec();
        s.exec = PathBuf::from("/opt/$(whoami)/zwebwrap");
        let text = render_entry(&s);
        assert!(text.contains(r#"Exec="/opt/\\$(whoami)/zwebwrap""#));
    }

    #[test]
    fn a_copy_lands_on_the_desktop_too() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());
        backend.create(&spec()).unwrap();

        let on_desktop = dir.path().join("desktop-copy/zwebwrap-youtube-com.desktop");
        assert!(on_desktop.exists(), "expected a copy at {}", on_desktop.display());
        assert!(std::fs::read_to_string(&on_desktop).unwrap().contains("Name=YouTube"));

        backend.remove("youtube-com").unwrap();
        assert!(!on_desktop.exists(), "the desktop copy should be removed too");
    }

    #[test]
    fn turning_the_desktop_copy_off_removes_a_stale_one() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());
        let on_desktop = dir.path().join("desktop-copy/zwebwrap-youtube-com.desktop");

        backend.create(&spec()).unwrap();
        assert!(on_desktop.exists());

        let mut without = spec();
        without.on_desktop = false;
        backend.create(&without).unwrap();
        assert!(!on_desktop.exists(), "the stale desktop copy should be cleared");

        assert!(dir.path().join("zwebwrap-youtube-com.desktop").exists());
    }

    #[test]
    fn create_list_remove_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());

        let path = backend.create(&spec()).unwrap();
        assert!(path.exists());
        assert_eq!(path.file_name().unwrap(), "zwebwrap-youtube-com.desktop");
        assert_eq!(backend.list().unwrap(), vec![path.clone()]);

        backend.remove("youtube-com").unwrap();
        assert!(!path.exists());
        assert!(backend.list().unwrap().is_empty());
    }

    #[test]
    fn removing_a_missing_shortcut_is_not_an_error() {
        let dir = tempfile::tempdir().unwrap();
        DesktopEntries::in_dir(dir.path()).remove("never-existed").unwrap();
    }

    #[test]
    fn list_ignores_files_we_do_not_own() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());
        std::fs::write(dir.path().join("firefox.desktop"), "[Desktop Entry]").unwrap();
        std::fs::write(dir.path().join("zwebwrap-notes.txt"), "x").unwrap();
        backend.create(&spec()).unwrap();

        let listed = backend.list().unwrap();
        assert_eq!(listed.len(), 1);
        assert!(listed[0].to_string_lossy().ends_with("zwebwrap-youtube-com.desktop"));
    }

    #[test]
    fn create_overwrites_an_existing_entry() {
        let dir = tempfile::tempdir().unwrap();
        let backend = DesktopEntries::in_dir(dir.path());
        backend.create(&spec()).unwrap();

        let mut renamed = spec();
        renamed.name = "YouTube Music".into();
        let path = backend.create(&renamed).unwrap();

        let text = std::fs::read_to_string(&path).unwrap();
        assert!(text.contains("Name=YouTube Music\n"));
        assert_eq!(backend.list().unwrap().len(), 1);
    }
}

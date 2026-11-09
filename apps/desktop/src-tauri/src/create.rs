use anyhow::{Context, Result};
use zww_core::site::{self, SiteMeta};
use zww_core::{icon, paths, Registry, WrappedApp};
use zww_shortcut::ShortcutSpec;

pub async fn create_app(input: &str, title: Option<String>, adblock: bool) -> Result<WrappedApp> {
    let url = site::normalize_url(input)?;
    let registry = Registry::open()?;
    let id = registry.allocate_id(&url)?;

    let client = site::client()?;
    let meta: SiteMeta = site::fetch_meta(&client, &url).await?;

    let title = title.map(|t| t.trim().to_string()).filter(|t| !t.is_empty()).unwrap_or(meta.title);

    let icon_path = icon::resolve_and_save(&client, &meta.icons, &id, &title).await;

    let mut app = WrappedApp::new(&id, &meta.url, &title);
    app.icon = (!icon_path.as_os_str().is_empty()).then_some(icon_path);

    app.blocking = if adblock { zww_core::BlockMode::Full } else { zww_core::BlockMode::Off };

    app.user_agent = site::default_user_agent_for(&url).map(str::to_string);

    registry.upsert(app.clone())?;
    write_shortcut(&app).context("the app was saved, but its shortcut could not be created")?;

    Ok(app)
}

pub fn write_shortcut(app: &WrappedApp) -> Result<()> {
    let exec = paths::current_exe()?;
    zww_shortcut::backend().create(&ShortcutSpec::from_app(app, exec))?;
    Ok(())
}

pub fn refresh_shortcuts_if_moved() {
    if cfg!(debug_assertions) {
        tracing::debug!("development build; leaving installed shortcuts alone");
        return;
    }

    let Ok(exe) = zww_core::paths::current_exe() else { return };
    let Ok(marker) = zww_core::paths::data_dir().map(|d| d.join("exe-path.txt")) else { return };

    let previous = std::fs::read_to_string(&marker).unwrap_or_default();
    let current = exe.to_string_lossy();
    if previous == current {
        return;
    }

    if previous.is_empty() {
        tracing::debug!(exe = %current, "first run for this data directory; refreshing shortcuts");
    } else {
        tracing::info!(from = %previous, to = %current, "executable moved; refreshing shortcuts");
    }

    let apps = match Registry::open().and_then(|r| r.list()) {
        Ok(apps) => apps,
        Err(e) => {
            tracing::warn!(error = %e, "could not read the registry to refresh shortcuts");
            return;
        }
    };
    for app in apps {
        if let Err(e) = write_shortcut(&app) {
            tracing::warn!(id = %app.id, error = %e, "could not refresh shortcut");
        }
    }

    if let Err(e) = std::fs::write(&marker, current.as_ref()) {
        tracing::debug!(error = %e, "could not record the executable path");
    }
}

pub fn sweep_orphaned_profiles() {
    let Ok(root) = paths::webview_data_dir() else { return };
    let Ok(apps) = Registry::open().and_then(|r| r.list()) else { return };
    let live: std::collections::HashSet<String> = apps.into_iter().map(|a| a.id).collect();

    let Ok(entries) = std::fs::read_dir(&root) else { return };
    for entry in entries.filter_map(|e| e.ok()) {
        if !entry.file_type().is_ok_and(|t| t.is_dir()) {
            continue;
        }
        let Some(name) = entry.file_name().to_str().map(str::to_string) else { continue };
        if live.contains(&name) {
            continue;
        }
        match std::fs::remove_dir_all(entry.path()) {
            Ok(()) => tracing::info!(id = %name, "removed a private session with no app"),
            Err(e) => {
                tracing::warn!(id = %name, error = %e, "could not remove an orphaned private session")
            }
        }
    }
}

pub fn delete_app(id: &str) -> Result<()> {
    zww_shortcut::backend().remove(id)?;
    let _ = icon::remove_icon(id);

    zww_core::credentials::forget(id);

    if let Ok(profile) = paths::webview_profile_dir(id) {
        if let Err(e) = std::fs::remove_dir_all(&profile) {
            if e.kind() != std::io::ErrorKind::NotFound {
                tracing::warn!(id, error = %e, "could not remove this app's private session");
            }
        }
    }

    Registry::open()?.remove(id)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::await_holding_lock)]

    use super::*;

    use std::path::PathBuf;

    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    struct Sandbox {
        root: PathBuf,
        tmp: tempfile::TempDir,

        _guard: std::sync::MutexGuard<'static, ()>,
    }

    impl Sandbox {
        fn new() -> Self {
            let guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let tmp = tempfile::tempdir().unwrap();
            let root = tmp.path().to_path_buf();

            let desktop = root.join("desktop");
            std::fs::create_dir_all(&desktop).ok();

            std::env::set_var(zww_core::paths::DATA_DIR_VAR, &root);
            std::env::set_var("XDG_DATA_HOME", &root);
            std::env::set_var("XDG_DESKTOP_DIR", &desktop);
            #[cfg(windows)]
            {
                std::env::set_var(
                    zww_shortcut::windows::START_MENU_DIR_VAR,
                    root.join("start-menu"),
                );
                std::env::set_var(zww_shortcut::windows::DESKTOP_DIR_VAR, &desktop);
            }

            Self { root, tmp, _guard: guard }
        }

        #[cfg(target_os = "linux")]
        fn menu_shortcut(&self, id: &str, _title: &str) -> PathBuf {
            self.root.join("applications").join(format!("zwebwrap-{id}.desktop"))
        }

        #[cfg(windows)]
        fn shortcut_in(&self, id: &str, folder: &str) -> PathBuf {
            let index = zww_shortcut::index::Index::load(&self.root.join("shortcuts.json"));
            let dir = self.root.join(folder);
            index
                .get(id)
                .iter()
                .find(|p| p.parent() == Some(dir.as_path()))
                .cloned()
                .unwrap_or_else(|| dir.join(format!("{id}-has-no-shortcut.lnk")))
        }

        #[cfg(windows)]
        fn menu_shortcut(&self, id: &str, _title: &str) -> PathBuf {
            self.shortcut_in(id, "start-menu")
        }

        #[cfg(target_os = "linux")]
        fn desktop_shortcut(&self, id: &str, _title: &str) -> PathBuf {
            self.root.join("desktop").join(format!("zwebwrap-{id}.desktop"))
        }

        #[cfg(windows)]
        fn desktop_shortcut(&self, id: &str, _title: &str) -> PathBuf {
            self.shortcut_in(id, "desktop")
        }

        fn assert_launches(&self, path: &std::path::Path, id: &str) {
            assert!(path.exists(), "shortcut missing at {}", path.display());

            let expected = format!("--app \"{id}\"");
            #[cfg(target_os = "linux")]
            {
                let entry = std::fs::read_to_string(path).unwrap();
                assert!(entry.contains(&expected), "{} does not launch {id}", path.display());
            }
            #[cfg(windows)]
            {
                let bytes = std::fs::read(path).unwrap();
                let wide: Vec<u8> = expected.encode_utf16().flat_map(u16::to_le_bytes).collect();
                assert!(
                    bytes.windows(wide.len()).any(|w| w == wide),
                    "{} does not carry `{expected}`",
                    path.display()
                );
            }
        }
    }

    impl Drop for Sandbox {
        fn drop(&mut self) {
            std::env::remove_var(zww_core::paths::DATA_DIR_VAR);
            std::env::remove_var("XDG_DATA_HOME");
            std::env::remove_var("XDG_DESKTOP_DIR");
            #[cfg(windows)]
            {
                std::env::remove_var(zww_shortcut::windows::START_MENU_DIR_VAR);
                std::env::remove_var(zww_shortcut::windows::DESKTOP_DIR_VAR);
            }

            let _ = self.tmp.path();
        }
    }

    #[tokio::test]
    async fn create_writes_registry_icon_and_shortcut() {
        let env = Sandbox::new();

        let app = create_app("https://nonexistent.invalid/", None, true)
            .await
            .expect("creation should survive an unreachable site");

        assert_eq!(app.id, "nonexistent-invalid");
        assert_eq!(app.title, "Nonexistent");
        assert_eq!(app.blocking, zww_core::BlockMode::Full);

        let listed = Registry::open().unwrap().list().unwrap();
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, app.id);

        let icon_path = app.icon.as_ref().expect("an icon should always be produced");
        assert!(icon_path.exists(), "icon file missing at {}", icon_path.display());

        let menu = env.menu_shortcut(&app.id, &app.title);
        env.assert_launches(&menu, &app.id);
        env.assert_launches(&env.desktop_shortcut(&app.id, &app.title), &app.id);

        #[cfg(target_os = "linux")]
        {
            let entry = std::fs::read_to_string(&menu).unwrap();
            assert!(entry.contains("StartupWMClass=zwebwrap-nonexistent-invalid"));
            assert!(entry.contains(&format!("Icon={}", icon_path.display())));
        }
    }

    #[tokio::test]
    async fn delete_removes_every_trace() {
        let env = Sandbox::new();

        let app = create_app("https://gone.invalid/", Some("Gone".into()), false).await.unwrap();
        let icon_path = app.icon.clone().unwrap();
        let menu = env.menu_shortcut(&app.id, &app.title);
        let desktop = env.desktop_shortcut(&app.id, &app.title);
        assert!(menu.exists());

        delete_app(&app.id).unwrap();

        assert!(Registry::open().unwrap().list().unwrap().is_empty());
        assert!(!menu.exists(), "shortcut should be gone");
        assert!(!desktop.exists(), "the desktop copy should be gone too");
        assert!(!icon_path.exists(), "icon should be gone");
    }

    #[tokio::test]
    async fn an_orphaned_private_session_is_swept_but_a_live_one_is_left() {
        let _env = Sandbox::new();

        let app = create_app("https://kept.invalid/", None, false).await.unwrap();
        let live = zww_core::paths::webview_profile_dir(&app.id).unwrap();
        let orphan = zww_core::paths::webview_profile_dir("deleted-long-ago").unwrap();
        std::fs::write(live.join("cookies"), "x").unwrap();
        std::fs::write(orphan.join("cookies"), "x").unwrap();

        sweep_orphaned_profiles();

        assert!(live.exists(), "an app that still exists must keep its session");
        assert!(!orphan.exists(), "a session with no app must be swept");
    }

    #[tokio::test]
    async fn a_custom_title_overrides_the_site_name() {
        let env = Sandbox::new();

        let app =
            create_app("https://titled.invalid/", Some("  My Name  ".into()), true).await.unwrap();
        assert_eq!(app.title, "My Name");

        let menu = env.menu_shortcut(&app.id, &app.title);
        assert!(menu.exists(), "shortcut missing at {}", menu.display());
        #[cfg(target_os = "linux")]
        assert!(std::fs::read_to_string(&menu).unwrap().contains("Name=My Name\n"));
        #[cfg(windows)]
        assert_eq!(menu.file_name().unwrap(), "My Name.lnk");
    }

    #[tokio::test]
    #[ignore = "requires network"]
    async fn resolves_a_real_site() {
        let _env = Sandbox::new();

        let app = create_app("youtube.com", None, true).await.unwrap();
        println!("title={:?} url={:?} icon={:?}", app.title, app.url, app.icon);

        assert!(app.title.to_lowercase().contains("youtube"), "got title {:?}", app.title);

        let icon = app.icon.clone().unwrap();
        let decoded = image::open(&icon).unwrap();
        assert_eq!(decoded.width(), 256);
    }

    #[tokio::test]
    async fn wrapping_the_same_host_twice_does_not_collide() {
        let env = Sandbox::new();

        let first = create_app("https://dup.invalid/one", None, true).await.unwrap();
        let second = create_app("https://dup.invalid/two", None, true).await.unwrap();

        assert_eq!(first.id, "dup-invalid");
        assert_eq!(second.id, "dup-invalid-2");
        assert_eq!(Registry::open().unwrap().list().unwrap().len(), 2);

        let a = env.menu_shortcut(&first.id, &first.title);
        let b = env.menu_shortcut(&second.id, &second.title);
        assert_ne!(a, b, "two apps must not share one shortcut file");
        env.assert_launches(&a, &first.id);
        env.assert_launches(&b, &second.id);
    }
}

use crate::{create, window};
use serde::Serialize;
use tauri::{AppHandle, Manager};
use zww_core::site;
use zww_core::{paths, Registry, WrappedApp};

pub type CmdResult<T> = Result<T, String>;

fn fail(e: impl std::fmt::Display) -> String {
    e.to_string()
}

fn refresh_launchers(handle: &AppHandle) {
    let handle = handle.clone();

    let _ = handle.clone().run_on_main_thread(move || {
        crate::tray::sync(&handle);
        crate::hotkeys::sync(&handle);
    });
}

#[derive(Serialize)]
pub struct Preview {
    pub url: String,
    pub title: String,
    pub icon_url: Option<String>,
}

#[tauri::command]
pub fn list_apps() -> CmdResult<Vec<WrappedApp>> {
    Registry::open().map_err(fail)?.list().map_err(fail)
}

#[tauri::command]
pub async fn preview_url(input: String) -> CmdResult<Preview> {
    let url = site::normalize_url(&input).map_err(fail)?;
    let client = site::client().map_err(fail)?;
    let meta = site::fetch_meta(&client, &url).await.map_err(fail)?;
    Ok(Preview {
        url: meta.url,
        title: meta.title,
        icon_url: meta.icons.first().map(|c| c.url.clone()),
    })
}

#[tauri::command]
pub async fn create_app(
    handle: AppHandle,
    input: String,
    title: Option<String>,
    adblock: Option<bool>,
) -> CmdResult<WrappedApp> {
    let app = create::create_app(&input, title, adblock.unwrap_or(true)).await.map_err(fail)?;

    refresh_launchers(&handle);
    Ok(app)
}

#[tauri::command]
pub fn update_app(handle: AppHandle, app: WrappedApp) -> CmdResult<()> {
    Registry::open().map_err(fail)?.upsert(app.clone()).map_err(fail)?;

    window::refresh_window_frame(&handle, &app);

    crate::tray::sync(&handle);

    crate::hotkeys::sync(&handle);

    create::write_shortcut(&app).map_err(fail)
}

#[tauri::command]
pub fn has_desktop_shortcut(id: String) -> CmdResult<bool> {
    Ok(zww_shortcut::backend().desktop_copy(&id).is_some())
}

#[tauri::command]
pub fn delete_app(handle: AppHandle, id: String) -> CmdResult<()> {
    if let Some(window) = handle.get_webview_window(&format!("app-{id}")) {
        if let Err(e) = window.destroy() {
            tracing::debug!(id, error = %e, "could not close the window before deleting");
        }
    }

    create::delete_app(&id).map_err(fail)?;
    crate::tray::sync(&handle);
    crate::hotkeys::sync(&handle);
    Ok(())
}

#[tauri::command]
pub async fn open_app(app: AppHandle, id: String) -> CmdResult<()> {
    let wrapped = Registry::open()
        .map_err(fail)?
        .get(&id)
        .map_err(fail)?
        .ok_or_else(|| format!("no app with id {id}"))?;

    let (tx, rx) = std::sync::mpsc::channel();
    app.clone()
        .run_on_main_thread(move || {
            let result = window::open_site(&app, &wrapped, true).map(|_| ()).map_err(fail);
            let _ = tx.send(result);
        })
        .map_err(fail)?;
    rx.recv().map_err(fail)?
}

#[tauri::command]
pub fn icon_data_url(id: String) -> CmdResult<Option<String>> {
    use base64::Engine as _;

    let path = paths::icon_dir().map_err(fail)?.join(format!("{id}.png"));
    match std::fs::read(&path) {
        Ok(bytes) => {
            let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
            Ok(Some(format!("data:image/png;base64,{encoded}")))
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(fail(e)),
    }
}

#[tauri::command]
pub fn get_preferences() -> zww_core::Preferences {
    zww_core::prefs::load()
}

#[tauri::command]
pub fn set_preferences(handle: AppHandle, prefs: zww_core::Preferences) -> CmdResult<()> {
    let previous = zww_core::prefs::load();
    zww_core::prefs::save(&prefs).map_err(fail)?;

    if previous.desktop_shortcut != prefs.desktop_shortcut {
        for app in Registry::open().map_err(fail)?.list().map_err(fail)? {
            if let Err(e) = create::write_shortcut(&app) {
                tracing::warn!(id = %app.id, error = %e, "could not update shortcut");
            }
        }
    }

    if previous.start_with_os != prefs.start_with_os {
        zww_core::autostart::apply(prefs.start_with_os).map_err(fail)?;
    }

    crate::tray::sync(&handle);
    Ok(())
}

#[tauri::command]
pub fn platform_report() -> crate::platform::Capabilities {
    crate::platform::capabilities()
}

#[tauri::command]
pub fn discover_bookmarks() -> Vec<zww_core::bookmarks::Bookmark> {
    let mut found = zww_core::bookmarks::discover();

    if let Ok(existing) = Registry::open().and_then(|r| r.list()) {
        let taken: std::collections::HashSet<String> =
            existing.iter().map(|a| a.url.trim_end_matches('/').to_ascii_lowercase()).collect();
        found.retain(|b| !taken.contains(&b.url.trim_end_matches('/').to_ascii_lowercase()));
    }
    found
}

#[tauri::command]
pub async fn import_bookmarks(
    handle: AppHandle,
    urls: Vec<String>,
    titles: Vec<String>,
) -> CmdResult<usize> {
    let adblock = zww_core::prefs::load().adblock_by_default;
    let mut made = 0;

    for (index, url) in urls.iter().enumerate() {
        let title = titles.get(index).cloned().filter(|t| !t.trim().is_empty());
        match create::create_app(url, title, adblock).await {
            Ok(_) => made += 1,
            Err(e) => tracing::warn!(url, error = %e, "could not import a bookmark"),
        }
    }

    refresh_launchers(&handle);
    Ok(made)
}

#[tauri::command]
pub fn valid_hotkey(accelerator: String) -> bool {
    crate::hotkeys::is_valid(&accelerator)
}

#[tauri::command]
pub fn autostart_active() -> bool {
    zww_core::autostart::is_enabled()
}

#[tauri::command]
pub fn set_app_icon(id: String, data_url: String) -> CmdResult<String> {
    use base64::Engine as _;

    let encoded = data_url.split_once(",").map(|(_, rest)| rest).unwrap_or(&data_url);
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|_| "that file could not be read".to_string())?;

    let path = zww_core::icon::save_custom(&bytes, &id).map_err(fail)?;

    let registry = Registry::open().map_err(fail)?;
    if let Some(mut app) = registry.get(&id).map_err(fail)? {
        app.icon = Some(path);
        registry.upsert(app.clone()).map_err(fail)?;
        create::write_shortcut(&app).map_err(fail)?;
    }

    icon_data_url(id)?.ok_or_else(|| "the icon was saved but could not be read back".to_string())
}

#[tauri::command]
pub async fn reset_app_icon(id: String) -> CmdResult<Option<String>> {
    let registry = Registry::open().map_err(fail)?;
    let app = registry.get(&id).map_err(fail)?.ok_or_else(|| format!("no app with id {id}"))?;

    let url = site::normalize_url(&app.url).map_err(fail)?;
    let client = site::client().map_err(fail)?;
    let meta = site::fetch_meta(&client, &url).await.map_err(fail)?;
    let path = zww_core::icon::resolve_and_save(&client, &meta.icons, &id, &app.title).await;

    let mut app = app;
    app.icon = (!path.as_os_str().is_empty()).then_some(path);
    registry.upsert(app.clone()).map_err(fail)?;
    create::write_shortcut(&app).map_err(fail)?;

    icon_data_url(id)
}

#[tauri::command]
pub fn media_support() -> zww_core::MediaSupport {
    zww_core::media::check()
}

#[tauri::command]
pub fn user_agent_presets() -> Vec<serde_json::Value> {
    site::USER_AGENT_PRESETS
        .iter()
        .map(|p| serde_json::json!({ "label": p.label, "value": p.value, "hint": p.hint }))
        .collect()
}

#[tauri::command]
pub fn adblock_status() -> crate::adblock::Status {
    crate::adblock::status()
}

#[tauri::command]
pub fn hidden_counts() -> serde_json::Value {
    let apps = Registry::open().and_then(|r| r.list()).unwrap_or_default();
    let per_app: serde_json::Map<String, serde_json::Value> = apps
        .iter()
        .map(|app| (app.id.clone(), crate::adblock::hidden_for(&app.id).into()))
        .collect();

    serde_json::json!({ "total": crate::adblock::hidden_total(), "apps": per_app })
}

#[tauri::command]
pub async fn refresh_filters() -> CmdResult<crate::adblock::Status> {
    tauri::async_runtime::spawn_blocking(crate::adblock::refresh_now)
        .await
        .map_err(fail)?
        .map_err(fail)
}

#[tauri::command]
pub fn reveal_data_dir(app: AppHandle) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;
    let dir = paths::data_dir().map_err(fail)?;
    app.opener().open_path(dir.to_string_lossy(), None::<&str>).map_err(fail)
}

#[tauri::command]
pub fn list_downloads() -> Vec<crate::downloads::Download> {
    crate::downloads::refresh_active();
    crate::downloads::list()
}

#[tauri::command]
pub fn open_download(app: AppHandle, id: u64) -> CmdResult<()> {
    crate::downloads::open(&app, id)
}

#[tauri::command]
pub fn reveal_download(app: AppHandle, id: u64) -> CmdResult<()> {
    crate::downloads::reveal(&app, id)
}

#[tauri::command]
pub fn clear_downloads() -> Vec<crate::downloads::Download> {
    crate::downloads::clear_finished();
    crate::downloads::list()
}

#[cfg(test)]
mod tests {

    #[test]
    fn every_command_the_ui_calls_is_permitted() {
        let api = include_str!("../../src/api.ts");
        let allowlist = include_str!("../permissions/manager.toml");

        let mut called: Vec<&str> = Vec::new();
        for (index, _) in api.match_indices("invoke") {
            let rest = &api[index..];
            let Some(open) = rest.find('(') else { continue };
            let after = &rest[open + 1..];
            let trimmed = after.trim_start();
            let Some(stripped) = trimmed.strip_prefix('"') else { continue };
            let Some(end) = stripped.find('"') else { continue };
            called.push(&stripped[..end]);
        }
        called.sort_unstable();
        called.dedup();

        assert!(
            called.len() > 15,
            "the extractor found only {} commands; it has drifted from api.ts",
            called.len()
        );

        for command in called {
            if command.contains(':') {
                continue;
            }
            assert!(
                allowlist.contains(&format!("\"{command}\"")),
                "the manager calls `{command}` but permissions/manager.toml does not allow it, \
                 so the call is denied and whatever depends on it silently does nothing"
            );
        }
    }
}

#[tauri::command]
pub fn data_location() -> CmdResult<String> {
    Ok(paths::data_dir().map_err(fail)?.display().to_string())
}

#[tauri::command]
pub fn app_credential(id: String) -> CmdResult<Option<zww_core::credentials::Credential>> {
    Ok(zww_core::credentials::get(&id))
}

#[tauri::command]
pub fn set_app_credential(
    id: String,
    credential: Option<zww_core::credentials::Credential>,
) -> CmdResult<()> {
    let keep = credential.as_ref().is_some_and(zww_core::credentials::Credential::is_set);
    zww_core::credentials::set(&id, credential).map_err(fail)?;

    let registry = Registry::open().map_err(fail)?;
    if let Some(mut app) = registry.get(&id).map_err(fail)? {
        if app.has_auth != keep {
            app.has_auth = keep;
            registry.upsert(app).map_err(fail)?;
        }
    }
    Ok(())
}

#[tauri::command]
pub fn valid_proxy(address: String) -> CmdResult<()> {
    zww_core::model::parse_proxy(&address).map(|_| ())
}

const ABOUT_LINKS: &[&str] = &[
    "https://github.com/TheHolyOneZ",
    "https://github.com/TheHolyOneZ/ZWebWrap",
    "https://zsync.eu",
    "https://zsync.eu/zwebwrap/",
];

#[tauri::command]
pub fn open_link(app: AppHandle, url: String) -> CmdResult<()> {
    use tauri_plugin_opener::OpenerExt;

    if !ABOUT_LINKS.contains(&url.as_str()) {
        return Err(format!("{url} is not one of this app's own links"));
    }

    if app.opener().open_url(&url, None::<&str>).is_ok() {
        return Ok(());
    }
    tracing::debug!(%url, "the opener plugin refused; falling back to the platform handler");

    let spawned = if cfg!(target_os = "linux") {
        std::process::Command::new("xdg-open").arg(&url).spawn()
    } else if cfg!(target_os = "windows") {
        std::process::Command::new("cmd").args(["/C", "start", "", &url]).spawn()
    } else {
        std::process::Command::new("open").arg(&url).spawn()
    };

    spawned.map(|_| ()).map_err(|e| format!("could not open {url}: {e}"))
}

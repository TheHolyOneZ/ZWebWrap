use anyhow::{Context, Result};
use tauri::menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIcon, TrayIconBuilder, TrayIconEvent};
use tauri::AppHandle;
use zww_core::Registry;

pub const TRAY_ID: &str = "zwebwrap";
const OPEN_LIBRARY: &str = "zww:library";
const QUIT: &str = "zww:quit";

const APP_PREFIX: &str = "zww:app:";

pub fn wanted() -> bool {
    let prefs = zww_core::prefs::load();
    prefs.close_to_tray || prefs.start_with_os
}

pub fn sync(app: &AppHandle) {
    if !wanted() {
        remove(app);
        return;
    }

    match app.tray_by_id(TRAY_ID) {
        Some(tray) => {
            if let Err(e) = rebuild_menu(app, &tray) {
                tracing::warn!(error = %e, "could not refresh the tray menu");
            }
        }
        None => {
            if let Err(e) = install(app) {
                tracing::warn!(error = %e, "could not create the tray icon");
            }
        }
    }
}

fn remove(app: &AppHandle) {
    if app.remove_tray_by_id(TRAY_ID).is_some() {
        tracing::debug!("removed the tray icon");
    }
}

fn build_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>> {
    let menu = Menu::new(app)?;
    menu.append(&MenuItem::with_id(app, OPEN_LIBRARY, "Open ZWebWrap", true, None::<&str>)?)?;

    let apps = Registry::open().and_then(|r| r.list()).unwrap_or_default();
    if !apps.is_empty() {
        menu.append(&PredefinedMenuItem::separator(app)?)?;

        for group in zww_core::model::groups_in_use(&apps) {
            let submenu = tauri::menu::Submenu::new(app, &group, true)?;
            for wrapped in apps.iter().filter(|a| {
                zww_core::model::group_of(a).is_some_and(|g| g.eq_ignore_ascii_case(&group))
            }) {
                submenu.append(&MenuItem::with_id(
                    app,
                    format!("{APP_PREFIX}{}", wrapped.id),
                    &wrapped.title,
                    true,
                    None::<&str>,
                )?)?;
            }
            menu.append(&submenu)?;
        }

        for wrapped in apps.iter().filter(|a| zww_core::model::group_of(a).is_none()) {
            menu.append(&MenuItem::with_id(
                app,
                format!("{APP_PREFIX}{}", wrapped.id),
                &wrapped.title,
                true,
                None::<&str>,
            )?)?;
        }
    }

    menu.append(&PredefinedMenuItem::separator(app)?)?;
    menu.append(&MenuItem::with_id(app, QUIT, "Quit ZWebWrap", true, None::<&str>)?)?;
    Ok(menu)
}

fn rebuild_menu(app: &AppHandle, tray: &TrayIcon) -> Result<()> {
    tray.set_menu(Some(build_menu(app)?)).context("setting the tray menu")
}

fn install(app: &AppHandle) -> Result<()> {
    let mut builder = TrayIconBuilder::with_id(TRAY_ID)
        .tooltip("ZWebWrap")
        .menu(&build_menu(app)?)
        .show_menu_on_left_click(false)
        .on_menu_event(handle_menu)
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                if let Err(e) = crate::window::open_manager(tray.app_handle(), None) {
                    tracing::warn!(error = %e, "could not open the library from the tray");
                }
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }

    builder.build(app).context("building the tray icon")?;
    tracing::debug!("tray icon created");
    Ok(())
}

fn handle_menu(app: &AppHandle, event: MenuEvent) {
    let id = event.id().as_ref();

    if id == QUIT {
        tracing::info!("quitting from the tray");
        app.exit(0);
        return;
    }

    if id == OPEN_LIBRARY {
        if let Err(e) = crate::window::open_manager(app, None) {
            tracing::warn!(error = %e, "could not open the library from the tray");
        }
        return;
    }

    if let Some(app_id) = id.strip_prefix(APP_PREFIX) {
        match Registry::open().and_then(|r| r.get(app_id)) {
            Ok(Some(wrapped)) => {
                if let Err(e) = crate::window::open_site(app, &wrapped, true) {
                    tracing::warn!(error = %e, id = app_id, "could not open app from the tray");
                }
            }

            Ok(None) => tracing::warn!(id = app_id, "tray entry refers to a missing app"),
            Err(e) => tracing::warn!(error = %e, "could not read the registry"),
        }
    }
}

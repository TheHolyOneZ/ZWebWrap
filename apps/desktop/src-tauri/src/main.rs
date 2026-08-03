#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod adblock;
mod cli;
mod commands;
mod create;
mod downloads;
mod host;
mod hotkeys;
mod overlay;
mod page;
mod platform;
mod tray;
mod window;

use anyhow::Result;
use cli::Mode;
use tauri::AppHandle;
use zww_core::{site, Registry, WrappedApp};

fn main() {
    init_tracing();
    apply_webkit_workarounds();

    let mode = match cli::parse(std::env::args().skip(1)) {
        Ok(mode) => mode,
        Err(message) => {
            eprintln!("zwebwrap: {message}\n\n{}", cli::HELP);
            std::process::exit(2);
        }
    };

    match mode {
        Mode::Help => println!("{}", cli::HELP),
        Mode::Version => println!("zwebwrap {}", env!("CARGO_PKG_VERSION")),

        Mode::NativeHost => {
            if let Err(e) = host::run() {
                eprintln!("zwebwrap: native host stopped: {e:#}");
                std::process::exit(1);
            }
        }

        headless @ (Mode::Add { .. } | Mode::List | Mode::Remove(_)) => {
            if let Err(e) = run_headless(headless) {
                eprintln!("zwebwrap: {e:#}");
                std::process::exit(1);
            }
        }

        gui_mode => run_gui(gui_mode),
    }
}

fn run_headless(mode: Mode) -> Result<()> {
    match mode {
        Mode::Add { url, name } => {
            let runtime = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("building a Tokio runtime");
            let app = runtime.block_on(create::create_app(&url, name, true))?;
            println!("Created {} ({})", app.title, app.id);
            println!("  url      {}", app.url);
            if let Some(icon) = &app.icon {
                println!("  icon     {}", icon.display());
            }
            println!("  shortcut {}", zww_shortcut::wm_class(&app.id));
        }

        Mode::List => {
            let apps = Registry::open()?.list()?;
            if apps.is_empty() {
                println!("No apps yet. Create one with: zwebwrap --add <url>");
            }
            for app in apps {
                println!("{:<24} {:<28} {}", app.id, app.title, app.url);
            }
        }

        Mode::Remove(id) => {
            let existed = Registry::open()?.get(&id)?.is_some();
            create::delete_app(&id)?;
            if existed {
                println!("Removed {id}");
            } else {
                println!("No app with id {id}");
            }
        }

        _ => unreachable!("run_headless only handles headless modes"),
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn apply_webkit_workarounds() {
    if std::env::var("ZWEBWRAP_NO_DMABUF").as_deref() == Ok("1") {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        tracing::debug!("disabled the WebKit DMABUF renderer on request");
    }

    if std::env::var("ZWEBWRAP_NO_COMPOSITING").as_deref() == Ok("1") {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
        tracing::debug!("disabled WebKit accelerated compositing on request");
    }
}

#[cfg(not(target_os = "linux"))]
fn apply_webkit_workarounds() {}

fn init_tracing() {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};
    let filter = EnvFilter::try_from_env("ZWEBWRAP_LOG")
        .unwrap_or_else(|_| EnvFilter::new("zwebwrap=info,zww_core=info,zww_shortcut=info"));

    tracing_subscriber::registry()
        .with(fmt::layer().compact().with_writer(std::io::stderr))
        .with(filter)
        .init();
}

fn run_gui(mode: Mode) {
    platform::log_once();

    adblock::start();

    std::thread::spawn(|| {
        zww_bridge::register::register_all();
        create::refresh_shortcuts_if_moved();

        create::sweep_orphaned_profiles();
    });

    tauri::Builder::default()
        .plugin(single_instance_plugin())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::list_apps,
            commands::preview_url,
            commands::create_app,
            commands::update_app,
            commands::delete_app,
            commands::has_desktop_shortcut,
            commands::open_app,
            commands::icon_data_url,
            commands::adblock_status,
            commands::hidden_counts,
            commands::media_support,
            commands::get_preferences,
            commands::autostart_active,
            commands::valid_hotkey,
            commands::platform_report,
            commands::discover_bookmarks,
            commands::import_bookmarks,
            commands::set_preferences,
            commands::set_app_icon,
            commands::reset_app_icon,
            commands::user_agent_presets,
            commands::refresh_filters,
            commands::reveal_data_dir,
            commands::data_location,
            commands::app_credential,
            commands::set_app_credential,
            commands::valid_proxy,
            commands::open_link,
            commands::list_downloads,
            commands::open_download,
            commands::reveal_download,
            commands::clear_downloads,
            page::zww_page_selectors,
            page::zww_page_hidden,
            page::zww_window_action,
        ])
        .setup(move |app| {
            tray::sync(app.handle());
            hotkeys::sync(app.handle());
            route(app.handle(), mode)?;
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("ZWebWrap failed to start")
        .run(|app, event| {
            if let tauri::RunEvent::ExitRequested { api, code, .. } = &event {
                if code.is_none() && tray::wanted() && app.tray_by_id(tray::TRAY_ID).is_some() {
                    api.prevent_exit();
                }
            }
        });
}

fn single_instance_plugin() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    let builder = tauri_plugin_single_instance::Builder::new().callback(
        |app: &AppHandle, argv: Vec<String>, _cwd: String| match cli::parse(
            argv.into_iter().skip(1),
        ) {
            Ok(mode) => {
                tracing::info!("another launch handed its arguments to this instance");
                if let Err(e) = route(app, mode) {
                    tracing::error!(error = %e, "could not handle the second instance");
                }
            }
            Err(message) => tracing::warn!(%message, "ignoring unparseable second instance"),
        },
    );

    #[cfg(all(debug_assertions, target_os = "linux"))]
    let builder = builder.dbus_id(debug_instance_id());

    builder.build()
}

#[cfg(all(debug_assertions, target_os = "linux"))]
fn debug_instance_id() -> String {
    use std::hash::{Hash, Hasher};

    let exe = zww_core::paths::current_exe().unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    exe.hash(&mut hasher);
    format!("dev.zwebwrap.app.dev{:x}", hasher.finish())
}

fn route(app: &AppHandle, mode: Mode) -> Result<()> {
    match mode {
        Mode::Tray => {
            tray::sync(app);
            if !tray::wanted() {
                tracing::warn!("--tray with tray settings off; opening the library instead");
                window::open_manager(app, None)?;
            }
        }

        Mode::OpenApp(id) => match Registry::open()?.get(&id)? {
            Some(wrapped) => {
                window::open_site(app, &wrapped, true)?;
            }
            None => {
                tracing::warn!(id, "shortcut refers to an app that no longer exists");
                window::open_manager(app, None)?;
            }
        },

        Mode::OpenUrl(input) => {
            let url = site::normalize_url(&input)?;
            let saved = Registry::open()?.list()?.into_iter().find(|a| a.url == url.as_str());

            match saved {
                Some(wrapped) => {
                    window::open_site(app, &wrapped, true)?;
                }
                None => {
                    let id = zww_core::model::slug_for_url(&url);
                    let title = url.host_str().unwrap_or("ZWebWrap").to_string();
                    let ephemeral = WrappedApp::new(id, url.as_str(), title);
                    window::open_site(app, &ephemeral, false)?;
                }
            }
        }

        Mode::Settings => {
            window::open_manager(app, Some("settings"))?;
        }

        Mode::Edit(id) => {
            window::open_manager(app, Some(&format!("app/{id}")))?;
        }

        _ => {
            window::open_manager(app, None)?;
        }
    }
    Ok(())
}

use serde::Serialize;
use tauri::{Manager, Runtime, WebviewWindow};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn zww_page_selectors(tokens: Vec<String>) -> Vec<String> {
    const MAX_TOKENS: usize = 8_000;
    let tokens: Vec<String> = tokens.into_iter().take(MAX_TOKENS).collect();
    let hidden = crate::adblock::generic_selectors(&tokens);
    tracing::debug!(asked = tokens.len(), hiding = hidden.len(), "generic cosmetic lookup");
    hidden
}

#[tauri::command]
pub fn zww_page_hidden<R: Runtime>(window: WebviewWindow<R>, count: u64) {
    let id = window.label().strip_prefix("app-").unwrap_or("").to_string();
    if id.is_empty() {
        return;
    }
    crate::adblock::record_hidden(&id, count.min(100_000));
}

#[derive(Debug, Clone, Serialize)]
pub struct WindowState {
    pub always_on_top: bool,
    pub zoom: f64,
    pub url: String,
}

#[tauri::command]
pub fn zww_window_action<R: Runtime>(
    window: WebviewWindow<R>,
    action: String,
    value: Option<f64>,
) -> Result<WindowState, String> {
    let id = window.label().strip_prefix("app-").unwrap_or("").to_string();

    match action.as_str() {
        "always_on_top" => {
            let on = value.map(|v| v != 0.0).unwrap_or(false);
            window.set_always_on_top(on).map_err(|e| e.to_string())?;
            crate::window::remember_always_on_top(&id, on);
        }
        "zoom" => {
            let zoom = value.unwrap_or(1.0).clamp(0.25, 5.0);
            window.set_zoom(zoom).map_err(|e| e.to_string())?;
            crate::window::remember_zoom(&id, zoom);
        }
        "reload" => {
            let _ = window.eval("window.location.reload()");
        }
        "back" => {
            let _ = window.eval("window.history.back()");
        }
        "forward" => {
            let _ = window.eval("window.history.forward()");
        }
        "open_in_browser" => {
            let url = window.url().map_err(|e| e.to_string())?.to_string();
            window.app_handle().opener().open_url(url, None::<&str>).map_err(|e| e.to_string())?;
        }
        "minimize" => {
            window.minimize().map_err(|e| e.to_string())?;
        }
        "toggle_maximize" => {
            if window.is_maximized().unwrap_or(false) {
                window.unmaximize().map_err(|e| e.to_string())?;
            } else {
                window.maximize().map_err(|e| e.to_string())?;
            }
        }

        "drag" => {
            window.start_dragging().map_err(|e| e.to_string())?;
        }
        "close" => {
            window.close().map_err(|e| e.to_string())?;
        }

        "compact" => {
            let aspect = value.unwrap_or(0.0);
            if aspect > 0.0 {
                let aspect = aspect.clamp(0.3, 4.0);
                if let Ok(size) = window.inner_size() {
                    let scale = window.scale_factor().unwrap_or(1.0);
                    let logical = size.to_logical::<f64>(scale);
                    crate::window::remember_compact(&id, logical.width, logical.height);

                    let height = (logical.width / aspect).round().max(120.0);
                    let _ = window.set_size(tauri::LogicalSize::new(logical.width, height));
                }
                window.set_always_on_top(true).map_err(|e| e.to_string())?;
                crate::window::remember_always_on_top(&id, true);
            } else if let Some((width, height)) = crate::window::stored_compact(&id) {
                let _ = window.set_size(tauri::LogicalSize::new(width, height));
                let on_top = crate::window::compact_restores_on_top(&id);
                window.set_always_on_top(on_top).map_err(|e| e.to_string())?;
                crate::window::remember_always_on_top(&id, on_top);
            }
        }

        "badge" => {
            let n = value.unwrap_or(0.0);
            let base = crate::window::stored_title(&id);
            let title = if n >= 1.0 {
                format!("({}) {base}", (n as i64).clamp(1, 9_999))
            } else if n < 0.0 {
                format!("• {base}")
            } else {
                base
            };
            window.set_title(&title).map_err(|e| e.to_string())?;
        }

        "download_open" | "download_show" => {
            let download_id = value.unwrap_or(0.0) as u64;
            if !crate::downloads::belongs_to(&id, download_id) {
                return Err("that download belongs to another app".into());
            }
            let handle = window.app_handle();
            if action == "download_open" {
                crate::downloads::open(handle, download_id)?;
            } else {
                crate::downloads::reveal(handle, download_id)?;
            }
        }
        "state" => {}
        other => return Err(format!("unknown action: {other}")),
    }

    Ok(WindowState {
        always_on_top: crate::window::stored_always_on_top(&id),
        zoom: crate::window::stored_zoom(&id),
        url: window.url().map(|u| u.to_string()).unwrap_or_default(),
    })
}

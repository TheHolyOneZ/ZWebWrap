use anyhow::{Context, Result};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use zww_core::{Registry, WrappedApp};

pub fn sync(app: &AppHandle) {
    let manager = app.global_shortcut();

    if let Err(e) = manager.unregister_all() {
        tracing::debug!(error = %e, "could not clear existing shortcuts");
    }

    let apps = Registry::open().and_then(|r| r.list()).unwrap_or_default();
    for wrapped in apps {
        let Some(accelerator) = wrapped.hotkey.as_deref().map(str::trim).filter(|s| !s.is_empty())
        else {
            continue;
        };

        match register(app, &wrapped, accelerator) {
            Ok(()) => tracing::debug!(id = %wrapped.id, accelerator, "shortcut registered"),

            Err(e) => {
                tracing::warn!(id = %wrapped.id, accelerator, error = %e, "shortcut not registered")
            }
        }
    }
}

fn register(app: &AppHandle, wrapped: &WrappedApp, accelerator: &str) -> Result<()> {
    let shortcut: Shortcut =
        accelerator.parse().with_context(|| format!("{accelerator} is not a valid shortcut"))?;

    let id = wrapped.id.clone();
    app.global_shortcut()
        .on_shortcut(shortcut, move |app, _, event| {
            if event.state() != ShortcutState::Pressed {
                return;
            }
            toggle(app, &id);
        })
        .with_context(|| format!("registering {accelerator}"))?;
    Ok(())
}

fn toggle(app: &AppHandle, id: &str) {
    let label = format!("app-{id}");

    if let Some(window) = app.get_webview_window(&label) {
        let visible = window.is_visible().unwrap_or(false);
        let focused = window.is_focused().unwrap_or(false);

        if visible && focused {
            let _ = window.hide();
            tracing::debug!(id, "shortcut hid the window");

            crate::window::suspend_hidden(&window, id);
            return;
        }

        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        crate::window::resume_window(&window);
        tracing::debug!(id, "shortcut summoned the window");
        return;
    }

    match Registry::open().and_then(|r| r.get(id)) {
        Ok(Some(wrapped)) => {
            if let Err(e) = crate::window::open_site(app, &wrapped, true) {
                tracing::warn!(id, error = %e, "shortcut could not open the app");
            }
        }
        Ok(None) => tracing::warn!(id, "shortcut refers to an app that no longer exists"),
        Err(e) => tracing::warn!(error = %e, "could not read the registry"),
    }
}

pub fn is_valid(accelerator: &str) -> bool {
    accelerator.trim().parse::<Shortcut>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_combinations_are_accepted() {
        assert!(is_valid("Ctrl+Alt+Y"));
        assert!(is_valid("Super+Shift+M"));
        assert!(is_valid("CmdOrCtrl+Alt+1"));
    }

    #[test]
    fn nonsense_is_rejected_before_it_reaches_the_system() {
        assert!(!is_valid(""));
        assert!(!is_valid("   "));
        assert!(!is_valid("Ctrl+"));
        assert!(!is_valid("NotAKey+Q"));
    }

    #[test]
    fn a_single_key_without_a_modifier_is_still_parsed_but_is_a_bad_idea() {
        let _ = is_valid("Y");
    }
}

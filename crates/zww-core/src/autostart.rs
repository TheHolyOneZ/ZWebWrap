use anyhow::Result;

pub const TRAY_ARG: &str = "--tray";

const ENTRY_NAME: &str = "zwebwrap";

#[cfg(not(any(target_os = "linux", windows)))]
pub use elsewhere::{disable, enable, is_enabled};
#[cfg(target_os = "linux")]
pub use unix::{disable, enable, is_enabled};
#[cfg(windows)]
pub use win::{disable, enable, is_enabled};

#[cfg(any(target_os = "linux", test))]
pub fn autostart_entry(exec: &str) -> String {
    format!(
        "[Desktop Entry]\n\
         Type=Application\n\
         Version=1.0\n\
         Name=ZWebWrap\n\
         Comment=Keep wrapped apps available in the tray\n\
         Exec=\"{exec}\" {TRAY_ARG}\n\
         Terminal=false\n\
         NoDisplay=true\n\
         Hidden=false\n\
         X-GNOME-Autostart-enabled=true\n"
    )
}

#[cfg(target_os = "linux")]
mod unix {
    use super::*;
    use anyhow::Context;
    use std::path::PathBuf;

    fn autostart_dir() -> Option<PathBuf> {
        let config =
            if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
                PathBuf::from(xdg)
            } else {
                directories::BaseDirs::new()?.home_dir().join(".config")
            };
        Some(config.join("autostart"))
    }

    fn entry_path() -> Option<PathBuf> {
        Some(autostart_dir()?.join(format!("{ENTRY_NAME}.desktop")))
    }

    pub fn is_enabled() -> bool {
        entry_path().is_some_and(|p| p.is_file())
    }

    pub fn enable() -> Result<()> {
        let path = entry_path().context("no config directory to autostart from")?;
        let parent = path.parent().context("autostart path has no parent")?;
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;

        let exe = crate::paths::current_exe()?;
        std::fs::write(&path, autostart_entry(&exe.to_string_lossy()))
            .with_context(|| format!("writing {}", path.display()))?;

        tracing::info!(path = %path.display(), "enabled autostart");
        Ok(())
    }

    pub fn disable() -> Result<()> {
        let Some(path) = entry_path() else { return Ok(()) };
        match std::fs::remove_file(&path) {
            Ok(()) => tracing::info!(path = %path.display(), "disabled autostart"),

            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => {
                return Err(e).with_context(|| format!("removing {}", path.display()));
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
mod win {
    use super::*;
    use anyhow::Context;
    use winreg::enums::{HKEY_CURRENT_USER, KEY_READ, KEY_WRITE};
    use winreg::RegKey;

    const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";

    fn run_command() -> Result<String> {
        let exe = crate::paths::current_exe()?;
        Ok(format!("\"{}\" {TRAY_ARG}", exe.display()))
    }

    pub fn is_enabled() -> bool {
        RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey_with_flags(RUN_KEY, KEY_READ)
            .and_then(|k| k.get_value::<String, _>(ENTRY_NAME))
            .is_ok()
    }

    pub fn enable() -> Result<()> {
        let (key, _) = RegKey::predef(HKEY_CURRENT_USER)
            .create_subkey_with_flags(RUN_KEY, KEY_WRITE)
            .context("opening the Run key")?;
        key.set_value(ENTRY_NAME, &run_command()?).context("setting the Run value")?;
        tracing::info!("enabled autostart");
        Ok(())
    }

    pub fn disable() -> Result<()> {
        let Ok(key) = RegKey::predef(HKEY_CURRENT_USER).open_subkey_with_flags(RUN_KEY, KEY_WRITE)
        else {
            return Ok(());
        };
        match key.delete_value(ENTRY_NAME) {
            Ok(()) => tracing::info!("disabled autostart"),

            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e).context("removing the Run value"),
        }
        Ok(())
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
mod elsewhere {
    use super::*;

    pub fn is_enabled() -> bool {
        false
    }

    pub fn enable() -> Result<()> {
        anyhow::bail!("ZWebWrap cannot start with the system on this platform yet")
    }

    pub fn disable() -> Result<()> {
        Ok(())
    }
}

pub fn apply(wanted: bool) -> Result<()> {
    if wanted {
        enable()
    } else {
        disable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_entry_starts_in_the_tray_not_in_a_window() {
        let entry = autostart_entry("/usr/bin/zwebwrap");
        assert!(entry.contains(&format!("Exec=\"/usr/bin/zwebwrap\" {TRAY_ARG}")));
        assert!(entry.starts_with("[Desktop Entry]\n"));
        assert!(entry.contains("Type=Application\n"));
    }

    #[test]
    fn both_the_gnome_and_the_spec_enable_keys_are_written() {
        let entry = autostart_entry("/usr/bin/zwebwrap");
        assert!(entry.contains("X-GNOME-Autostart-enabled=true"), "GNOME reads this one");
        assert!(entry.contains("Hidden=false"), "KDE and the spec read this one");
    }

    #[test]
    fn a_path_with_spaces_stays_one_argument() {
        let entry = autostart_entry("/opt/My Apps/zwebwrap");
        assert!(entry.contains("Exec=\"/opt/My Apps/zwebwrap\" --tray"));
    }

    #[test]
    fn the_entry_is_hidden_from_the_app_menu() {
        assert!(autostart_entry("/usr/bin/zwebwrap").contains("NoDisplay=true"));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn enabling_then_disabling_leaves_no_trace() {
        let dir = tempfile::tempdir().unwrap();

        temp_env::with_var("XDG_CONFIG_HOME", Some(dir.path()), || {
            assert!(!is_enabled());

            enable().unwrap();
            assert!(is_enabled());
            let entry = dir.path().join("autostart/zwebwrap.desktop");
            assert!(entry.is_file());
            assert!(std::fs::read_to_string(&entry).unwrap().contains("--tray"));

            disable().unwrap();
            assert!(!is_enabled());
            assert!(!entry.exists());

            disable().unwrap();
        });
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn apply_follows_the_preference_either_way() {
        let dir = tempfile::tempdir().unwrap();
        temp_env::with_var("XDG_CONFIG_HOME", Some(dir.path()), || {
            apply(true).unwrap();
            assert!(is_enabled());
            apply(false).unwrap();
            assert!(!is_enabled());
        });
    }
}

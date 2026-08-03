use anyhow::{Context, Result};
use std::path::PathBuf;

pub const HOST_NAME: &str = "com.zwebwrap.bridge";

pub const CHROME_EXTENSION_ID: &str = "mmkbenogeabjjnpccliaekbeafnkjhkb";

pub const GECKO_EXTENSION_ID: &str = "zwebwrap@zlogic.eu";

struct Browser {
    name: &'static str,

    #[cfg_attr(windows, allow(dead_code))]
    dir: &'static str,

    #[cfg_attr(not(windows), allow(dead_code))]
    key: &'static str,
    flavour: Flavour,
}

#[derive(Clone, Copy, PartialEq)]
enum Flavour {
    Chromium,

    Gecko,
}

const BROWSERS: &[Browser] = &[
    Browser {
        name: "Chrome",
        dir: "google-chrome",
        key: r"Software\Google\Chrome\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Chromium",
        dir: "chromium",
        key: r"Software\Chromium\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Brave",
        dir: "BraveSoftware/Brave-Browser",
        key: r"Software\BraveSoftware\Brave-Browser\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Edge",
        dir: "microsoft-edge",
        key: r"Software\Microsoft\Edge\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Vivaldi",
        dir: "vivaldi",
        key: r"Software\Vivaldi\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Opera",
        dir: "opera",
        key: r"Software\Opera Software\NativeMessagingHosts",
        flavour: Flavour::Chromium,
    },
    Browser {
        name: "Firefox",
        dir: "",
        key: r"Software\Mozilla\NativeMessagingHosts",
        flavour: Flavour::Gecko,
    },
];

#[cfg(target_os = "linux")]
fn config_root() -> Option<PathBuf> {
    if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        return Some(PathBuf::from(xdg));
    }
    directories::BaseDirs::new().map(|d| d.home_dir().join(".config"))
}

#[cfg(target_os = "linux")]
fn home() -> Option<PathBuf> {
    directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf())
}

#[cfg(target_os = "linux")]
fn manifest_targets() -> Vec<(&'static str, PathBuf, Flavour)> {
    let mut targets = Vec::new();

    if let Some(config) = config_root() {
        for browser in BROWSERS {
            if browser.dir.is_empty() {
                continue;
            }
            let profile = config.join(browser.dir);

            if profile.is_dir() {
                targets.push((
                    browser.name,
                    profile.join("NativeMessagingHosts").join(format!("{HOST_NAME}.json")),
                    browser.flavour,
                ));
            }
        }
    }

    if let Some(home) = home() {
        let mozilla = home.join(".mozilla");
        if mozilla.is_dir() {
            targets.push((
                "Firefox",
                mozilla.join("native-messaging-hosts").join(format!("{HOST_NAME}.json")),
                Flavour::Gecko,
            ));
        }
    }

    targets
}

#[cfg(windows)]
fn manifest_targets() -> Vec<(&'static str, PathBuf, Flavour)> {
    let Ok(dir) = zww_core::paths::data_dir() else { return Vec::new() };
    vec![
        ("Chromium browsers", dir.join(format!("{HOST_NAME}.json")), Flavour::Chromium),
        ("Firefox", dir.join(format!("{HOST_NAME}.firefox.json")), Flavour::Gecko),
    ]
}

#[cfg(target_os = "linux")]
fn write_launcher(exe: &std::path::Path) -> Result<PathBuf> {
    let dir = zww_core::paths::data_dir()?;
    let script = dir.join("zwebwrap-nm-host");

    let body = format!(
        "#!/bin/sh\n\
         # Written by ZWebWrap. The browser cannot pass arguments through a\n\
         # native-messaging manifest, so this adds the mode flag.\n\
         exec {} --nm-host \"$@\"\n",
        shell_quote(&exe.to_string_lossy())
    );
    std::fs::write(&script, body).with_context(|| format!("writing {}", script.display()))?;

    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755))?;

    Ok(script)
}

#[cfg(windows)]
fn write_launcher(exe: &std::path::Path) -> Result<PathBuf> {
    let dir = zww_core::paths::data_dir()?;
    let script = dir.join("zwebwrap-nm-host.bat");

    let body = format!(
        "@echo off\r\n\
         rem Written by ZWebWrap. The browser cannot pass arguments through a\r\n\
         rem native-messaging manifest, so this adds the mode flag.\r\n\
         \"{}\" --nm-host %*\r\n",
        exe.display()
    );
    std::fs::write(&script, body).with_context(|| format!("writing {}", script.display()))?;

    Ok(script)
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r"'\''"))
}

fn manifest_json(launcher: &std::path::Path, flavour: Flavour) -> String {
    let allowed = match flavour {
        Flavour::Chromium => {
            format!(r#""allowed_origins": ["chrome-extension://{CHROME_EXTENSION_ID}/"]"#)
        }
        Flavour::Gecko => format!(r#""allowed_extensions": ["{GECKO_EXTENSION_ID}"]"#),
    };

    format!(
        "{{\n  \"name\": \"{HOST_NAME}\",\n  \"description\": \"ZWebWrap desktop bridge\",\n  \"path\": {},\n  \"type\": \"stdio\",\n  {allowed}\n}}\n",
        serde_json::Value::String(launcher.to_string_lossy().to_string())
    )
}

pub fn register_all() -> Vec<&'static str> {
    let exe = match zww_core::paths::current_exe() {
        Ok(exe) => exe,
        Err(e) => {
            tracing::warn!(error = %e, "could not locate the executable; skipping host registration");
            return Vec::new();
        }
    };

    let launcher = match write_launcher(&exe) {
        Ok(path) => path,
        Err(e) => {
            tracing::warn!(error = %e, "could not write the native host launcher");
            return Vec::new();
        }
    };

    let mut registered = Vec::new();
    for (name, path, flavour) in manifest_targets() {
        let Some(parent) = path.parent() else { continue };
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::debug!(browser = name, error = %e, "could not create the host directory");
            continue;
        }
        match std::fs::write(&path, manifest_json(&launcher, flavour)) {
            Ok(()) => {
                tracing::debug!(browser = name, path = %path.display(), "registered native host");
                registered.push(name);
            }
            Err(e) => {
                tracing::debug!(browser = name, error = %e, "could not write the host manifest")
            }
        }
    }

    #[cfg(windows)]
    {
        registered = point_registry_at_manifests();
    }

    if registered.is_empty() {
        tracing::debug!("no supported browser profiles found");
    } else {
        tracing::info!(browsers = ?registered, "browser extension bridge registered");
    }
    registered
}

#[cfg(windows)]
fn point_registry_at_manifests() -> Vec<&'static str> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
    use winreg::RegKey;

    let Ok(dir) = zww_core::paths::data_dir() else { return Vec::new() };
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let mut registered = Vec::new();

    for browser in BROWSERS {
        let manifest = match browser.flavour {
            Flavour::Chromium => dir.join(format!("{HOST_NAME}.json")),
            Flavour::Gecko => dir.join(format!("{HOST_NAME}.firefox.json")),
        };

        let path = format!("{}\\{HOST_NAME}", browser.key);
        match hkcu.create_subkey_with_flags(&path, KEY_WRITE) {
            Ok((key, _)) => match key.set_value("", &manifest.to_string_lossy().to_string()) {
                Ok(()) => registered.push(browser.name),
                Err(e) => {
                    tracing::debug!(browser = browser.name, error = %e, "could not set the host key")
                }
            },
            Err(e) => {
                tracing::debug!(browser = browser.name, error = %e, "could not open the host key")
            }
        }
    }

    registered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chromium_manifests_name_the_extension_origin() {
        let json = manifest_json(std::path::Path::new("/opt/zwebwrap-nm-host"), Flavour::Chromium);
        assert!(json.contains(&format!("chrome-extension://{CHROME_EXTENSION_ID}/")));
        assert!(json.contains(r#""type": "stdio""#));
        assert!(json.contains(HOST_NAME));

        assert!(!json.contains("allowed_extensions"));
    }

    #[test]
    fn gecko_manifests_use_the_other_key_and_id_form() {
        let json = manifest_json(std::path::Path::new("/opt/zwebwrap-nm-host"), Flavour::Gecko);
        assert!(json.contains("allowed_extensions"));
        assert!(json.contains(GECKO_EXTENSION_ID));
        assert!(!json.contains("chrome-extension://"));
    }

    #[test]
    fn manifests_are_valid_json_even_with_an_awkward_path() {
        let path = std::path::Path::new(r#"/home/a "b"/zwebwrap-nm-host"#);
        let json = manifest_json(path, Flavour::Chromium);
        let parsed: serde_json::Value = serde_json::from_str(&json).expect("valid json");
        assert_eq!(parsed["path"], r#"/home/a "b"/zwebwrap-nm-host"#);
    }

    #[test]
    fn a_path_with_a_quote_cannot_break_out_of_the_launcher_script() {
        let quoted = shell_quote("/home/o'brien/zwebwrap");
        assert_eq!(quoted, r"'/home/o'\''brien/zwebwrap'");
        assert!(!quoted.contains("; "));
    }

    #[test]
    fn the_extension_id_is_the_shape_chrome_uses() {
        assert_eq!(CHROME_EXTENSION_ID.len(), 32);
        assert!(CHROME_EXTENSION_ID.chars().all(|c| ('a'..='p').contains(&c)));
    }
}

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::path::PathBuf;

pub const DATA_DIR_VAR: &str = "ZWEBWRAP_DATA_DIR";

fn override_dir() -> Option<PathBuf> {
    std::env::var_os(DATA_DIR_VAR).filter(|v| !v.is_empty()).map(PathBuf::from)
}

pub fn data_dir() -> Result<PathBuf> {
    let dir = match override_dir() {
        Some(dir) => dir,
        None => ProjectDirs::from("dev", "ZWebWrap", "ZWebWrap")
            .context("could not determine a home directory for this user")?
            .data_dir()
            .to_path_buf(),
    };
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating data dir {}", dir.display()))?;
    Ok(dir)
}

pub fn config_dir() -> Result<PathBuf> {
    let dir = match override_dir() {
        Some(dir) => dir.join("config"),
        None => ProjectDirs::from("dev", "ZWebWrap", "ZWebWrap")
            .context("could not determine a home directory for this user")?
            .config_dir()
            .to_path_buf(),
    };
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating config dir {}", dir.display()))?;
    Ok(dir)
}

pub fn registry_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("apps.json"))
}

pub fn registry_lock_file() -> Result<PathBuf> {
    Ok(data_dir()?.join("apps.lock"))
}

pub fn icon_dir() -> Result<PathBuf> {
    let dir = data_dir()?.join("icons");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating icon dir {}", dir.display()))?;
    Ok(dir)
}

pub fn webview_data_dir() -> Result<PathBuf> {
    let dir = data_dir()?.join("webview");
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating webview data dir {}", dir.display()))?;
    Ok(dir)
}

pub fn webview_profile_dir(id: &str) -> Result<PathBuf> {
    let dir = webview_data_dir()?.join(id);
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating profile dir {}", dir.display()))?;
    Ok(dir)
}

pub fn current_exe() -> Result<PathBuf> {
    if let Some(appimage) = std::env::var_os("APPIMAGE").filter(|v| !v.is_empty()) {
        let path = PathBuf::from(appimage);
        if path.is_file() {
            return Ok(path);
        }
        tracing::warn!(path = %path.display(), "$APPIMAGE is set but is not a file; ignoring it");
    }

    let exe = std::env::current_exe().context("locating the running executable")?;
    let resolved = std::fs::canonicalize(&exe).unwrap_or(exe);
    Ok(strip_verbatim(resolved))
}

#[cfg(windows)]
fn strip_verbatim(path: PathBuf) -> PathBuf {
    let text = path.to_string_lossy();
    if let Some(rest) = text.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{rest}"));
    }
    if let Some(rest) = text.strip_prefix(r"\\?\") {
        let bytes = rest.as_bytes();
        if bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':' {
            return PathBuf::from(rest);
        }
    }
    path
}

#[cfg(not(windows))]
fn strip_verbatim(path: PathBuf) -> PathBuf {
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_appimage_shortcut_points_at_the_bundle_not_its_mount() {
        let dir = tempfile::tempdir().unwrap();
        let bundle = dir.path().join("ZWebWrap.AppImage");
        std::fs::write(&bundle, "x").unwrap();

        temp_env::with_var("APPIMAGE", Some(&bundle), || {
            assert_eq!(current_exe().unwrap(), bundle);
        });
    }

    #[test]
    fn a_stale_appimage_variable_is_ignored_rather_than_trusted() {
        temp_env::with_var("APPIMAGE", Some("/tmp/gone.AppImage"), || {
            let exe = current_exe().unwrap();
            assert!(exe.is_file(), "fell back to the real executable");
        });
    }

    #[test]
    fn an_ordinary_build_uses_its_own_path() {
        temp_env::with_var("APPIMAGE", None::<&str>, || {
            let exe = current_exe().unwrap();
            assert!(exe.is_absolute() && exe.is_file());
        });
    }

    #[cfg(windows)]
    #[test]
    fn the_verbatim_prefix_is_removed_so_cmd_and_shortcuts_accept_the_path() {
        assert_eq!(
            strip_verbatim(PathBuf::from(r"\\?\C:\Users\x\zwebwrap.exe")),
            PathBuf::from(r"C:\Users\x\zwebwrap.exe")
        );

        assert_eq!(
            strip_verbatim(PathBuf::from(r"\\?\UNC\server\share\zwebwrap.exe")),
            PathBuf::from(r"\\server\share\zwebwrap.exe")
        );

        assert_eq!(
            strip_verbatim(PathBuf::from(r"C:\Users\x\zwebwrap.exe")),
            PathBuf::from(r"C:\Users\x\zwebwrap.exe")
        );
    }
}

use crate::paths;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

impl Credential {
    pub fn is_set(&self) -> bool {
        !self.username.trim().is_empty()
    }
}

fn file() -> Result<PathBuf> {
    Ok(paths::data_dir()?.join("credentials.json"))
}

type Store = BTreeMap<String, Credential>;

fn read() -> Store {
    let Ok(path) = file() else { return Store::new() };
    match std::fs::read_to_string(&path) {
        Ok(raw) => serde_json::from_str(&raw).unwrap_or_else(|e| {
            tracing::warn!(error = %e, "stored logins unreadable; treating as empty");
            Store::new()
        }),
        Err(_) => Store::new(),
    }
}

fn write(store: &Store) -> Result<()> {
    let path = file()?;
    let parent = path.parent().context("credential path has no parent")?;
    std::fs::create_dir_all(parent)?;

    let tmp = parent.join(".credentials.json.tmp");
    let json = serde_json::to_string_pretty(store)?;
    owner_only(&tmp)?;
    std::fs::write(&tmp, json)?;
    owner_only(&tmp)?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

#[cfg(unix)]
fn owner_only(path: &std::path::Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    if !path.exists() {
        std::fs::File::create(path)?;
    }
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
    Ok(())
}

#[cfg(not(unix))]
fn owner_only(_path: &std::path::Path) -> Result<()> {
    Ok(())
}

pub fn get(app_id: &str) -> Option<Credential> {
    read().remove(app_id)
}

pub fn set(app_id: &str, credential: Option<Credential>) -> Result<()> {
    let mut store = read();
    match credential.filter(Credential::is_set) {
        Some(credential) => {
            store.insert(app_id.to_string(), credential);
        }
        None => {
            store.remove(app_id);
        }
    }
    write(&store)
}

pub fn forget(app_id: &str) {
    if let Err(e) = set(app_id, None) {
        tracing::warn!(app = app_id, error = %e, "could not remove a stored login");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_login_needs_a_username_but_not_a_password() {
        assert!(Credential { username: "kim".into(), password: String::new() }.is_set());
        assert!(!Credential { username: "  ".into(), password: "secret".into() }.is_set());
    }

    #[cfg(unix)]
    #[test]
    fn the_file_is_owner_only_from_the_moment_it_exists() {
        use std::os::unix::fs::PermissionsExt;

        let dir = std::env::temp_dir().join(format!("zww-cred-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("a temp dir");
        let path = dir.join("credentials.json");

        owner_only(&path).expect("locking a file down");
        let mode = std::fs::metadata(&path).expect("the file exists").permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "another user on this machine could read it");

        std::fs::write(&path, "{}").expect("writing the file");
        owner_only(&path).expect("locking it again");
        let mode = std::fs::metadata(&path).expect("still there").permissions().mode();
        assert_eq!(mode & 0o777, 0o600, "writing must not widen the permissions");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_registry_only_records_that_a_login_exists() {
        let mut app = crate::WrappedApp::new("x", "https://x.test/", "X");
        app.has_auth = true;
        let json = serde_json::to_string(&app).expect("an app serialises");
        assert!(json.contains("has_auth"));
        assert!(!json.contains("password"));
    }
}

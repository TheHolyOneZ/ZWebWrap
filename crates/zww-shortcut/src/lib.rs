use anyhow::Result;
use std::path::PathBuf;
use zww_core::WrappedApp;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(not(any(target_os = "linux", windows)))]
pub mod unsupported;
#[cfg(windows)]
pub mod windows;

#[cfg(any(windows, test))]
pub mod ico;
#[cfg(any(windows, test))]
pub mod index;

#[derive(Debug, Clone)]
pub struct ShortcutSpec {
    pub id: String,
    pub name: String,
    pub url: String,

    pub exec: PathBuf,
    pub icon: Option<PathBuf>,

    pub on_desktop: bool,
}

impl ShortcutSpec {
    pub fn from_app(app: &WrappedApp, exec: PathBuf) -> Self {
        Self {
            id: app.id.clone(),
            name: app.title.clone(),
            url: app.url.clone(),
            exec,
            icon: app.icon.clone(),

            on_desktop: app
                .desktop_shortcut
                .unwrap_or_else(|| zww_core::prefs::load().desktop_shortcut),
        }
    }
}

pub trait ShortcutBackend {
    fn create(&self, spec: &ShortcutSpec) -> Result<PathBuf>;

    fn remove(&self, id: &str) -> Result<()>;

    fn list(&self) -> Result<Vec<PathBuf>>;

    fn desktop_copy(&self, id: &str) -> Option<PathBuf>;
}

pub fn wm_class(id: &str) -> String {
    format!("zwebwrap-{id}")
}

pub fn backend() -> Box<dyn ShortcutBackend> {
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::DesktopEntries::new())
    }
    #[cfg(windows)]
    {
        Box::new(windows::ShellLinks::new())
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        Box::new(unsupported::NoShortcuts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zww_core::WrappedApp;

    fn app() -> WrappedApp {
        WrappedApp::new("youtube-com", "https://youtube.com", "YouTube")
    }

    #[test]
    fn a_per_app_choice_beats_the_global_preference() {
        let mut wanted = app();
        wanted.desktop_shortcut = Some(true);
        assert!(ShortcutSpec::from_app(&wanted, "/usr/bin/zwebwrap".into()).on_desktop);

        let mut unwanted = app();
        unwanted.desktop_shortcut = Some(false);
        assert!(!ShortcutSpec::from_app(&unwanted, "/usr/bin/zwebwrap".into()).on_desktop);
    }

    #[test]
    fn the_spec_carries_what_a_backend_needs_to_name_and_launch_it() {
        let spec = ShortcutSpec::from_app(&app(), "/usr/bin/zwebwrap".into());
        assert_eq!(spec.id, "youtube-com");
        assert_eq!(spec.name, "YouTube");
        assert_eq!(wm_class(&spec.id), "zwebwrap-youtube-com");
    }
}

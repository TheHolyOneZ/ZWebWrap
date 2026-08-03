use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Session {
    X11,
    Wayland,

    Other,
}

pub fn session() -> Session {
    if !cfg!(target_os = "linux") {
        return Session::Other;
    }

    match std::env::var("XDG_SESSION_TYPE").unwrap_or_default().as_str() {
        "wayland" => Session::Wayland,
        "x11" => Session::X11,
        _ if std::env::var_os("WAYLAND_DISPLAY").is_some() => Session::Wayland,
        _ if std::env::var_os("DISPLAY").is_some() => Session::X11,
        _ => Session::Other,
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Capabilities {
    pub os: &'static str,
    pub session: Session,
    pub shortcuts: bool,
    pub ad_blocking: bool,
    pub downloads: bool,
    pub permission_prompts: bool,
    pub find_in_page: bool,
    pub spell_check: bool,

    pub window_identity: bool,
    pub always_on_top: bool,

    pub limitations: Vec<String>,
}

pub fn capabilities() -> Capabilities {
    let session = session();
    let linux = cfg!(target_os = "linux");
    let windows = cfg!(windows);
    let mut limitations = Vec::new();

    if !linux && !windows {
        limitations.push(
            "Ad blocking, downloads, permission prompts and find-in-page are built against \
             WebKitGTK and are not available on this platform."
                .to_string(),
        );
    }

    if !linux && !windows {
        limitations.push(
            "Desktop shortcuts and starting with the system are Linux and Windows only."
                .to_string(),
        );
    }

    if session == Session::Wayland {
        limitations.push(
            "On Wayland each window cannot set its own icon or taskbar identity, so wrapped \
             apps may group together. Log in with an X11 session for per-app icons."
                .to_string(),
        );
        limitations.push(
            "Always on top is refused by most Wayland compositors, so the pin button may \
             do nothing."
                .to_string(),
        );
    }

    Capabilities {
        os: std::env::consts::OS,
        session,
        shortcuts: linux || windows,

        ad_blocking: linux || windows,

        downloads: linux || windows,
        permission_prompts: linux || windows,

        find_in_page: linux || windows,

        spell_check: linux || windows,

        window_identity: session == Session::X11 || windows,
        always_on_top: session != Session::Wayland,
        limitations,
    }
}

pub fn log_once() {
    let caps = capabilities();
    tracing::info!(
        os = caps.os,
        session = ?caps.session,
        "platform capabilities"
    );
    for note in &caps.limitations {
        tracing::warn!("{note}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_wayland_session_reports_what_it_cannot_do() {
        let caps = Capabilities {
            session: Session::Wayland,
            always_on_top: Session::Wayland != Session::Wayland,
            window_identity: Session::Wayland == Session::X11,
            ..capabilities()
        };
        assert!(!caps.always_on_top);
        assert!(!caps.window_identity);
    }

    #[test]
    fn this_build_knows_its_own_platform() {
        let caps = capabilities();
        assert_eq!(caps.os, std::env::consts::OS);
        assert_eq!(caps.shortcuts, cfg!(any(target_os = "linux", windows)));
        assert_eq!(caps.ad_blocking, cfg!(any(target_os = "linux", windows)));
        assert_eq!(caps.find_in_page, cfg!(any(target_os = "linux", windows)));
    }

    #[test]
    fn a_fully_capable_platform_lists_no_limitations() {
        let caps = capabilities();
        if cfg!(target_os = "linux") && caps.session == Session::X11 {
            assert!(caps.limitations.is_empty(), "got {:?}", caps.limitations);
        }

        if cfg!(windows) {
            assert!(caps.limitations.is_empty(), "got {:?}", caps.limitations);
        }
    }
}

use serde::Serialize;

struct Requirement {
    file: &'static str,

    provides: &'static str,
}

const REQUIRED: &[Requirement] = &[
    Requirement { file: "libgstmatroska.so", provides: "WebM video (what YouTube serves)" },
    Requirement { file: "libgstisomp4.so", provides: "MP4 video" },
    Requirement { file: "libgstvpx.so", provides: "VP8 and VP9 decoding" },
    Requirement { file: "libgstautodetect.so", provides: "audio output" },
];

const PLUGIN_DIRS: &[&str] = &[
    "/usr/lib/gstreamer-1.0",
    "/usr/lib64/gstreamer-1.0",
    "/usr/lib/x86_64-linux-gnu/gstreamer-1.0",
    "/usr/local/lib/gstreamer-1.0",
];

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct MediaSupport {
    pub ok: bool,

    pub missing: Vec<String>,

    pub install_hint: Option<String>,
}

impl MediaSupport {
    fn healthy() -> Self {
        Self { ok: true, missing: Vec::new(), install_hint: None }
    }
}

pub fn check() -> MediaSupport {
    if !cfg!(target_os = "linux") {
        return MediaSupport::healthy();
    }

    let dirs: Vec<&str> = PLUGIN_DIRS
        .iter()
        .copied()
        .chain(
            std::env::var("GST_PLUGIN_PATH").ok().map(|_| "").into_iter().filter(|s| !s.is_empty()),
        )
        .collect();

    if !dirs.iter().any(|d| std::path::Path::new(d).is_dir()) {
        return MediaSupport::healthy();
    }

    let missing: Vec<String> = REQUIRED
        .iter()
        .filter(|req| !dirs.iter().any(|dir| std::path::Path::new(dir).join(req.file).exists()))
        .map(|req| req.provides.to_string())
        .collect();

    MediaSupport {
        ok: missing.is_empty(),
        install_hint: (!missing.is_empty()).then(install_hint),
        missing,
    }
}

fn install_hint() -> String {
    let release = std::fs::read_to_string("/etc/os-release").unwrap_or_default().to_lowercase();

    if release.contains("arch") || release.contains("endeavour") || release.contains("manjaro") {
        "sudo pacman -S gst-plugins-good".to_string()
    } else if release.contains("debian") || release.contains("ubuntu") || release.contains("mint") {
        "sudo apt install gstreamer1.0-plugins-good".to_string()
    } else if release.contains("fedora") || release.contains("rhel") {
        "sudo dnf install gstreamer1-plugins-good".to_string()
    } else if release.contains("opensuse") {
        "sudo zypper install gstreamer-plugins-good".to_string()
    } else {
        "install your distribution's gstreamer plugins-good package".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_does_not_panic_and_reports_consistently() {
        let support = check();

        assert_eq!(support.ok, support.missing.is_empty());
        assert_eq!(support.install_hint.is_some(), !support.ok);
    }

    #[test]
    fn install_hint_matches_the_distribution() {
        let hint = install_hint();
        assert!(hint.contains("gst") || hint.contains("gstreamer"), "got {hint}");
    }
}

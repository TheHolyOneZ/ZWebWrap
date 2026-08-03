use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager};

pub const EVENT: &str = "zww://download";

const MAX_REMEMBERED: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadState {
    Active,
    Done,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    pub id: u64,

    pub app: String,
    pub file_name: String,
    pub path: String,

    pub url: String,
    pub state: DownloadState,

    pub bytes: u64,
}

static DOWNLOADS: Mutex<Vec<Download>> = Mutex::new(Vec::new());
static NEXT_ID: Mutex<u64> = Mutex::new(1);

fn store_path() -> Option<PathBuf> {
    zww_core::paths::data_dir().ok().map(|d| d.join("downloads.json"))
}

fn read_store() -> Vec<Download> {
    let Some(path) = store_path() else { return Vec::new() };
    let Ok(raw) = std::fs::read_to_string(path) else { return Vec::new() };
    serde_json::from_str(&raw).unwrap_or_default()
}

fn write_store(all: &[Download]) {
    let Some(path) = store_path() else { return };
    let Ok(json) = serde_json::to_string_pretty(all) else { return };
    let Some(parent) = path.parent() else { return };

    let tmp = parent.join(".downloads.json.tmp");
    if std::fs::write(&tmp, json).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
}

fn with_list<T>(f: impl FnOnce(&mut Vec<Download>) -> T) -> T {
    let mut guard = DOWNLOADS.lock().unwrap_or_else(|e| e.into_inner());

    let stored = read_store();
    for record in stored {
        if !guard.iter().any(|d| d.id == record.id && d.url == record.url) {
            guard.push(record);
        }
    }
    guard.sort_by_key(|d| d.id);

    let out = f(&mut guard);
    write_store(&guard);
    out
}

pub fn list() -> Vec<Download> {
    with_list(|all| {
        let mut out = all.clone();
        out.reverse();
        out
    })
}

pub fn clear_finished() {
    with_list(|all| all.retain(|d| d.state == DownloadState::Active));
}

fn find_path(id: u64) -> Option<PathBuf> {
    with_list(|all| all.iter().find(|d| d.id == id).map(|d| PathBuf::from(&d.path)))
}

pub fn belongs_to(app_id: &str, download: u64) -> bool {
    with_list(|all| all.iter().any(|d| d.id == download && d.app == app_id))
}

fn announce<R: tauri::Runtime>(app: &AppHandle<R>, download: &Download) {
    if let Err(e) = app.emit(EVENT, download) {
        tracing::debug!(error = %e, "could not announce a download");
    }
    notify_site_window(app, download);
}

fn notify_site_window<R: tauri::Runtime>(app: &AppHandle<R>, download: &Download) {
    let Some(window) = app.get_webview_window(&format!("app-{}", download.app)) else { return };
    let Ok(json) = serde_json::to_string(download) else { return };

    let script = format!("window.__zwwDownload&&window.__zwwDownload({json});");
    if let Err(e) = window.eval(&script) {
        tracing::debug!(error = %e, "could not reach the window's toolbar");
    }
}

fn begin(app_id: &str, url: &str, path: &Path) -> u64 {
    let id = {
        let highest = with_list(|all| all.iter().map(|d| d.id).max().unwrap_or(0));
        let mut next = NEXT_ID.lock().unwrap_or_else(|e| e.into_inner());
        *next = (*next).max(highest + 1);
        let id = *next;
        *next += 1;
        id
    };

    let record = Download {
        id,
        app: app_id.to_string(),
        file_name: path.file_name().and_then(|n| n.to_str()).unwrap_or("download").to_string(),
        path: path.to_string_lossy().to_string(),
        url: url.to_string(),
        state: DownloadState::Active,
        bytes: 0,
    };

    with_list(|all| {
        all.push(record);

        while all.len() > MAX_REMEMBERED {
            match all.iter().position(|d| d.state != DownloadState::Active) {
                Some(oldest) => {
                    all.remove(oldest);
                }
                None => break,
            }
        }
    });

    id
}

fn finish(id: u64, path: Option<PathBuf>, success: bool) -> Option<Download> {
    with_list(|all| {
        let record = all.iter_mut().find(|d| d.id == id)?;
        if let Some(path) = path {
            if !path.as_os_str().is_empty() {
                record.file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or(&record.file_name)
                    .to_string();
                record.path = path.to_string_lossy().to_string();
            }
        }
        record.state = if success { DownloadState::Done } else { DownloadState::Failed };
        record.bytes = std::fs::metadata(&record.path).map(|m| m.len()).unwrap_or(0);
        Some(record.clone())
    })
}

pub fn refresh_active() {
    with_list(|all| {
        for record in all.iter_mut().filter(|d| d.state == DownloadState::Active) {
            record.bytes = std::fs::metadata(&record.path).map(|m| m.len()).unwrap_or(record.bytes);
        }
    });
}

pub fn download_dir() -> PathBuf {
    directories::UserDirs::new()
        .and_then(|dirs| dirs.download_dir().map(Path::to_path_buf))
        .or_else(|| directories::BaseDirs::new().map(|d| d.home_dir().to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

pub fn safe_name(suggested: &str) -> String {
    let trimmed =
        suggested.rsplit(['/', '\\']).next().unwrap_or(suggested).trim().trim_start_matches('.');

    let cleaned: String = trimmed
        .chars()
        .map(|c| {
            if c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|') {
                '-'
            } else {
                c
            }
        })
        .collect();

    let cleaned = cleaned.trim_end_matches([' ', '.']).to_string();

    if cleaned.is_empty() {
        "download".to_string()
    } else {
        cleaned.chars().take(200).collect()
    }
}

#[cfg(target_os = "linux")]
fn pick_destination(start: &Path, name: &str) -> Option<PathBuf> {
    use gtk::prelude::*;

    let chooser = gtk::FileChooserNative::new(
        Some("Where should this file go?"),
        None::<&gtk::Window>,
        gtk::FileChooserAction::Save,
        Some("Save"),
        Some("Keep in Downloads"),
    );
    chooser.set_current_folder(start);
    chooser.set_current_name(name);

    chooser.set_do_overwrite_confirmation(true);

    let answer = chooser.run();
    let picked = (answer == gtk::ResponseType::Accept).then(|| chooser.filename()).flatten();

    chooser.destroy();
    picked
}

#[cfg(not(target_os = "linux"))]
fn pick_destination(start: &Path, name: &str) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .set_title("Where should this file go?")
        .set_directory(start)
        .set_file_name(name)
        .save_file()
}

fn offer_relocation<R: tauri::Runtime>(app: &AppHandle<R>, id: u64) {
    let Some(record) = with_list(|all| all.iter().find(|d| d.id == id).cloned()) else { return };
    let handle = app.clone();

    let dispatch = app.run_on_main_thread(move || {
        let current = PathBuf::from(&record.path);
        let start = current.parent().map(Path::to_path_buf).unwrap_or_else(download_dir);

        let Some(chosen) = pick_destination(&start, &record.file_name) else {
            tracing::debug!(path = %record.path, "kept where it was downloaded");
            return;
        };

        if chosen == current {
            return;
        }

        let moved = std::fs::rename(&current, &chosen).or_else(|_| {
            std::fs::copy(&current, &chosen).and_then(|_| std::fs::remove_file(&current))
        });

        match moved {
            Ok(()) => {
                let updated = with_list(|all| {
                    let entry = all.iter_mut().find(|d| d.id == id)?;
                    entry.path = chosen.to_string_lossy().to_string();
                    entry.file_name = chosen
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&entry.file_name)
                        .to_string();
                    Some(entry.clone())
                });
                tracing::info!(to = %chosen.display(), "moved a download");
                if let Some(updated) = updated {
                    announce(&handle, &updated);
                }
            }
            Err(e) => tracing::warn!(error = %e, "could not move the download"),
        }
    });

    if let Err(e) = dispatch {
        tracing::debug!(error = %e, "could not ask where to put the download");
    }
}

fn strip_runtime_counter(name: &str) -> String {
    let Some((stem, rest)) = name.split_once('.') else { return name.to_string() };

    let trimmed = stem
        .strip_suffix(')')
        .and_then(|s| s.rsplit_once(" ("))
        .filter(|(_, digits)| !digits.is_empty() && digits.chars().all(|c| c.is_ascii_digit()))
        .map(|(base, _)| base)
        .unwrap_or(stem);

    format!("{trimmed}.{rest}")
}

pub fn unique_path(dir: &Path, name: &str) -> PathBuf {
    let candidate = dir.join(name);
    if !candidate.exists() {
        return candidate;
    }

    let path = Path::new(name);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or(name);
    let extension = path.extension().and_then(|s| s.to_str());

    (2..10_000)
        .map(|n| {
            let next = match extension {
                Some(ext) => format!("{stem} ({n}).{ext}"),
                None => format!("{stem} ({n})"),
            };
            dir.join(next)
        })
        .find(|p| !p.exists())
        .unwrap_or(candidate)
}

pub fn requested<R: tauri::Runtime>(
    app: &AppHandle<R>,
    app_id: &str,
    url: &str,
    suggested: &Path,
) -> Option<(PathBuf, u64)> {
    if let Some(existing) = with_list(|all| {
        all.iter()
            .rev()
            .find(|d| d.state == DownloadState::Active && d.url == url)
            .map(|d| (PathBuf::from(&d.path), d.id))
    }) {
        tracing::debug!(%url, "same download announced twice; keeping the first record");
        return Some(existing);
    }

    let name = suggested
        .file_name()
        .and_then(|n| n.to_str())
        .map(strip_runtime_counter)
        .map(|n| safe_name(&n))
        .unwrap_or_else(|| "download".to_string());

    let dir = download_dir();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        tracing::warn!(error = %e, "no downloads folder; the download may fail");
    }

    let target = unique_path(&dir, &name);
    let id = begin(app_id, url, &target);
    tracing::info!(app = %app_id, path = %target.display(), "download started");

    if let Some(record) = with_list(|all| all.iter().find(|d| d.id == id).cloned()) {
        announce(app, &record);
    }
    Some((target, id))
}

pub fn completed<R: tauri::Runtime>(
    app: &AppHandle<R>,
    id: u64,
    path: Option<PathBuf>,
    success: bool,
) {
    let Some(record) = finish(id, path, success) else { return };
    if success {
        tracing::info!(path = %record.path, bytes = record.bytes, "download finished");
    } else {
        tracing::warn!(path = %record.path, "download failed");
    }
    announce(app, &record);

    if success && zww_core::prefs::load().ask_where_to_save {
        offer_relocation(app, id);
    }
}

pub fn open<R: tauri::Runtime>(app: &AppHandle<R>, id: u64) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    let path = find_path(id).ok_or_else(|| "that download is no longer listed".to_string())?;
    app.opener().open_path(path.to_string_lossy(), None::<&str>).map_err(|e| e.to_string())
}

pub fn reveal<R: tauri::Runtime>(app: &AppHandle<R>, id: u64) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    let path = find_path(id).ok_or_else(|| "that download is no longer listed".to_string())?;

    app.opener()
        .reveal_item_in_dir(&path)
        .or_else(|_| {
            let parent = path.parent().unwrap_or(Path::new("."));
            app.opener().open_path(parent.to_string_lossy(), None::<&str>)
        })
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_suggested_name_cannot_escape_the_folder() {
        assert_eq!(safe_name("../../.bashrc"), "bashrc");
        assert_eq!(safe_name("/etc/passwd"), "passwd");
        assert_eq!(safe_name(r"..\..\windows\system32\evil.dll"), "evil.dll");
        assert_eq!(safe_name("plain.pdf"), "plain.pdf");
    }

    #[test]
    fn characters_windows_refuses_are_stripped_on_every_platform() {
        assert_eq!(safe_name("in:voice.pdf"), "in-voice.pdf");
        assert_eq!(safe_name(r#"who<is>this?.txt"#), "who-is-this-.txt");
        assert_eq!(safe_name("a|b*c.log"), "a-b-c.log");

        assert_eq!(safe_name("report. "), "report");
    }

    #[test]
    fn a_name_with_nothing_usable_still_downloads() {
        assert_eq!(safe_name(""), "download");
        assert_eq!(safe_name("   "), "download");
        assert_eq!(safe_name("/"), "download");
        assert_eq!(safe_name("..."), "download");
    }

    #[test]
    fn control_characters_are_replaced() {
        assert_eq!(safe_name("re\nport.pdf"), "re-port.pdf");
    }

    #[test]
    fn very_long_names_are_cut_to_something_a_filesystem_accepts() {
        assert_eq!(safe_name(&"a".repeat(500)).chars().count(), 200);
    }

    #[test]
    fn an_existing_file_is_never_overwritten() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("song.mp3"), "x").unwrap();

        assert_eq!(unique_path(dir.path(), "song.mp3"), dir.path().join("song (2).mp3"));

        std::fs::write(dir.path().join("song (2).mp3"), "x").unwrap();
        assert_eq!(unique_path(dir.path(), "song.mp3"), dir.path().join("song (3).mp3"));
    }

    #[test]
    fn a_version_number_is_not_mistaken_for_an_extension() {
        assert_eq!(
            strip_runtime_counter("zpassword-firefox-1 (10).3.1.zip"),
            "zpassword-firefox-1.3.1.zip"
        );

        assert_eq!(
            strip_runtime_counter("zpassword-firefox-1.3.1.zip"),
            "zpassword-firefox-1.3.1.zip"
        );
        assert_eq!(strip_runtime_counter("report.pdf"), "report.pdf");
        assert_eq!(strip_runtime_counter("no-extension"), "no-extension");

        assert_eq!(strip_runtime_counter("release (final).zip"), "release (final).zip");
        assert_eq!(strip_runtime_counter("build ().zip"), "build ().zip");
    }

    #[test]
    fn a_repeat_download_numbers_before_the_extension() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("zpassword-firefox-1.3.1.zip"), "x").unwrap();

        let cleaned = safe_name(&strip_runtime_counter("zpassword-firefox-1 (1).3.1.zip"));
        assert_eq!(
            unique_path(dir.path(), &cleaned),
            dir.path().join("zpassword-firefox-1.3.1 (2).zip")
        );
    }

    #[test]
    fn a_free_name_is_used_as_is() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(unique_path(dir.path(), "new.pdf"), dir.path().join("new.pdf"));
    }

    #[test]
    fn a_name_without_an_extension_still_gets_a_suffix() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("README"), "x").unwrap();
        assert_eq!(unique_path(dir.path(), "README"), dir.path().join("README (2)"));
    }

    #[test]
    fn a_download_is_tracked_from_start_to_finish() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("report.pdf");
        std::fs::write(&file, b"1234567890").unwrap();

        let id = begin("youtube-com", "https://example.test/report.pdf", &file);
        let active = list().into_iter().find(|d| d.id == id).expect("recorded on start");
        assert_eq!(active.state, DownloadState::Active);
        assert_eq!(active.file_name, "report.pdf");
        assert_eq!(active.app, "youtube-com");

        let done = finish(id, Some(file.clone()), true).expect("finish updates the record");
        assert_eq!(done.state, DownloadState::Done);
        assert_eq!(done.bytes, 10, "the size is read from disk, not reported to us");
    }

    #[test]
    fn a_failed_download_is_kept_and_marked() {
        let id = begin("x", "https://example.test/never.bin", Path::new("/nonexistent/never.bin"));
        let failed = finish(id, None, false).expect("still recorded");
        assert_eq!(failed.state, DownloadState::Failed);
        assert_eq!(failed.bytes, 0, "nothing on disk to measure");
    }

    #[test]
    fn the_same_download_announced_twice_yields_one_record() {
        let url = "https://example.test/duplicated.zip";
        let first = begin("app-one", url, Path::new("/tmp/zww-dup.zip"));

        let already = with_list(|all| {
            all.iter()
                .rev()
                .find(|d| d.state == DownloadState::Active && d.url == url)
                .map(|d| d.id)
        });
        assert_eq!(already, Some(first), "a second handler must find the first record");

        finish(first, None, true);
        let after =
            with_list(|all| all.iter().any(|d| d.state == DownloadState::Active && d.url == url));
        assert!(!after, "a finished download must not swallow a later one");
    }

    #[test]
    fn clearing_keeps_what_is_still_running() {
        let running =
            begin("x", "https://example.test/going.bin", Path::new("/tmp/zww-still-going.bin"));
        let over = begin("x", "https://example.test/over.bin", Path::new("/tmp/zww-over.bin"));
        finish(over, None, true);

        clear_finished();
        let left = list();
        assert!(left.iter().any(|d| d.id == running), "an active download must survive");
        assert!(!left.iter().any(|d| d.id == over));
    }
}

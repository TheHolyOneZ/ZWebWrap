use serde::Serialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Bookmark {
    pub title: String,
    pub url: String,

    pub source: String,
}

struct Source {
    name: &'static str,

    #[cfg_attr(windows, allow(dead_code))]
    unix_dir: &'static str,

    #[cfg_attr(not(windows), allow(dead_code))]
    windows_dir: &'static str,

    #[cfg_attr(not(windows), allow(dead_code))]
    roaming: bool,
}

const SOURCES: &[Source] = &[
    Source {
        name: "Chrome",
        unix_dir: "google-chrome",
        windows_dir: r"Google\Chrome\User Data",
        roaming: false,
    },
    Source {
        name: "Chromium",
        unix_dir: "chromium",
        windows_dir: r"Chromium\User Data",
        roaming: false,
    },
    Source {
        name: "Brave",
        unix_dir: "BraveSoftware/Brave-Browser",
        windows_dir: r"BraveSoftware\Brave-Browser\User Data",
        roaming: false,
    },
    Source {
        name: "Edge",
        unix_dir: "microsoft-edge",
        windows_dir: r"Microsoft\Edge\User Data",
        roaming: false,
    },
    Source {
        name: "Vivaldi",
        unix_dir: "vivaldi",
        windows_dir: r"Vivaldi\User Data",
        roaming: false,
    },
    Source {
        name: "Opera",
        unix_dir: "opera",
        windows_dir: r"Opera Software\Opera Stable",
        roaming: true,
    },
];

#[cfg(target_os = "linux")]
fn profile_root(source: &Source) -> Option<PathBuf> {
    let config = if let Some(xdg) = std::env::var_os("XDG_CONFIG_HOME").filter(|v| !v.is_empty()) {
        PathBuf::from(xdg)
    } else {
        directories::BaseDirs::new()?.home_dir().join(".config")
    };
    Some(config.join(source.unix_dir))
}

#[cfg(windows)]
fn profile_root(source: &Source) -> Option<PathBuf> {
    let base = if source.roaming { "APPDATA" } else { "LOCALAPPDATA" };
    let root = std::env::var_os(base).filter(|v| !v.is_empty())?;
    Some(PathBuf::from(root).join(source.windows_dir))
}

#[cfg(not(any(target_os = "linux", windows)))]
fn profile_root(_source: &Source) -> Option<PathBuf> {
    None
}

fn bookmark_files() -> Vec<(&'static str, PathBuf)> {
    let mut found = Vec::new();

    for source in SOURCES {
        let Some(profile) = profile_root(source) else { continue };
        if !profile.is_dir() {
            continue;
        }

        for name in ["Default", "Profile 1", "Profile 2", "Profile 3"] {
            let file = profile.join(name).join("Bookmarks");
            if file.is_file() {
                found.push((source.name, file));
            }
        }
    }
    found
}

fn collect(node: &serde_json::Value, source: &str, out: &mut Vec<Bookmark>) {
    match node.get("type").and_then(|t| t.as_str()) {
        Some("url") => {
            let Some(url) = node.get("url").and_then(|u| u.as_str()) else { return };

            if !url.starts_with("http://") && !url.starts_with("https://") {
                return;
            }
            let title = node
                .get("name")
                .and_then(|n| n.as_str())
                .filter(|n| !n.trim().is_empty())
                .unwrap_or(url);
            out.push(Bookmark {
                title: title.to_string(),
                url: url.to_string(),
                source: source.to_string(),
            });
        }
        Some("folder") => {
            if let Some(children) = node.get("children").and_then(|c| c.as_array()) {
                for child in children {
                    collect(child, source, out);
                }
            }
        }
        _ => {}
    }
}

pub fn parse_chromium(json: &str, source: &str) -> Vec<Bookmark> {
    let Ok(root) = serde_json::from_str::<serde_json::Value>(json) else { return Vec::new() };
    let Some(roots) = root.get("roots").and_then(|r| r.as_object()) else { return Vec::new() };

    let mut out = Vec::new();
    for (_, tree) in roots {
        collect(tree, source, &mut out);
    }
    out
}

fn firefox_root() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        directories::BaseDirs::new().map(|d| d.home_dir().join(".mozilla/firefox"))
    }
    #[cfg(windows)]
    {
        std::env::var_os("APPDATA")
            .filter(|v| !v.is_empty())
            .map(|base| PathBuf::from(base).join(r"Mozilla\Firefox\Profiles"))
    }
    #[cfg(not(any(target_os = "linux", windows)))]
    {
        None
    }
}

fn firefox_databases() -> Vec<PathBuf> {
    let Some(root) = firefox_root() else { return Vec::new() };
    let Ok(entries) = std::fs::read_dir(&root) else { return Vec::new() };

    let mut found: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path().join("places.sqlite"))
        .filter(|p| p.is_file())
        .collect();

    found.sort();
    found
}

pub fn firefox_bookmarks(database: &Path) -> Vec<Bookmark> {
    let Ok(temp) = tempdir_for_copy() else { return Vec::new() };
    let copy = temp.join("places.sqlite");

    if std::fs::copy(database, &copy).is_err() {
        tracing::debug!(path = %database.display(), "could not copy the Firefox database");
        return Vec::new();
    }

    let wal = database.with_extension("sqlite-wal");
    if wal.is_file() {
        let _ = std::fs::copy(&wal, temp.join("places.sqlite-wal"));
    }

    let found = read_places(&copy);

    let _ = std::fs::remove_dir_all(&temp);
    found
}

fn tempdir_for_copy() -> std::io::Result<PathBuf> {
    let dir = std::env::temp_dir().join(format!("zwebwrap-places-{}", std::process::id()));
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

pub fn read_places(database: &Path) -> Vec<Bookmark> {
    let open = rusqlite::Connection::open_with_flags(
        database,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_URI,
    );
    let Ok(connection) = open else {
        tracing::debug!(path = %database.display(), "could not open the Firefox database");
        return Vec::new();
    };

    let query = "SELECT COALESCE(b.title, ''), p.url \
                 FROM moz_bookmarks b JOIN moz_places p ON b.fk = p.id \
                 WHERE b.type = 1 AND p.url IS NOT NULL \
                 ORDER BY b.id";

    let Ok(mut statement) = connection.prepare(query) else {
        tracing::debug!("the Firefox bookmark schema is not what was expected");
        return Vec::new();
    };

    let rows =
        statement.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)));
    let Ok(rows) = rows else { return Vec::new() };

    rows.filter_map(Result::ok)
        .filter(|(_, url)| url.starts_with("http://") || url.starts_with("https://"))
        .map(|(title, url)| Bookmark {
            title: if title.trim().is_empty() { url.clone() } else { title },
            url,
            source: "Firefox".to_string(),
        })
        .collect()
}

pub fn discover() -> Vec<Bookmark> {
    let mut all = Vec::new();
    for (source, path) in bookmark_files() {
        match std::fs::read_to_string(&path) {
            Ok(text) => all.extend(parse_chromium(&text, source)),
            Err(e) => tracing::debug!(path = %path.display(), error = %e, "unreadable bookmarks"),
        }
    }

    for database in firefox_databases() {
        all.extend(firefox_bookmarks(&database));
    }

    dedupe(all)
}

pub fn dedupe(items: Vec<Bookmark>) -> Vec<Bookmark> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for item in items {
        let key = item.url.trim_end_matches('/').to_ascii_lowercase();
        if seen.insert(key) {
            out.push(item);
        }
    }
    out
}

pub fn sources() -> Vec<(&'static str, PathBuf)> {
    bookmark_files()
}

pub fn read_file(path: &Path, source: &str) -> Vec<Bookmark> {
    std::fs::read_to_string(path).map(|t| parse_chromium(&t, source)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "roots": {
        "bookmark_bar": {
          "type": "folder",
          "name": "Bookmarks bar",
          "children": [
            { "type": "url", "name": "YouTube", "url": "https://www.youtube.com/" },
            { "type": "url", "name": "", "url": "https://example.com/no-title" },
            { "type": "url", "name": "Bookmarklet", "url": "javascript:void(0)" },
            { "type": "url", "name": "Settings", "url": "chrome://settings" },
            {
              "type": "folder",
              "name": "Work",
              "children": [
                { "type": "url", "name": "GitHub", "url": "https://github.com/" }
              ]
            }
          ]
        },
        "other": { "type": "folder", "children": [] }
      }
    }"#;

    #[test]
    fn nested_folders_are_walked() {
        let found = parse_chromium(SAMPLE, "Chrome");
        let titles: Vec<&str> = found.iter().map(|b| b.title.as_str()).collect();
        assert!(titles.contains(&"YouTube"));
        assert!(titles.contains(&"GitHub"), "a bookmark inside a folder must be found");
    }

    #[test]
    fn only_things_that_can_become_apps_are_offered() {
        let found = parse_chromium(SAMPLE, "Chrome");
        let urls: Vec<&str> = found.iter().map(|b| b.url.as_str()).collect();
        assert!(!urls.iter().any(|u| u.starts_with("javascript:")), "bookmarklets are not apps");
        assert!(!urls.iter().any(|u| u.starts_with("chrome://")), "browser pages are not apps");
        assert_eq!(found.len(), 3);
    }

    #[test]
    fn an_untitled_bookmark_falls_back_to_its_address() {
        let found = parse_chromium(SAMPLE, "Chrome");
        let untitled = found.iter().find(|b| b.url.contains("no-title")).unwrap();
        assert_eq!(untitled.title, "https://example.com/no-title");
    }

    #[test]
    fn the_browser_is_recorded() {
        assert!(parse_chromium(SAMPLE, "Brave").iter().all(|b| b.source == "Brave"));
    }

    #[test]
    fn a_file_we_do_not_understand_yields_nothing_rather_than_failing() {
        assert!(parse_chromium("not json at all", "Chrome").is_empty());
        assert!(parse_chromium("{}", "Chrome").is_empty());
        assert!(parse_chromium(r#"{"roots":{"x":{"type":"url"}}}"#, "Chrome").is_empty());
    }

    #[test]
    fn the_same_site_in_two_browsers_is_one_app() {
        let items = vec![
            Bookmark {
                title: "YouTube".into(),
                url: "https://youtube.com".into(),
                source: "Chrome".into(),
            },
            Bookmark {
                title: "YT".into(),
                url: "https://youtube.com/".into(),
                source: "Brave".into(),
            },
            Bookmark {
                title: "GitHub".into(),
                url: "https://github.com".into(),
                source: "Brave".into(),
            },
        ];

        let out = dedupe(items);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].title, "YouTube", "the first title seen wins");
    }

    #[test]
    fn reading_a_file_that_is_not_there_is_empty_not_an_error() {
        assert!(read_file(Path::new("/nonexistent/Bookmarks"), "Chrome").is_empty());
    }

    fn firefox_fixture(dir: &Path) -> PathBuf {
        let path = dir.join("places.sqlite");
        let db = rusqlite::Connection::open(&path).unwrap();
        db.execute_batch(
            "CREATE TABLE moz_places (id INTEGER PRIMARY KEY, url TEXT);
             CREATE TABLE moz_bookmarks (id INTEGER PRIMARY KEY, type INTEGER, fk INTEGER, title TEXT);
             INSERT INTO moz_places (id, url) VALUES
               (1, 'https://www.youtube.com/'),
               (2, 'https://github.com/'),
               (3, 'place:type=6&sort=14'),
               (4, 'javascript:void(0)'),
               (5, 'https://example.com/untitled');
             INSERT INTO moz_bookmarks (id, type, fk, title) VALUES
               (10, 2, NULL, 'Bookmarks Toolbar'),
               (11, 1, 1, 'YouTube'),
               (12, 1, 2, 'GitHub'),
               (13, 1, 3, 'Recent Tags'),
               (14, 1, 4, 'Bookmarklet'),
               (15, 1, 5, '');",
        )
        .unwrap();
        path
    }

    #[test]
    fn firefox_bookmarks_are_read_from_its_database() {
        let dir = tempfile::tempdir().unwrap();
        let found = read_places(&firefox_fixture(dir.path()));

        let titles: Vec<&str> = found.iter().map(|b| b.title.as_str()).collect();
        assert!(titles.contains(&"YouTube"));
        assert!(titles.contains(&"GitHub"));
        assert!(found.iter().all(|b| b.source == "Firefox"));
    }

    #[test]
    fn firefox_folders_and_unwrappable_entries_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let found = read_places(&firefox_fixture(dir.path()));
        let urls: Vec<&str> = found.iter().map(|b| b.url.as_str()).collect();

        assert!(!urls.iter().any(|u| u.starts_with("place:")), "saved searches are not apps");
        assert!(!urls.iter().any(|u| u.starts_with("javascript:")), "bookmarklets are not apps");

        assert_eq!(found.len(), 3, "got {found:?}");
    }

    #[test]
    fn an_untitled_firefox_bookmark_falls_back_to_its_address() {
        let dir = tempfile::tempdir().unwrap();
        let found = read_places(&firefox_fixture(dir.path()));
        let untitled = found.iter().find(|b| b.url.contains("untitled")).unwrap();
        assert_eq!(untitled.title, "https://example.com/untitled");
    }

    #[test]
    fn a_database_we_cannot_read_yields_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let bogus = dir.path().join("places.sqlite");
        std::fs::write(&bogus, b"not a database at all").unwrap();
        assert!(read_places(&bogus).is_empty());
        assert!(read_places(&dir.path().join("missing.sqlite")).is_empty());
    }

    #[test]
    fn firefox_and_chromium_duplicates_collapse_together() {
        let dir = tempfile::tempdir().unwrap();
        let mut all = parse_chromium(SAMPLE, "Chrome");
        all.extend(read_places(&firefox_fixture(dir.path())));

        let out = dedupe(all);
        let youtube: Vec<&Bookmark> =
            out.iter().filter(|b| b.url.contains("youtube.com")).collect();
        assert_eq!(youtube.len(), 1, "the same site in two browsers is one app");
    }
}

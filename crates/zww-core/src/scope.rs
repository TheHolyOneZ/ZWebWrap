use crate::model::{OffsitePolicy, WrappedApp};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Allow,

    HandOff,

    Block,
}

pub fn decide(target: &Url, app: &WrappedApp) -> Decision {
    match target.scheme() {
        "about" | "blob" => Decision::Allow,

        "javascript" | "file" => Decision::Block,

        "data" => {
            if renderable_data_url(target) {
                Decision::Block
            } else {
                Decision::Allow
            }
        }

        "http" | "https" => {
            if is_in_scope(target, app) {
                Decision::Allow
            } else {
                match app.offsite {
                    OffsitePolicy::Browser => Decision::HandOff,
                    OffsitePolicy::InWindow => Decision::Allow,
                }
            }
        }

        _ => Decision::HandOff,
    }
}

const DOWNLOAD_EXTENSIONS: &[&str] = &[
    "zip", "7z", "rar", "tar", "gz", "tgz", "bz2", "xz", "zst", "iso", "exe", "msi", "msix", "dmg",
    "pkg", "deb", "rpm", "appimage", "apk", "jar", "whl", "crx", "xpi", "pdf", "doc", "docx",
    "xls", "xlsx", "ppt", "pptx", "odt", "ods", "epub", "mp3", "flac", "wav", "mkv", "avi", "mov",
];

pub fn looks_like_a_download(target: &Url) -> bool {
    if !matches!(target.scheme(), "http" | "https") {
        return false;
    }
    let Some(last) = target.path_segments().and_then(|mut s| s.next_back()) else { return false };
    let Some((_, extension)) = last.rsplit_once('.') else { return false };

    let extension = extension.to_ascii_lowercase();
    DOWNLOAD_EXTENSIONS.contains(&extension.as_str())
}

fn renderable_data_url(target: &Url) -> bool {
    let payload = target.path();
    let mime = payload.split([',', ';']).next().unwrap_or("").trim().to_ascii_lowercase();

    matches!(
        mime.as_str(),
        "text/html" | "application/xhtml+xml" | "text/xml" | "application/xml" | "image/svg+xml"
    )
}

pub const AUTH_DOMAINS: &[&str] = &[
    "www.google.com/recaptcha",
    "www.gstatic.com/recaptcha",
    "recaptcha.net",
    "challenges.cloudflare.com",
    "hcaptcha.com",
    "newassets.hcaptcha.com",
    "js.arkoselabs.com",
    "client-api.arkoselabs.com",
    "accounts.google.com",
    "accounts.youtube.com",
    "login.microsoftonline.com",
    "login.live.com",
    "appleid.apple.com",
    "github.com/login",
    "auth0.com",
    "okta.com",
    "duosecurity.com",
    "id.atlassian.com",
    "accounts.spotify.com",
    "discord.com/login",
    "discord.com/oauth2",
    "discord.com/api/oauth2",
    "www.reddit.com/login",
    "gitlab.com/oauth",
    "gitlab.com/users/sign_in",
    "twitch.tv/login",
    "id.twitch.tv",
    "linkedin.com/oauth",
    "www.linkedin.com/uas/login",
    "www.facebook.com/dialog",
    "api.twitter.com/oauth",
    "x.com/i/oauth2",
    "slack.com/openid",
    "cloudflare.com/cdn-cgi/challenge-platform",
];

pub fn registrable_domain(host: &str) -> Option<String> {
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    if host.parse::<std::net::IpAddr>().is_ok() {
        return Some(host);
    }
    psl::domain_str(&host).map(|d| d.to_string())
}

fn host_matches(host: &str, pattern: &str) -> bool {
    let host = host.to_ascii_lowercase();
    let pattern = pattern.to_ascii_lowercase();
    host == pattern || host.ends_with(&format!(".{pattern}"))
}

pub fn auth_domain_for(target: &Url) -> Option<String> {
    if !is_auth_url(target) {
        return None;
    }
    registrable_domain(target.host_str()?)
}

pub fn continues_auth(target: &Url, auth_domain: &str) -> bool {
    if !matches!(target.scheme(), "http" | "https") {
        return false;
    }
    target.host_str().and_then(registrable_domain).is_some_and(|d| d == auth_domain)
}

pub fn is_auth_url(target: &Url) -> bool {
    let Some(host) = target.host_str() else { return false };
    AUTH_DOMAINS.iter().any(|entry| match entry.split_once('/') {
        Some((h, path)) => {
            host_matches(host, h) && target.path().trim_start_matches('/').starts_with(path)
        }
        None => host_matches(host, entry),
    })
}

pub fn is_in_scope(target: &Url, app: &WrappedApp) -> bool {
    if !matches!(target.scheme(), "http" | "https") {
        return false;
    }
    let Some(target_host) = target.host_str() else { return false };

    if let Ok(app_url) = Url::parse(&app.url) {
        if let Some(app_host) = app_url.host_str() {
            match (registrable_domain(app_host), registrable_domain(target_host)) {
                (Some(a), Some(t)) if a == t => return true,

                (None, None) if app_host.eq_ignore_ascii_case(target_host) => return true,
                _ => {}
            }
        }
    }

    if app.extra_scope.iter().any(|p| host_matches(target_host, p)) {
        return true;
    }

    is_auth_url(target)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app_for(url: &str) -> WrappedApp {
        WrappedApp::new("t", url, "T")
    }

    fn u(s: &str) -> Url {
        Url::parse(s).unwrap()
    }

    #[test]
    fn registrable_domain_respects_multipart_tlds() {
        assert_eq!(registrable_domain("m.youtube.com").as_deref(), Some("youtube.com"));
        assert_eq!(registrable_domain("foo.example.co.uk").as_deref(), Some("example.co.uk"));
        assert_eq!(registrable_domain("example.co.uk").as_deref(), Some("example.co.uk"));
    }

    #[test]
    fn subdomains_of_the_app_stay_in_window() {
        let app = app_for("https://www.youtube.com/");
        assert!(is_in_scope(&u("https://m.youtube.com/watch?v=1"), &app));
        assert!(is_in_scope(&u("https://youtube.com/feed"), &app));
    }

    #[test]
    fn unrelated_domains_go_to_the_browser() {
        let app = app_for("https://www.youtube.com/");
        assert!(!is_in_scope(&u("https://github.com/torvalds"), &app));
        assert!(!is_in_scope(&u("https://youtube.com.evil.test/"), &app));
    }

    #[test]
    fn different_sites_under_one_public_suffix_are_not_in_scope() {
        let app = app_for("https://example.co.uk/");
        assert!(!is_in_scope(&u("https://other.co.uk/"), &app));
    }

    #[test]
    fn captcha_frames_never_reach_the_browser() {
        let app = app_for("https://www.reddit.com/");
        let recaptcha = u("https://www.google.com/recaptcha/enterprise/anchor?ar=1&size=invisible");
        assert_eq!(decide(&recaptcha, &app), Decision::Allow);

        for url in [
            "https://challenges.cloudflare.com/turnstile/v0/api.js",
            "https://newassets.hcaptcha.com/captcha/v1/frame",
            "https://www.gstatic.com/recaptcha/releases/abc/recaptcha__en.js",
        ] {
            assert_eq!(decide(&u(url), &app), Decision::Allow, "for {url}");
        }

        assert_eq!(decide(&u("https://www.google.com/search?q=x"), &app), Decision::HandOff);
    }

    #[test]
    fn an_oauth_flow_stays_in_window_across_its_whole_conversation() {
        let entry = u("https://discord.com/oauth2/authorize?client_id=1&scope=identify");
        let domain = auth_domain_for(&entry).expect("oauth entry starts a flow");
        assert_eq!(domain, "discord.com");

        for step in [
            "https://discord.com/login?redirect_to=%2Foauth2%2Fauthorize",
            "https://discord.com/login/mfa",
            "https://discord.com/api/v9/auth/login",
        ] {
            assert!(continues_auth(&u(step), &domain), "flow broke at {step}");
        }

        assert!(!continues_auth(&u("https://github.com/"), &domain));
        assert!(!continues_auth(&u("https://evil.test/"), &domain));
    }

    #[test]
    fn a_non_auth_url_starts_no_session() {
        assert!(auth_domain_for(&u("https://github.com/torvalds/linux")).is_none());
        assert!(auth_domain_for(&u("https://news.example.com/")).is_none());
    }

    #[test]
    fn oauth_is_allowed_in_window() {
        let app = app_for("https://www.youtube.com/");
        assert!(is_in_scope(&u("https://accounts.google.com/signin/v2"), &app));
    }

    #[test]
    fn path_scoped_auth_entries_do_not_widen_the_whole_host() {
        let app = app_for("https://www.youtube.com/");
        assert!(is_in_scope(&u("https://github.com/login/oauth/authorize"), &app));
        assert!(!is_in_scope(&u("https://github.com/torvalds/linux"), &app));
    }

    #[test]
    fn extra_scope_widens_to_named_hosts() {
        let mut app = app_for("https://www.youtube.com/");
        app.extra_scope.push("youtu.be".into());
        assert!(is_in_scope(&u("https://youtu.be/abc123"), &app));
    }

    #[test]
    fn non_web_schemes_are_out_of_scope() {
        let app = app_for("https://www.youtube.com/");
        assert!(!is_in_scope(&u("mailto:someone@youtube.com"), &app));
    }

    #[test]
    fn webview_internal_schemes_are_allowed() {
        let app = app_for("https://www.youtube.com/");
        assert_eq!(decide(&u("about:blank"), &app), Decision::Allow);
        assert_eq!(decide(&u("about:srcdoc"), &app), Decision::Allow);
        assert_eq!(decide(&u("blob:https://www.youtube.com/abc"), &app), Decision::Allow);
    }

    #[test]
    fn dangerous_schemes_are_blocked_not_handed_to_the_desktop() {
        let app = app_for("https://www.youtube.com/");
        assert_eq!(decide(&u("file:///etc/passwd"), &app), Decision::Block);
        assert_eq!(decide(&u("javascript:alert(1)"), &app), Decision::Block);
    }

    #[test]
    fn renderable_data_urls_are_blocked_and_downloadable_ones_are_not() {
        let app = app_for("https://www.youtube.com/");

        for phishy in [
            "data:text/html,<h1>Sign in</h1>",
            "data:text/html;base64,PGgxPngK",
            "data:image/svg+xml,<svg onload=alert(1)>",
            "data:application/xhtml+xml,<html/>",
        ] {
            assert_eq!(decide(&u(phishy), &app), Decision::Block, "for {phishy}");
        }

        for download in [
            "data:application/octet-stream;base64,ZGF0YQ==",
            "data:text/csv;charset=utf-8,a%2Cb%0A1%2C2",
            "data:application/pdf;base64,JVBERi0=",
            "data:image/png;base64,iVBORw0KGgo=",
            "data:,hello",
        ] {
            assert_eq!(decide(&u(download), &app), Decision::Allow, "for {download}");
        }
    }

    #[test]
    fn in_scope_navigation_is_allowed_and_offsite_hands_off() {
        let app = app_for("https://www.youtube.com/");
        assert_eq!(decide(&u("https://m.youtube.com/watch"), &app), Decision::Allow);
        assert_eq!(decide(&u("https://github.com/x"), &app), Decision::HandOff);
    }

    #[test]
    fn a_file_link_is_recognised_but_an_ordinary_one_is_not() {
        for download in [
            "https://cdn.example.com/releases/app-1.3.1.zip",
            "https://files.test/setup.EXE",
            "https://docs.test/report.pdf",
            "https://x.test/a/b/archive.tar.gz",
        ] {
            assert!(looks_like_a_download(&u(download)), "for {download}");
        }

        for page in [
            "https://example.com/",
            "https://example.com/some/article",
            "https://example.com/index.html",
            "https://example.com/logo.png",
            "https://example.com/notes.txt",
            "https://example.com/download",
        ] {
            assert!(!looks_like_a_download(&u(page)), "for {page}");
        }
    }

    #[test]
    fn mailto_goes_to_the_desktop() {
        let app = app_for("https://www.youtube.com/");
        assert_eq!(decide(&u("mailto:hi@example.com"), &app), Decision::HandOff);
    }

    #[test]
    fn the_in_window_policy_keeps_offsite_links_inside() {
        let mut app = app_for("https://www.youtube.com/");
        app.offsite = OffsitePolicy::InWindow;
        assert_eq!(decide(&u("https://github.com/x"), &app), Decision::Allow);

        assert_eq!(decide(&u("file:///etc/passwd"), &app), Decision::Block);
    }
}

use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use url::Url;

pub const DESKTOP_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub const WEBVIEW_UA: &str = DESKTOP_UA;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconCandidate {
    pub url: String,

    pub size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteMeta {
    pub url: String,
    pub title: String,
    pub icons: Vec<IconCandidate>,
}

pub fn client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .user_agent(DESKTOP_UA)
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .context("building HTTP client")
}

pub fn normalize_url(input: &str) -> Result<Url> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("enter a URL"));
    }
    let candidate =
        if trimmed.contains("://") { trimmed.to_string() } else { format!("https://{trimmed}") };
    let url = Url::parse(&candidate).with_context(|| format!("{trimmed} is not a valid URL"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(anyhow!("only http and https URLs can be wrapped"));
    }
    if url.host_str().is_none() {
        return Err(anyhow!("{trimmed} has no host"));
    }
    Ok(url)
}

#[derive(Debug, Clone, Serialize)]
pub struct UserAgentPreset {
    pub label: &'static str,

    pub value: &'static str,
    pub hint: &'static str,
}

pub const CHROME_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";
pub const FIREFOX_UA: &str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0";
pub const SAFARI_UA: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.3 Safari/605.1.15";
pub const MOBILE_UA: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.0 Mobile/15E148 Safari/604.1";

pub const USER_AGENT_PRESETS: &[UserAgentPreset] = &[
    UserAgentPreset {
        label: "Default (WebKit)",
        value: "",
        hint: "Most honest, and what bot checks expect from this engine",
    },
    UserAgentPreset {
        label: "Chrome",
        value: CHROME_UA,
        hint: "For sites that reject WebKit, such as claude.ai",
    },
    UserAgentPreset { label: "Firefox", value: FIREFOX_UA, hint: "Another non-WebKit option" },
    UserAgentPreset { label: "Safari", value: SAFARI_UA, hint: "Presents as macOS Safari" },
    UserAgentPreset {
        label: "iPhone",
        value: MOBILE_UA,
        hint: "Serves the mobile layout, useful for narrow windows",
    },
];

const UA_OVERRIDES: &[(&str, &str)] = &[("claude.ai", CHROME_UA), ("anthropic.com", CHROME_UA)];

pub fn default_user_agent_for(url: &Url) -> Option<&'static str> {
    let host = url.host_str()?.to_ascii_lowercase();
    UA_OVERRIDES
        .iter()
        .find(|(domain, _)| host == *domain || host.ends_with(&format!(".{domain}")))
        .map(|(_, ua)| *ua)
}

const INTERSTITIAL_PREFIXES: &[&str] = &[
    "just a moment",
    "attention required",
    "checking your browser",
    "checking if the site connection is secure",
    "one moment, please",
    "are you a robot",
    "verifying you are human",
    "access denied",
    "access to this page has been denied",
];

const INTERSTITIAL_EXACT: &[&str] =
    &["error", "please wait", "security check", "403 forbidden", "404 not found", "not found"];

fn is_interstitial(title: &str) -> bool {
    let lower = title.trim().to_lowercase();
    let lower = lower.trim_end_matches(['.', '!', '…']).trim();
    INTERSTITIAL_EXACT.contains(&lower)
        || INTERSTITIAL_PREFIXES.iter().any(|bad| lower.starts_with(bad))
}

fn parse_sizes(attr: &str) -> u32 {
    attr.split_whitespace()
        .filter_map(|token| {
            let (w, h) = token.split_once(['x', 'X'])?;
            Some(w.parse::<u32>().ok()?.max(h.parse::<u32>().ok()?))
        })
        .max()
        .unwrap_or(0)
}

pub async fn fetch_meta(client: &reqwest::Client, url: &Url) -> Result<SiteMeta> {
    let response = match client.get(url.clone()).send().await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(url = %url, error = %e, "could not fetch page; using fallback metadata");
            return Ok(fallback_meta(url));
        }
    };

    let final_url = response.url().clone();
    let body = match response.text().await {
        Ok(b) => b,
        Err(e) => {
            tracing::warn!(url = %final_url, error = %e, "could not read page body");
            return Ok(fallback_meta(&final_url));
        }
    };

    let mut meta = parse_meta(&final_url, &body)?;

    if let Some(manifest_url) = meta_manifest_url(&final_url, &body) {
        if let Some((name, icons)) = fetch_manifest(client, &manifest_url).await {
            if let Some(name) = name.filter(|n| !is_interstitial(n)) {
                meta.title = name;
            }
            meta.icons.extend(icons);
        }
    }

    meta.url = settle_url(url, &final_url).to_string();

    meta.icons.push(IconCandidate {
        url: final_url.join("/favicon.ico").map(String::from).unwrap_or_default(),
        size: 0,
    });

    #[allow(clippy::unnecessary_sort_by)]
    meta.icons.sort_by(|a, b| b.size.cmp(&a.size));
    meta.icons.dedup_by(|a, b| a.url == b.url);
    meta.icons.retain(|c| !c.url.is_empty());

    Ok(meta)
}

fn settle_url(requested: &Url, final_url: &Url) -> Url {
    let user_asked_for_a_path = requested.path() != "/" || requested.query().is_some();
    if user_asked_for_a_path {
        let mut settled = final_url.clone();
        settled.set_path(requested.path());
        settled.set_query(requested.query());
        return settled;
    }

    let mut settled = final_url.clone();
    settled.set_path("/");
    settled.set_query(None);
    settled.set_fragment(None);
    settled
}

fn fallback_meta(url: &Url) -> SiteMeta {
    SiteMeta {
        url: url.to_string(),
        title: title_from_host(url),
        icons: vec![IconCandidate {
            url: url.join("/favicon.ico").map(String::from).unwrap_or_default(),
            size: 0,
        }],
    }
}

fn title_from_host(url: &Url) -> String {
    let host = url.host_str().unwrap_or("App");
    let host = host.strip_prefix("www.").unwrap_or(host);
    let stem = host.split('.').next().unwrap_or(host);
    let mut chars = stem.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => "App".to_string(),
    }
}

fn meta_manifest_url(base: &Url, body: &str) -> Option<Url> {
    let doc = Html::parse_document(body);
    let sel = Selector::parse("link[rel~=manifest]").ok()?;
    let href = doc.select(&sel).next()?.value().attr("href")?;
    base.join(href).ok()
}

async fn fetch_manifest(
    client: &reqwest::Client,
    manifest_url: &Url,
) -> Option<(Option<String>, Vec<IconCandidate>)> {
    let text = client.get(manifest_url.clone()).send().await.ok()?.text().await.ok()?;
    let json: serde_json::Value = serde_json::from_str(&text).ok()?;

    let name = json
        .get("short_name")
        .or_else(|| json.get("name"))
        .and_then(|v| v.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let icons = json
        .get("icons")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|icon| {
                    let src = icon.get("src")?.as_str()?;
                    let size =
                        icon.get("sizes").and_then(|v| v.as_str()).map(parse_sizes).unwrap_or(0);
                    Some(IconCandidate { url: manifest_url.join(src).ok()?.to_string(), size })
                })
                .collect()
        })
        .unwrap_or_default();

    Some((name, icons))
}

pub fn parse_meta(base: &Url, body: &str) -> Result<SiteMeta> {
    let doc = Html::parse_document(body);

    let attr_of = |selector: &str, attr: &str| -> Option<String> {
        let sel = Selector::parse(selector).ok()?;
        doc.select(&sel)
            .find_map(|el| el.value().attr(attr))
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
    };

    let title = attr_of(r#"meta[property="og:site_name"]"#, "content")
        .or_else(|| attr_of(r#"meta[name="application-name"]"#, "content"))
        .or_else(|| {
            let sel = Selector::parse("title").ok()?;
            doc.select(&sel)
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .filter(|t| !is_interstitial(t))
        .unwrap_or_else(|| title_from_host(base));

    let mut icons = Vec::new();

    let mut collect = |selector: &str, default_size: u32| {
        if let Ok(sel) = Selector::parse(selector) {
            for el in doc.select(&sel) {
                let Some(href) = el.value().attr("href") else { continue };
                let Ok(abs) = base.join(href) else { continue };
                let size = el
                    .value()
                    .attr("sizes")
                    .map(parse_sizes)
                    .filter(|s| *s > 0)
                    .unwrap_or(default_size);
                icons.push(IconCandidate { url: abs.to_string(), size });
            }
        }
    };

    collect("link[rel~=apple-touch-icon]", 180);
    collect("link[rel~=apple-touch-icon-precomposed]", 180);
    collect("link[rel~=icon]", 32);

    Ok(SiteMeta { url: base.to_string(), title, icons })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> Url {
        Url::parse("https://example.com/some/page").unwrap()
    }

    #[test]
    fn normalize_accepts_bare_hosts() {
        assert_eq!(normalize_url("youtube.com").unwrap().as_str(), "https://youtube.com/");
        assert_eq!(normalize_url("  claude.ai ").unwrap().host_str(), Some("claude.ai"));
    }

    #[test]
    fn normalize_rejects_non_web_schemes_and_junk() {
        assert!(normalize_url("file:///etc/passwd").is_err());
        assert!(normalize_url("").is_err());
    }

    #[test]
    fn sizes_attribute_parsing() {
        assert_eq!(parse_sizes("180x180"), 180);
        assert_eq!(parse_sizes("16x16 32x32 48x48"), 48);
        assert_eq!(parse_sizes("any"), 0);
    }

    #[test]
    fn og_site_name_wins_over_page_title() {
        let html = r#"<html><head>
            <title>Some Video - YouTube</title>
            <meta property="og:site_name" content="YouTube">
        </head></html>"#;
        assert_eq!(parse_meta(&base(), html).unwrap().title, "YouTube");
    }

    #[test]
    fn falls_back_to_title_then_host() {
        let with_title = r#"<html><head><title>Claude</title></head></html>"#;
        assert_eq!(parse_meta(&base(), with_title).unwrap().title, "Claude");

        let bare = "<html><head></head></html>";
        assert_eq!(parse_meta(&base(), bare).unwrap().title, "Example");
    }

    #[test]
    fn relative_icon_hrefs_resolve_against_the_page() {
        let html = r#"<html><head>
            <link rel="apple-touch-icon" href="/icons/touch.png">
            <link rel="icon" sizes="32x32" href="fav.png">
        </head></html>"#;
        let meta = parse_meta(&base(), html).unwrap();
        let urls: Vec<_> = meta.icons.iter().map(|i| i.url.as_str()).collect();
        assert!(urls.contains(&"https://example.com/icons/touch.png"));
        assert!(urls.contains(&"https://example.com/some/fav.png"));
    }

    #[test]
    fn sites_that_reject_webkit_get_a_working_user_agent() {
        let ua = |s: &str| default_user_agent_for(&Url::parse(s).unwrap());
        assert_eq!(ua("https://claude.ai/"), Some(CHROME_UA));
        assert_eq!(ua("https://www.claude.ai/x"), Some(CHROME_UA));
        assert_eq!(ua("https://youtube.com/"), None);

        assert_eq!(ua("https://claude.ai.evil.test/"), None);
    }

    #[test]
    fn a_cloudflare_challenge_does_not_become_the_app_name() {
        let html = r#"<html><head><title>Just a moment...</title></head></html>"#;
        assert_eq!(parse_meta(&base(), html).unwrap().title, "Example");

        for bad in ["Attention Required! | Cloudflare", "Access Denied", "404 Not Found"] {
            let page = format!("<html><head><title>{bad}</title></head></html>");
            assert_eq!(parse_meta(&base(), &page).unwrap().title, "Example", "for {bad}");
        }
    }

    #[test]
    fn a_legitimate_title_containing_error_is_kept() {
        let html = r#"<html><head><title>Error Prone - a blog</title></head></html>"#;
        assert_eq!(parse_meta(&base(), html).unwrap().title, "Error Prone - a blog");
    }

    #[test]
    fn a_login_redirect_is_not_baked_into_the_app_url() {
        let requested = Url::parse("https://claude.ai/").unwrap();
        let landed = Url::parse("https://claude.ai/login?returnTo=%2F").unwrap();
        assert_eq!(settle_url(&requested, &landed).as_str(), "https://claude.ai/");
    }

    #[test]
    fn a_host_change_from_a_redirect_is_kept() {
        let requested = Url::parse("https://youtube.com/").unwrap();
        let landed = Url::parse("https://www.youtube.com/").unwrap();
        assert_eq!(settle_url(&requested, &landed).as_str(), "https://www.youtube.com/");
    }

    #[test]
    fn a_deliberate_deep_link_survives() {
        let requested = Url::parse("https://youtube.com/feed/subscriptions").unwrap();
        let landed = Url::parse("https://www.youtube.com/").unwrap();
        assert_eq!(
            settle_url(&requested, &landed).as_str(),
            "https://www.youtube.com/feed/subscriptions"
        );
    }

    #[test]
    fn apple_touch_icon_outranks_a_small_favicon() {
        let html = r#"<html><head>
            <link rel="icon" sizes="16x16" href="/small.png">
            <link rel="apple-touch-icon" href="/big.png">
        </head></html>"#;
        let mut meta = parse_meta(&base(), html).unwrap();
        #[allow(clippy::unnecessary_sort_by)]
        meta.icons.sort_by(|a, b| b.size.cmp(&a.size));
        assert_eq!(meta.icons[0].url, "https://example.com/big.png");
    }
}

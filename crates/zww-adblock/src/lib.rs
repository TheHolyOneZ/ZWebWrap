pub mod scriptlets;
pub mod site_scripts;
pub mod sources;
pub mod store;

use adblock::lists::{FilterSet, ParseOptions, RuleTypes};
use adblock::resources::Resource;
use adblock::Engine;
use anyhow::{Context, Result};
use store::Assets;

pub use store::{Store, DEFAULT_MAX_AGE_SECS};

const MAX_RULES_PER_FILTER: usize = 40_000;

pub struct AdBlock {
    engine: Engine,

    content_blocker: Vec<String>,
    stats: Stats,
}

#[derive(Debug, Clone, Copy, Default, serde::Serialize)]
pub struct Stats {
    pub lists_loaded: usize,
    pub network_rules: usize,
    pub scriptlets: usize,
    pub seed_only: bool,
}

impl AdBlock {
    pub fn build(assets: &Assets) -> Result<Self> {
        let mut full = FilterSet::new(false);
        for text in &assets.lists {
            full.add_filter_list(text.clone(), ParseOptions::default());
        }
        let mut engine = Engine::new_with_filter_set(full);

        let mut resources: Vec<Resource> = Vec::new();

        if let Some(path) = &assets.scriptlets_path {
            resources.extend(load_scriptlets(path));
        }

        if let Some(json) = &assets.resources_json {
            match serde_json::from_str::<Vec<Resource>>(json) {
                Ok(extra) => resources.extend(extra),
                Err(e) => tracing::warn!(error = %e, "extra resources did not parse"),
            }
        }

        let scriptlets = resources.len();
        if scriptlets > 0 {
            engine.use_resources(resources);
        }

        let mut network_only = FilterSet::new(true);
        let opts = ParseOptions { rule_types: RuleTypes::NetworkOnly, ..Default::default() };
        for text in &assets.lists {
            network_only.add_filter_list(text.clone(), opts);
        }

        let (rules, failures) = network_only.into_content_blocking().map_err(|()| {
            anyhow::anyhow!("could not convert filters to content-blocking syntax")
        })?;
        if !failures.is_empty() {
            tracing::debug!(count = failures.len(), "filters with no content-blocking equivalent");
        }

        let network_rules = rules.len();
        let values = rules
            .iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()
            .context("serialising content-blocking rules")?;
        let content_blocker = chunk_content_blocking(values)?;

        tracing::info!(
            lists = assets.lists.len(),
            network_rules,
            filters = content_blocker.len(),
            scriptlets,
            seed_only = assets.is_seed_only,
            "ad block engine ready"
        );

        Ok(Self {
            engine,
            content_blocker,
            stats: Stats {
                lists_loaded: assets.lists.len(),
                network_rules,
                scriptlets,
                seed_only: assets.is_seed_only,
            },
        })
    }

    pub fn load_prepared(dir: &std::path::Path, assets: &Assets) -> Option<Self> {
        let stamp = std::fs::read_to_string(dir.join("engine.stamp")).ok()?;
        if stamp.trim() != lists_fingerprint(assets) {
            tracing::debug!("prepared engine is for different filter lists; rebuilding");
            return None;
        }

        let serialized = std::fs::read(dir.join("engine.bin")).ok()?;

        let mut content_blocker = Vec::new();
        for index in 0.. {
            match std::fs::read_to_string(dir.join(format!("content-blocking-{index}.json"))) {
                Ok(text) => content_blocker.push(text),
                Err(_) => break,
            }
        }
        if content_blocker.is_empty() {
            return None;
        }

        let mut engine = Engine::new_with_filter_set(FilterSet::new(false));
        if let Err(e) = engine.deserialize(&serialized) {
            tracing::warn!(error = ?e, "prepared engine did not load; rebuilding");
            return None;
        }

        let mut resources: Vec<Resource> = Vec::new();
        if let Some(path) = &assets.scriptlets_path {
            resources.extend(load_scriptlets(path));
        }
        if let Some(json) = &assets.resources_json {
            if let Ok(extra) = serde_json::from_str::<Vec<Resource>>(json) {
                resources.extend(extra);
            }
        }
        let scriptlets = resources.len();
        if scriptlets > 0 {
            engine.use_resources(resources);
        }

        let network_rules: usize =
            content_blocker.iter().map(|f| f.matches("\"trigger\"").count()).sum();
        tracing::info!(
            network_rules,
            filters = content_blocker.len(),
            scriptlets,
            "loaded prepared ad block engine"
        );

        Some(Self {
            engine,
            content_blocker,
            stats: Stats {
                lists_loaded: assets.lists.len(),
                network_rules,
                scriptlets,
                seed_only: assets.is_seed_only,
            },
        })
    }

    pub fn save_prepared(&self, dir: &std::path::Path, assets: &Assets) -> Result<()> {
        std::fs::create_dir_all(dir)?;
        std::fs::write(dir.join("engine.bin"), self.engine.serialize())?;

        for index in 0..64 {
            let stale = dir.join(format!("content-blocking-{index}.json"));
            if index >= self.content_blocker.len() && stale.exists() {
                let _ = std::fs::remove_file(stale);
            }
        }
        for (index, filter) in self.content_blocker.iter().enumerate() {
            std::fs::write(dir.join(format!("content-blocking-{index}.json")), filter)?;
        }

        std::fs::write(dir.join("engine.stamp"), lists_fingerprint(assets))?;
        Ok(())
    }

    pub fn stats(&self) -> Stats {
        self.stats
    }

    pub fn content_blocker_filters(&self) -> &[String] {
        &self.content_blocker
    }

    pub fn content_blocker_id(&self, index: usize) -> String {
        let mut hash: u64 = 0xcbf29ce484222325;
        for byte in self.content_blocker.get(index).map(|s| s.as_bytes()).unwrap_or_default() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        format!("zwebwrap-{index}-{hash:016x}")
    }

    pub fn would_block(&self, url: &str, source_url: &str, request_type: &str) -> bool {
        match adblock::request::Request::new(url, source_url, request_type, "GET") {
            Ok(request) => self.engine.check_network_request(&request).should_block(),
            Err(_) => false,
        }
    }

    pub fn generic_selectors(&self, tokens: &[String]) -> Vec<String> {
        let classes: Vec<&str> = tokens.iter().filter_map(|t| t.strip_prefix('.')).collect();
        let ids: Vec<&str> = tokens.iter().filter_map(|t| t.strip_prefix('#')).collect();
        if classes.is_empty() && ids.is_empty() {
            return Vec::new();
        }
        self.engine.hidden_class_id_selectors(classes, ids, &Default::default())
    }

    pub fn injection_script(&self, url: &str) -> String {
        let resources = self.engine.url_cosmetic_resources(url);

        let mut selectors: Vec<&str> =
            resources.hide_selectors.iter().map(|s| s.as_str()).collect();

        selectors.sort_unstable();

        let mut script = build_injection(&selectors, &resources.injected_script);

        if let Some(site) = site_scripts::for_url(url) {
            script.insert_str(0, site);
        }
        script
    }
}

fn load_scriptlets(path: &std::path::Path) -> Vec<Resource> {
    let Ok(source) = std::fs::read_to_string(path) else {
        tracing::warn!(path = %path.display(), "scriptlet library unreadable");
        return Vec::new();
    };

    let parsed = scriptlets::parse(&source);
    if !scriptlets::looks_complete(&parsed) {
        tracing::warn!(
            count = parsed.len(),
            "scriptlet library parsed incompletely; uBO may have changed its format"
        );
    } else {
        tracing::debug!(count = parsed.len(), "loaded uBlock Origin scriptlets");
    }
    parsed
}

fn build_injection(selectors: &[&str], scriptlets: &str) -> String {
    let mut out = String::new();
    out.push_str("(function(){\n'use strict';\nif(window.__zwwBlock)return;window.__zwwBlock=1;\n");

    if !selectors.is_empty() {
        let css = format!("{}{{display:none!important}}", selectors.join(","));

        out.push_str("try{var s=document.createElement('style');");
        out.push_str("s.setAttribute('data-zww','cosmetic');s.textContent=");
        out.push_str(&json_string(&css));
        out.push_str(";(document.head||document.documentElement).appendChild(s);}catch(e){}\n");
    }

    if !scriptlets.trim().is_empty() {
        out.push_str("try{\n");
        out.push_str(scriptlets);
        out.push_str("\n}catch(e){}\n");
    }

    out.push_str("})();\n");
    out
}

const NEVER_BLOCK_HOSTS: &[&str] = &[
    "challenges.cloudflare.com",
    "hcaptcha.com",
    "newassets.hcaptcha.com",
    "recaptcha.net",
    "www.google.com/recaptcha",
    "www.gstatic.com/recaptcha",
    "js.arkoselabs.com",
    "client-api.arkoselabs.com",
];

pub fn is_never_blocked(url: &str) -> bool {
    let Ok(parsed) = url::Url::parse(url) else { return false };
    let Some(host) = parsed.host_str() else { return false };
    let host = host.trim_start_matches("www.").to_ascii_lowercase();

    NEVER_BLOCK_HOSTS.iter().any(|entry| match entry.split_once('/') {
        Some((entry_host, path)) => {
            let entry_host = entry_host.trim_start_matches("www.");
            host == entry_host && parsed.path().trim_start_matches('/').starts_with(path)
        }
        None => host == *entry || host.ends_with(&format!(".{entry}")),
    })
}

fn never_block_rules() -> Vec<serde_json::Value> {
    NEVER_BLOCK_HOSTS
        .iter()
        .map(|host| {
            let escaped = host.replace('.', "\\.");
            serde_json::json!({
                "action": { "type": "ignore-previous-rules" },
                "trigger": { "url-filter": format!("^[^:]+:(//)?([^/]+\\.)?{escaped}") }
            })
        })
        .collect()
}

fn is_ignore_rule(value: &serde_json::Value) -> bool {
    value.get("action").and_then(|a| a.get("type")).and_then(|t| t.as_str())
        == Some("ignore-previous-rules")
}

fn chunk_content_blocking(values: Vec<serde_json::Value>) -> Result<Vec<String>> {
    let (mut ignores, blocks): (Vec<_>, Vec<_>) = values.into_iter().partition(is_ignore_rule);
    ignores.extend(never_block_rules());

    if blocks.len() + ignores.len() <= MAX_RULES_PER_FILTER {
        let mut all = blocks;
        all.extend(ignores);
        return Ok(vec![serde_json::to_string(&all)?]);
    }

    let per_chunk = MAX_RULES_PER_FILTER.saturating_sub(ignores.len()).max(1);

    let mut chunks = Vec::new();
    for group in blocks.chunks(per_chunk) {
        let mut list: Vec<&serde_json::Value> = group.iter().collect();
        list.extend(ignores.iter());
        chunks.push(serde_json::to_string(&list)?);
    }

    if chunks.is_empty() && !ignores.is_empty() {
        chunks.push(serde_json::to_string(&ignores)?);
    }
    Ok(chunks)
}

fn lists_fingerprint(assets: &Assets) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let mut feed = |bytes: &[u8]| {
        for byte in bytes {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x100000001b3);
        }
    };
    for list in &assets.lists {
        feed(list.as_bytes());
    }
    if let Some(json) = &assets.resources_json {
        feed(json.as_bytes());
    }
    format!("{hash:016x}")
}

fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine_from(list: &str) -> AdBlock {
        AdBlock::build(&Assets {
            lists: vec![list.to_string()],
            resources_json: None,
            scriptlets_path: None,
            is_seed_only: false,
        })
        .unwrap()
    }

    #[test]
    fn generic_rules_are_returned_for_tokens_a_page_reports() {
        let ab = engine_from("##.ad-banner\n##.sponsored-post\n###leaderboard-ad\n");

        let hidden = ab.generic_selectors(&[
            ".ad-banner".into(),
            ".article-body".into(),
            "#leaderboard-ad".into(),
        ]);

        assert!(hidden.contains(&".ad-banner".to_string()), "got {hidden:?}");
        assert!(hidden.contains(&"#leaderboard-ad".to_string()), "got {hidden:?}");
        assert!(
            !hidden.iter().any(|s| s.contains("article-body")),
            "ordinary page content must never be hidden: {hidden:?}"
        );
    }

    #[test]
    fn a_page_that_reports_nothing_gets_nothing_back() {
        let ab = engine_from("##.ad-banner\n");
        assert!(ab.generic_selectors(&[]).is_empty());

        assert!(ab.generic_selectors(&[".main-content".into(), "#header".into()]).is_empty());
    }

    #[test]
    fn the_seed_list_is_network_rules_only() {
        let ab = engine_from(sources::SEED_LIST);
        let probes: Vec<String> =
            ["#ad-banner", ".ad-banner", ".advertisement", ".sponsored", "#AD_300"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        assert!(ab.generic_selectors(&probes).is_empty());

        assert!(ab.would_block(
            "https://doubleclick.net/ad.js",
            "https://news.example.com/",
            "script"
        ));
    }

    #[tokio::test]
    #[ignore = "requires network"]
    async fn the_real_lists_hide_the_commonest_ad_slots() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        let client = reqwest::Client::builder()
            .user_agent("zwebwrap")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();

        let assets = store.refresh(&client).await.unwrap();
        assert!(!assets.is_seed_only, "expected real lists to download");
        let ab = AdBlock::build(&assets).unwrap();

        let probes: Vec<String> =
            ["#AD_300", "#ad-banner", ".ad-banner", ".advertisement", "#leaderboard-ad"]
                .iter()
                .map(|s| s.to_string())
                .collect();
        let hidden = ab.generic_selectors(&probes);
        assert!(!hidden.is_empty(), "expected generic hide rules, got none");
        println!("{} of {} probes hidden: {hidden:?}", hidden.len(), probes.len());
    }

    #[test]
    fn builds_from_the_seed_list_and_blocks_a_known_ad_host() {
        let ab = engine_from(sources::SEED_LIST);
        assert!(ab.would_block(
            "https://doubleclick.net/ad.js",
            "https://news.example.com/",
            "script"
        ));
        assert!(!ab.would_block(
            "https://news.example.com/app.js",
            "https://news.example.com/",
            "script"
        ));
    }

    #[test]
    fn produces_content_blocking_rules() {
        let ab = engine_from(sources::SEED_LIST);
        let filters = ab.content_blocker_filters();
        assert_eq!(filters.len(), 1, "a small list needs only one filter");
        let json = &filters[0];
        assert!(json.starts_with('['), "expected a JSON array, got {:.40}", json);

        let parsed: Vec<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(!parsed.is_empty());
        assert!(parsed[0].get("trigger").is_some());
        assert!(parsed[0].get("action").is_some());
        assert!(ab.stats().network_rules > 0);
    }

    #[test]
    fn captcha_infrastructure_is_never_blocked() {
        let ab = engine_from("||challenges.cloudflare.com^\n||hcaptcha.com^\n");
        let rules: Vec<serde_json::Value> =
            serde_json::from_str(&ab.content_blocker_filters()[0]).unwrap();

        for host in ["challenges.cloudflare.com", "hcaptcha.com"] {
            let needle = host.replace('.', "\\.");
            let last_match = rules
                .iter()
                .rfind(|r| r["trigger"]["url-filter"].as_str().is_some_and(|f| f.contains(&needle)))
                .unwrap_or_else(|| panic!("no rule mentions {host}"));

            assert_eq!(
                last_match["action"]["type"], "ignore-previous-rules",
                "{host} must end up allowed, got {last_match}"
            );
        }
    }

    #[test]
    fn the_never_block_check_matches_what_the_exception_rules_do() {
        assert!(is_never_blocked("https://challenges.cloudflare.com/turnstile/v0/api.js"));
        assert!(is_never_blocked("https://hcaptcha.com/1/api.js"));

        assert!(is_never_blocked("https://newassets.hcaptcha.com/captcha/v1/x/static.js"));
        assert!(is_never_blocked("https://api.hcaptcha.com/getcaptcha"));
        assert!(is_never_blocked("https://www.hcaptcha.com/"));
        assert!(is_never_blocked("https://client-api.arkoselabs.com/v2/x/api.js"));

        assert!(is_never_blocked("https://www.google.com/recaptcha/api.js"));
        assert!(is_never_blocked("https://www.gstatic.com/recaptcha/releases/x.js"));
        assert!(!is_never_blocked("https://www.google.com/ads/pixel.gif"));
        assert!(!is_never_blocked("https://www.gstatic.com/ads/banner.png"));

        assert!(!is_never_blocked("https://doubleclick.net/ad.js"));
        assert!(!is_never_blocked("https://nothcaptcha.com/x.js"));
        assert!(!is_never_blocked("not a url"));
    }

    #[test]
    fn every_never_block_host_is_actually_matched_by_the_check() {
        for entry in NEVER_BLOCK_HOSTS {
            let url = format!("https://{entry}");
            assert!(is_never_blocked(&url), "{entry} is in the list but reads as blockable");
        }
    }

    #[test]
    fn every_filter_carries_the_never_block_exceptions() {
        let values: Vec<serde_json::Value> = (0..MAX_RULES_PER_FILTER + 10)
            .map(|i| {
                serde_json::json!({
                    "action": { "type": "block" },
                    "trigger": { "url-filter": format!("^ads{i}\\.example") }
                })
            })
            .collect();

        let chunks = chunk_content_blocking(values).unwrap();
        assert!(chunks.len() > 1, "expected the ruleset to be split");
        for (i, chunk) in chunks.iter().enumerate() {
            assert!(
                chunk.contains("challenges\\\\.cloudflare\\\\.com")
                    || chunk.contains("challenges\\.cloudflare\\.com"),
                "filter {i} is missing the never-block exceptions"
            );
        }
    }

    #[test]
    fn ruleset_id_changes_when_the_rules_change() {
        let a = engine_from(sources::SEED_LIST);
        let b = engine_from("||only-one-rule.example^");
        assert_ne!(a.content_blocker_id(0), b.content_blocker_id(0));

        assert_eq!(a.content_blocker_id(0), engine_from(sources::SEED_LIST).content_blocker_id(0));
    }

    #[test]
    fn cosmetic_rules_become_a_stylesheet() {
        let ab = engine_from("example.com##.ad-banner\nexample.com##.sponsored\n");
        let script = ab.injection_script("https://example.com/page");
        assert!(script.contains(".ad-banner"));
        assert!(script.contains(".sponsored"));
        assert!(script.contains("display:none!important"));
        assert!(script.contains("createElement('style')"));
        assert!(
            script.contains("data-zww"),
            "our sheets must be findable again, or nothing can count what they hide"
        );
    }

    #[test]
    fn cosmetic_rules_do_not_leak_across_sites() {
        let ab = engine_from("example.com##.ad-banner\n");
        assert!(!ab.injection_script("https://other.test/page").contains(".ad-banner"));
    }

    #[test]
    fn injection_is_idempotent_and_self_contained() {
        let script = build_injection(&[".ad"], "");
        assert!(script.contains("if(window.__zwwBlock)return"));
        assert!(script.starts_with("(function(){"));
        assert!(script.trim_end().ends_with("})();"));
    }

    #[test]
    fn a_selector_containing_a_quote_cannot_break_out_of_the_literal() {
        let script = build_injection(&["a[title=\"</style><script>x\"]"], "");

        assert!(!script.contains("</style><script>x\"]{display"));
        assert!(script.contains("\\\""));
    }

    #[test]
    fn scriptlet_failures_cannot_take_out_the_stylesheet() {
        let script = build_injection(&[".ad"], "throw new Error('boom')");
        assert!(script.contains("try{"));
        assert!(script.contains("catch(e){}"));

        let style_at = script.find("createElement('style')").unwrap();
        let scriptlet_at = script.find("throw new Error").unwrap();
        assert!(style_at < scriptlet_at);
    }

    #[tokio::test]
    #[ignore = "requires network"]
    async fn builds_from_the_real_filter_lists() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path()).unwrap();
        let client = reqwest::Client::builder()
            .user_agent("zwebwrap")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();

        let assets = store.refresh(&client).await.unwrap();
        assert!(!assets.is_seed_only, "expected real lists to download");

        let ab = AdBlock::build(&assets).unwrap();
        println!("stats: {:?}", ab.stats());
        let filters = ab.content_blocker_filters();
        println!(
            "content-blocker: {} filters, {} bytes total",
            filters.len(),
            filters.iter().map(|f| f.len()).sum::<usize>()
        );

        for (i, f) in filters.iter().enumerate() {
            let count = serde_json::from_str::<Vec<serde_json::Value>>(f).unwrap().len();
            println!("  filter {i}: {count} rules");
            assert!(count <= MAX_RULES_PER_FILTER, "filter {i} has {count} rules");
        }
        assert!(filters.len() > 1, "the real lists should need several filters");

        assert!(ab.stats().network_rules > 5_000, "got {}", ab.stats().network_rules);

        assert!(ab.stats().scriptlets > 0, "no scriptlet resources loaded at all");
        assert!(ab.would_block(
            "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js",
            "https://news.example.com/",
            "script"
        ));

        let youtube = ab.injection_script("https://www.youtube.com/watch?v=jNQXAC9IVRw");
        println!("youtube injection: {} bytes", youtube.len());
        assert!(youtube.len() > 200, "expected YouTube-specific rules");

        if youtube.contains("adPlacements") {
            println!("json-prune resolved: YouTube ad slots are pruned");
        } else {
            println!("KNOWN GAP: scriptlets loaded but no injection emitted");
        }

        let prep = dir.path().join("prepared");
        let t0 = std::time::Instant::now();
        ab.save_prepared(&prep, &assets).unwrap();
        println!("save_prepared: {:?}", t0.elapsed());

        let t1 = std::time::Instant::now();
        let loaded = AdBlock::load_prepared(&prep, &assets).expect("prepared engine should load");
        println!("load_prepared: {:?}", t1.elapsed());

        assert!(loaded.would_block(
            "https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js",
            "https://news.example.com/",
            "script"
        ));
        assert_eq!(loaded.content_blocker_id(0), ab.content_blocker_id(0));
        assert_eq!(loaded.content_blocker_filters().len(), ab.content_blocker_filters().len());
        assert_eq!(
            loaded.injection_script("https://www.youtube.com/watch?v=jNQXAC9IVRw").len(),
            youtube.len()
        );
    }

    #[test]
    fn a_prepared_engine_is_rejected_when_the_lists_change() {
        let dir = tempfile::tempdir().unwrap();
        let assets = Assets {
            lists: vec![sources::SEED_LIST.to_string()],
            resources_json: None,
            scriptlets_path: None,
            is_seed_only: false,
        };
        AdBlock::build(&assets).unwrap().save_prepared(dir.path(), &assets).unwrap();
        assert!(AdBlock::load_prepared(dir.path(), &assets).is_some());

        let changed = Assets {
            lists: vec!["||something-else.example^".to_string()],
            resources_json: None,
            scriptlets_path: None,
            is_seed_only: false,
        };
        assert!(
            AdBlock::load_prepared(dir.path(), &changed).is_none(),
            "a stale engine must not be used after the lists change"
        );
    }

    #[test]
    fn an_empty_ruleset_still_produces_valid_script() {
        let script = build_injection(&[], "");
        assert!(script.starts_with("(function(){"));
        assert!(!script.contains("createElement('style')"));
    }
}

use adblock::resources::{MimeType, Resource, ResourceType};

pub const CRITICAL: &[&str] = &["json-prune.js", "set-constant.js"];

pub fn parse(source: &str) -> Vec<Resource> {
    let mut entries: Vec<Entry> = find_entries(source);
    entries.extend(find_registered(source));

    let by_function: std::collections::HashMap<&str, &str> =
        entries.iter().map(|e| (e.function.as_str(), e.name.as_str())).collect();

    let dependency_names: Vec<Vec<String>> = entries
        .iter()
        .map(|entry| {
            entry
                .dependencies
                .iter()
                .map(|dep| {
                    by_function
                        .get(dep.as_str())
                        .map(|n| n.to_string())
                        .unwrap_or_else(|| dep.clone())
                })
                .collect()
        })
        .collect();

    for (entry, dependencies) in entries.iter_mut().zip(dependency_names) {
        entry.dependencies = dependencies;
    }

    let mut claimed: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut resources = Vec::new();

    for entry in entries {
        let Some(body) = function_source(source, &entry.function) else {
            tracing::debug!(name = %entry.name, function = %entry.function, "no function body found");
            continue;
        };

        let mut aliases = entry.aliases;
        for name in
            std::iter::once(entry.name.as_str()).chain(aliases.clone().iter().map(|s| s.as_str()))
        {
            if let Some(stem) = name.strip_suffix(".js") {
                if !stem.is_empty() && !aliases.iter().any(|a| a == stem) {
                    aliases.push(stem.to_string());
                }
            }
        }

        if claimed.contains(&entry.name) {
            continue;
        }

        aliases.retain(|alias| !claimed.contains(alias) && *alias != entry.name);
        aliases.dedup();

        claimed.insert(entry.name.clone());
        claimed.extend(aliases.iter().cloned());

        let kind = if entry.name.ends_with(".fn") {
            MimeType::FnJavascript
        } else {
            MimeType::ApplicationJavascript
        };

        resources.push(Resource {
            name: entry.name,
            aliases,
            kind: ResourceType::Mime(kind),
            content: base64_encode(body.as_bytes()),
            dependencies: entry.dependencies,
            permission: Default::default(),
        });
    }

    resources
}

pub fn looks_complete(resources: &[Resource]) -> bool {
    if resources.len() < 100 {
        return false;
    }
    CRITICAL.iter().all(|needed| resources.iter().any(|r| r.name == *needed))
}

struct Entry {
    name: String,
    aliases: Vec<String>,
    function: String,
    dependencies: Vec<String>,
}

fn find_entries(source: &str) -> Vec<Entry> {
    const MARKER: &str = "builtinScriptlets.push({";
    let mut entries = Vec::new();
    let mut cursor = 0;

    while let Some(found) = source[cursor..].find(MARKER) {
        let start = cursor + found + MARKER.len() - 1;
        let Some(end) = matching_brace(source, start) else { break };
        let block = &source[start..=end];
        cursor = end;

        let (Some(name), Some(function)) = (field(block, "name"), field(block, "fn")) else {
            continue;
        };

        entries.push(Entry {
            name,
            aliases: list_field(block, "aliases"),
            function,
            dependencies: list_field(block, "dependencies"),
        });
    }

    entries
}

fn find_registered(source: &str) -> Vec<Entry> {
    const MARKER: &str = "registerScriptlet(";
    let mut entries = Vec::new();
    let mut cursor = 0;

    while let Some(found) = source[cursor..].find(MARKER) {
        let at = cursor + found + MARKER.len();
        cursor = at;

        let function: String = source[at..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$')
            .collect();
        if function.is_empty() {
            continue;
        }

        let Some(brace) = source[at..].find('{').map(|i| at + i) else { continue };
        let Some(end) = matching_brace(source, brace) else { continue };
        let block = &source[brace..=end];
        cursor = end;

        let Some(name) = field(block, "name") else { continue };

        entries.push(Entry {
            name,
            aliases: list_field(block, "aliases"),
            function,

            dependencies: identifier_list(block, "dependencies"),
        });
    }

    entries
}

fn identifier_list(block: &str, key: &str) -> Vec<String> {
    let Some(at) = find_key(block, key) else { return Vec::new() };
    let rest = block[at..].trim_start();
    if !rest.starts_with('[') {
        return Vec::new();
    }
    let Some(close) = rest.find(']') else { return Vec::new() };

    rest[1..close]
        .split(',')
        .map(|token| token.trim().trim_matches(['\'', '"', '`']).to_string())
        .filter(|token| {
            !token.is_empty()
                && token
                    .chars()
                    .all(|c| c.is_alphanumeric() || c == '_' || c == '$' || c == '.' || c == '-')
        })
        .collect()
}

fn field(block: &str, key: &str) -> Option<String> {
    let at = find_key(block, key)?;
    let rest = block[at..].trim_start();
    let value: String = match rest.chars().next()? {
        q @ ('\'' | '"' | '`') => rest[1..].chars().take_while(|c| *c != q).collect(),

        _ => rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect(),
    };
    (!value.is_empty()).then_some(value)
}

fn list_field(block: &str, key: &str) -> Vec<String> {
    let Some(at) = find_key(block, key) else { return Vec::new() };
    let rest = block[at..].trim_start();
    if !rest.starts_with('[') {
        return Vec::new();
    }
    let Some(close) = rest.find(']') else { return Vec::new() };

    let mut out = Vec::new();
    let mut chars = rest[1..close].chars().peekable();
    while let Some(c) = chars.next() {
        if !matches!(c, '\'' | '"' | '`') {
            continue;
        }
        let value: String = chars.by_ref().take_while(|ch| *ch != c).collect();
        if !value.is_empty() {
            out.push(value);
        }
    }
    out
}

fn find_key(block: &str, key: &str) -> Option<usize> {
    let mut cursor = 0;
    while let Some(found) = block[cursor..].find(key) {
        let at = cursor + found;
        cursor = at + key.len();

        let before_ok = block[..at]
            .chars()
            .next_back()
            .is_none_or(|c| !c.is_alphanumeric() && c != '_' && c != '$');
        let after = block[cursor..].trim_start();
        if before_ok && after.starts_with(':') {
            return Some(cursor + (block[cursor..].len() - after.len()) + 1);
        }
    }
    None
}

fn function_source(source: &str, name: &str) -> Option<String> {
    let mut cursor = 0;
    while let Some(found) = source[cursor..].find("function") {
        let at = cursor + found;
        cursor = at + "function".len();

        let after = source[cursor..].trim_start();
        if !after.starts_with(name) {
            continue;
        }

        let next = after[name.len()..].trim_start();
        if !next.starts_with('(') {
            continue;
        }

        let open = source[at..].find('{').map(|i| at + i)?;
        let close = matching_brace(source, open)?;
        return Some(source[at..=close].to_string());
    }
    None
}

fn matching_brace(source: &str, open: usize) -> Option<usize> {
    let bytes = source.as_bytes();
    let mut depth = 0i32;
    let mut i = open;

    while i < bytes.len() {
        match bytes[i] {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            q @ (b'\'' | b'"' | b'`') => {
                i += 1;
                while i < bytes.len() && bytes[i] != q {
                    if bytes[i] == b'\\' {
                        i += 1;
                    }
                    i += 1;
                }
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'/' => {
                while i < bytes.len() && bytes[i] != b'\n' {
                    i += 1;
                }
            }
            b'/' if i + 1 < bytes.len() && bytes[i + 1] == b'*' => {
                i += 2;
                while i + 1 < bytes.len() && !(bytes[i] == b'*' && bytes[i + 1] == b'/') {
                    i += 1;
                }
                i += 1;
            }
            _ => {}
        }
        i += 1;
    }
    None
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);

    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 { TABLE[(n >> 6) as usize & 63] as char } else { '=' });
        out.push(if chunk.len() > 2 { TABLE[n as usize & 63] as char } else { '=' });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
export const builtinScriptlets = registeredScriptlets;

builtinScriptlets.push({
    name: 'safe-self.fn',
    fn: safeSelf,
});
function safeSelf() {
    const s = "a } brace in a string";
    return s;
}

builtinScriptlets.push({
    name: 'json-prune.js',
    aliases: [
        'jsonp',
    ],
    fn: jsonPrune,
    dependencies: [
        'object-prune.fn',
        'safe-self.fn',
    ],
});
function jsonPrune(
    rawPrunePaths = '',
    rawNeedlePaths = ''
) {
    // a comment with { an unbalanced brace
    if ( rawPrunePaths ) { return 1; }
    return 0;
}

builtinScriptlets.push({
    name: 'json-prune-fetch-response.js',
    fn: jsonPruneFetchResponse,
});
function jsonPruneFetchResponse() { return 2; }
"#;

    #[test]
    fn scriptlets_are_invokable_and_helpers_are_not() {
        let parsed = parse(SAMPLE);

        let scriptlet = parsed
            .iter()
            .find(|r| r.name == "json-prune.js")
            .expect("the sample defines json-prune.js");
        assert_eq!(
            scriptlet.kind,
            ResourceType::Mime(MimeType::ApplicationJavascript),
            "a rule names this one directly, so the engine must be willing to invoke it"
        );

        let helper = parsed
            .iter()
            .find(|r| r.name.ends_with(".fn"))
            .expect("the sample defines at least one .fn helper");
        assert_eq!(
            helper.kind,
            ResourceType::Mime(MimeType::FnJavascript),
            "helpers stay helpers; only their dependents are invoked"
        );
    }

    #[test]
    fn a_rule_that_names_a_scriptlet_actually_injects_it() {
        use adblock::lists::ParseOptions;
        use adblock::{Engine, FilterSet};

        let mut set = FilterSet::new(false);

        set.add_filter_list(
            "example.com##+js(json-prune-fetch-response)\n".to_string(),
            ParseOptions::default(),
        );
        let mut engine = Engine::new_with_filter_set(set);
        engine.use_resources(parse(SAMPLE));

        let injected = engine.url_cosmetic_resources("https://example.com/").injected_script;
        assert!(
            injected.contains("jsonPruneFetchResponse"),
            "the rule resolved to nothing; injected script was {injected:?}"
        );
    }

    #[test]
    fn parses_name_aliases_and_dependencies() {
        let resources = parse(SAMPLE);
        let prune = resources.iter().find(|r| r.name == "json-prune.js").expect("json-prune");

        assert_eq!(prune.aliases, vec!["jsonp", "json-prune"]);
        assert_eq!(prune.dependencies, vec!["object-prune.fn", "safe-self.fn"]);

        assert!(matches!(prune.kind, ResourceType::Mime(MimeType::ApplicationJavascript)));
    }

    #[test]
    fn function_body_is_captured_whole() {
        let resources = parse(SAMPLE);
        let prune = resources.iter().find(|r| r.name == "json-prune.js").unwrap();
        let decoded = decode(&prune.content);

        assert!(decoded.starts_with("function jsonPrune("));
        assert!(decoded.trim_end().ends_with('}'));
        assert!(decoded.contains("return 0;"));

        assert!(decoded.contains("an unbalanced brace"));
    }

    #[test]
    fn a_brace_inside_a_string_does_not_end_the_function() {
        let resources = parse(SAMPLE);
        let safe = resources.iter().find(|r| r.name == "safe-self.fn").unwrap();
        let decoded = decode(&safe.content);
        assert!(decoded.contains("a } brace in a string"));
        assert!(decoded.contains("return s;"));
    }

    #[test]
    fn a_longer_name_is_not_mistaken_for_a_shorter_one() {
        let resources = parse(SAMPLE);
        let prune = resources.iter().find(|r| r.name == "json-prune.js").unwrap();
        assert!(!decode(&prune.content).contains("return 2;"));

        let fetch = resources.iter().find(|r| r.name == "json-prune-fetch-response.js").unwrap();
        assert!(decode(&fetch.content).contains("return 2;"));
    }

    #[test]
    fn entries_without_a_function_are_skipped_not_fatal() {
        let broken = "builtinScriptlets.push({ name: 'ghost.js', fn: missingFunction });";
        assert!(parse(broken).is_empty());
    }

    #[test]
    fn completeness_check_rejects_a_thin_parse() {
        assert!(!looks_complete(&parse(SAMPLE)), "three scriptlets is not a real library");
        assert!(!looks_complete(&[]));
    }

    #[tokio::test]
    #[ignore = "diagnostic"]
    async fn report_unresolved_dependencies() {
        let dir = tempfile::tempdir().unwrap();
        let store = crate::Store::at(dir.path()).unwrap();
        let client = reqwest::Client::builder()
            .user_agent("zwebwrap")
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .unwrap();
        let assets = store.refresh(&client).await.unwrap();
        let src = std::fs::read_to_string(assets.scriptlets_path.as_ref().unwrap()).unwrap();
        let parsed = parse(&src);
        let names: std::collections::HashSet<&str> =
            parsed.iter().map(|r| r.name.as_str()).collect();

        println!("parsed {} resources", parsed.len());
        let mut missing = std::collections::BTreeSet::new();
        for r in &parsed {
            for dep in &r.dependencies {
                if !names.contains(dep.as_str()) {
                    missing.insert(format!("{} -> {}", r.name, dep));
                }
            }
        }
        println!("unresolved dependencies: {}", missing.len());
        for m in missing.iter().take(20) {
            println!("  {m}");
        }
        let jp = parsed.iter().find(|r| r.name == "json-prune.js");
        println!("json-prune deps: {:?}", jp.map(|r| &r.dependencies));
    }

    #[tokio::test]
    #[ignore = "diagnostic"]
    async fn scriptlet_vs_cosmetic_matching() {
        use adblock::lists::{FilterSet, ParseOptions};
        use adblock::Engine;

        let dir = tempfile::tempdir().unwrap();
        let store = crate::Store::at(dir.path()).unwrap();
        let client = reqwest::Client::builder().user_agent("zwebwrap").build().unwrap();
        let assets = store.refresh(&client).await.unwrap();
        let src = std::fs::read_to_string(assets.scriptlets_path.as_ref().unwrap()).unwrap();
        let resources = parse(&src);

        let mut fs = FilterSet::new(true);
        fs.add_filter_list(
            "example.com##.plain-hide\nexample.com##+js(json-prune, adPlacements)\n".to_string(),
            ParseOptions::default(),
        );
        let mut engine = Engine::new_with_filter_set(fs);
        engine.use_resources(resources);

        let r = engine.url_cosmetic_resources("https://example.com/x");
        println!("hide_selectors: {:?}", r.hide_selectors);
        println!("injected_script len: {}", r.injected_script.len());
        println!("injected head: {:.200}", r.injected_script);
    }

    #[test]
    fn base64_matches_the_standard_alphabet_and_padding() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foob"), "Zm9vYg==");
        assert_eq!(base64_encode(b"function x(){}"), "ZnVuY3Rpb24geCgpe30=");
    }

    fn decode(b64: &str) -> String {
        use base64::Engine as _;
        String::from_utf8(base64::engine::general_purpose::STANDARD.decode(b64).unwrap()).unwrap()
    }
}

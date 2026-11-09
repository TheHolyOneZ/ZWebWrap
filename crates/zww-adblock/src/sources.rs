pub struct Source {
    pub name: &'static str,
    pub url: &'static str,

    pub description: &'static str,
}

pub const DEFAULT_SOURCES: &[Source] = &[
    Source {
        name: "easylist.txt",
        url: "https://easylist.to/easylist/easylist.txt",
        description: "EasyList - the standard ad blocking list",
    },
    Source {
        name: "easyprivacy.txt",
        url: "https://easylist.to/easylist/easyprivacy.txt",
        description: "EasyPrivacy - trackers and analytics",
    },
    Source {
        name: "ubo-filters.txt",
        url: "https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/filters.txt",
        description: "uBlock Origin - site-specific rules and scriptlets",
    },
    Source {
        name: "ubo-privacy.txt",
        url: "https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/privacy.txt",
        description: "uBlock Origin - privacy",
    },
    Source {
        name: "ubo-quick-fixes.txt",
        url:
            "https://raw.githubusercontent.com/uBlockOrigin/uAssets/master/filters/quick-fixes.txt",
        description: "uBlock Origin - quick fixes for breakage",
    },
];

pub const SCRIPTLETS_DIR_API: &str =
    "https://api.github.com/repos/gorhill/uBlock/contents/src/js/resources";

pub const SCRIPTLET_SUPPORT_FILES: &[&str] = &[
    "https://raw.githubusercontent.com/gorhill/uBlock/master/src/js/arglist-parser.js",
    "https://raw.githubusercontent.com/gorhill/uBlock/master/src/js/jsonpath.js",
    "https://raw.githubusercontent.com/gorhill/uBlock/master/src/js/urlskip.js",
];

pub const SCRIPTLETS_URL: &str =
    "https://raw.githubusercontent.com/gorhill/uBlock/master/src/js/resources/scriptlets.js";

pub const RESOURCES_URL: &str =
    "https://raw.githubusercontent.com/brave/adblock-resources/master/dist/resources.json";

pub const SEED_LIST: &str = r#"
||doubleclick.net^
||googlesyndication.com^
||googleadservices.com^
||google-analytics.com^
||googletagmanager.com^
||googletagservices.com^
||adservice.google.com^
||pagead2.googlesyndication.com^
||amazon-adsystem.com^
||adnxs.com^
||criteo.com^
||criteo.net^
||taboola.com^
||outbrain.com^
||scorecardresearch.com^
||quantserve.com^
||moatads.com^
||adcolony.com^
||rubiconproject.com^
||pubmatic.com^
||openx.net^
||casalemedia.com^
||smartadserver.com^
||zedo.com^
||adform.net^
||teads.tv^
||bidswitch.net^
||sharethrough.com^
||facebook.com/tr^
||connect.facebook.net/en_US/fbevents.js
||hotjar.com^
||mixpanel.com^
||segment.io^
||branch.io^
||onesignal.com^
||crazyegg.com^
||fullstory.com^
||mouseflow.com^
||clarity.ms^
"#;

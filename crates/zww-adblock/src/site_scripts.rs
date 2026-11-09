use url::Url;

pub fn for_url(url: &str) -> Option<&'static str> {
    let host = Url::parse(url).ok()?.host_str()?.to_ascii_lowercase();
    let host = host.strip_prefix("www.").unwrap_or(&host).to_string();

    match host.as_str() {
        "youtube.com" | "m.youtube.com" | "music.youtube.com" | "youtube-nocookie.com" => {
            Some(YOUTUBE)
        }
        _ => None,
    }
}

const YOUTUBE: &str = r#"
(function () {
  'use strict';
  if (window.__zwwYt) return;
  window.__zwwYt = true;



  var AD_KEYS = [
    'adPlacements',
    'playerAds',
    'adSlots',
    'adBreakHeartbeatParams',
    'adParams',
    'importantForAds',
  ];


  function stripHere(obj) {
    if (!obj || typeof obj !== 'object') return false;
    var hit = false;
    for (var i = 0; i < AD_KEYS.length; i++) {
      if (AD_KEYS[i] in obj) {
        try {
          delete obj[AD_KEYS[i]];
          hit = true;
        } catch (e) {}
      }
    }
    return hit;
  }


  function prune(value) {
    if (!value || typeof value !== 'object') return value;
    stripHere(value);
    if (value.playerResponse) stripHere(value.playerResponse);
    return value;
  }


  var nativeParse = JSON.parse;
  JSON.parse = function () {
    return prune(nativeParse.apply(this, arguments));
  };


  if (window.Response && Response.prototype.json) {
    var nativeJson = Response.prototype.json;
    Response.prototype.json = function () {
      return nativeJson.call(this).then(function (data) {
        return prune(data);
      });
    };
  }


  function blankHere(obj) {
    if (!obj || typeof obj !== 'object') return obj;
    for (var i = 0; i < AD_KEYS.length; i++) {
      if (AD_KEYS[i] in obj) obj[AD_KEYS[i]] = undefined;
    }
    return obj;
  }

  function hookInitialResponse() {
    var key = 'ytInitialPlayerResponse';
    var existing = Object.getOwnPropertyDescriptor(window, key);
    if (existing && !existing.configurable) return;
    var stored = existing && 'value' in existing ? blankHere(existing.value) : undefined;
    try {
      Object.defineProperty(window, key, {
        configurable: true,
        enumerable: true,
        get: function () {
          return stored;
        },
        set: function (next) {
          stored = blankHere(next);
        },
      });
    } catch (e) {}
  }
  hookInitialResponse();

})();
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn youtube_hosts_are_covered() {
        for url in [
            "https://www.youtube.com/watch?v=x",
            "https://youtube.com/",
            "https://m.youtube.com/watch?v=x",
            "https://music.youtube.com/",
        ] {
            assert!(for_url(url).is_some(), "no script for {url}");
        }
    }

    #[test]
    fn other_sites_get_nothing() {
        assert!(for_url("https://github.com/").is_none());
        assert!(for_url("https://youtube.com.evil.test/").is_none());
        assert!(for_url("not a url").is_none());
    }

    #[test]
    fn the_script_covers_every_delivery_path() {
        let script = for_url("https://www.youtube.com/watch?v=x").unwrap();
        assert!(script.contains("adPlacements"));

        assert!(script.contains("JSON.parse"));
        assert!(script.contains("Response.prototype.json"));
        assert!(script.contains("ytInitialPlayerResponse"));
        assert!(
            script.contains("= undefined"),
            "the inline response must have its ad keys blanked, not deleted: deleting them kills the player"
        );

        assert!(script.contains("__zwwYt"));
    }
}

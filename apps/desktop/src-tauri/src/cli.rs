#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mode {
    Manager,

    OpenApp(String),

    OpenUrl(String),

    NativeHost,

    Add { url: String, name: Option<String> },

    List,

    Remove(String),

    Settings,

    Tray,

    Edit(String),
    Help,
    Version,
}

pub const HELP: &str = "\
zwebwrap - turn any webpage into a minimal desktop app

USAGE:
    zwebwrap                      Open the app library
    zwebwrap --app <id>           Open a saved app (what desktop shortcuts run)
    zwebwrap --url <url>          Open a URL without saving it
    zwebwrap --tray               Start in the tray with no window (autostart)

MANAGING APPS WITHOUT THE GUI:
    zwebwrap --add <url> [--name <name>]
                                  Create an app and its desktop shortcut
    zwebwrap --list               List saved apps
    zwebwrap --settings           Open the library on the settings panel
    zwebwrap --edit <id>          Open the library on that app's settings
    zwebwrap --remove <id>        Delete an app, its icon and its shortcut

BROWSER EXTENSION:
    zwebwrap --nm-host            Native-messaging host

OPTIONS:
    -h, --help                    Show this message
    -V, --version                 Show the version

ENVIRONMENT:
    ZWEBWRAP_LOG                  Log filter, e.g. zwebwrap=debug
    ZWEBWRAP_UA                   Override the user agent wrapped windows send
    ZWEBWRAP_NO_DMABUF=1          Disable WebKit's DMABUF renderer. Only for
                                  hardware where it genuinely misrenders; it
                                  breaks hardware-decoded video
    ZWEBWRAP_NO_ADBLOCK=1         Turn ad blocking off for every window
    ZWEBWRAP_NO_COMPOSITING=1     Force software rendering (last resort)
";

pub fn parse<I: IntoIterator<Item = String>>(args: I) -> Result<Mode, String> {
    let args: Vec<String> = args.into_iter().collect();
    let mut mode: Option<Mode> = None;
    let mut name: Option<String> = None;
    let mut i = 0;

    while i < args.len() {
        let arg = args[i].as_str();

        if arg == "--nm-host" {
            return Ok(Mode::NativeHost);
        }

        let value = |i: usize, flag: &str| -> Result<String, String> {
            args.get(i + 1)
                .filter(|v| !v.starts_with('-'))
                .cloned()
                .ok_or_else(|| format!("{flag} needs a value"))
        };

        let (parsed, step): (Option<Mode>, usize) = match arg {
            "-h" | "--help" => (Some(Mode::Help), 1),
            "-V" | "--version" => (Some(Mode::Version), 1),
            "--app" => (Some(Mode::OpenApp(value(i, "--app")?)), 2),
            "--url" => (Some(Mode::OpenUrl(value(i, "--url")?)), 2),
            "--add" => (Some(Mode::Add { url: value(i, "--add")?, name: None }), 2),
            "--list" => (Some(Mode::List), 1),
            "--settings" => (Some(Mode::Settings), 1),
            "--tray" => (Some(Mode::Tray), 1),
            "--edit" => (Some(Mode::Edit(value(i, "--edit")?)), 2),
            "--remove" => (Some(Mode::Remove(value(i, "--remove")?)), 2),

            "--name" => {
                name = Some(value(i, "--name")?);
                (None, 2)
            }
            other if other.starts_with('-') => return Err(format!("unknown option: {other}")),

            other => (Some(Mode::OpenUrl(other.to_string())), 1),
        };

        if let Some(m) = parsed {
            if mode.is_some() {
                return Err("give only one action at a time".into());
            }
            mode = Some(m);
        }
        i += step;
    }

    match mode {
        Some(Mode::Add { url, .. }) => Ok(Mode::Add { url, name }),
        Some(other) => Ok(other),
        None => Ok(Mode::Manager),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_str(args: &[&str]) -> Result<Mode, String> {
        parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn no_arguments_opens_the_manager() {
        assert_eq!(parse_str(&[]).unwrap(), Mode::Manager);
    }

    #[test]
    fn shortcuts_launch_a_saved_app() {
        assert_eq!(
            parse_str(&["--app", "youtube-com"]).unwrap(),
            Mode::OpenApp("youtube-com".into())
        );
    }

    #[test]
    fn url_can_be_a_flag_or_bare() {
        let expected = Mode::OpenUrl("https://claude.ai".into());
        assert_eq!(parse_str(&["--url", "https://claude.ai"]).unwrap(), expected);
        assert_eq!(parse_str(&["https://claude.ai"]).unwrap(), expected);
    }

    #[test]
    fn host_mode_is_recognised() {
        assert_eq!(parse_str(&["--nm-host"]).unwrap(), Mode::NativeHost);
    }

    #[test]
    fn the_retired_bridge_flag_is_now_an_unknown_option() {
        assert!(parse_str(&["--bridge"]).is_err());
    }

    #[test]
    fn help_and_version() {
        assert_eq!(parse_str(&["--help"]).unwrap(), Mode::Help);
        assert_eq!(parse_str(&["-h"]).unwrap(), Mode::Help);
        assert_eq!(parse_str(&["-V"]).unwrap(), Mode::Version);
    }

    #[test]
    fn missing_values_and_unknown_flags_are_errors() {
        assert!(parse_str(&["--app"]).is_err());
        assert!(parse_str(&["--url", "--app"]).is_err());
        assert!(parse_str(&["--nope"]).is_err());
    }

    #[test]
    fn headless_management_flags() {
        assert_eq!(parse_str(&["--list"]).unwrap(), Mode::List);
        assert_eq!(
            parse_str(&["--remove", "youtube-com"]).unwrap(),
            Mode::Remove("youtube-com".into())
        );
        assert_eq!(
            parse_str(&["--add", "youtube.com"]).unwrap(),
            Mode::Add { url: "youtube.com".into(), name: None }
        );
    }

    #[test]
    fn name_attaches_to_add_in_either_order() {
        let expected = Mode::Add { url: "youtube.com".into(), name: Some("Tube".into()) };
        assert_eq!(parse_str(&["--add", "youtube.com", "--name", "Tube"]).unwrap(), expected);
        assert_eq!(parse_str(&["--name", "Tube", "--add", "youtube.com"]).unwrap(), expected);
    }

    #[test]
    fn panel_deep_links() {
        assert_eq!(parse_str(&["--settings"]).unwrap(), Mode::Settings);
        assert_eq!(parse_str(&["--tray"]).unwrap(), Mode::Tray);
        assert_eq!(
            parse_str(&["--edit", "youtube-com"]).unwrap(),
            Mode::Edit("youtube-com".into())
        );
        assert!(parse_str(&["--edit"]).is_err());
    }

    #[test]
    fn name_alone_is_not_an_action() {
        assert_eq!(parse_str(&["--name", "Tube"]).unwrap(), Mode::Manager);
    }

    #[test]
    fn conflicting_modes_are_rejected() {
        assert!(parse_str(&["--app", "x", "--url", "https://y.com"]).is_err());
    }

    #[test]
    fn native_host_argv_from_chrome_still_parses() {
        let argv =
            vec!["--nm-host".to_string(), "chrome-extension://abcdefghijklmnop/".to_string()];
        assert_eq!(parse(argv).unwrap(), Mode::NativeHost);
    }
}

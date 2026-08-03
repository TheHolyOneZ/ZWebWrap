use anyhow::Result;
use zww_bridge::protocol::{AppSummary, Request, Response};
use zww_bridge::{native, PROTOCOL_VERSION};
use zww_core::Registry;

const LIST_ICON: u32 = 32;

pub fn run() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();

    loop {
        let request = match native::read_request(&mut input) {
            Ok(Some(request)) => request,
            Ok(None) => break,
            Err(e) => {
                tracing::warn!(error = %e, "unreadable request from the extension");
                let _ = native::write_response(&mut output, &Response::error(e));
                break;
            }
        };

        let response = handle(request);
        native::write_response(&mut output, &response)?;
    }

    Ok(())
}

fn handle(request: Request) -> Response {
    match request {
        Request::Ping => Response::Pong {
            version: PROTOCOL_VERSION,
            app_version: env!("CARGO_PKG_VERSION").to_string(),
        },

        Request::ListApps => match list_apps() {
            Ok(apps) => Response::Apps { apps },
            Err(e) => Response::error(e),
        },

        Request::CreateApp { url, title, adblock } => match create(url, title, adblock) {
            Ok(app) => Response::Created { app },
            Err(e) => Response::error(e),
        },

        Request::OpenApp { id } => match open(&id) {
            Ok(()) => Response::Ok,
            Err(e) => Response::error(e),
        },

        Request::RemoveApp { id } => match crate::create::delete_app(&id) {
            Ok(()) => Response::Ok,
            Err(e) => Response::error(e),
        },
    }
}

fn list_apps() -> Result<Vec<AppSummary>> {
    Ok(Registry::open()?.list()?.into_iter().map(summarise).collect())
}

fn create(url: String, title: Option<String>, adblock: Option<bool>) -> Result<AppSummary> {
    let prefs = zww_core::prefs::load();
    let adblock = adblock.unwrap_or(prefs.adblock_by_default);

    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let app = runtime.block_on(crate::create::create_app(&url, title, adblock))?;
    Ok(summarise(app))
}

fn open(id: &str) -> Result<()> {
    anyhow::ensure!(Registry::open()?.get(id)?.is_some(), "no app with id {id}");

    let exe = zww_core::paths::current_exe()?;
    let mut command = std::process::Command::new(exe);
    command
        .arg("--app")
        .arg(id)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }

    command.spawn()?;
    Ok(())
}

fn summarise(app: zww_core::WrappedApp) -> AppSummary {
    AppSummary {
        icon: app.icon.as_deref().and_then(small_icon),
        id: app.id,
        title: app.title,
        url: app.url,

        adblock: app.blocking != zww_core::BlockMode::Off,
    }
}

fn small_icon(path: &std::path::Path) -> Option<String> {
    use base64::Engine as _;

    let image = image::open(path).ok()?;
    let small = image.resize(LIST_ICON, LIST_ICON, image::imageops::FilterType::Lanczos3);

    let mut png = std::io::Cursor::new(Vec::new());
    small.write_to(&mut png, image::ImageFormat::Png).ok()?;

    let encoded = base64::engine::general_purpose::STANDARD.encode(png.into_inner());
    Some(format!("data:image/png;base64,{encoded}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_reports_both_versions() {
        match handle(Request::Ping) {
            Response::Pong { version, app_version } => {
                assert_eq!(version, PROTOCOL_VERSION);
                assert!(!app_version.is_empty());
            }
            other => panic!("expected a pong, got {other:?}"),
        }
    }

    #[test]
    fn opening_an_unknown_app_is_an_error_not_a_silent_success() {
        match handle(Request::OpenApp { id: "does-not-exist".into() }) {
            Response::Error { message } => assert!(message.contains("does-not-exist")),
            other => panic!("expected an error, got {other:?}"),
        }
    }

    #[test]
    fn a_bad_url_is_reported_rather_than_creating_something_broken() {
        match handle(Request::CreateApp {
            url: "file:///etc/passwd".into(),
            title: None,
            adblock: None,
        }) {
            Response::Error { .. } => {}
            other => panic!("expected an error, got {other:?}"),
        }
    }
}

use anyhow::{anyhow, Context, Result};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent};
use tauri_plugin_opener::OpenerExt;
use url::Url;
use zww_core::{scope, Registry, WindowState, WrappedApp};

pub const MANAGER_LABEL: &str = "manager";

const PROBES_OFFSITE_DOWNLOADS: bool = cfg!(target_os = "linux");

fn site_label(id: &str) -> String {
    format!("app-{id}")
}

const NAV_SHIM: &str = r#"
(function () {
  if (window.__zwwShim) return;
  window.__zwwShim = true;

  var go = function (raw) {
    if (!raw) return;
    try { window.location.assign(new URL(raw, document.baseURI).href); } catch (e) {}
  };


  var nativeOpen = window.open;
  window.open = function (url) {
    if (!url) return nativeOpen.apply(window, arguments);
    go(url);
    return { closed: false, close: function () {}, focus: function () {}, blur: function () {},
             postMessage: function () {}, document: null, location: { href: String(url) } };
  };

  document.addEventListener('click', function (e) {
    if (e.defaultPrevented || e.button !== 0) return;
    var el = e.target && e.target.closest ? e.target.closest('a[target="_blank"]') : null;
    if (!el || !el.href) return;
    e.preventDefault();
    go(el.href);
  }, true);


  document.addEventListener('keydown', function (e) {
    var key = e.key;
    if (key === 'F5' || ((e.ctrlKey || e.metaKey) && (key === 'r' || key === 'R'))) {
      e.preventDefault();
      window.location.reload();
    } else if (e.altKey && key === 'ArrowLeft') {
      e.preventDefault();
      window.history.back();
    } else if (e.altKey && key === 'ArrowRight') {
      e.preventDefault();
      window.history.forward();
    } else if ((e.ctrlKey || e.metaKey) && e.shiftKey && (key === 'c' || key === 'C')) {
      e.preventDefault();
      if (navigator.clipboard) navigator.clipboard.writeText(window.location.href);
    }
  }, true);
})();
"#;

const HID_SHIM: &str = include_str!("hid_shim.js");

pub fn open_manager(app: &AppHandle, panel: Option<&str>) -> Result<WebviewWindow> {
    if let Some(existing) = app.get_webview_window(MANAGER_LABEL) {
        let _ = existing.show();
        let _ = existing.unminimize();
        let _ = existing.set_focus();
        if let Some(panel) = panel {
            navigate_to_panel(&existing, panel);
        }
        return Ok(existing);
    }

    let builder =
        WebviewWindowBuilder::new(app, MANAGER_LABEL, WebviewUrl::App("index.html".into()))
            .title("ZWebWrap")
            .inner_size(1000.0, 700.0)
            .min_inner_size(560.0, 420.0);

    let window = with_download_handling(builder, MANAGER_LABEL)
        .build()
        .context("creating the manager window")?;

    suppress_context_menu(&window);

    if let Some(panel) = panel {
        navigate_to_panel(&window, panel);
    }
    Ok(window)
}

#[cfg(target_os = "linux")]
fn suppress_context_menu(window: &WebviewWindow) {
    use webkit2gtk::WebViewExt;

    let result = window.with_webview(|platform| {
        platform.inner().connect_context_menu(|_, _, _, _| true);
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not suppress the default context menu");
    }
}

#[cfg(windows)]
fn suppress_context_menu(window: &WebviewWindow) {
    use webview2_com::ContextMenuRequestedEventHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_11;
    use windows::core::Interface;

    let result = window.with_webview(|platform| {
        let Ok(webview) = (unsafe { platform.controller().CoreWebView2() }) else { return };
        let Ok(menus) = webview.cast::<ICoreWebView2_11>() else {
            tracing::debug!("WebView2 too old to suppress the context menu");
            return;
        };

        let handler = ContextMenuRequestedEventHandler::create(Box::new(|_webview, args| {
            let Some(args) = args else { return Ok(()) };

            unsafe { args.SetHandled(true)? };
            Ok(())
        }));

        let mut token = 0_i64;
        if let Err(e) = unsafe { menus.add_ContextMenuRequested(&handler, &mut token) } {
            tracing::debug!(error = %e, "could not suppress the default context menu");
        }
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not suppress the default context menu");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn suppress_context_menu(_window: &WebviewWindow) {}

fn navigate_to_panel(window: &WebviewWindow, panel: &str) {
    let escaped = serde_json::to_string(panel).unwrap_or_else(|_| "\"\"".into());
    if let Err(e) = window.eval(format!("window.location.hash = {escaped};")) {
        tracing::debug!(error = %e, "could not open the requested panel");
    }
}

pub fn open_site(app: &AppHandle, wrapped: &WrappedApp, persist: bool) -> Result<WebviewWindow> {
    let label = site_label(&wrapped.id);
    if let Some(existing) = app.get_webview_window(&label) {
        let _ = existing.show();
        let _ = existing.unminimize();
        let _ = existing.set_focus();

        resume_window(&existing);
        return Ok(existing);
    }

    let url =
        Url::parse(&wrapped.url).map_err(|e| anyhow!("{} is not a valid URL: {e}", wrapped.url))?;

    let blocking_off = std::env::var("ZWEBWRAP_NO_ADBLOCK").as_deref() == Ok("1");

    let block_network = wrapped.blocking.blocks_network() && !blocking_off;
    let hide_elements = wrapped.blocking.hides_elements() && !blocking_off;
    let engine = block_network.then(crate::adblock::engine).flatten();

    let deferred = cfg!(any(target_os = "linux", windows)) && engine.is_some();
    let start_url =
        if deferred { Url::parse("about:blank").expect("about:blank parses") } else { url.clone() };

    let policy_app = wrapped.clone();
    let policy_handle = app.clone();

    let probe: OffsiteProbe = std::sync::Arc::new(std::sync::Mutex::new(None));
    let policy_probe = probe.clone();

    let auth_session: std::sync::Arc<std::sync::Mutex<Option<String>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));

    let mut builder =
        WebviewWindowBuilder::new(app, &label, WebviewUrl::External(start_url))
            .title(&wrapped.title)
            .inner_size(wrapped.window.width, wrapped.window.height)
            .min_inner_size(400.0, 300.0)

            .focused(true)

            .visible(false)
            .on_navigation(move |target| {
                let mut session = auth_session.lock().unwrap_or_else(|e| e.into_inner());

                if scope::is_in_scope(target, &policy_app) {
                    *session = None;
                    return true;
                }

                if let Some(domain) = scope::auth_domain_for(target) {
                    tracing::debug!(app = %policy_app.id, %domain, "sign-in flow started");
                    *session = Some(domain);
                    return true;
                }

                if session.as_deref().is_some_and(|d| scope::continues_auth(target, d)) {
                    return true;
                }

                match scope::decide(target, &policy_app) {
                    scope::Decision::Allow => {
                        tracing::debug!(url = %target, app = %policy_app.id, "navigation allowed");
                        true
                    }
                    scope::Decision::Block => {
                        tracing::debug!(url = %target, app = %policy_app.id, "navigation blocked");
                        false
                    }
                    scope::Decision::HandOff => {
                        if PROBES_OFFSITE_DOWNLOADS && scope::looks_like_a_download(target) {
                            tracing::debug!(url = %target, app = %policy_app.id, "off-site link looks like a file; letting it start");

                            if let Ok(mut slot) = policy_probe.lock() {
                                *slot = Some(target.to_string());
                            }
                            return true;
                        }

                        tracing::info!(url = %target, app = %policy_app.id, "off-site link handed to the desktop");
                        if let Err(e) = policy_handle.opener().open_url(target.as_str(), None::<&str>) {
                            tracing::error!(error = %e, "could not open the system browser");
                        }

                        false
                    }
                }
            });

    if wrapped.session == zww_core::SessionMode::Isolated {
        match zww_core::paths::webview_profile_dir(&wrapped.id) {
            Ok(dir) => {
                tracing::debug!(app = %wrapped.id, dir = %dir.display(), "isolated session");
                builder = builder.data_directory(dir);
            }

            Err(e) => {
                return Err(e).context("could not create this app's private session directory");
            }
        }
    }

    if std::env::var("ZWEBWRAP_NO_SHIM").as_deref() != Ok("1") {
        builder = builder.initialization_script(NAV_SHIM);
    }

    if hide_elements {
        if let Some(script) = crate::adblock::injection_script(&wrapped.url) {
            tracing::debug!(app = %wrapped.id, bytes = script.len(), "injecting cosmetic filters");
            builder = builder.initialization_script(&script);
        }

        builder = builder.initialization_script(crate::overlay::GENERIC_COSMETIC);
    }

    builder = builder.initialization_script(crate::overlay::FIND_JS);

    builder = builder.initialization_script(HID_SHIM);

    builder = builder.initialization_script(crate::overlay::localized_toolbar());

    if wrapped.title_badge {
        builder = builder.initialization_script(crate::overlay::TITLE_BADGE);
    }

    if let Some(script) = crate::overlay::user_scripts(wrapped) {
        tracing::debug!(app = %wrapped.id, bytes = script.len(), "injecting user CSS/JS");
        builder = builder.initialization_script(&script);
    }

    match (wrapped.proxy.as_deref(), wrapped.proxy_is_usable()) {
        (Some(_), Some(raw)) => match zww_core::model::parse_proxy(raw) {
            Ok(url) => {
                tracing::debug!(app = %wrapped.id, proxy = %url, "routing this app through a proxy");
                builder = builder.proxy_url(url);
            }
            Err(e) => tracing::warn!(app = %wrapped.id, error = %e, "ignoring an unusable proxy"),
        },
        (Some(_), None) => tracing::warn!(
            app = %wrapped.id,
            "ignoring this app's proxy: it shares the common session, and a proxy there would apply to every app"
        ),
        (None, _) => {}
    }

    builder = with_download_handling(builder, &wrapped.id);

    if let (Some(x), Some(y)) = (wrapped.window.x, wrapped.window.y) {
        builder = builder.position(x, y);
    }

    let ua_override = std::env::var("ZWEBWRAP_UA").ok().filter(|v| !v.trim().is_empty());
    if let Some(ua) = ua_override.as_deref().or(wrapped.user_agent.as_deref()) {
        builder = builder.user_agent(ua);
    }

    let window = builder.build().context("creating the site window")?;

    set_desktop_identity(&window, &wrapped.id);
    apply_window_frame(&window, wrapped.frame);
    apply_window_icon(&window, wrapped);
    install_context_menu(&window, app.clone());
    relax_tracking_prevention(&window);
    apply_fullscreen_containment(&window, &wrapped.id);
    install_permission_policy(&window, &wrapped.id);
    release_hid_on_close(&window);
    install_offsite_response_policy(&window, wrapped, probe);
    install_basic_auth(&window, &wrapped.id);
    enable_spell_checking(&window);

    match engine {
        Some(engine) => {
            crate::adblock::apply_content_filter(&window, engine, Some(url.to_string()));
        }
        None if block_network => {
            tracing::debug!(app = %wrapped.id, "engine not ready; this window has no network blocking");
        }
        None => {}
    }

    let _ = window.show();

    if wrapped.window.maximized {
        let _ = window.maximize();
    }
    if (wrapped.zoom - 1.0).abs() > f64::EPSILON {
        let _ = window.set_zoom(wrapped.zoom);
    }
    if wrapped.always_on_top {
        let _ = window.set_always_on_top(true);
    }
    track_window(wrapped);

    if persist {
        remember_geometry(&window, &wrapped.id);
    }

    Ok(window)
}

#[cfg(target_os = "linux")]
fn install_basic_auth(window: &WebviewWindow, app_id: &str) {
    use gtk::glib::translate::{ToGlibPtr, ToGlibPtrMut};
    use webkit2gtk::{AuthenticationRequestExt, WebViewExt};

    let Some(credential) = zww_core::credentials::get(app_id) else { return };
    let id = app_id.to_string();

    let result = window.with_webview(move |platform| {
        platform.inner().connect_authenticate(move |_view, request| {
            if request.is_for_proxy() {
                return false;
            }
            if request.is_retry() {
                tracing::warn!(app = %id, "the stored login was rejected; leaving it to the site");
                return false;
            }

            tracing::debug!(app = %id, host = ?request.host(), "answering a login prompt");
            let mut answer = webkit2gtk::Credential::new(
                &credential.username,
                &credential.password,
                webkit2gtk::CredentialPersistence::ForSession,
            );

            unsafe {
                webkit2gtk::ffi::webkit_authentication_request_authenticate(
                    request.to_glib_none().0,
                    answer.to_glib_none_mut().0,
                );
            }
            true
        });
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not install the stored login");
    }
}

#[cfg(windows)]
fn install_basic_auth(window: &WebviewWindow, app_id: &str) {
    use webview2_com::BasicAuthenticationRequestedEventHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_10;
    use windows::core::{Interface, HSTRING, PCWSTR};

    let Some(credential) = zww_core::credentials::get(app_id) else { return };
    let id = app_id.to_string();

    let result = window.with_webview(move |platform| {
        let Ok(webview) = (unsafe { platform.controller().CoreWebView2() }) else { return };

        let Ok(webview) = webview.cast::<ICoreWebView2_10>() else {
            tracing::warn!(
                app = %id,
                "this WebView2 runtime cannot answer login prompts; the site will ask instead"
            );
            return;
        };

        let answered: std::cell::RefCell<std::collections::HashSet<String>> =
            std::cell::RefCell::new(std::collections::HashSet::new());

        let handler =
            BasicAuthenticationRequestedEventHandler::create(Box::new(move |_webview, args| {
                let Some(args) = args else { return Ok(()) };

                let mut raw = windows::core::PWSTR::null();
                unsafe { args.Uri(&mut raw)? };
                let uri = webview2_com::take_pwstr(raw);

                if !answered.borrow_mut().insert(uri.clone()) {
                    tracing::warn!(
                        app = %id,
                        "the stored login was rejected; leaving it to the site"
                    );
                    return Ok(());
                }

                tracing::debug!(app = %id, %uri, "answering a login prompt");
                let response = unsafe { args.Response()? };
                let user = HSTRING::from(credential.username.as_str());
                let password = HSTRING::from(credential.password.as_str());
                unsafe {
                    response.SetUserName(PCWSTR(user.as_ptr()))?;
                    response.SetPassword(PCWSTR(password.as_ptr()))?;
                }
                Ok(())
            }));

        let mut token = 0_i64;
        if let Err(e) = unsafe { webview.add_BasicAuthenticationRequested(&handler, &mut token) } {
            tracing::warn!(error = %e, "could not install the login handler");
        } else {
            tracing::debug!("stored login ready");
        }
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not reach the webview to install the login handler");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn install_basic_auth(_window: &WebviewWindow, _app_id: &str) {}

type OffsiteProbe = std::sync::Arc<std::sync::Mutex<Option<String>>>;

#[cfg(target_os = "linux")]
fn install_offsite_response_policy(
    window: &WebviewWindow,
    wrapped: &WrappedApp,
    _probe: OffsiteProbe,
) {
    use gtk::glib::translate::ToGlibPtr;
    use webkit2gtk::glib::Cast;
    use webkit2gtk::{ResponsePolicyDecisionExt, URIRequestExt, WebViewExt};

    let app = wrapped.clone();
    let handle = window.app_handle().clone();

    let result = window.with_webview(move |platform| {
        platform.inner().connect_decide_policy(move |_view, decision, kind| {
            if kind != webkit2gtk::PolicyDecisionType::Response {
                return false;
            }
            let Some(response) = decision.dynamic_cast_ref::<webkit2gtk::ResponsePolicyDecision>()
            else {
                return false;
            };

            if !response.is_mime_type_supported() {
                tracing::debug!("response cannot be displayed; downloading it instead");
                unsafe {
                    webkit2gtk::ffi::webkit_policy_decision_download(decision.to_glib_none().0);
                }
                return true;
            }

            if !response.is_main_frame_main_resource() {
                return false;
            }

            let Some(uri) = response.request().and_then(|r| r.uri()) else { return false };
            let Ok(target) = Url::parse(&uri) else { return false };

            if scope::is_in_scope(&target, &app)
                || app.offsite == zww_core::OffsitePolicy::InWindow
            {
                return false;
            }

            tracing::info!(url = %target, app = %app.id, "off-site link was a page after all; handing it over");
            if let Err(e) = handle.opener().open_url(target.as_str(), None::<&str>) {
                tracing::error!(error = %e, "could not open the system browser");
            }
            unsafe {
                webkit2gtk::ffi::webkit_policy_decision_ignore(decision.to_glib_none().0);
            }
            true
        });
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not install the off-site response policy");
    }
}

#[cfg(windows)]
fn install_offsite_response_policy(
    window: &WebviewWindow,
    wrapped: &WrappedApp,
    probe: OffsiteProbe,
) {
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_2;
    use webview2_com::WebResourceResponseReceivedEventHandler;
    use windows::core::{Interface, HSTRING, PCWSTR};

    let app = wrapped.clone();
    let handle = window.app_handle().clone();
    let home = wrapped.url.clone();

    let result = window.with_webview(move |platform| {
        let Ok(webview) = (unsafe { platform.controller().CoreWebView2() }) else { return };
        let Ok(responses) = webview.cast::<ICoreWebView2_2>() else {
            tracing::warn!("WebView2 too old to see responses; off-site probing stays off");
            return;
        };

        let handler =
            WebResourceResponseReceivedEventHandler::create(Box::new(move |webview, args| {
                let (Some(webview), Some(args)) = (webview, args) else { return Ok(()) };

                let expected = { probe.lock().unwrap_or_else(|e| e.into_inner()).clone() };
                let Some(expected) = expected else { return Ok(()) };

                let request = unsafe { args.Request()? };
                let mut raw = windows::core::PWSTR::null();
                unsafe { request.Uri(&mut raw)? };
                let uri = webview2_com::take_pwstr(raw);
                if uri != expected {
                    return Ok(());
                }

                let response = unsafe { args.Response()? };
                let headers = unsafe { response.Headers()? };
                let content_type = {
                    let name = HSTRING::from("Content-Type");
                    let mut value = windows::core::PWSTR::null();
                    match unsafe { headers.GetHeader(PCWSTR(name.as_ptr()), &mut value) } {
                        Ok(()) => webview2_com::take_pwstr(value).to_ascii_lowercase(),
                        Err(_) => String::new(),
                    }
                };
                let renderable = content_type.starts_with("text/html")
                    || content_type.starts_with("application/xhtml");
                if !renderable {
                    return Ok(());
                }

                let _ = probe.lock().unwrap_or_else(|e| e.into_inner()).take();

                if Url::parse(&uri).is_ok_and(|u| scope::is_in_scope(&u, &app)) {
                    return Ok(());
                }

                tracing::info!(
                    url = %uri,
                    app = %app.id,
                    content_type,
                    "off-site link was a page after all; handing it over"
                );
                if let Err(e) = unsafe { webview.Stop() } {
                    tracing::warn!(error = %e, "could not stop the off-site page");
                }
                let wide = HSTRING::from(home.as_str());
                if let Err(e) = unsafe { webview.Navigate(PCWSTR(wide.as_ptr())) } {
                    tracing::warn!(error = %e, "could not return the window to its own site");
                }
                if let Err(e) = handle.opener().open_url(uri.as_str(), None::<&str>) {
                    tracing::error!(error = %e, "could not open the system browser");
                }
                Ok(())
            }));

        let mut token = 0_i64;
        if let Err(e) = unsafe { responses.add_WebResourceResponseReceived(&handler, &mut token) } {
            tracing::warn!(error = %e, "could not install the off-site response policy");
        }
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not install the off-site response policy");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn install_offsite_response_policy(
    _window: &WebviewWindow,
    _wrapped: &WrappedApp,
    _probe: OffsiteProbe,
) {
}

fn with_download_handling<'a, R: tauri::Runtime, M: Manager<R>>(
    builder: WebviewWindowBuilder<'a, R, M>,
    fallback_app: &str,
) -> WebviewWindowBuilder<'a, R, M> {
    let dl_app = fallback_app.to_string();

    let in_flight: std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, u64>>> =
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));

    builder.on_download(move |webview, event| {
        let handle = webview.app_handle().clone();
        let mut tracked = in_flight.lock().unwrap_or_else(|e| e.into_inner());

        match event {
            tauri::webview::DownloadEvent::Requested { url, destination } => {
                let owner = focused_app_id(&handle).unwrap_or_else(|| dl_app.clone());

                match crate::downloads::requested(&handle, &owner, url.as_str(), destination) {
                    Some((target, id)) => {
                        tracked.insert(url.to_string(), id);
                        *destination = target;
                    }
                    None => return false,
                }
            }
            tauri::webview::DownloadEvent::Finished { url, path, success } => {
                if let Some(id) = tracked.remove(&url.to_string()) {
                    crate::downloads::completed(&handle, id, path, success);
                }
            }
            _ => {}
        }

        true
    })
}

fn focused_app_id<R: tauri::Runtime>(app: &AppHandle<R>) -> Option<String> {
    app.webview_windows().iter().find_map(|(label, window)| {
        let id = label.strip_prefix("app-")?;
        window.is_focused().ok().filter(|f| *f).map(|_| id.to_string())
    })
}

fn apply_window_icon(window: &WebviewWindow, wrapped: &WrappedApp) {
    let Some(path) = wrapped.icon.as_ref().filter(|p| p.exists()) else { return };

    match tauri::image::Image::from_path(path) {
        Ok(image) => {
            if let Err(e) = window.set_icon(image) {
                tracing::debug!(error = %e, "could not apply the window icon");
            }
        }
        Err(e) => tracing::debug!(error = %e, path = %path.display(), "icon could not be read"),
    }
}

#[cfg(target_os = "linux")]
fn relax_tracking_prevention(window: &WebviewWindow) {
    use gtk::glib::translate::ToGlibPtr;
    use webkit2gtk::{WebContextExt, WebViewExt};

    let result = window.with_webview(|platform| {
        let webview = platform.inner();
        let Some(context) = webview.context() else { return };
        let Some(manager) = context.website_data_manager() else { return };

        unsafe {
            webkit2gtk_sys::webkit_website_data_manager_set_itp_enabled(
                manager.to_glib_none().0,
                0,
            );
        }
        tracing::debug!("tracking prevention disabled so bot checks can complete");
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not adjust tracking prevention");
    }
}

#[cfg(not(target_os = "linux"))]
fn relax_tracking_prevention(_window: &WebviewWindow) {}

#[cfg(target_os = "linux")]
fn install_context_menu(window: &WebviewWindow, app: AppHandle) {
    use webkit2gtk::{ContextMenuAction, ContextMenuExt, ContextMenuItem, WebViewExt};

    let result = window.with_webview(move |platform| {
        let webview = platform.inner();

        webview.connect_context_menu(move |webview, menu, _event, hit| {
            menu.remove_all();

            let stock = |action, label: &str| {
                menu.append(&ContextMenuItem::from_stock_action_with_label(action, label));
            };

            let locale = zww_core::lang::active();
            use zww_core::lang::{t, Key};
            stock(ContextMenuAction::GoBack, t(locale, Key::Back));
            stock(ContextMenuAction::GoForward, t(locale, Key::Forward));
            stock(ContextMenuAction::Reload, t(locale, Key::Reload));
            menu.append(&ContextMenuItem::new_separator());

            use webkit2gtk::HitTestResultExt;
            if hit.context_is_selection() {
                stock(ContextMenuAction::Copy, "Copy");
            }
            if hit.context_is_link() {
                stock(ContextMenuAction::CopyLinkToClipboard, "Copy link address");
            }

            let page_url = webview.uri().map(|u| u.to_string()).unwrap_or_default();

            let copy_url = gtk::gio::SimpleAction::new("zww-copy-url", None);
            {
                let page_url = page_url.clone();
                copy_url.connect_activate(move |_, _| {
                    if let Some(display) = gtk::gdk::Display::default() {
                        let clipboard = gtk::Clipboard::default(&display);
                        if let Some(clipboard) = clipboard {
                            clipboard.set_text(&page_url);
                            clipboard.store();
                        }
                    }
                });
            }
            menu.append(&ContextMenuItem::from_gaction(
                &copy_url,
                t(locale, Key::CopyAddress),
                None,
            ));

            let open_external = gtk::gio::SimpleAction::new("zww-open-browser", None);
            {
                let app = app.clone();
                let page_url = page_url.clone();
                open_external.connect_activate(move |_, _| {
                    if let Err(e) = app.opener().open_url(&page_url, None::<&str>) {
                        tracing::warn!(error = %e, "could not open the system browser");
                    }
                });
            }
            menu.append(&ContextMenuItem::from_gaction(
                &open_external,
                t(locale, Key::OpenInBrowser),
                None,
            ));

            false
        });
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not install the context menu");
    }
}

#[cfg(windows)]
fn install_context_menu(window: &WebviewWindow, app: AppHandle) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2Environment9, ICoreWebView2_11, ICoreWebView2_2,
        COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_COMMAND, COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_SEPARATOR,
        COREWEBVIEW2_CONTEXT_MENU_TARGET_KIND_SELECTED_TEXT,
    };
    use webview2_com::{ContextMenuRequestedEventHandler, CustomItemSelectedEventHandler};
    use windows::core::{Interface, HSTRING, PCWSTR};

    let result = window.with_webview(move |platform| {
        let webview = match unsafe { platform.controller().CoreWebView2() } {
            Ok(webview) => webview,
            Err(e) => {
                tracing::warn!(error = %e, "no CoreWebView2; keeping the default context menu");
                return;
            }
        };

        let (menus, environment) = match (
            webview.cast::<ICoreWebView2_11>(),
            webview
                .cast::<ICoreWebView2_2>()
                .and_then(|w| unsafe { w.Environment() })
                .and_then(|e| e.cast::<ICoreWebView2Environment9>()),
        ) {
            (Ok(menus), Ok(environment)) => (menus, environment),
            _ => {
                tracing::warn!(
                    "this WebView2 runtime is too old to customise the context menu; \
                     the browser's own menu will be shown"
                );
                return;
            }
        };

        let handler = ContextMenuRequestedEventHandler::create(Box::new(move |webview, args| {
            let (Some(webview), Some(args)) = (webview, args) else { return Ok(()) };

            let items = unsafe { args.MenuItems()? };

            let mut count = 0_u32;
            unsafe { items.Count(&mut count)? };
            for index in (0..count).rev() {
                unsafe { items.RemoveValueAtIndex(index)? };
            }

            let locale = zww_core::lang::active();
            use zww_core::lang::{t, Key};

            let target = unsafe { args.ContextMenuTarget()? };
            let mut kind = Default::default();
            unsafe { target.Kind(&mut kind)? };
            let has_selection = kind == COREWEBVIEW2_CONTEXT_MENU_TARGET_KIND_SELECTED_TEXT;
            let mut has_link = windows::core::BOOL::default();
            unsafe { target.HasLinkUri(&mut has_link)? };
            let has_link = has_link.as_bool();

            let mut raw = windows::core::PWSTR::null();
            unsafe { webview.Source(&mut raw)? };
            let page_url = webview2_com::take_pwstr(raw);

            let link_url = if has_link {
                let mut raw = windows::core::PWSTR::null();
                unsafe { target.LinkUri(&mut raw)? };
                Some(webview2_com::take_pwstr(raw))
            } else {
                None
            };

            let at = std::cell::Cell::new(0_u32);

            let append = |item: &webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2ContextMenuItem|
             -> windows::core::Result<()> {
                unsafe { items.InsertValueAtIndex(at.get(), item)? };
                at.set(at.get() + 1);
                Ok(())
            };

            let add = |label: &str,
                       action: Box<dyn Fn() + 'static>|
             -> windows::core::Result<()> {
                let item = unsafe {
                    environment.CreateContextMenuItem(
                        PCWSTR(HSTRING::from(label).as_ptr()),
                        None,
                        COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_COMMAND,
                    )?
                };
                let selected = CustomItemSelectedEventHandler::create(Box::new(move |_, _| {
                    action();
                    Ok(())
                }));
                let mut token = 0_i64;
                unsafe { item.add_CustomItemSelected(&selected, &mut token)? };
                append(&item)
            };

            {
                let webview = webview.clone();
                add(t(locale, Key::Back), Box::new(move || unsafe {
                    let _ = webview.GoBack();
                }))?;
            }
            {
                let webview = webview.clone();
                add(t(locale, Key::Forward), Box::new(move || unsafe {
                    let _ = webview.GoForward();
                }))?;
            }
            {
                let webview = webview.clone();
                add(t(locale, Key::Reload), Box::new(move || unsafe {
                    let _ = webview.Reload();
                }))?;
            }

            {
                let separator = unsafe {
                    environment.CreateContextMenuItem(
                        PCWSTR(HSTRING::from("").as_ptr()),
                        None,
                        COREWEBVIEW2_CONTEXT_MENU_ITEM_KIND_SEPARATOR,
                    )?
                };
                append(&separator)?;
            }

            if has_selection {
                let webview = webview.clone();
                add("Copy", Box::new(move || copy_selection(&webview)))?;
            }
            if let Some(link) = link_url {
                add("Copy link address", Box::new(move || copy_text(&link)))?;
            }

            {
                let page_url = page_url.clone();
                add(t(locale, Key::CopyAddress), Box::new(move || copy_text(&page_url)))?;
            }
            {
                let app = app.clone();
                let page_url = page_url.clone();
                add(
                    t(locale, Key::OpenInBrowser),
                    Box::new(move || {
                        if let Err(e) = app.opener().open_url(&page_url, None::<&str>) {
                            tracing::warn!(error = %e, "could not open the system browser");
                        }
                    }),
                )?;
            }

            Ok(())
        }));

        let mut token = 0_i64;
        if let Err(e) = unsafe { menus.add_ContextMenuRequested(&handler, &mut token) } {
            tracing::warn!(error = %e, "could not install the context menu");
        } else {
            tracing::debug!("context menu installed");
        }
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not reach the webview to install the context menu");
    }
}

#[cfg(windows)]
fn copy_text(text: &str) {
    use windows::Win32::Foundation::HANDLE;
    use windows::Win32::System::DataExchange::{
        CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
    };
    use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
    use windows::Win32::System::Ole::CF_UNICODETEXT;

    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = std::mem::size_of_val(wide.as_slice());

    unsafe {
        if OpenClipboard(None).is_err() {
            tracing::debug!("could not open the clipboard");
            return;
        }
        let outcome = (|| -> windows::core::Result<()> {
            EmptyClipboard()?;
            let handle = GlobalAlloc(GMEM_MOVEABLE, bytes)?;
            let target = GlobalLock(handle);
            if target.is_null() {
                return Err(windows::core::Error::from_win32());
            }
            std::ptr::copy_nonoverlapping(wide.as_ptr(), target as *mut u16, wide.len());
            let _ = GlobalUnlock(handle);

            SetClipboardData(CF_UNICODETEXT.0 as u32, Some(HANDLE(handle.0)))?;
            Ok(())
        })();
        if let Err(e) = outcome {
            tracing::debug!(error = %e, "could not put text on the clipboard");
        }
        let _ = CloseClipboard();
    }
}

#[cfg(windows)]
fn copy_selection(webview: &webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2) {
    use windows::core::{HSTRING, PCWSTR};

    let script = HSTRING::from("document.execCommand('copy')");
    if let Err(e) = unsafe { webview.ExecuteScript(PCWSTR(script.as_ptr()), None) } {
        tracing::debug!(error = %e, "could not copy the selection");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn install_context_menu(_window: &WebviewWindow, _app: AppHandle) {}

#[cfg(target_os = "linux")]
fn set_desktop_identity(window: &WebviewWindow, id: &str) {
    use gtk::prelude::*;

    let class = zww_shortcut::wm_class(id);

    let gtk_window = match window.gtk_window() {
        Ok(w) => w,
        Err(e) => {
            tracing::debug!(error = %e, "no GTK window; skipping WM_CLASS");
            return;
        }
    };

    gtk_window.realize();

    let Some(gdk_window) = gtk_window.window() else {
        tracing::warn!("GTK window did not realise; skipping WM_CLASS");
        return;
    };

    let Ok(x11_window) = gdk_window.clone().downcast::<gdkx11::X11Window>() else {
        tracing::debug!("not an X11 session; per-window WM_CLASS is unavailable");
        return;
    };

    let xid = x11_window.xid();
    let Some(display) = gdk_window.display().downcast::<gdkx11::X11Display>().ok() else {
        return;
    };

    let mut value = Vec::with_capacity(class.len() * 2 + 2);
    value.extend_from_slice(class.as_bytes());
    value.push(0);
    value.extend_from_slice(class.as_bytes());
    value.push(0);

    unsafe {
        let xlib = match x11_dl::xlib::Xlib::open() {
            Ok(x) => x,
            Err(e) => {
                tracing::debug!(error = %e, "could not load Xlib; skipping WM_CLASS");
                return;
            }
        };
        use gtk::glib::translate::ToGlibPtr;
        let xdisplay = gdkx11::ffi::gdk_x11_display_get_xdisplay(display.to_glib_none().0)
            as *mut x11_dl::xlib::Display;
        (xlib.XChangeProperty)(
            xdisplay,
            xid,
            x11_dl::xlib::XA_WM_CLASS,
            x11_dl::xlib::XA_STRING,
            8,
            x11_dl::xlib::PropModeReplace,
            value.as_ptr(),
            value.len() as i32,
        );
        (xlib.XFlush)(xdisplay);
    }

    tracing::debug!(class, "set per-window WM_CLASS");
}

#[cfg(windows)]
fn set_desktop_identity(window: &WebviewWindow, id: &str) {
    use windows::Win32::Foundation::PROPERTYKEY;
    use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
    use windows::Win32::UI::Shell::PropertiesSystem::{
        IPropertyStore, SHGetPropertyStoreForWindow,
    };

    let app_user_model_id = zww_shortcut::wm_class(id);

    let hwnd = match window.hwnd() {
        Ok(hwnd) => hwnd,
        Err(e) => {
            tracing::debug!(error = %e, "no HWND; skipping the taskbar identity");
            return;
        }
    };

    unsafe {
        let store: IPropertyStore = match SHGetPropertyStoreForWindow(hwnd) {
            Ok(store) => store,
            Err(e) => {
                tracing::warn!(error = %e, "no property store for this window; taskbar identity unset");
                return;
            }
        };

        const PKEY_APP_USER_MODEL_ID: PROPERTYKEY = PROPERTYKEY {
            fmtid: windows::core::GUID::from_u128(0x9F4C2855_9F79_4B39_A8D0_E1D42DE1D5F3),
            pid: 5,
        };

        let value = PROPVARIANT::from(app_user_model_id.as_str());
        if let Err(e) = store.SetValue(&PKEY_APP_USER_MODEL_ID, &value) {
            tracing::warn!(error = %e, "could not set the taskbar identity");
            return;
        }

        if let Err(e) = store.Commit() {
            tracing::warn!(error = %e, "could not commit the taskbar identity");
            return;
        }
        tracing::debug!(app_user_model_id, "set per-window taskbar identity");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn set_desktop_identity(_window: &WebviewWindow, _id: &str) {}

#[cfg(any(target_os = "linux", windows))]
fn contains_fullscreen_now(id: &str) -> bool {
    let per_app =
        Registry::open().and_then(|r| r.get(id)).ok().flatten().and_then(|a| a.contain_fullscreen);
    let contain = zww_core::model::resolved(per_app, zww_core::prefs::load().contain_fullscreen);
    tracing::debug!(id, contain, "page asked to change fullscreen");
    contain
}

#[cfg(target_os = "linux")]
fn apply_fullscreen_containment(window: &WebviewWindow, id: &str) {
    let id = id.to_string();

    let result = window.with_webview(move |platform| {
        use webkit2gtk::WebViewExt;
        let webview = platform.inner();

        let entering = id.clone();
        webview.connect_enter_fullscreen(move |_| contains_fullscreen_now(&entering));
        webview.connect_leave_fullscreen(move |_| contains_fullscreen_now(&id));
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not hook fullscreen");
    }
}

#[cfg(windows)]
fn apply_fullscreen_containment(window: &WebviewWindow, id: &str) {
    use webview2_com::ContainsFullScreenElementChangedEventHandler;
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_4;
    use windows::core::Interface;

    let id = id.to_string();
    let handle = window.clone();

    let result = window.with_webview(move |platform| {
        let Ok(webview) = (unsafe { platform.controller().CoreWebView2() }) else { return };
        let Ok(webview) = webview.cast::<ICoreWebView2_4>() else {
            tracing::debug!("WebView2 too old to report fullscreen elements");
            return;
        };

        let handler =
            ContainsFullScreenElementChangedEventHandler::create(Box::new(move |webview, _| {
                let Some(webview) = webview else { return Ok(()) };
                let mut contains = windows::core::BOOL::default();
                unsafe { webview.ContainsFullScreenElement(&mut contains)? };

                if !contains_fullscreen_now(&id) {
                    return Ok(());
                }

                undo_window_fullscreen(handle.clone(), id.clone(), contains.as_bool());
                Ok(())
            }));

        let mut token = 0_i64;
        if let Err(e) =
            unsafe { webview.add_ContainsFullScreenElementChanged(&handler, &mut token) }
        {
            tracing::warn!(error = %e, "could not hook fullscreen");
        }
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not hook fullscreen");
    }
}

#[cfg(windows)]
fn undo_window_fullscreen(window: WebviewWindow, id: String, entering: bool) {
    const CHECKS: &[u64] = &[0, 40, 120, 300];

    for (attempt, delay_ms) in CHECKS.iter().enumerate() {
        let window = window.clone();
        let id = id.clone();
        std::thread::spawn(move || {
            if *delay_ms > 0 {
                std::thread::sleep(std::time::Duration::from_millis(*delay_ms));
            }
            let _ = window.clone().run_on_main_thread(move || match window.is_fullscreen() {
                Ok(true) => {
                    if let Err(e) = window.set_fullscreen(false) {
                        tracing::debug!(error = %e, "could not keep fullscreen inside the window");
                    } else if attempt == 0 {
                        tracing::debug!(id = %id, entering, "kept fullscreen inside the window");
                    } else {
                        tracing::debug!(
                            id = %id,
                            attempt,
                            "the runtime re-applied fullscreen; undone again"
                        );
                    }
                }
                Ok(false) => {}
                Err(e) => tracing::debug!(error = %e, "could not read the fullscreen state"),
            });
        });
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
fn apply_fullscreen_containment(_window: &WebviewWindow, _id: &str) {}

#[cfg(target_os = "linux")]
pub fn apply_window_frame(window: &WebviewWindow, frame: zww_core::FrameStyle) {
    use zww_core::FrameStyle;

    let _ = window.set_decorations(frame != FrameStyle::Frameless);
    if frame == FrameStyle::Frameless {
        return;
    }

    use gtk::prelude::*;

    const MWM_HINTS_DECORATIONS: std::os::raw::c_ulong = 1 << 1;
    const MWM_DECOR_ALL: std::os::raw::c_ulong = 1 << 0;
    const MWM_DECOR_TITLE: std::os::raw::c_ulong = 1 << 3;
    const MWM_DECOR_MENU: std::os::raw::c_ulong = 1 << 4;
    const MWM_DECOR_MINIMIZE: std::os::raw::c_ulong = 1 << 5;
    const MWM_DECOR_MAXIMIZE: std::os::raw::c_ulong = 1 << 6;

    let Ok(gtk_window) = window.gtk_window() else { return };
    gtk_window.realize();
    let Some(gdk_window) = gtk_window.window() else { return };
    let Ok(x11_window) = gdk_window.clone().downcast::<gdkx11::X11Window>() else {
        tracing::debug!("not an X11 session; the window frame is left to the compositor");
        return;
    };
    let Ok(display) = gdk_window.display().downcast::<gdkx11::X11Display>() else { return };

    let decorations = if frame == FrameStyle::NoBorder {
        MWM_DECOR_TITLE | MWM_DECOR_MENU | MWM_DECOR_MINIMIZE | MWM_DECOR_MAXIMIZE
    } else {
        MWM_DECOR_ALL
    };

    unsafe {
        let Ok(xlib) = x11_dl::xlib::Xlib::open() else { return };
        use gtk::glib::translate::ToGlibPtr;
        let xdisplay = gdkx11::ffi::gdk_x11_display_get_xdisplay(display.to_glib_none().0)
            as *mut x11_dl::xlib::Display;

        let name = std::ffi::CString::new("_MOTIF_WM_HINTS").expect("no interior NUL");
        let atom = (xlib.XInternAtom)(xdisplay, name.as_ptr(), 0);
        if atom == 0 {
            return;
        }

        let hints: [std::os::raw::c_ulong; 5] = [MWM_HINTS_DECORATIONS, 0, decorations, 0, 0];
        (xlib.XChangeProperty)(
            xdisplay,
            x11_window.xid(),
            atom,
            atom,
            32,
            x11_dl::xlib::PropModeReplace,
            hints.as_ptr() as *const u8,
            hints.len() as i32,
        );
        (xlib.XFlush)(xdisplay);
    }

    tracing::debug!(?frame, "set window frame hints");
}

#[cfg(not(target_os = "linux"))]
pub fn apply_window_frame(window: &WebviewWindow, frame: zww_core::FrameStyle) {
    let _ = window.set_decorations(frame != zww_core::FrameStyle::Frameless);
}

pub fn refresh_window_frame(app: &AppHandle, wrapped: &WrappedApp) {
    if let Some(window) = app.get_webview_window(&site_label(&wrapped.id)) {
        apply_window_frame(&window, wrapped.frame);
    }
}

#[cfg(windows)]
pub fn suspend_hidden(window: &WebviewWindow, id: &str) {
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_3;
    use webview2_com::{ExecuteScriptCompletedHandler, TrySuspendCompletedHandler};
    use windows::core::{Interface, HSTRING, PCWSTR};

    const IS_PLAYING: &str = r#"(function(){
        try {
            var m = document.querySelectorAll('video,audio');
            for (var i = 0; i < m.length; i++) {
                if (!m[i].paused && !m[i].ended && m[i].readyState > 2) return true;
            }
            if (navigator.mediaSession && navigator.mediaSession.playbackState === 'playing') return true;
        } catch (e) {}
        return false;
    })()"#;

    let id = id.to_string();
    let result = window.with_webview(move |platform| {
        let controller = platform.controller().clone();
        let Ok(webview) = (unsafe { controller.CoreWebView2() }) else { return };
        let Ok(suspendable) = webview.cast::<ICoreWebView2_3>() else {
            tracing::debug!("this WebView2 runtime cannot suspend; hidden windows keep their memory");
            return;
        };

        let script = HSTRING::from(IS_PLAYING);
        let handler = ExecuteScriptCompletedHandler::create(Box::new(move |error, result| {
            if let Err(e) = error {
                tracing::debug!(id = %id, error = %e, "could not ask the page; left running");
                return Ok(());
            }

            if result.trim() == "true" {
                tracing::debug!(id = %id, "still playing; left running");
                return Ok(());
            }
            tracing::trace!(id = %id, answer = %result.trim(), "page reports nothing playing");

            if let Err(e) = unsafe { controller.SetIsVisible(false) } {
                tracing::debug!(id = %id, error = %e, "could not hide the controller; not suspending");
                return Ok(());
            }

            let id = id.clone();
            let controller = controller.clone();
            let done = TrySuspendCompletedHandler::create(Box::new(move |error, succeeded| {
                match (error, succeeded) {
                    (Ok(()), true) => tracing::debug!(id = %id, "suspended while hidden"),

                    (Ok(()), false) => {
                        tracing::debug!(id = %id, "suspend refused; left running");
                        let _ = unsafe { controller.SetIsVisible(true) };
                    }
                    (Err(e), _) => {
                        tracing::debug!(id = %id, error = %e, "could not suspend");
                        let _ = unsafe { controller.SetIsVisible(true) };
                    }
                }
                Ok(())
            }));
            if let Err(e) = unsafe { suspendable.TrySuspend(&done) } {
                tracing::debug!(error = %e, "could not ask the webview to suspend");
            }
            Ok(())
        }));

        if let Err(e) = unsafe { webview.ExecuteScript(PCWSTR(script.as_ptr()), &handler) } {
            tracing::debug!(error = %e, "could not ask the page whether it is playing");
        }
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not reach the webview to suspend it");
    }
}

#[cfg(windows)]
pub fn resume_window(window: &WebviewWindow) {
    use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2_3;
    use windows::core::Interface;

    let result = window.with_webview(|platform| {
        let controller = platform.controller();
        let Ok(webview) = (unsafe { controller.CoreWebView2() }) else { return };
        let Ok(suspendable) = webview.cast::<ICoreWebView2_3>() else { return };

        if let Err(e) = unsafe { suspendable.Resume() } {
            tracing::debug!(error = %e, "could not resume the webview");
        }

        if let Err(e) = unsafe { controller.SetIsVisible(true) } {
            tracing::debug!(error = %e, "could not show the controller");
        }
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not reach the webview to resume it");
    }
}

#[cfg(not(windows))]
pub fn suspend_hidden(_window: &WebviewWindow, _id: &str) {}

#[cfg(not(windows))]
pub fn resume_window(_window: &WebviewWindow) {}

fn remember_geometry(window: &WebviewWindow, id: &str) {
    let id = id.to_string();
    let handle = window.clone();

    let latest: std::sync::Arc<std::sync::Mutex<Option<WindowState>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let sampler = latest.clone();

    window.on_window_event(move |event| match event {
        WindowEvent::Resized(size) => {
            if size.width == 0 || size.height == 0 {
                suspend_hidden(&handle, &id);
                return;
            }

            resume_window(&handle);

            if let Some(state) = sample_geometry(&handle) {
                if let Ok(mut slot) = sampler.lock() {
                    *slot = Some(state);
                }
            }
        }
        WindowEvent::Moved(_) => {
            if let Some(state) = sample_geometry(&handle) {
                if let Ok(mut slot) = sampler.lock() {
                    *slot = Some(state);
                }
            }
        }

        WindowEvent::Focused(true) => resume_window(&handle),
        WindowEvent::CloseRequested { api, .. } => {
            let state = sampler.lock().ok().and_then(|slot| *slot);
            if let Some(state) = state {
                if let Err(e) = persist_geometry(&id, state) {
                    tracing::debug!(id, error = %e, "could not save window geometry");
                }
            }

            if zww_core::prefs::load().close_to_tray
                && handle.app_handle().tray_by_id(crate::tray::TRAY_ID).is_some()
            {
                api.prevent_close();
                let _ = handle.hide();
                tracing::debug!(id, "hidden to the tray");

                suspend_hidden(&handle, &id);
            }
        }
        _ => {}
    });
}

fn release_hid_on_close(window: &WebviewWindow) {
    let label = window.label().to_string();
    window.on_window_event(move |event| {
        if matches!(event, WindowEvent::Destroyed) {
            crate::hid::release_window(&label);
        }
    });
}

fn sample_geometry(window: &WebviewWindow) -> Option<WindowState> {
    let maximized = window.is_maximized().unwrap_or(false);
    let scale = window.scale_factor().unwrap_or(1.0);
    let size = window.inner_size().ok()?.to_logical::<f64>(scale);
    let position = window.outer_position().ok()?.to_logical::<f64>(scale);

    Some(WindowState {
        width: size.width,
        height: size.height,
        x: Some(position.x),
        y: Some(position.y),
        maximized,
    })
}

fn persist_geometry(id: &str, state: WindowState) -> Result<()> {
    let registry = Registry::open()?;

    let state = if state.maximized {
        let previous = registry.get(id)?.map(|a| a.window).unwrap_or_default();
        WindowState { maximized: true, ..previous }
    } else {
        state
    };

    registry.set_window(id, state)
}

static LIVE: std::sync::Mutex<Option<std::collections::HashMap<String, (bool, f64)>>> =
    std::sync::Mutex::new(None);

fn with_live<T>(f: impl FnOnce(&mut std::collections::HashMap<String, (bool, f64)>) -> T) -> T {
    let mut guard = LIVE.lock().unwrap_or_else(|e| e.into_inner());
    f(guard.get_or_insert_with(Default::default))
}

pub fn track_window(app: &WrappedApp) {
    with_live(|live| {
        live.insert(app.id.clone(), (app.always_on_top, app.zoom));
    });
}

pub fn remember_always_on_top(id: &str, on: bool) {
    with_live(|live| {
        let entry = live.entry(id.to_string()).or_insert((false, 1.0));
        entry.0 = on;
    });
    update_app(id, |app| app.always_on_top = on);
}

pub fn stored_always_on_top(id: &str) -> bool {
    with_live(|live| live.get(id).map(|s| s.0)).unwrap_or(false)
}

pub fn remember_zoom(id: &str, zoom: f64) {
    with_live(|live| {
        let entry = live.entry(id.to_string()).or_insert((false, 1.0));
        entry.1 = zoom;
    });
    update_app(id, |app| app.zoom = zoom);
}

pub fn stored_zoom(id: &str) -> f64 {
    with_live(|live| live.get(id).map(|s| s.1)).unwrap_or(1.0)
}

type CompactState = (f64, f64, bool);

static COMPACT: std::sync::Mutex<Option<std::collections::HashMap<String, CompactState>>> =
    std::sync::Mutex::new(None);

pub fn remember_compact(id: &str, width: f64, height: f64) {
    let on_top = stored_always_on_top(id);
    let mut guard = COMPACT.lock().unwrap_or_else(|e| e.into_inner());
    guard.get_or_insert_with(Default::default).insert(id.to_string(), (width, height, on_top));
}

pub fn stored_compact(id: &str) -> Option<(f64, f64)> {
    let guard = COMPACT.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref()?.get(id).map(|(w, h, _)| (*w, *h))
}

pub fn compact_restores_on_top(id: &str) -> bool {
    let guard = COMPACT.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().and_then(|m| m.get(id)).map(|(_, _, top)| *top).unwrap_or(false)
}

pub fn stored_title(id: &str) -> String {
    Registry::open()
        .and_then(|r| r.get(id))
        .ok()
        .flatten()
        .map(|a| a.title)
        .unwrap_or_else(|| "ZWebWrap".to_string())
}

fn update_app(id: &str, edit: impl FnOnce(&mut WrappedApp)) {
    let Ok(registry) = Registry::open() else { return };
    let Ok(Some(mut app)) = registry.get(id) else { return };
    edit(&mut app);
    if let Err(e) = registry.upsert(app) {
        tracing::debug!(id, error = %e, "could not save the window setting");
    }
}

#[cfg(target_os = "linux")]
#[derive(Debug, Clone, Copy)]
enum Capability {
    Notifications,
    Microphone,
    Camera,
    Location,
}

#[cfg(target_os = "linux")]
impl Capability {
    fn rule(self, p: &zww_core::Permissions) -> zww_core::Rule {
        match self {
            Self::Notifications => p.notifications,
            Self::Microphone => p.microphone,
            Self::Camera => p.camera,
            Self::Location => p.location,
        }
    }

    fn apply(self, p: &mut zww_core::Permissions, rule: zww_core::Rule) {
        match self {
            Self::Notifications => p.notifications = rule,
            Self::Microphone => p.microphone = rule,
            Self::Camera => p.camera = rule,
            Self::Location => p.location = rule,
        }
    }

    fn prompt(self, locale: &str) -> &'static str {
        use zww_core::lang::{t, Key};
        match self {
            Self::Notifications => t(locale, Key::AskNotifications),
            Self::Microphone => t(locale, Key::AskMicrophone),
            Self::Camera => t(locale, Key::AskCamera),
            Self::Location => t(locale, Key::AskLocation),
        }
    }
}

#[cfg(target_os = "linux")]
fn install_permission_policy(window: &WebviewWindow, id: &str) {
    use webkit2gtk::WebViewExt;

    let id = id.to_string();
    let result = window.with_webview(move |platform| {
        platform.inner().connect_permission_request(move |_, request| match classify(request) {
            Some(capability) => {
                decide(&id, capability, request);

                true
            }

            None => false,
        });
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not install the permission policy");
    }
}

#[cfg(target_os = "linux")]
fn classify(request: &webkit2gtk::PermissionRequest) -> Option<Capability> {
    use gtk::glib::object::{Cast, ObjectExt};

    if request.is::<webkit2gtk::NotificationPermissionRequest>() {
        return Some(Capability::Notifications);
    }
    if request.is::<webkit2gtk::GeolocationPermissionRequest>() {
        return Some(Capability::Location);
    }
    if let Some(media) = request.dynamic_cast_ref::<webkit2gtk::UserMediaPermissionRequest>() {
        return Some(if media.property::<bool>("is-for-video-device") {
            Capability::Camera
        } else {
            Capability::Microphone
        });
    }
    None
}

#[cfg(target_os = "linux")]
fn decide(id: &str, capability: Capability, request: &webkit2gtk::PermissionRequest) {
    use webkit2gtk::PermissionRequestExt;
    use zww_core::Rule;

    let app = Registry::open().and_then(|r| r.get(id)).ok().flatten();
    let permissions = app.map(|a| a.permissions).unwrap_or_default();

    match capability.rule(&permissions) {
        Rule::Allow => {
            tracing::debug!(id, ?capability, "permission allowed");
            request.allow();
        }
        Rule::Deny => {
            tracing::debug!(id, ?capability, "permission denied");
            request.deny();
        }
        Rule::Ask => ask(id, capability, request),
    }
}

#[cfg(target_os = "linux")]
fn ask(id: &str, capability: Capability, request: &webkit2gtk::PermissionRequest) {
    use gtk::prelude::*;
    use webkit2gtk::PermissionRequestExt;
    use zww_core::lang::{t, Key};
    use zww_core::Rule;

    let locale = zww_core::lang::active();
    let dialog = gtk::MessageDialog::new(
        None::<&gtk::Window>,
        gtk::DialogFlags::MODAL,
        gtk::MessageType::Question,
        gtk::ButtonsType::None,
        capability.prompt(locale),
    );
    dialog.set_title(&format!("ZWebWrap — {id}"));
    dialog.add_button(t(locale, Key::DontAllow), gtk::ResponseType::No);
    dialog.add_button(t(locale, Key::Allow), gtk::ResponseType::Yes);

    let id = id.to_string();

    let request = request.clone();
    dialog.connect_response(move |dialog, response| {
        let rule = if response == gtk::ResponseType::Yes { Rule::Allow } else { Rule::Deny };
        if rule == Rule::Allow {
            request.allow();
        } else {
            request.deny();
        }
        remember(&id, capability, rule);
        dialog.close();
    });

    dialog.show_all();
}

#[cfg(target_os = "linux")]
fn remember(id: &str, capability: Capability, rule: zww_core::Rule) {
    let outcome = || -> Result<()> {
        let registry = Registry::open()?;
        let Some(mut app) = registry.get(id)? else { return Ok(()) };
        capability.apply(&mut app.permissions, rule);
        registry.upsert(app)?;
        Ok(())
    }();

    match outcome {
        Ok(()) => tracing::info!(id, ?capability, ?rule, "remembered a permission answer"),
        Err(e) => tracing::warn!(id, error = %e, "could not store the permission answer"),
    }
}

#[cfg(windows)]
fn install_permission_policy(window: &WebviewWindow, id: &str) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_PERMISSION_KIND, COREWEBVIEW2_PERMISSION_KIND_CAMERA,
        COREWEBVIEW2_PERMISSION_KIND_GEOLOCATION, COREWEBVIEW2_PERMISSION_KIND_MICROPHONE,
        COREWEBVIEW2_PERMISSION_KIND_NOTIFICATIONS, COREWEBVIEW2_PERMISSION_STATE_ALLOW,
        COREWEBVIEW2_PERMISSION_STATE_DEFAULT, COREWEBVIEW2_PERMISSION_STATE_DENY,
    };
    use webview2_com::PermissionRequestedEventHandler;
    use zww_core::Rule;

    let id = id.to_string();
    let result = window.with_webview(move |platform| {
        let webview = match unsafe { platform.controller().CoreWebView2() } {
            Ok(webview) => webview,
            Err(e) => {
                tracing::warn!(error = %e, "no CoreWebView2; permission prompts are off");
                return;
            }
        };

        let handler = PermissionRequestedEventHandler::create(Box::new(move |_webview, args| {
            let Some(args) = args else { return Ok(()) };

            let mut kind = COREWEBVIEW2_PERMISSION_KIND::default();
            unsafe { args.PermissionKind(&mut kind)? };

            let rule = if kind == COREWEBVIEW2_PERMISSION_KIND_NOTIFICATIONS {
                permission_rule(&id, |p| p.notifications)
            } else if kind == COREWEBVIEW2_PERMISSION_KIND_MICROPHONE {
                permission_rule(&id, |p| p.microphone)
            } else if kind == COREWEBVIEW2_PERMISSION_KIND_CAMERA {
                permission_rule(&id, |p| p.camera)
            } else if kind == COREWEBVIEW2_PERMISSION_KIND_GEOLOCATION {
                permission_rule(&id, |p| p.location)
            } else {
                return Ok(());
            };

            let state = match rule {
                Rule::Allow => COREWEBVIEW2_PERMISSION_STATE_ALLOW,
                Rule::Deny => COREWEBVIEW2_PERMISSION_STATE_DENY,
                Rule::Ask => COREWEBVIEW2_PERMISSION_STATE_DEFAULT,
            };
            unsafe { args.SetState(state)? };
            Ok(())
        }));

        let mut token = 0_i64;
        if let Err(e) = unsafe { webview.add_PermissionRequested(&handler, &mut token) } {
            tracing::warn!(error = %e, "could not install the permission handler");
        } else {
            tracing::debug!("WebView2 permission policy installed");
        }
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not reach the webview to set permissions");
    }
}

#[cfg(windows)]
fn permission_rule(
    id: &str,
    pick: impl Fn(&zww_core::Permissions) -> zww_core::Rule,
) -> zww_core::Rule {
    Registry::open()
        .and_then(|r| r.get(id))
        .ok()
        .flatten()
        .map(|app| pick(&app.permissions))
        .unwrap_or(zww_core::Rule::Ask)
}

#[cfg(not(any(target_os = "linux", windows)))]
fn install_permission_policy(_window: &WebviewWindow, _id: &str) {}

#[cfg(target_os = "linux")]
fn enable_spell_checking(window: &WebviewWindow) {
    use webkit2gtk::{WebContextExt, WebViewExt};

    let result = window.with_webview(|platform| {
        let Some(context) = platform.inner().context() else { return };
        context.set_spell_checking_enabled(true);

        let locale = zww_core::lang::active();
        let session = std::env::var("LANG").unwrap_or_default();
        let session = session.split('.').next().unwrap_or("").to_string();

        let mut languages: Vec<&str> = vec![locale];
        if !session.is_empty() && !session.starts_with(locale) {
            languages.push(Box::leak(session.into_boxed_str()));
        }
        context.set_spell_checking_languages(&languages);
        tracing::debug!(?languages, "spell checking enabled");
    });

    if let Err(e) = result {
        tracing::debug!(error = %e, "could not enable spell checking");
    }
}

#[cfg(not(target_os = "linux"))]
fn enable_spell_checking(_window: &WebviewWindow) {}

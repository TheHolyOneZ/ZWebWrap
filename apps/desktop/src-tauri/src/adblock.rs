use std::path::PathBuf;
use std::sync::{Arc, OnceLock, RwLock};
use zww_adblock::{AdBlock, Store, DEFAULT_MAX_AGE_SECS};

static ENGINE: OnceLock<RwLock<Option<Arc<AdBlock>>>> = OnceLock::new();

fn slot() -> &'static RwLock<Option<Arc<AdBlock>>> {
    ENGINE.get_or_init(|| RwLock::new(None))
}

pub fn engine() -> Option<Arc<AdBlock>> {
    slot().read().ok()?.clone()
}

fn set_engine(engine: Arc<AdBlock>) {
    if let Ok(mut guard) = slot().write() {
        *guard = Some(engine);
    }
}

#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub fn filter_store_dir() -> Option<PathBuf> {
    let dir = zww_core::paths::data_dir().ok()?.join("adblock/compiled");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir)
}

fn prepared_dir(store: &Store) -> PathBuf {
    store.root().join("prepared")
}

pub fn start() {
    let store = match Store::open() {
        Ok(store) => store,
        Err(e) => {
            tracing::warn!(error = %e, "ad blocking unavailable: no cache directory");
            return;
        }
    };

    let assets = store.load_cached();
    let prepared = prepared_dir(&store);

    if let Some(engine) = AdBlock::load_prepared(&prepared, &assets) {
        set_engine(Arc::new(engine));
    } else {
        tracing::info!("building the ad block engine for the first time");
        std::thread::spawn(move || {
            let assets = store.load_cached();
            match AdBlock::build(&assets) {
                Ok(engine) => {
                    if let Err(e) = engine.save_prepared(&prepared_dir(&store), &assets) {
                        tracing::warn!(error = %e, "could not cache the prepared engine");
                    }
                    set_engine(Arc::new(engine));
                }
                Err(e) => tracing::warn!(error = %e, "could not build the ad block engine"),
            }
        });
    }

    spawn_refresh();
}

fn spawn_refresh() {
    std::thread::spawn(|| {
        let Ok(store) = Store::open() else { return };
        if !store.is_stale(DEFAULT_MAX_AGE_SECS) {
            return;
        }

        let Ok(runtime) = tokio::runtime::Builder::new_current_thread().enable_all().build() else {
            return;
        };
        let Ok(client) = zww_core::site::client() else { return };

        let assets = runtime.block_on(store.ensure(&client, DEFAULT_MAX_AGE_SECS));
        match AdBlock::build(&assets) {
            Ok(engine) => {
                if let Err(e) = engine.save_prepared(&prepared_dir(&store), &assets) {
                    tracing::warn!(error = %e, "could not cache the refreshed engine");
                }

                set_engine(Arc::new(engine));
                tracing::info!("ad block filters refreshed");
            }
            Err(e) => tracing::warn!(error = %e, "could not rebuild after refresh"),
        }
    });
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Status {
    pub ready: bool,
    pub network_rules: usize,
    pub scriptlets: usize,
    pub lists_loaded: usize,

    pub seed_only: bool,

    pub age_secs: Option<u64>,
    pub sources: Vec<SourceInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SourceInfo {
    pub name: String,
    pub description: String,
    pub cached: bool,
}

pub fn status() -> Status {
    let engine = engine();
    let stats = engine.as_ref().map(|e| e.stats());

    let (age_secs, sources) = match Store::open() {
        Ok(store) => {
            let cached = store.meta().lists;
            let sources = Store::source_list()
                .iter()
                .map(|s| SourceInfo {
                    name: s.name.to_string(),
                    description: s.description.to_string(),
                    cached: cached.iter().any(|c| c == s.name),
                })
                .collect();
            (store.age_secs(), sources)
        }
        Err(_) => (None, Vec::new()),
    };

    Status {
        ready: engine.is_some(),
        network_rules: stats.map(|s| s.network_rules).unwrap_or(0),
        scriptlets: stats.map(|s| s.scriptlets).unwrap_or(0),
        lists_loaded: stats.map(|s| s.lists_loaded).unwrap_or(0),
        seed_only: stats.map(|s| s.seed_only).unwrap_or(true),
        age_secs,
        sources,
    }
}

pub fn refresh_now() -> anyhow::Result<Status> {
    let store = Store::open()?;
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    let client = zww_core::site::client()?;

    let assets = runtime.block_on(store.refresh(&client))?;
    let engine = AdBlock::build(&assets)?;
    if let Err(e) = engine.save_prepared(&prepared_dir(&store), &assets) {
        tracing::warn!(error = %e, "could not cache the refreshed engine");
    }
    set_engine(Arc::new(engine));
    Ok(status())
}

static HIDDEN: std::sync::Mutex<Option<std::collections::HashMap<String, u64>>> =
    std::sync::Mutex::new(None);

pub fn record_hidden(app_id: &str, count: u64) {
    let mut guard = HIDDEN.lock().unwrap_or_else(|e| e.into_inner());
    let map = guard.get_or_insert_with(Default::default);
    let entry = map.entry(app_id.to_string()).or_insert(0);
    if count > *entry {
        *entry = count;
        tracing::debug!(app = app_id, hidden = count, "cosmetic filtering hid elements");
    }
}

pub fn hidden_for(app_id: &str) -> u64 {
    let guard = HIDDEN.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().and_then(|m| m.get(app_id)).copied().unwrap_or(0)
}

pub fn hidden_total() -> u64 {
    let guard = HIDDEN.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().map(|m| m.values().sum()).unwrap_or(0)
}

pub fn generic_selectors(tokens: &[String]) -> Vec<String> {
    engine().map(|e| e.generic_selectors(tokens)).unwrap_or_default()
}

pub fn injection_script(url: &str) -> Option<String> {
    let engine = engine()?;
    let script = engine.injection_script(url);

    (script.len() > 80).then_some(script)
}

#[cfg(target_os = "linux")]
const FILTER_WAIT: std::time::Duration = std::time::Duration::from_secs(20);

#[cfg(target_os = "linux")]
struct FilterJob {
    manager: webkit2gtk::UserContentManager,
    webview: webkit2gtk::WebView,
    store: *mut webkit2gtk_sys::WebKitUserContentFilterStore,

    remaining: std::cell::Cell<usize>,

    pending_url: std::cell::RefCell<Option<String>>,

    specs: Vec<(std::ffi::CString, Vec<u8>)>,
}

#[cfg(target_os = "linux")]
struct FilterSlot {
    job: std::rc::Rc<FilterJob>,
    index: usize,
}

#[cfg(target_os = "linux")]
impl Drop for FilterJob {
    fn drop(&mut self) {
        if !self.store.is_null() {
            unsafe {
                gtk::glib::gobject_ffi::g_object_unref(
                    self.store as *mut gtk::glib::gobject_ffi::GObject,
                );
            }
        }
    }
}

#[cfg(target_os = "linux")]
impl FilterJob {
    fn release(&self) {
        use webkit2gtk::WebViewExt;
        if let Some(url) = self.pending_url.borrow_mut().take() {
            tracing::debug!(%url, "filters ready; loading page");
            self.webview.load_uri(&url);
        }
    }

    fn finish_one(&self) {
        let left = self.remaining.get().saturating_sub(1);
        self.remaining.set(left);
        if left == 0 {
            tracing::info!("network blocking active");
            self.release();
        }
    }

    unsafe fn add(&self, filter: *mut webkit2gtk_sys::WebKitUserContentFilter) {
        use gtk::glib::translate::ToGlibPtr;
        webkit2gtk_sys::webkit_user_content_manager_add_filter(
            self.manager.to_glib_none().0,
            filter,
        );
        webkit2gtk_sys::webkit_user_content_filter_unref(filter);
    }
}

#[cfg(target_os = "linux")]
pub fn apply_content_filter(
    window: &tauri::WebviewWindow,
    engine: Arc<AdBlock>,
    navigate_to: Option<String>,
) {
    use std::ffi::CString;
    use webkit2gtk::WebViewExt;

    let Some(store_dir) = filter_store_dir() else { return };
    let Ok(store_path) = CString::new(store_dir.to_string_lossy().as_bytes()) else { return };

    let result = window.with_webview(move |platform| {
        let webview = platform.inner();
        let Some(manager) = webview.user_content_manager() else {
            tracing::warn!("webview has no user content manager; network blocking is off");
            if let Some(url) = navigate_to {
                webview.load_uri(&url);
            }
            return;
        };

        let filters = engine.content_blocker_filters();
        if filters.is_empty() {
            if let Some(url) = navigate_to {
                webview.load_uri(&url);
            }
            return;
        }

        unsafe {
            let store = webkit2gtk_sys::webkit_user_content_filter_store_new(store_path.as_ptr());
            if store.is_null() {
                tracing::warn!("could not open the content filter store");
                if let Some(url) = navigate_to {
                    webview.load_uri(&url);
                }
                return;
            }

            let specs: Vec<(CString, Vec<u8>)> = filters
                .iter()
                .enumerate()
                .filter_map(|(index, json)| {
                    CString::new(engine.content_blocker_id(index))
                        .ok()
                        .map(|id| (id, json.as_bytes().to_vec()))
                })
                .collect();

            let job = std::rc::Rc::new(FilterJob {
                manager,
                webview,
                store,
                remaining: std::cell::Cell::new(specs.len()),
                pending_url: std::cell::RefCell::new(navigate_to),
                specs,
            });

            for index in 0..job.specs.len() {
                let slot = Box::into_raw(Box::new(FilterSlot { job: job.clone(), index }));
                webkit2gtk_sys::webkit_user_content_filter_store_load(
                    store,
                    job.specs[index].0.as_ptr(),
                    std::ptr::null_mut(),
                    Some(on_filter_loaded),
                    slot as *mut std::ffi::c_void,
                );
            }

            let guard = job.clone();
            gtk::glib::timeout_add_local_once(FILTER_WAIT, move || {
                if guard.pending_url.borrow().is_some() {
                    tracing::warn!("ad block filters took too long; loading the page unblocked");
                    guard.release();
                }
            });
        }
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not reach the webview to apply network blocking");
    }
}

#[cfg(target_os = "linux")]
unsafe extern "C" fn on_filter_loaded(
    source: *mut gtk::glib::gobject_ffi::GObject,
    result: *mut gtk::gio::ffi::GAsyncResult,
    user_data: *mut std::ffi::c_void,
) {
    use gtk::glib::translate::ToGlibPtr;

    let slot: Box<FilterSlot> = Box::from_raw(user_data as *mut FilterSlot);
    let store = source as *mut webkit2gtk_sys::WebKitUserContentFilterStore;

    let mut error: *mut gtk::glib::ffi::GError = std::ptr::null_mut();
    let filter =
        webkit2gtk_sys::webkit_user_content_filter_store_load_finish(store, result, &mut error);

    if !filter.is_null() {
        slot.job.add(filter);
        slot.job.finish_one();
        return;
    }

    if !error.is_null() {
        gtk::glib::ffi::g_error_free(error);
    }

    let (identifier, json) = &slot.job.specs[slot.index];
    let bytes = gtk::glib::Bytes::from_owned(json.clone());
    let identifier = identifier.as_ptr();

    tracing::debug!(index = slot.index, "compiling ad block filter (not cached yet)");
    let next = Box::into_raw(slot) as *mut std::ffi::c_void;

    webkit2gtk_sys::webkit_user_content_filter_store_save(
        store,
        identifier,
        bytes.to_glib_none().0,
        std::ptr::null_mut(),
        Some(on_filter_saved),
        next,
    );
}

#[cfg(target_os = "linux")]
unsafe extern "C" fn on_filter_saved(
    source: *mut gtk::glib::gobject_ffi::GObject,
    result: *mut gtk::gio::ffi::GAsyncResult,
    user_data: *mut std::ffi::c_void,
) {
    use gtk::glib::translate::FromGlibPtrFull;

    let slot: Box<FilterSlot> = Box::from_raw(user_data as *mut FilterSlot);

    let mut error: *mut gtk::glib::ffi::GError = std::ptr::null_mut();
    let filter = webkit2gtk_sys::webkit_user_content_filter_store_save_finish(
        source as *mut webkit2gtk_sys::WebKitUserContentFilterStore,
        result,
        &mut error,
    );

    if filter.is_null() {
        if !error.is_null() {
            let message = gtk::glib::Error::from_glib_full(error);
            tracing::warn!(error = %message, "could not compile an ad block filter");
        } else {
            tracing::warn!("could not compile an ad block filter");
        }
    } else {
        slot.job.add(filter);
    }

    slot.job.finish_one();
}

#[cfg(windows)]
fn request_type_for(
    context: webview2_com::Microsoft::Web::WebView2::Win32::COREWEBVIEW2_WEB_RESOURCE_CONTEXT,
    is_subframe: bool,
) -> &'static str {
    use webview2_com::Microsoft::Web::WebView2::Win32::*;

    if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_DOCUMENT {
        if is_subframe {
            "sub_frame"
        } else {
            "document"
        }
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_SCRIPT {
        "script"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_IMAGE {
        "image"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_STYLESHEET {
        "stylesheet"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_FONT {
        "font"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_MEDIA {
        "media"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_XML_HTTP_REQUEST
        || context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_FETCH
    {
        "xmlhttprequest"
    } else if context == COREWEBVIEW2_WEB_RESOURCE_CONTEXT_WEBSOCKET {
        "websocket"
    } else {
        "other"
    }
}

#[cfg(windows)]
pub fn apply_content_filter(
    window: &tauri::WebviewWindow,
    engine: Arc<AdBlock>,
    navigate_to: Option<String>,
) {
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        ICoreWebView2_2, COREWEBVIEW2_WEB_RESOURCE_CONTEXT, COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL,
    };
    use webview2_com::WebResourceRequestedEventHandler;
    use windows::core::{Interface, HSTRING, PCWSTR};

    let result = window.with_webview(move |platform| {
        let webview = match unsafe { platform.controller().CoreWebView2() } {
            Ok(webview) => webview,
            Err(e) => {
                tracing::warn!(error = %e, "no CoreWebView2; this window has no network blocking");
                navigate_anyway(None, navigate_to);
                return;
            }
        };

        let environment = match webview.cast::<ICoreWebView2_2>().and_then(|w| unsafe { w.Environment() }) {
            Ok(environment) => environment,
            Err(e) => {
                tracing::warn!(error = %e, "WebView2 is too old to answer requests; network blocking is off");
                navigate_anyway(Some(&webview), navigate_to);
                return;
            }
        };

        let handler = WebResourceRequestedEventHandler::create(Box::new(move |webview, args| {
            let (Some(webview), Some(args)) = (webview, args) else { return Ok(()) };

            let request = unsafe { args.Request()? };
            let mut raw = windows::core::PWSTR::null();
            unsafe { request.Uri(&mut raw)? };
            let url = webview2_com::take_pwstr(raw);

            let mut raw = windows::core::PWSTR::null();
            unsafe { webview.Source(&mut raw)? };
            let page_url = webview2_com::take_pwstr(raw);

            let mut context = COREWEBVIEW2_WEB_RESOURCE_CONTEXT::default();
            unsafe { args.ResourceContext(&mut context)? };

            let is_subframe = url != page_url;
            let request_type = request_type_for(context, is_subframe);
            if request_type == "document" {
                return Ok(());
            }

            if zww_adblock::is_never_blocked(&url) {
                tracing::trace!(%url, "allowed: bot-check host");
                return Ok(());
            }
            let blocked = engine.would_block(&url, &page_url, request_type);
            tracing::trace!(%url, %page_url, request_type, blocked, "request seen");
            if !blocked {
                return Ok(());
            }

            let response = unsafe {
                environment.CreateWebResourceResponse(
                    None,
                    403,
                    &HSTRING::from("Blocked"),
                    &HSTRING::from("Content-Length: 0"),
                )
            };
            match response {
                Ok(response) => {
                    unsafe { args.SetResponse(&response)? };
                    tracing::trace!(%url, request_type, "blocked");
                    record_blocked();
                }
                Err(e) => tracing::debug!(error = %e, "could not build a block response"),
            }
            Ok(())
        }));

        if let Err(e) = unsafe {
            webview.AddWebResourceRequestedFilter(
                PCWSTR(HSTRING::from("*").as_ptr()),
                COREWEBVIEW2_WEB_RESOURCE_CONTEXT_ALL,
            )
        } {
            tracing::warn!(error = %e, "could not register the request filter; blocking is off");
            navigate_anyway(Some(&webview), navigate_to);
            return;
        }

        let mut token = 0_i64;
        if let Err(e) = unsafe { webview.add_WebResourceRequested(&handler, &mut token) } {
            tracing::warn!(error = %e, "could not install the request handler; blocking is off");
            navigate_anyway(Some(&webview), navigate_to);
            return;
        }

        tracing::info!("network blocking active");
        navigate_anyway(Some(&webview), navigate_to);
    });

    if let Err(e) = result {
        tracing::warn!(error = %e, "could not reach the webview to apply network blocking");
    }
}

#[cfg(windows)]
fn navigate_anyway(
    webview: Option<&webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2>,
    url: Option<String>,
) {
    use windows::core::{HSTRING, PCWSTR};

    let Some(url) = url else { return };
    let Some(webview) = webview else {
        tracing::error!(%url, "no webview to navigate; this window will stay blank");
        return;
    };
    let wide = HSTRING::from(url.as_str());
    if let Err(e) = unsafe { webview.Navigate(PCWSTR(wide.as_ptr())) } {
        tracing::error!(error = %e, %url, "could not load the page");
    }
}

#[cfg(windows)]
static BLOCKED: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

#[cfg(windows)]
fn record_blocked() {
    let total = BLOCKED.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;

    if total % 100 == 0 {
        tracing::debug!(total, "network requests blocked so far");
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
pub fn apply_content_filter(
    _window: &tauri::WebviewWindow,
    _engine: Arc<AdBlock>,
    _navigate_to: Option<String>,
) {
}

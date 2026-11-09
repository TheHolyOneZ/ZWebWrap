use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;

use base64::{engine::general_purpose::STANDARD as B64, Engine};
use hidapi::{HidApi, HidDevice};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::{Manager, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const FIDO_PAGE: u16 = 0xF1D0;
const GENERIC_DESKTOP: u16 = 0x01;
const PROTECTED_DESKTOP_USAGES: [u16; 4] = [0x01, 0x02, 0x06, 0x07];

const MAX_REPORT: usize = 64 * 1024;

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub usage_page: Option<u16>,
    pub usage: Option<u16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDto {
    pub id: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub product_name: String,
    pub collections: Value,
}

#[derive(Clone)]
struct Found {
    id: String,
    path: std::ffi::CString,
    vendor_id: u16,
    product_id: u16,
    name: String,
    usage_page: u16,
    usage: u16,
    tops: Vec<(u16, u16)>,
    descriptor: Option<Vec<u8>>,
}

struct Open {
    device: Arc<Mutex<HidDevice>>,
    stop: Arc<AtomicBool>,
    numbered: bool,
}

struct Pending {
    parent: String,
    candidates: Vec<Found>,
    multiple: bool,
    reply: Option<tokio::sync::oneshot::Sender<Vec<String>>>,
}

#[derive(Default)]
struct State {
    api: Option<HidApi>,
    granted: HashMap<String, HashSet<String>>,
    known: HashMap<String, Found>,
    open: HashMap<(String, String), Open>,
    pending: HashMap<String, Pending>,
}

fn state() -> &'static Mutex<State> {
    static STATE: OnceLock<Mutex<State>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(State::default()))
}

fn lock() -> std::sync::MutexGuard<'static, State> {
    state().lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn is_protected(page: u16, usage: u16) -> bool {
    page == FIDO_PAGE || (page == GENERIC_DESKTOP && PROTECTED_DESKTOP_USAGES.contains(&usage))
}

fn offerable(tops: &[(u16, u16)]) -> bool {
    tops.is_empty() || tops.iter().any(|&(p, u)| !is_protected(p, u))
}

fn matches(filters: &[Filter], found: &Found) -> bool {
    if filters.is_empty() {
        return true;
    }
    filters.iter().any(|f| {
        f.vendor_id.is_none_or(|v| v == found.vendor_id)
            && f.product_id.is_none_or(|p| p == found.product_id)
            && match (f.usage_page, f.usage) {
                (None, None) => true,
                (page, usage) => found
                    .tops
                    .iter()
                    .any(|&(p, u)| page.is_none_or(|x| x == p) && usage.is_none_or(|x| x == u)),
            }
    })
}

fn path_id(path: &std::ffi::CStr) -> String {
    path.to_string_lossy().into_owned()
}

#[cfg(target_os = "linux")]
fn read_descriptor(path: &std::ffi::CStr, _api: &HidApi) -> Option<Vec<u8>> {
    let node = path.to_str().ok()?.rsplit('/').next()?.to_string();
    if !node.starts_with("hidraw") {
        return None;
    }
    std::fs::read(format!("/sys/class/hidraw/{node}/device/report_descriptor")).ok()
}

#[cfg(not(target_os = "linux"))]
fn read_descriptor(path: &std::ffi::CStr, api: &HidApi) -> Option<Vec<u8>> {
    let device = api.open_path(path).ok()?;
    let mut buf = vec![0u8; 4096];
    let n = device.get_report_descriptor(&mut buf).ok()?;
    buf.truncate(n);
    Some(buf)
}

fn enumerate(st: &mut State) -> Vec<Found> {
    if st.api.is_none() {
        match HidApi::new() {
            Ok(api) => st.api = Some(api),
            Err(e) => {
                tracing::warn!(error = %e, "HID is unavailable");
                return Vec::new();
            }
        }
    }
    let Some(api) = st.api.as_mut() else { return Vec::new() };
    if let Err(e) = api.refresh_devices() {
        tracing::warn!(error = %e, "could not enumerate HID devices");
        return Vec::new();
    }
    let api = &*api;

    let mut out = Vec::new();
    for info in api.device_list() {
        let path = info.path().to_owned();
        let descriptor = match st.known.get(&path_id(&path)) {
            Some(known) => known.descriptor.clone(),
            None if cfg!(target_os = "linux") => read_descriptor(&path, api),
            None => None,
        };
        let mut tops: Vec<(u16, u16)> = descriptor
            .as_deref()
            .map(|d| parse_descriptor(d).iter().map(|c| (c.usage_page, c.usage)).collect())
            .unwrap_or_default();
        if tops.is_empty() && (info.usage_page() != 0 || info.usage() != 0) {
            tops.push((info.usage_page(), info.usage()));
        }
        let name = info
            .product_string()
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .unwrap_or_else(|| format!("{:04x}:{:04x}", info.vendor_id(), info.product_id()));
        out.push(Found {
            id: path_id(&path),
            path,
            vendor_id: info.vendor_id(),
            product_id: info.product_id(),
            name,
            usage_page: info.usage_page(),
            usage: info.usage(),
            tops,
            descriptor,
        });
    }
    for f in &out {
        st.known.insert(f.id.clone(), f.clone());
    }
    out
}

fn fill_descriptor(st: &mut State, id: &str) {
    if cfg!(target_os = "linux") {
        return;
    }
    let Some(path) = st.known.get(id).filter(|f| f.descriptor.is_none()).map(|f| f.path.clone())
    else {
        return;
    };
    let Some(api) = st.api.as_ref() else { return };
    let descriptor = read_descriptor(&path, api);
    if let Some(found) = st.known.get_mut(id) {
        found.descriptor = descriptor;
    }
}

fn dto(found: &Found) -> DeviceDto {
    let collections = match found.descriptor.as_deref().map(parse_descriptor) {
        Some(parsed) if !parsed.is_empty() => {
            Value::Array(parsed.iter().map(Collection::to_json).collect())
        }
        _ => json!([{
            "usagePage": found.usage_page,
            "usage": found.usage,
            "type": 1,
            "children": [],
            "inputReports": [],
            "outputReports": [],
            "featureReports": [],
        }]),
    };
    DeviceDto {
        id: found.id.clone(),
        vendor_id: found.vendor_id,
        product_id: found.product_id,
        product_name: found.name.clone(),
        collections,
    }
}

fn site_label<R: Runtime>(window: &WebviewWindow<R>) -> Result<String, String> {
    let label = window.label();
    if label.starts_with("app-") {
        Ok(label.to_string())
    } else {
        Err("HID is only available to wrapped sites".into())
    }
}

fn ensure_granted(st: &State, label: &str, id: &str) -> Result<(), String> {
    if st.granted.get(label).is_some_and(|set| set.contains(id)) {
        Ok(())
    } else {
        Err("NotAllowedError: this site has not been given that device".into())
    }
}

#[tauri::command]
pub fn zww_hid_get_devices<R: Runtime>(window: WebviewWindow<R>) -> Result<Vec<DeviceDto>, String> {
    let label = site_label(&window)?;
    let mut st = lock();
    let present = enumerate(&mut st);
    let granted = st.granted.get(&label).cloned().unwrap_or_default();
    let ids: Vec<String> =
        present.iter().filter(|f| granted.contains(&f.id)).map(|f| f.id.clone()).collect();
    for id in &ids {
        fill_descriptor(&mut st, id);
    }
    Ok(ids.iter().filter_map(|id| st.known.get(id)).map(dto).collect())
}

#[tauri::command]
pub async fn zww_hid_request_device<R: Runtime>(
    window: WebviewWindow<R>,
    filters: Vec<Filter>,
    exclusion_filters: Option<Vec<Filter>>,
    multiple: Option<bool>,
) -> Result<Vec<DeviceDto>, String> {
    let label = site_label(&window)?;
    let app = window.app_handle().clone();
    let exclusions = exclusion_filters.unwrap_or_default();

    let (tx, rx) = tokio::sync::oneshot::channel();
    let chooser = {
        let mut st = lock();
        if st.pending.values().any(|p| p.parent == label) {
            return Err("InvalidStateError: a device chooser is already open".into());
        }
        let candidates: Vec<Found> = enumerate(&mut st)
            .into_iter()
            .filter(|f| offerable(&f.tops))
            .filter(|f| matches(&filters, f))
            .filter(|f| exclusions.is_empty() || !matches(&exclusions, f))
            .collect();

        static NEXT: AtomicU64 = AtomicU64::new(1);
        let chooser = format!("hid-chooser-{}", NEXT.fetch_add(1, Ordering::Relaxed));
        st.pending.insert(
            chooser.clone(),
            Pending {
                parent: label.clone(),
                candidates,
                multiple: multiple.unwrap_or(false),
                reply: Some(tx),
            },
        );
        chooser
    };

    let built =
        WebviewWindowBuilder::new(&app, &chooser, WebviewUrl::App("hid-chooser.html".into()))
            .title("ZWebWrap")
            .inner_size(440.0, 460.0)
            .resizable(false)
            .minimizable(false)
            .maximizable(false)
            .always_on_top(true)
            .center()
            .parent(&window);

    let built = match built {
        Ok(b) => b.build(),
        Err(e) => {
            lock().pending.remove(&chooser);
            return Err(e.to_string());
        }
    };
    let chooser_window = match built {
        Ok(w) => w,
        Err(e) => {
            lock().pending.remove(&chooser);
            return Err(e.to_string());
        }
    };

    let closing = chooser.clone();
    chooser_window.on_window_event(move |event| {
        if matches!(event, tauri::WindowEvent::Destroyed) {
            finish(&closing, Vec::new());
        }
    });

    let chosen = rx.await.unwrap_or_default();

    let mut st = lock();
    let mut out = Vec::new();
    for id in chosen {
        st.granted.entry(label.clone()).or_default().insert(id.clone());
        fill_descriptor(&mut st, &id);
        if let Some(found) = st.known.get(&id) {
            out.push(dto(found));
        }
    }
    Ok(out)
}

#[tauri::command]
pub fn zww_hid_forget<R: Runtime>(window: WebviewWindow<R>, id: String) -> Result<(), String> {
    let label = site_label(&window)?;
    close_one(&label, &id);
    if let Some(set) = lock().granted.get_mut(&label) {
        set.remove(&id);
    }
    Ok(())
}

#[tauri::command]
pub fn zww_hid_open<R: Runtime>(window: WebviewWindow<R>, id: String) -> Result<(), String> {
    let label = site_label(&window)?;
    let mut st = lock();
    ensure_granted(&st, &label, &id)?;
    if st.open.contains_key(&(label.clone(), id.clone())) {
        return Err("InvalidStateError: the device is already open".into());
    }
    fill_descriptor(&mut st, &id);
    let found =
        st.known.get(&id).cloned().ok_or("NotFoundError: the device is no longer connected")?;
    let api = st.api.as_ref().ok_or("NotFoundError: HID is unavailable")?;
    let device = api.open_path(&found.path).map_err(|e| {
        let hint = if cfg!(target_os = "linux") && e.to_string().contains("Permission denied") {
            " (on Linux this needs a udev rule that grants your user access to the hidraw node)"
        } else {
            ""
        };
        format!("NotAllowedError: failed to open the device: {e}{hint}")
    })?;
    device.set_blocking_mode(false).map_err(|e| format!("NetworkError: {e}"))?;

    let numbered = found
        .descriptor
        .as_deref()
        .is_some_and(|d| parse_descriptor(d).iter().any(Collection::uses_report_ids));

    let device = Arc::new(Mutex::new(device));
    let stop = Arc::new(AtomicBool::new(false));
    spawn_reader(window.clone(), id.clone(), device.clone(), stop.clone(), numbered);
    st.open.insert((label, id), Open { device, stop, numbered });
    Ok(())
}

#[tauri::command]
pub fn zww_hid_close<R: Runtime>(window: WebviewWindow<R>, id: String) -> Result<(), String> {
    let label = site_label(&window)?;
    close_one(&label, &id);
    Ok(())
}

fn with_open<T>(
    label: &str,
    id: &str,
    f: impl FnOnce(&HidDevice, bool) -> Result<T, String>,
) -> Result<T, String> {
    let (device, numbered) = {
        let st = lock();
        ensure_granted(&st, label, id)?;
        let open = st
            .open
            .get(&(label.to_string(), id.to_string()))
            .ok_or("InvalidStateError: the device is not open")?;
        (open.device.clone(), open.numbered)
    };
    let guard = device.lock().unwrap_or_else(|p| p.into_inner());
    f(&guard, numbered)
}

fn decode(data: &str) -> Result<Vec<u8>, String> {
    if data.len() > MAX_REPORT * 2 {
        return Err("DataError: the report is too large".into());
    }
    let bytes = B64.decode(data).map_err(|_| "DataError: bad report data".to_string())?;
    if bytes.len() > MAX_REPORT {
        return Err("DataError: the report is too large".into());
    }
    Ok(bytes)
}

#[tauri::command]
pub fn zww_hid_send_report<R: Runtime>(
    window: WebviewWindow<R>,
    id: String,
    report_id: u8,
    data: String,
) -> Result<(), String> {
    let label = site_label(&window)?;
    let payload = decode(&data)?;
    with_open(&label, &id, |device, _| {
        let mut buf = Vec::with_capacity(payload.len() + 1);
        buf.push(report_id);
        buf.extend_from_slice(&payload);
        device
            .write(&buf)
            .map(|_| ())
            .map_err(|e| format!("NotAllowedError: failed to write the report: {e}"))
    })
}

#[tauri::command]
pub fn zww_hid_send_feature_report<R: Runtime>(
    window: WebviewWindow<R>,
    id: String,
    report_id: u8,
    data: String,
) -> Result<(), String> {
    let label = site_label(&window)?;
    let payload = decode(&data)?;
    with_open(&label, &id, |device, _| {
        let mut buf = Vec::with_capacity(payload.len() + 1);
        buf.push(report_id);
        buf.extend_from_slice(&payload);
        device
            .send_feature_report(&buf)
            .map_err(|e| format!("NotAllowedError: failed to send the feature report: {e}"))
    })
}

#[tauri::command]
pub fn zww_hid_receive_feature_report<R: Runtime>(
    window: WebviewWindow<R>,
    id: String,
    report_id: u8,
) -> Result<String, String> {
    let label = site_label(&window)?;
    with_open(&label, &id, |device, _| {
        let mut buf = vec![0u8; 4097];
        buf[0] = report_id;
        let n = device
            .get_feature_report(&mut buf)
            .map_err(|e| format!("NotAllowedError: failed to read the feature report: {e}"))?;
        let body = if n > 1 { &buf[1..n] } else { &[][..] };
        Ok(B64.encode(body))
    })
}

fn close_one(label: &str, id: &str) {
    if let Some(open) = lock().open.remove(&(label.to_string(), id.to_string())) {
        open.stop.store(true, Ordering::Relaxed);
    }
}

pub fn release_window(label: &str) {
    let mut st = lock();
    st.granted.remove(label);
    let keys: Vec<_> = st.open.keys().filter(|(l, _)| l == label).cloned().collect();
    for key in keys {
        if let Some(open) = st.open.remove(&key) {
            open.stop.store(true, Ordering::Relaxed);
        }
    }
}

fn spawn_reader<R: Runtime>(
    window: WebviewWindow<R>,
    id: String,
    device: Arc<Mutex<HidDevice>>,
    stop: Arc<AtomicBool>,
    numbered: bool,
) {
    std::thread::spawn(move || {
        let id_json = Value::String(id).to_string();
        let mut buf = vec![0u8; 4096];
        while !stop.load(Ordering::Relaxed) {
            let read = {
                let guard = device.lock().unwrap_or_else(|p| p.into_inner());
                guard.read(&mut buf)
            };
            match read {
                Ok(0) => std::thread::sleep(Duration::from_millis(2)),
                Ok(n) => {
                    let (report_id, body) =
                        if numbered { (buf[0], &buf[1..n]) } else { (0, &buf[..n]) };
                    let script = format!(
                        "window.__zwwHid&&window.__zwwHid.input({id_json},{report_id},'{}')",
                        B64.encode(body)
                    );
                    if window.eval(&script).is_err() {
                        break;
                    }
                }
                Err(e) => {
                    tracing::debug!(error = %e, "HID read ended; the device is gone");
                    let script = format!("window.__zwwHid&&window.__zwwHid.gone({id_json})");
                    let _ = window.eval(&script);
                    break;
                }
            }
        }
    });
}

fn chooser_label<R: Runtime>(window: &WebviewWindow<R>) -> Result<String, String> {
    let label = window.label();
    if label.starts_with("hid-chooser-") {
        Ok(label.to_string())
    } else {
        Err("not a device chooser".into())
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChooserState {
    pub title: String,
    pub none: String,
    pub connect: String,
    pub cancel: String,
    pub multiple: bool,
    pub devices: Vec<ChooserDevice>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChooserDevice {
    pub id: String,
    pub name: String,
    pub detail: String,
}

#[tauri::command]
pub fn hid_chooser_state<R: Runtime>(window: WebviewWindow<R>) -> Result<ChooserState, String> {
    use zww_core::lang::{active, t, Key};
    let label = chooser_label(&window)?;
    let st = lock();
    let pending = st.pending.get(&label).ok_or("no request is pending")?;
    let locale = active();
    Ok(ChooserState {
        title: t(locale, Key::HidTitle).to_string(),
        none: t(locale, Key::HidNone).to_string(),
        connect: t(locale, Key::HidConnect).to_string(),
        cancel: t(locale, Key::HidCancel).to_string(),
        multiple: pending.multiple,
        devices: pending
            .candidates
            .iter()
            .map(|f| ChooserDevice {
                id: f.id.clone(),
                name: f.name.clone(),
                detail: match f.tops.first() {
                    Some((page, usage)) => {
                        format!("{:04x}:{:04x} · {page:04x}/{usage:04x}", f.vendor_id, f.product_id)
                    }
                    None => format!("{:04x}:{:04x}", f.vendor_id, f.product_id),
                },
            })
            .collect(),
    })
}

#[tauri::command]
pub fn hid_chooser_choose<R: Runtime>(
    window: WebviewWindow<R>,
    ids: Vec<String>,
) -> Result<(), String> {
    let label = chooser_label(&window)?;
    let allowed: HashSet<String> = {
        let st = lock();
        let pending = st.pending.get(&label).ok_or("no request is pending")?;
        pending.candidates.iter().map(|f| f.id.clone()).collect()
    };
    let multiple = lock().pending.get(&label).is_some_and(|p| p.multiple);
    let mut picked: Vec<String> = ids.into_iter().filter(|id| allowed.contains(id)).collect();
    if !multiple {
        picked.truncate(1);
    }
    finish(&label, picked);
    let _ = window.close();
    Ok(())
}

#[tauri::command]
pub fn hid_chooser_cancel<R: Runtime>(window: WebviewWindow<R>) -> Result<(), String> {
    let label = chooser_label(&window)?;
    finish(&label, Vec::new());
    let _ = window.close();
    Ok(())
}

fn finish(chooser: &str, picked: Vec<String>) {
    let reply = lock().pending.remove(chooser).and_then(|mut p| p.reply.take());
    if let Some(reply) = reply {
        let _ = reply.send(picked);
    }
}

#[derive(Debug, Default, Clone)]
pub struct Collection {
    pub usage_page: u16,
    pub usage: u16,
    pub kind: u32,
    pub children: Vec<Collection>,
    input: Vec<Report>,
    output: Vec<Report>,
    feature: Vec<Report>,
}

#[derive(Debug, Default, Clone)]
struct Report {
    id: u8,
    items: Vec<Value>,
}

impl Collection {
    fn uses_report_ids(&self) -> bool {
        self.input.iter().chain(&self.output).chain(&self.feature).any(|r| r.id != 0)
            || self.children.iter().any(Collection::uses_report_ids)
    }

    pub fn to_json(&self) -> Value {
        let reports = |list: &Vec<Report>| {
            Value::Array(
                list.iter().map(|r| json!({ "reportId": r.id, "items": r.items })).collect(),
            )
        };
        json!({
            "usagePage": self.usage_page,
            "usage": self.usage,
            "type": self.kind,
            "children": self.children.iter().map(Collection::to_json).collect::<Vec<_>>(),
            "inputReports": reports(&self.input),
            "outputReports": reports(&self.output),
            "featureReports": reports(&self.feature),
        })
    }
}

#[derive(Clone, Default)]
struct Globals {
    usage_page: u32,
    logical_min: i64,
    logical_max: i64,
    physical_min: i64,
    physical_max: i64,
    unit_exponent: i64,
    unit: u32,
    report_size: u32,
    report_count: u32,
    report_id: u8,
}

#[derive(Default)]
struct Locals {
    usages: Vec<u32>,
    min: Option<u32>,
    max: Option<u32>,
}

fn unsigned(bytes: &[u8]) -> u32 {
    bytes.iter().enumerate().fold(0u32, |acc, (i, b)| acc | (u32::from(*b) << (8 * i)))
}

fn signed(bytes: &[u8]) -> i64 {
    let raw = i64::from(unsigned(bytes));
    match bytes.len() {
        1 => i64::from(raw as u8 as i8),
        2 => i64::from(raw as u16 as i16),
        4 => i64::from(raw as u32 as i32),
        _ => raw,
    }
}

pub fn parse_descriptor(bytes: &[u8]) -> Vec<Collection> {
    let mut roots: Vec<Collection> = Vec::new();
    let mut open: Vec<usize> = Vec::new();
    let mut globals = Globals::default();
    let mut stack: Vec<Globals> = Vec::new();
    let mut locals = Locals::default();

    fn current<'a>(roots: &'a mut [Collection], path: &[usize]) -> Option<&'a mut Collection> {
        let (first, rest) = path.split_first()?;
        let mut node = roots.get_mut(*first)?;
        for &i in rest {
            node = node.children.get_mut(i)?;
        }
        Some(node)
    }

    let mut i = 0;
    while i < bytes.len() {
        let prefix = bytes[i];
        if prefix == 0xFE {
            let len = bytes.get(i + 1).copied().unwrap_or(0) as usize;
            i += 3 + len;
            continue;
        }
        let size = match prefix & 0x03 {
            3 => 4,
            n => n as usize,
        };
        let kind = (prefix >> 2) & 0x03;
        let tag = prefix >> 4;
        let Some(data) = bytes.get(i + 1..i + 1 + size) else { break };
        i += 1 + size;

        match (kind, tag) {
            (1, 0x0) => globals.usage_page = unsigned(data),
            (1, 0x1) => globals.logical_min = signed(data),
            (1, 0x2) => globals.logical_max = signed(data),
            (1, 0x3) => globals.physical_min = signed(data),
            (1, 0x4) => globals.physical_max = signed(data),
            (1, 0x5) => {
                let nibble = (unsigned(data) & 0x0F) as i64;
                globals.unit_exponent = if nibble > 7 { nibble - 16 } else { nibble };
            }
            (1, 0x6) => globals.unit = unsigned(data),
            (1, 0x7) => globals.report_size = unsigned(data),
            (1, 0x8) => globals.report_id = unsigned(data) as u8,
            (1, 0x9) => globals.report_count = unsigned(data),
            (1, 0xA) => stack.push(globals.clone()),
            (1, 0xB) => {
                if let Some(g) = stack.pop() {
                    globals = g;
                }
            }
            (2, 0x0) => {
                let v = unsigned(data);
                locals.usages.push(if size == 4 { v } else { (globals.usage_page << 16) | v });
            }
            (2, 0x1) => locals.min = Some(unsigned(data)),
            (2, 0x2) => locals.max = Some(unsigned(data)),
            (0, 0xA) => {
                let kind = unsigned(data);
                let usage = locals.usages.first().copied().unwrap_or(0);
                let page = if usage >> 16 != 0 { usage >> 16 } else { globals.usage_page };
                let collection = Collection {
                    usage_page: page as u16,
                    usage: (usage & 0xFFFF) as u16,
                    kind,
                    ..Collection::default()
                };
                if open.is_empty() {
                    roots.push(collection);
                    open.push(roots.len() - 1);
                } else if let Some(parent) = current(&mut roots, &open) {
                    parent.children.push(collection);
                    let index = parent.children.len() - 1;
                    open.push(index);
                }
                locals = Locals::default();
            }
            (0, 0xC) => {
                open.pop();
                locals = Locals::default();
            }
            (0, 0x8) | (0, 0x9) | (0, 0xB) => {
                let flags = unsigned(data);
                let item = main_item(&globals, &locals, flags);
                if let Some(node) = current(&mut roots, &open) {
                    let list = match tag {
                        0x8 => &mut node.input,
                        0x9 => &mut node.output,
                        _ => &mut node.feature,
                    };
                    match list.iter_mut().find(|r| r.id == globals.report_id) {
                        Some(report) => report.items.push(item),
                        None => list.push(Report { id: globals.report_id, items: vec![item] }),
                    }
                }
                locals = Locals::default();
            }
            _ => {}
        }
    }
    roots
}

fn main_item(globals: &Globals, locals: &Locals, flags: u32) -> Value {
    let bit = |n: u32| flags & (1 << n) != 0;
    let is_range = locals.min.is_some() && locals.max.is_some();
    let full = |v: u32| if v >> 16 != 0 { v } else { (globals.usage_page << 16) | v };
    json!({
        "isConstant": bit(0),
        "isArray": !bit(1),
        "isAbsolute": !bit(2),
        "wrap": bit(3),
        "isLinear": !bit(4),
        "hasPreferredState": !bit(5),
        "hasNull": bit(6),
        "isVolatile": bit(7),
        "isBufferedBytes": bit(8),
        "isRange": is_range,
        "usages": if is_range { Vec::new() } else { locals.usages.clone() },
        "usageMinimum": locals.min.map(full).unwrap_or(0),
        "usageMaximum": locals.max.map(full).unwrap_or(0),
        "reportSize": globals.report_size,
        "reportCount": globals.report_count,
        "unitExponent": globals.unit_exponent,
        "unitSystem": "none",
        "unitFactorLengthExponent": 0,
        "unitFactorMassExponent": 0,
        "unitFactorTimeExponent": 0,
        "unitFactorTemperatureExponent": 0,
        "unitFactorCurrentExponent": 0,
        "unitFactorLuminousIntensityExponent": 0,
        "logicalMinimum": globals.logical_min,
        "logicalMaximum": globals.logical_max,
        "physicalMinimum": globals.physical_min,
        "physicalMaximum": globals.physical_max,
        "strings": [],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIA: &[u8] = &[
        0x06, 0x60, 0xFF, 0x09, 0x61, 0xA1, 0x01, 0x09, 0x62, 0x15, 0x00, 0x26, 0xFF, 0x00, 0x95,
        0x20, 0x75, 0x08, 0x81, 0x02, 0x09, 0x63, 0x15, 0x00, 0x26, 0xFF, 0x00, 0x95, 0x20, 0x75,
        0x08, 0x91, 0x02, 0xC0,
    ];

    const KEYBOARD: &[u8] = &[
        0x05, 0x01, 0x09, 0x06, 0xA1, 0x01, 0x85, 0x01, 0x05, 0x07, 0x19, 0xE0, 0x29, 0xE7, 0x15,
        0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0xC0,
    ];

    #[test]
    fn a_vendor_defined_interface_parses_with_its_reports() {
        let tree = parse_descriptor(VIA);
        assert_eq!(tree.len(), 1);
        assert_eq!((tree[0].usage_page, tree[0].usage), (0xFF60, 0x61));
        let json = tree[0].to_json();
        assert_eq!(json["inputReports"][0]["items"][0]["reportCount"], 32);
        assert_eq!(json["outputReports"][0]["items"][0]["reportSize"], 8);
        assert_eq!(json["inputReports"][0]["items"][0]["logicalMaximum"], 255);
        assert!(!tree[0].uses_report_ids());
    }

    #[test]
    fn report_ids_are_noticed() {
        let tree = parse_descriptor(KEYBOARD);
        assert_eq!((tree[0].usage_page, tree[0].usage), (0x01, 0x06));
        assert!(tree[0].uses_report_ids());
        assert_eq!(tree[0].to_json()["inputReports"][0]["reportId"], 1);
    }

    #[test]
    fn garbage_does_not_panic() {
        assert!(parse_descriptor(&[0xA1]).len() <= 1);
        assert!(parse_descriptor(&[0xFE, 0xFF]).is_empty());
        assert!(parse_descriptor(&[]).is_empty());
    }

    #[test]
    fn keyboards_mice_and_security_keys_are_never_offered() {
        assert!(!offerable(&[(0x01, 0x06)]), "keyboard");
        assert!(!offerable(&[(0x01, 0x02)]), "mouse");
        assert!(!offerable(&[(FIDO_PAGE, 0x01)]), "security key");
        assert!(!offerable(&[(0x01, 0x06), (0x01, 0x02)]), "keyboard plus mouse");
    }

    #[test]
    fn a_keyboards_vendor_interface_is_still_offered() {
        assert!(offerable(&[(0x01, 0x06), (0xFF60, 0x61)]));
        assert!(offerable(&[(0xFF00, 0x01)]));
        assert!(offerable(&[]), "unknown usage is not a reason to hide it");
    }

    fn found(vid: u16, pid: u16, tops: Vec<(u16, u16)>) -> Found {
        Found {
            id: "x".into(),
            path: std::ffi::CString::new("x").unwrap(),
            vendor_id: vid,
            product_id: pid,
            name: String::new(),
            usage_page: tops.first().map(|t| t.0).unwrap_or(0),
            usage: tops.first().map(|t| t.1).unwrap_or(0),
            tops,
            descriptor: None,
        }
    }

    #[test]
    fn filters_follow_the_webhid_rules() {
        let device = found(0x1234, 0x5678, vec![(0xFF60, 0x61)]);
        assert!(matches(&[], &device), "no filters means everything");
        let vid = Filter { vendor_id: Some(0x1234), ..Filter::default() };
        assert!(matches(std::slice::from_ref(&vid), &device));
        let other = Filter { vendor_id: Some(0x9999), ..Filter::default() };
        assert!(!matches(std::slice::from_ref(&other), &device));
        assert!(matches(&[other, vid], &device), "any filter may match");
        let usage = Filter { usage_page: Some(0xFF60), usage: Some(0x61), ..Filter::default() };
        assert!(matches(&[usage], &device));
        let wrong = Filter { usage_page: Some(0x01), ..Filter::default() };
        assert!(!matches(&[wrong], &device));
    }
}

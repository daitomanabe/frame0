use anyhow::{anyhow, Context, Result};
use frame0_plugin_api::{
    load_plugin_manifest, verify_plugin, PluginManifest, PluginVerification,
    FRAME0_PLUGIN_API_VERSION,
};
use libloading::Library;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::path::{Path, PathBuf};

const FRAME0_OK: c_int = 0;

#[repr(C)]
#[derive(Clone, Copy)]
struct FfiPluginDescriptor {
    api_version: u32,
    plugin_id: *const c_char,
    plugin_name: *const c_char,
    plugin_version: *const c_char,
}

impl Default for FfiPluginDescriptor {
    fn default() -> Self {
        Self {
            api_version: 0,
            plugin_id: std::ptr::null(),
            plugin_name: std::ptr::null(),
            plugin_version: std::ptr::null(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FfiDeviceDescriptor {
    id: *const c_char,
    device_type: *const c_char,
    vendor: *const c_char,
    capabilities_json: *const c_char,
    modes_json: *const c_char,
    permissions_json: *const c_char,
    vendor_properties_json: *const c_char,
}

#[repr(C)]
struct FfiCaptureConfig {
    device_id: *const c_char,
    mode_id: *const c_char,
    pixel_format: *const c_char,
    options_json: *const c_char,
}

#[repr(C)]
struct FfiFramePacket {
    stream_id: *const c_char,
    pts_ns: u64,
    duration_ns: u64,
    frame_index: u64,
    width: u32,
    height: u32,
    pixel_format: *const c_char,
    color_space: *const c_char,
    storage_json: *const c_char,
}

#[repr(C)]
struct FfiAudioPacket {
    stream_id: *const c_char,
    pts_ns: u64,
    duration_ns: u64,
    sample_rate: u32,
    channels: u16,
    frames: u32,
    format: *const c_char,
    storage_json: *const c_char,
}

impl Default for FfiDeviceDescriptor {
    fn default() -> Self {
        Self {
            id: std::ptr::null(),
            device_type: std::ptr::null(),
            vendor: std::ptr::null(),
            capabilities_json: std::ptr::null(),
            modes_json: std::ptr::null(),
            permissions_json: std::ptr::null(),
            vendor_properties_json: std::ptr::null(),
        }
    }
}

type GetDescriptorFn = unsafe extern "C" fn(*mut FfiPluginDescriptor) -> c_int;
type InitializeFn = unsafe extern "C" fn(*mut c_void) -> c_int;
type ShutdownFn = unsafe extern "C" fn() -> c_int;
type EnumerateDevicesFn =
    unsafe extern "C" fn(*mut FfiDeviceDescriptor, c_uint, *mut c_uint) -> c_int;
type LastErrorFn = unsafe extern "C" fn() -> *const c_char;
type OpenDeviceFn = unsafe extern "C" fn(*const c_char, *mut *mut c_void) -> c_int;
type CloseDeviceFn = unsafe extern "C" fn(*mut c_void) -> c_int;
type StartStreamFn =
    unsafe extern "C" fn(*mut c_void, *const FfiCaptureConfig, *mut *mut c_void) -> c_int;
type StopStreamFn = unsafe extern "C" fn(*mut c_void) -> c_int;
type FrameCallback = extern "C" fn(*const FfiFramePacket, *mut c_void);
type AudioCallback = extern "C" fn(*const FfiAudioPacket, *mut c_void);
type SetFrameCallbackFn =
    unsafe extern "C" fn(*mut c_void, Option<FrameCallback>, *mut c_void) -> c_int;
type SetAudioCallbackFn =
    unsafe extern "C" fn(*mut c_void, Option<AudioCallback>, *mut c_void) -> c_int;
type ControlJsonFn = unsafe extern "C" fn(*const c_char, *mut *const c_char) -> c_int;
type FreeStringFn = unsafe extern "C" fn(*const c_char);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NativePluginDescriptor {
    pub api_version: u32,
    pub plugin_id: String,
    pub plugin_name: String,
    pub plugin_version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeDeviceDescriptor {
    pub id: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub vendor: String,
    pub capabilities: Vec<String>,
    pub modes: Value,
    pub permissions: Vec<String>,
    pub vendor_properties: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeFramePacket {
    pub stream_id: String,
    pub pts_ns: u64,
    pub duration_ns: u64,
    pub frame_index: u64,
    pub width: u32,
    pub height: u32,
    pub pixel_format: String,
    pub color_space: String,
    pub storage: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NativeAudioPacket {
    pub stream_id: String,
    pub pts_ns: u64,
    pub duration_ns: u64,
    pub sample_rate: u32,
    pub channels: u16,
    pub frames: u32,
    pub format: String,
    pub storage: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostInspectReport {
    pub manifest: PluginManifest,
    pub verification: PluginVerification,
    pub library_path: PathBuf,
    pub descriptor: NativePluginDescriptor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostSmokeReport {
    pub manifest: PluginManifest,
    pub verification: PluginVerification,
    pub library_path: PathBuf,
    pub descriptor: NativePluginDescriptor,
    pub devices: Vec<NativeDeviceDescriptor>,
    pub initialized: bool,
    pub shutdown: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostStreamReport {
    pub manifest: PluginManifest,
    pub library_path: PathBuf,
    pub descriptor: NativePluginDescriptor,
    pub device_id: String,
    pub opened: bool,
    pub started: bool,
    pub stopped: bool,
    pub closed: bool,
    pub frames: Vec<NativeFramePacket>,
    pub audio: Vec<NativeAudioPacket>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostControlReport {
    pub manifest: PluginManifest,
    pub library_path: PathBuf,
    pub descriptor: NativePluginDescriptor,
    pub request: Value,
    pub response: Value,
}

#[derive(Default)]
struct CapturedPackets {
    frames: Vec<NativeFramePacket>,
    audio: Vec<NativeAudioPacket>,
}

pub struct PluginHost {
    library_path: PathBuf,
    library: Library,
}

impl PluginHost {
    pub fn load(library_path: PathBuf) -> Result<Self> {
        let library = unsafe { Library::new(&library_path) }
            .with_context(|| format!("failed to load {}", library_path.display()))?;
        Ok(Self {
            library_path,
            library,
        })
    }

    pub fn library_path(&self) -> &Path {
        &self.library_path
    }

    pub fn descriptor(&self) -> Result<NativePluginDescriptor> {
        let get_descriptor = unsafe {
            self.library
                .get::<GetDescriptorFn>(b"frame0_plugin_get_descriptor\0")
        }
        .context("missing symbol frame0_plugin_get_descriptor")?;
        let mut descriptor = FfiPluginDescriptor::default();
        let result = unsafe { get_descriptor(&mut descriptor) };
        self.check_result(result, "frame0_plugin_get_descriptor")?;
        Ok(NativePluginDescriptor {
            api_version: descriptor.api_version,
            plugin_id: c_string(descriptor.plugin_id)?,
            plugin_name: c_string(descriptor.plugin_name)?,
            plugin_version: c_string(descriptor.plugin_version)?,
        })
    }

    pub fn initialize(&self) -> Result<()> {
        let initialize = unsafe {
            self.library
                .get::<InitializeFn>(b"frame0_plugin_initialize\0")
        }
        .context("missing symbol frame0_plugin_initialize")?;
        let result = unsafe { initialize(std::ptr::null_mut()) };
        self.check_result(result, "frame0_plugin_initialize")
    }

    pub fn shutdown(&self) -> Result<()> {
        let shutdown = unsafe { self.library.get::<ShutdownFn>(b"frame0_plugin_shutdown\0") }
            .context("missing symbol frame0_plugin_shutdown")?;
        let result = unsafe { shutdown() };
        self.check_result(result, "frame0_plugin_shutdown")
    }

    pub fn enumerate_devices(&self) -> Result<Vec<NativeDeviceDescriptor>> {
        let enumerate = unsafe {
            self.library
                .get::<EnumerateDevicesFn>(b"frame0_plugin_enumerate_devices\0")
        }
        .context("missing symbol frame0_plugin_enumerate_devices")?;

        let mut count = 0_u32;
        let result = unsafe { enumerate(std::ptr::null_mut(), 0, &mut count) };
        self.check_result(result, "frame0_plugin_enumerate_devices(count)")?;

        let mut descriptors = vec![FfiDeviceDescriptor::default(); count as usize];
        let result = unsafe { enumerate(descriptors.as_mut_ptr(), count, &mut count) };
        self.check_result(result, "frame0_plugin_enumerate_devices")?;

        descriptors
            .into_iter()
            .take(count as usize)
            .map(convert_device_descriptor)
            .collect()
    }

    fn stream_test(&self, device_id: &str) -> Result<CapturedPackets> {
        let open_device = unsafe {
            self.library
                .get::<OpenDeviceFn>(b"frame0_plugin_open_device\0")
        }
        .context("missing symbol frame0_plugin_open_device")?;
        let close_device = unsafe {
            self.library
                .get::<CloseDeviceFn>(b"frame0_plugin_close_device\0")
        }
        .context("missing symbol frame0_plugin_close_device")?;
        let start_stream = unsafe {
            self.library
                .get::<StartStreamFn>(b"frame0_plugin_start_stream\0")
        }
        .context("missing symbol frame0_plugin_start_stream")?;
        let stop_stream = unsafe {
            self.library
                .get::<StopStreamFn>(b"frame0_plugin_stop_stream\0")
        }
        .context("missing symbol frame0_plugin_stop_stream")?;
        let set_frame_callback = unsafe {
            self.library
                .get::<SetFrameCallbackFn>(b"frame0_plugin_set_frame_callback\0")
        }
        .context("missing symbol frame0_plugin_set_frame_callback")?;
        let set_audio_callback = unsafe {
            self.library
                .get::<SetAudioCallbackFn>(b"frame0_plugin_set_audio_callback\0")
        }
        .context("missing symbol frame0_plugin_set_audio_callback")?;

        let device_id_c = CString::new(device_id)?;
        let mode_id = CString::new("auto")?;
        let pixel_format = CString::new("auto")?;
        let options_json = CString::new("{}")?;
        let config = FfiCaptureConfig {
            device_id: device_id_c.as_ptr(),
            mode_id: mode_id.as_ptr(),
            pixel_format: pixel_format.as_ptr(),
            options_json: options_json.as_ptr(),
        };

        let mut device = std::ptr::null_mut();
        let result = unsafe { open_device(device_id_c.as_ptr(), &mut device) };
        self.check_result(result, "frame0_plugin_open_device")?;

        let mut stream = std::ptr::null_mut();
        let start_result = unsafe { start_stream(device, &config, &mut stream) };
        if let Err(error) = self.check_result(start_result, "frame0_plugin_start_stream") {
            let _ = unsafe { close_device(device) };
            return Err(error);
        }

        let mut captures = CapturedPackets::default();
        let user_data = &mut captures as *mut CapturedPackets as *mut c_void;
        let frame_result =
            unsafe { set_frame_callback(stream, Some(capture_frame_callback), user_data) };
        if let Err(error) = self.check_result(frame_result, "frame0_plugin_set_frame_callback") {
            let _ = unsafe { stop_stream(stream) };
            let _ = unsafe { close_device(device) };
            return Err(error);
        }
        let audio_result =
            unsafe { set_audio_callback(stream, Some(capture_audio_callback), user_data) };
        if let Err(error) = self.check_result(audio_result, "frame0_plugin_set_audio_callback") {
            let _ = unsafe { stop_stream(stream) };
            let _ = unsafe { close_device(device) };
            return Err(error);
        }

        let stop_result = unsafe { stop_stream(stream) };
        if let Err(error) = self.check_result(stop_result, "frame0_plugin_stop_stream") {
            let _ = unsafe { close_device(device) };
            return Err(error);
        }
        let close_result = unsafe { close_device(device) };
        self.check_result(close_result, "frame0_plugin_close_device")?;
        Ok(captures)
    }

    pub fn control_json(&self, request: &Value) -> Result<Value> {
        let control = unsafe {
            self.library
                .get::<ControlJsonFn>(b"frame0_plugin_control_json\0")
        }
        .context("missing optional symbol frame0_plugin_control_json")?;
        let free_string = unsafe {
            self.library
                .get::<FreeStringFn>(b"frame0_plugin_free_string\0")
        }
        .context("missing symbol frame0_plugin_free_string")?;
        let request_json = CString::new(serde_json::to_string(request)?)?;
        let mut response_ptr: *const c_char = std::ptr::null();
        let result = unsafe { control(request_json.as_ptr(), &mut response_ptr) };
        self.check_result(result, "frame0_plugin_control_json")?;
        let response_text = c_string(response_ptr)?;
        unsafe {
            free_string(response_ptr);
        }
        serde_json::from_str(&response_text)
            .with_context(|| format!("plugin returned invalid control JSON: {response_text}"))
    }

    fn check_result(&self, result: c_int, operation: &str) -> Result<()> {
        if result == FRAME0_OK {
            Ok(())
        } else {
            Err(anyhow!(
                "{} failed with code {}: {}",
                operation,
                result,
                self.last_error_json()
                    .unwrap_or_else(|_| "{\"error\":\"unavailable\"}".to_string())
            ))
        }
    }

    fn last_error_json(&self) -> Result<String> {
        let last_error = unsafe {
            self.library
                .get::<LastErrorFn>(b"frame0_plugin_last_error_json\0")
        }
        .context("missing symbol frame0_plugin_last_error_json")?;
        let ptr = unsafe { last_error() };
        c_string(ptr)
    }
}

pub fn inspect_manifest(manifest_path: impl AsRef<Path>) -> Result<HostInspectReport> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_plugin_manifest(manifest_path)?.plugin;
    let verification = verify_plugin(&manifest);
    let library_path = resolve_library_path(manifest_path, &manifest)?;
    let host = PluginHost::load(library_path.clone())?;
    let descriptor = host.descriptor()?;
    if descriptor.api_version != FRAME0_PLUGIN_API_VERSION {
        return Err(anyhow!(
            "plugin API version {} does not match host API version {}",
            descriptor.api_version,
            FRAME0_PLUGIN_API_VERSION
        ));
    }
    Ok(HostInspectReport {
        manifest,
        verification,
        library_path,
        descriptor,
    })
}

pub fn smoke_manifest(manifest_path: impl AsRef<Path>) -> Result<HostSmokeReport> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_plugin_manifest(manifest_path)?.plugin;
    let verification = verify_plugin(&manifest);
    let library_path = resolve_library_path(manifest_path, &manifest)?;
    let host = PluginHost::load(library_path.clone())?;
    let descriptor = host.descriptor()?;
    if descriptor.api_version != FRAME0_PLUGIN_API_VERSION {
        return Err(anyhow!(
            "plugin API version {} does not match host API version {}",
            descriptor.api_version,
            FRAME0_PLUGIN_API_VERSION
        ));
    }
    host.initialize()?;
    let devices = host.enumerate_devices()?;
    host.shutdown()?;
    Ok(HostSmokeReport {
        manifest,
        verification,
        library_path,
        descriptor,
        devices,
        initialized: true,
        shutdown: true,
    })
}

pub fn stream_test_manifest(
    manifest_path: impl AsRef<Path>,
    device_id: Option<String>,
) -> Result<HostStreamReport> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_plugin_manifest(manifest_path)?.plugin;
    let library_path = resolve_library_path(manifest_path, &manifest)?;
    let host = PluginHost::load(library_path.clone())?;
    let descriptor = host.descriptor()?;
    host.initialize()?;
    let devices = host.enumerate_devices()?;
    let device_id = device_id
        .or_else(|| {
            devices
                .iter()
                .find(|device| device.capabilities.iter().any(|item| item == "video.input"))
                .map(|device| device.id.clone())
        })
        .or_else(|| devices.first().map(|device| device.id.clone()))
        .ok_or_else(|| anyhow!("plugin reported no devices"))?;
    let captures = host.stream_test(&device_id);
    let shutdown_result = host.shutdown();
    shutdown_result?;
    let captures = captures?;
    Ok(HostStreamReport {
        manifest,
        library_path,
        descriptor,
        device_id,
        opened: true,
        started: true,
        stopped: true,
        closed: true,
        frames: captures.frames,
        audio: captures.audio,
    })
}

pub fn control_manifest(
    manifest_path: impl AsRef<Path>,
    request: Value,
) -> Result<HostControlReport> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_plugin_manifest(manifest_path)?.plugin;
    let library_path = resolve_library_path(manifest_path, &manifest)?;
    let host = PluginHost::load(library_path.clone())?;
    let descriptor = host.descriptor()?;
    host.initialize()?;
    let response = host.control_json(&request);
    let shutdown_result = host.shutdown();
    shutdown_result?;
    let response = response?;
    Ok(HostControlReport {
        manifest,
        library_path,
        descriptor,
        request,
        response,
    })
}

pub fn enumerate_manifest_devices(
    manifest_path: impl AsRef<Path>,
) -> Result<Vec<NativeDeviceDescriptor>> {
    let manifest_path = manifest_path.as_ref();
    let manifest = load_plugin_manifest(manifest_path)?.plugin;
    let library_path = resolve_library_path(manifest_path, &manifest)?;
    let host = PluginHost::load(library_path)?;
    host.initialize()?;
    let devices = host.enumerate_devices();
    let shutdown_result = host.shutdown();
    shutdown_result?;
    devices
}

pub fn resolve_library_path(manifest_path: &Path, manifest: &PluginManifest) -> Result<PathBuf> {
    let entry_key = platform_entry_key();
    let entry = manifest
        .entry
        .get(entry_key)
        .or_else(|| manifest.entry.get("dev"))
        .or_else(|| manifest.entry.values().next())
        .ok_or_else(|| anyhow!("plugin manifest has no entry for platform '{entry_key}'"))?;
    let raw = PathBuf::from(entry);
    let path = if raw.is_absolute() {
        raw
    } else {
        manifest_path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(raw)
    };
    Ok(path)
}

pub fn platform_entry_key() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("macos", "aarch64") => "macos_arm64",
        ("macos", "x86_64") => "macos_x86_64",
        ("linux", "x86_64") => "linux_x86_64",
        ("linux", "aarch64") => "linux_aarch64",
        ("windows", "x86_64") => "windows_x86_64",
        _ => "dev",
    }
}

fn convert_device_descriptor(descriptor: FfiDeviceDescriptor) -> Result<NativeDeviceDescriptor> {
    Ok(NativeDeviceDescriptor {
        id: c_string(descriptor.id)?,
        device_type: c_string(descriptor.device_type)?,
        vendor: c_string(descriptor.vendor)?,
        capabilities: parse_json_c_string(descriptor.capabilities_json)?,
        modes: parse_json_c_string(descriptor.modes_json)?,
        permissions: parse_json_c_string(descriptor.permissions_json)?,
        vendor_properties: parse_json_c_string(descriptor.vendor_properties_json)?,
    })
}

fn parse_json_c_string<T: serde::de::DeserializeOwned>(ptr: *const c_char) -> Result<T> {
    let value = c_string(ptr)?;
    serde_json::from_str(&value).with_context(|| format!("invalid JSON from plugin: {value}"))
}

extern "C" fn capture_frame_callback(packet: *const FfiFramePacket, user_data: *mut c_void) {
    if packet.is_null() || user_data.is_null() {
        return;
    }
    let packet = unsafe { &*packet };
    let captures = unsafe { &mut *(user_data as *mut CapturedPackets) };
    captures.frames.push(NativeFramePacket {
        stream_id: c_string_lossy(packet.stream_id),
        pts_ns: packet.pts_ns,
        duration_ns: packet.duration_ns,
        frame_index: packet.frame_index,
        width: packet.width,
        height: packet.height,
        pixel_format: c_string_lossy(packet.pixel_format),
        color_space: c_string_lossy(packet.color_space),
        storage: parse_json_lossy(packet.storage_json),
    });
}

extern "C" fn capture_audio_callback(packet: *const FfiAudioPacket, user_data: *mut c_void) {
    if packet.is_null() || user_data.is_null() {
        return;
    }
    let packet = unsafe { &*packet };
    let captures = unsafe { &mut *(user_data as *mut CapturedPackets) };
    captures.audio.push(NativeAudioPacket {
        stream_id: c_string_lossy(packet.stream_id),
        pts_ns: packet.pts_ns,
        duration_ns: packet.duration_ns,
        sample_rate: packet.sample_rate,
        channels: packet.channels,
        frames: packet.frames,
        format: c_string_lossy(packet.format),
        storage: parse_json_lossy(packet.storage_json),
    });
}

fn c_string_lossy(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe { CStr::from_ptr(ptr) }.to_string_lossy().to_string()
}

fn parse_json_lossy(ptr: *const c_char) -> Value {
    let value = c_string_lossy(ptr);
    serde_json::from_str(&value).unwrap_or_else(|_| Value::String(value))
}

fn c_string(ptr: *const c_char) -> Result<String> {
    if ptr.is_null() {
        return Err(anyhow!("plugin returned a null string pointer"));
    }
    let string = unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .context("plugin returned non-UTF-8 string")?
        .to_string();
    Ok(string)
}

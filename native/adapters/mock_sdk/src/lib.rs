use std::ffi::{c_char, c_void, CStr};
use std::ptr;

const FRAME0_OK: i32 = 0;
const FRAME0_ERROR_UNSUPPORTED: i32 = 2;
const FRAME0_ERROR_INVALID_ARGUMENT: i32 = 5;
const FRAME0_ERROR_OUT_OF_MEMORY: i32 = 7;

#[repr(C)]
pub struct Frame0PluginDescriptor {
    api_version: u32,
    plugin_id: *const c_char,
    plugin_name: *const c_char,
    plugin_version: *const c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Frame0DeviceDescriptor {
    id: *const c_char,
    device_type: *const c_char,
    vendor: *const c_char,
    capabilities_json: *const c_char,
    modes_json: *const c_char,
    permissions_json: *const c_char,
    vendor_properties_json: *const c_char,
}

#[repr(C)]
pub struct Frame0CaptureConfig {
    device_id: *const c_char,
    mode_id: *const c_char,
    pixel_format: *const c_char,
    options_json: *const c_char,
}

#[repr(C)]
pub struct Frame0FramePacket {
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
pub struct Frame0AudioPacket {
    stream_id: *const c_char,
    pts_ns: u64,
    duration_ns: u64,
    sample_rate: u32,
    channels: u16,
    frames: u32,
    format: *const c_char,
    storage_json: *const c_char,
}

type FrameCallback = extern "C" fn(*const Frame0FramePacket, *mut c_void);
type AudioCallback = extern "C" fn(*const Frame0AudioPacket, *mut c_void);
type EventCallback = extern "C" fn(*const c_char, *mut c_void);

static PLUGIN_ID: &[u8] = b"io.frame0.mock.sdk\0";
static PLUGIN_NAME: &[u8] = b"FRAME0 Mock SDK\0";
static PLUGIN_VERSION: &[u8] = b"0.1.0\0";
static VENDOR: &[u8] = b"frame0.mock\0";

static VIDEO_ID: &[u8] = b"device.video_input.mock.native.0\0";
static VIDEO_TYPE: &[u8] = b"device.video_input\0";
static VIDEO_CAPABILITIES: &[u8] = concat!(
    r#"["video.input","timecode.input","gpu.texture.export"]"#,
    "\0"
)
.as_bytes();
static VIDEO_MODES: &[u8] = concat!(
    r#"[{"id":"1080p60","width":1920,"height":1080,"fps":60.0},{"id":"720p60","width":1280,"height":720,"fps":60.0}]"#,
    "\0"
)
.as_bytes();
static VIDEO_PERMISSIONS: &[u8] = concat!(r#"["camera"]"#, "\0").as_bytes();
static VIDEO_VENDOR_PROPERTIES: &[u8] = concat!(
    r#"{"mock":true,"supports_crash_mode":true,"supports_hot_unplug_mode":true}"#,
    "\0"
)
.as_bytes();

static AUDIO_ID: &[u8] = b"device.audio_input.mock.native.0\0";
static AUDIO_TYPE: &[u8] = b"device.audio_input\0";
static AUDIO_CAPABILITIES: &[u8] = concat!(r#"["audio.input"]"#, "\0").as_bytes();
static AUDIO_MODES: &[u8] = concat!(
    r#"[{"id":"stereo_48k","sample_rate":48000,"channels":2},{"id":"mono_48k","sample_rate":48000,"channels":1}]"#,
    "\0"
)
.as_bytes();
static AUDIO_PERMISSIONS: &[u8] = concat!(r#"["microphone"]"#, "\0").as_bytes();
static AUDIO_VENDOR_PROPERTIES: &[u8] =
    concat!(r#"{"mock":true,"supports_xrun_mode":true}"#, "\0").as_bytes();

static LAST_ERROR_OK: &[u8] = concat!(
    r#"{"error":{"code":"OK","severity":"info","message":"ok","suggestions":[]}}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_INVALID_ARGUMENT: &[u8] = concat!(
    r#"{"error":{"code":"INVALID_ARGUMENT","severity":"error","message":"mock plugin received an invalid argument","suggestions":["Check pointer ownership and nullability before calling the C ABI"]}}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_UNSUPPORTED: &[u8] = concat!(
    r#"{"error":{"code":"UNSUPPORTED","severity":"error","message":"mock plugin does not support this operation for the requested handle","suggestions":["Use a mock video or mock audio device id"]}}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_OUT_OF_MEMORY: &[u8] = concat!(
    r#"{"error":{"code":"OUT_OF_MEMORY","severity":"error","message":"output buffer capacity is too small","suggestions":["Call enumerate with null output to get required count first"]}}"#,
    "\0"
)
.as_bytes();

static FRAME_STREAM_ID: &[u8] = b"stream.mock.video.0\0";
static AUDIO_STREAM_ID: &[u8] = b"stream.mock.audio.0\0";
static PIXEL_FORMAT: &[u8] = b"rgba8unorm\0";
static COLOR_SPACE: &[u8] = b"rec709\0";
static FRAME_STORAGE: &[u8] = concat!(
    r#"{"type":"external_handle","handle":"mock_frame_0"}"#,
    "\0"
)
.as_bytes();
static AUDIO_FORMAT: &[u8] = b"float32\0";
static AUDIO_STORAGE: &[u8] =
    concat!(r#"{"type":"cpu_buffer","size_bytes":4096}"#, "\0").as_bytes();
static EVENT_INITIALIZED: &[u8] = concat!(
    r#"{"event":"plugin.initialized","id":"io.frame0.mock.sdk"}"#,
    "\0"
)
.as_bytes();

#[derive(Debug)]
struct MockDevice {
    id: String,
}

#[derive(Debug)]
struct MockStream {
    device_id: String,
}

static mut LAST_ERROR: *const c_char = LAST_ERROR_OK.as_ptr() as *const c_char;
static mut EVENT_CALLBACK: Option<EventCallback> = None;
static mut EVENT_USER_DATA: *mut c_void = ptr::null_mut();

#[no_mangle]
pub extern "C" fn frame0_plugin_get_descriptor(out_descriptor: *mut Frame0PluginDescriptor) -> i32 {
    if out_descriptor.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        *out_descriptor = Frame0PluginDescriptor {
            api_version: 1,
            plugin_id: PLUGIN_ID.as_ptr() as *const c_char,
            plugin_name: PLUGIN_NAME.as_ptr() as *const c_char,
            plugin_version: PLUGIN_VERSION.as_ptr() as *const c_char,
        };
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_initialize(_context: *mut c_void) -> i32 {
    if std::env::var("FRAME0_MOCK_SDK_CRASH_ON_INIT")
        .ok()
        .as_deref()
        == Some("1")
    {
        std::process::abort();
    }
    unsafe {
        if let Some(callback) = EVENT_CALLBACK {
            callback(EVENT_INITIALIZED.as_ptr() as *const c_char, EVENT_USER_DATA);
        }
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_shutdown() -> i32 {
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_enumerate_devices(
    out_devices: *mut Frame0DeviceDescriptor,
    capacity: u32,
    out_count: *mut u32,
) -> i32 {
    if out_count.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }

    let devices = [video_descriptor(), audio_descriptor()];
    let device_count = devices.len();
    unsafe {
        *out_count = device_count as u32;
    }

    if out_devices.is_null() || capacity == 0 {
        set_last_error(LAST_ERROR_OK);
        return FRAME0_OK;
    }

    let writable = capacity.min(devices.len() as u32) as usize;
    for (index, descriptor) in devices.into_iter().take(writable).enumerate() {
        unsafe {
            *out_devices.add(index) = descriptor;
        }
    }

    if capacity < device_count as u32 {
        set_last_error(LAST_ERROR_OUT_OF_MEMORY);
        FRAME0_ERROR_OUT_OF_MEMORY
    } else {
        set_last_error(LAST_ERROR_OK);
        FRAME0_OK
    }
}

#[no_mangle]
pub extern "C" fn frame0_plugin_open_device(
    device_id: *const c_char,
    out_device: *mut *mut c_void,
) -> i32 {
    if device_id.is_null() || out_device.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    let Ok(id) = unsafe { CStr::from_ptr(device_id) }.to_str() else {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    };
    if !id.contains("mock.native") {
        set_last_error(LAST_ERROR_UNSUPPORTED);
        return FRAME0_ERROR_UNSUPPORTED;
    }
    let handle = Box::new(MockDevice { id: id.to_string() });
    unsafe {
        *out_device = Box::into_raw(handle) as *mut c_void;
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_close_device(device: *mut c_void) -> i32 {
    if device.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        drop(Box::from_raw(device as *mut MockDevice));
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_start_stream(
    device: *mut c_void,
    _config: *const Frame0CaptureConfig,
    out_stream: *mut *mut c_void,
) -> i32 {
    if device.is_null() || out_stream.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    let device = unsafe { &*(device as *mut MockDevice) };
    let stream = Box::new(MockStream {
        device_id: device.id.clone(),
    });
    unsafe {
        *out_stream = Box::into_raw(stream) as *mut c_void;
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_stop_stream(stream: *mut c_void) -> i32 {
    if stream.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        let stream = Box::from_raw(stream as *mut MockStream);
        let _ = stream.device_id.len();
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_set_frame_callback(
    stream: *mut c_void,
    callback: Option<FrameCallback>,
    user_data: *mut c_void,
) -> i32 {
    if stream.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    if let Some(callback) = callback {
        let packet = Frame0FramePacket {
            stream_id: FRAME_STREAM_ID.as_ptr() as *const c_char,
            pts_ns: 0,
            duration_ns: 16_666_667,
            frame_index: 0,
            width: 1920,
            height: 1080,
            pixel_format: PIXEL_FORMAT.as_ptr() as *const c_char,
            color_space: COLOR_SPACE.as_ptr() as *const c_char,
            storage_json: FRAME_STORAGE.as_ptr() as *const c_char,
        };
        callback(&packet, user_data);
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_set_audio_callback(
    stream: *mut c_void,
    callback: Option<AudioCallback>,
    user_data: *mut c_void,
) -> i32 {
    if stream.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    if let Some(callback) = callback {
        let packet = Frame0AudioPacket {
            stream_id: AUDIO_STREAM_ID.as_ptr() as *const c_char,
            pts_ns: 0,
            duration_ns: 21_333_333,
            sample_rate: 48_000,
            channels: 2,
            frames: 1024,
            format: AUDIO_FORMAT.as_ptr() as *const c_char,
            storage_json: AUDIO_STORAGE.as_ptr() as *const c_char,
        };
        callback(&packet, user_data);
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_set_event_callback(
    callback: Option<EventCallback>,
    user_data: *mut c_void,
) -> i32 {
    unsafe {
        EVENT_CALLBACK = callback;
        EVENT_USER_DATA = user_data;
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_last_error_json() -> *const c_char {
    unsafe { LAST_ERROR }
}

#[no_mangle]
pub extern "C" fn frame0_plugin_free_string(_value: *const c_char) {}

fn video_descriptor() -> Frame0DeviceDescriptor {
    Frame0DeviceDescriptor {
        id: VIDEO_ID.as_ptr() as *const c_char,
        device_type: VIDEO_TYPE.as_ptr() as *const c_char,
        vendor: VENDOR.as_ptr() as *const c_char,
        capabilities_json: nul_terminated_json(VIDEO_CAPABILITIES),
        modes_json: nul_terminated_json(VIDEO_MODES),
        permissions_json: nul_terminated_json(VIDEO_PERMISSIONS),
        vendor_properties_json: nul_terminated_json(VIDEO_VENDOR_PROPERTIES),
    }
}

fn audio_descriptor() -> Frame0DeviceDescriptor {
    Frame0DeviceDescriptor {
        id: AUDIO_ID.as_ptr() as *const c_char,
        device_type: AUDIO_TYPE.as_ptr() as *const c_char,
        vendor: VENDOR.as_ptr() as *const c_char,
        capabilities_json: nul_terminated_json(AUDIO_CAPABILITIES),
        modes_json: nul_terminated_json(AUDIO_MODES),
        permissions_json: nul_terminated_json(AUDIO_PERMISSIONS),
        vendor_properties_json: nul_terminated_json(AUDIO_VENDOR_PROPERTIES),
    }
}

fn nul_terminated_json(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

fn set_last_error(message: &'static [u8]) {
    unsafe {
        LAST_ERROR = message.as_ptr() as *const c_char;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptor_exports_plugin_id() {
        let mut descriptor = Frame0PluginDescriptor {
            api_version: 0,
            plugin_id: ptr::null(),
            plugin_name: ptr::null(),
            plugin_version: ptr::null(),
        };
        let result = frame0_plugin_get_descriptor(&mut descriptor);
        assert_eq!(result, FRAME0_OK);
        let id = unsafe { CStr::from_ptr(descriptor.plugin_id) }
            .to_str()
            .unwrap();
        assert_eq!(id, "io.frame0.mock.sdk");
    }

    #[test]
    fn enumerate_reports_count_without_buffer() {
        let mut count = 0;
        let result = frame0_plugin_enumerate_devices(ptr::null_mut(), 0, &mut count);
        assert_eq!(result, FRAME0_OK);
        assert_eq!(count, 2);
    }
}

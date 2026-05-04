use serde_json::{json, Value};
use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr;

const FRAME0_OK: i32 = 0;
const FRAME0_ERROR_UNSUPPORTED: i32 = 2;
const FRAME0_ERROR_INVALID_ARGUMENT: i32 = 5;

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

type EventCallback = extern "C" fn(*const c_char, *mut c_void);

static PLUGIN_ID: &[u8] = b"io.frame0.mock.ml\0";
static PLUGIN_NAME: &[u8] = b"FRAME0 Mock Native ML\0";
static PLUGIN_VERSION: &[u8] = b"0.1.0\0";
static DEVICE_ID: &[u8] = b"device.ml_accelerator.mock.native.0\0";
static DEVICE_TYPE: &[u8] = b"device.ml_accelerator\0";
static VENDOR: &[u8] = b"frame0.mock\0";
static CAPABILITIES: &[u8] = concat!(
    r#"["ml.inference","ml.model.load","ml.tensor","ml.classification","ml.embedding","coreml.model","metal.mps","ane.inference"]"#,
    "\0"
)
.as_bytes();
static MODES: &[u8] = concat!(
    r#"[{"id":"coreml_mock","backend":"coreml","precision":"fp16"},{"id":"mps_mock","backend":"metal.mps","precision":"fp16"},{"id":"cpu_mock","backend":"native_mock","precision":"fp32"}]"#,
    "\0"
)
.as_bytes();
static PERMISSIONS: &[u8] = concat!(r#"["file_read"]"#, "\0").as_bytes();
static VENDOR_PROPERTIES: &[u8] = concat!(
    r#"{"native_targets":["CoreML","MPSGraph","ANE"],"mock":true,"supports_control_json":true}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_OK: &[u8] = concat!(
    r#"{"error":{"code":"OK","severity":"info","message":"ok","suggestions":[]}}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_INVALID_ARGUMENT: &[u8] = concat!(
    r#"{"error":{"code":"INVALID_ARGUMENT","severity":"error","message":"mock ML plugin received an invalid argument","suggestions":["Pass a JSON request with method ml.describe or ml.infer"]}}"#,
    "\0"
)
.as_bytes();
static LAST_ERROR_UNSUPPORTED: &[u8] = concat!(
    r#"{"error":{"code":"UNSUPPORTED","severity":"error","message":"mock ML plugin does not support this operation","suggestions":["Use frame0_plugin_control_json for ML operations"]}}"#,
    "\0"
)
.as_bytes();
static EVENT_INITIALIZED: &[u8] = concat!(
    r#"{"event":"plugin.initialized","id":"io.frame0.mock.ml"}"#,
    "\0"
)
.as_bytes();

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
    unsafe {
        *out_count = 1;
    }
    if out_devices.is_null() || capacity == 0 {
        set_last_error(LAST_ERROR_OK);
        return FRAME0_OK;
    }
    unsafe {
        *out_devices = Frame0DeviceDescriptor {
            id: DEVICE_ID.as_ptr() as *const c_char,
            device_type: DEVICE_TYPE.as_ptr() as *const c_char,
            vendor: VENDOR.as_ptr() as *const c_char,
            capabilities_json: CAPABILITIES.as_ptr() as *const c_char,
            modes_json: MODES.as_ptr() as *const c_char,
            permissions_json: PERMISSIONS.as_ptr() as *const c_char,
            vendor_properties_json: VENDOR_PROPERTIES.as_ptr() as *const c_char,
        };
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_open_device(
    _device_id: *const c_char,
    _out_device: *mut *mut c_void,
) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
}

#[no_mangle]
pub extern "C" fn frame0_plugin_close_device(_device: *mut c_void) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
}

#[no_mangle]
pub extern "C" fn frame0_plugin_start_stream(
    _device: *mut c_void,
    _config: *const Frame0CaptureConfig,
    _out_stream: *mut *mut c_void,
) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
}

#[no_mangle]
pub extern "C" fn frame0_plugin_stop_stream(_stream: *mut c_void) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
}

#[no_mangle]
pub extern "C" fn frame0_plugin_set_frame_callback(
    _stream: *mut c_void,
    _callback: *mut c_void,
    _user_data: *mut c_void,
) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
}

#[no_mangle]
pub extern "C" fn frame0_plugin_set_audio_callback(
    _stream: *mut c_void,
    _callback: *mut c_void,
    _user_data: *mut c_void,
) -> i32 {
    set_last_error(LAST_ERROR_UNSUPPORTED);
    FRAME0_ERROR_UNSUPPORTED
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
pub extern "C" fn frame0_plugin_control_json(
    request_json: *const c_char,
    out_response_json: *mut *const c_char,
) -> i32 {
    if request_json.is_null() || out_response_json.is_null() {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    }
    let Ok(request_text) = (unsafe { CStr::from_ptr(request_json) }).to_str() else {
        set_last_error(LAST_ERROR_INVALID_ARGUMENT);
        return FRAME0_ERROR_INVALID_ARGUMENT;
    };
    let request: Value = match serde_json::from_str(request_text) {
        Ok(value) => value,
        Err(_) => {
            set_last_error(LAST_ERROR_INVALID_ARGUMENT);
            return FRAME0_ERROR_INVALID_ARGUMENT;
        }
    };
    let method = request
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let response = match method {
        "ml.describe" => describe_response(),
        "ml.infer" => infer_response(&request),
        _ => {
            set_last_error(LAST_ERROR_UNSUPPORTED);
            return FRAME0_ERROR_UNSUPPORTED;
        }
    };
    let response = CString::new(response.to_string()).expect("JSON does not contain NUL");
    unsafe {
        *out_response_json = response.into_raw();
    }
    set_last_error(LAST_ERROR_OK);
    FRAME0_OK
}

#[no_mangle]
pub extern "C" fn frame0_plugin_last_error_json() -> *const c_char {
    unsafe { LAST_ERROR }
}

#[no_mangle]
pub extern "C" fn frame0_plugin_free_string(value: *const c_char) {
    if value.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(value as *mut c_char));
    }
}

fn describe_response() -> Value {
    json!({
        "ok": true,
        "plugin": "io.frame0.mock.ml",
        "device": "device.ml_accelerator.mock.native.0",
        "native_targets": ["CoreML", "MPSGraph", "ANE"],
        "models": [
            {
                "id": "mock_classifier",
                "backend": "native_mock",
                "precision": "fp32",
                "inputs": [
                    {
                        "name": "image",
                        "dtype": "f32",
                        "shape": [1, 3, 224, 224],
                        "semantic": "image"
                    }
                ],
                "outputs": [
                    {
                        "name": "classifications",
                        "dtype": "f32",
                        "shape": [1, 3],
                        "semantic": "classification"
                    },
                    {
                        "name": "embedding",
                        "dtype": "f32",
                        "shape": [1, 8],
                        "semantic": "embedding"
                    }
                ],
                "capabilities": ["ml.inference", "ml.classification", "ml.embedding"]
            }
        ]
    })
}

fn infer_response(request: &Value) -> Value {
    let model_id = request
        .pointer("/params/model_id")
        .and_then(Value::as_str)
        .unwrap_or("mock_classifier");
    json!({
        "type": "ml.inference",
        "model_id": model_id,
        "backend": "native_mock",
        "pts_ns": request.pointer("/params/pts_ns").and_then(Value::as_u64).unwrap_or(0),
        "duration_ns": 850_000,
        "input_ref": request.pointer("/params/input_ref").and_then(Value::as_str).unwrap_or("tensor.mock.input"),
        "outputs": {
            "classifications": [
                {"label": "frame0.native.ml.mock", "score": 0.972},
                {"label": "realtime.media", "score": 0.021},
                {"label": "background", "score": 0.007}
            ],
            "embedding": [0.125, -0.25, 0.5, 0.75, -0.125, 0.0, 0.375, 0.625]
        },
        "diagnostics": []
    })
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
    fn describe_control_returns_model() {
        let request = CString::new(r#"{"method":"ml.describe"}"#).unwrap();
        let mut response = ptr::null();
        let result = frame0_plugin_control_json(request.as_ptr(), &mut response);
        assert_eq!(result, FRAME0_OK);
        let response_text = unsafe { CStr::from_ptr(response) }.to_str().unwrap();
        assert!(response_text.contains("mock_classifier"));
        frame0_plugin_free_string(response);
    }
}

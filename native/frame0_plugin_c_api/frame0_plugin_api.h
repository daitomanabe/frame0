#ifndef FRAME0_PLUGIN_API_H
#define FRAME0_PLUGIN_API_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32)
#define FRAME0_EXPORT __declspec(dllexport)
#else
#define FRAME0_EXPORT __attribute__((visibility("default")))
#endif

#define FRAME0_PLUGIN_API_VERSION 1u

typedef struct frame0_plugin_context frame0_plugin_context;
typedef struct frame0_device_handle frame0_device_handle;
typedef struct frame0_stream_handle frame0_stream_handle;

typedef enum frame0_result {
    FRAME0_OK = 0,
    FRAME0_ERROR_UNKNOWN = 1,
    FRAME0_ERROR_UNSUPPORTED = 2,
    FRAME0_ERROR_DEVICE_BUSY = 3,
    FRAME0_ERROR_PERMISSION_DENIED = 4,
    FRAME0_ERROR_INVALID_ARGUMENT = 5,
    FRAME0_ERROR_API_VERSION_MISMATCH = 6,
    FRAME0_ERROR_OUT_OF_MEMORY = 7,
    FRAME0_ERROR_TRANSPORT_FAILED = 8
} frame0_result;

typedef struct frame0_plugin_descriptor {
    uint32_t api_version;
    const char* plugin_id;
    const char* plugin_name;
    const char* plugin_version;
} frame0_plugin_descriptor;

typedef struct frame0_device_descriptor {
    const char* id;
    const char* type;
    const char* vendor;
    const char* capabilities_json;
    const char* modes_json;
    const char* permissions_json;
    const char* vendor_properties_json;
} frame0_device_descriptor;

typedef struct frame0_capture_config {
    const char* device_id;
    const char* mode_id;
    const char* pixel_format;
    const char* options_json;
} frame0_capture_config;

typedef struct frame0_frame_packet {
    const char* stream_id;
    uint64_t pts_ns;
    uint64_t duration_ns;
    uint64_t frame_index;
    uint32_t width;
    uint32_t height;
    const char* pixel_format;
    const char* color_space;
    const char* storage_json;
} frame0_frame_packet;

typedef struct frame0_audio_packet {
    const char* stream_id;
    uint64_t pts_ns;
    uint64_t duration_ns;
    uint32_t sample_rate;
    uint16_t channels;
    uint32_t frames;
    const char* format;
    const char* storage_json;
} frame0_audio_packet;

typedef void (*frame0_frame_callback)(
    const frame0_frame_packet* packet,
    void* user_data
);

typedef void (*frame0_audio_callback)(
    const frame0_audio_packet* packet,
    void* user_data
);

typedef void (*frame0_event_callback)(
    const char* event_json,
    void* user_data
);

FRAME0_EXPORT frame0_result frame0_plugin_get_descriptor(
    frame0_plugin_descriptor* out_descriptor
);

FRAME0_EXPORT frame0_result frame0_plugin_initialize(
    frame0_plugin_context* context
);

FRAME0_EXPORT frame0_result frame0_plugin_shutdown(void);

FRAME0_EXPORT frame0_result frame0_plugin_enumerate_devices(
    frame0_device_descriptor* out_devices,
    uint32_t capacity,
    uint32_t* out_count
);

FRAME0_EXPORT frame0_result frame0_plugin_open_device(
    const char* device_id,
    frame0_device_handle** out_device
);

FRAME0_EXPORT frame0_result frame0_plugin_close_device(
    frame0_device_handle* device
);

FRAME0_EXPORT frame0_result frame0_plugin_start_stream(
    frame0_device_handle* device,
    const frame0_capture_config* config,
    frame0_stream_handle** out_stream
);

FRAME0_EXPORT frame0_result frame0_plugin_stop_stream(
    frame0_stream_handle* stream
);

FRAME0_EXPORT frame0_result frame0_plugin_set_frame_callback(
    frame0_stream_handle* stream,
    frame0_frame_callback callback,
    void* user_data
);

FRAME0_EXPORT frame0_result frame0_plugin_set_audio_callback(
    frame0_stream_handle* stream,
    frame0_audio_callback callback,
    void* user_data
);

FRAME0_EXPORT frame0_result frame0_plugin_set_event_callback(
    frame0_event_callback callback,
    void* user_data
);

FRAME0_EXPORT frame0_result frame0_plugin_control_json(
    const char* request_json,
    const char** out_response_json
);

FRAME0_EXPORT const char* frame0_plugin_last_error_json(void);

FRAME0_EXPORT void frame0_plugin_free_string(const char* value);

#ifdef __cplusplus
}
#endif

#endif

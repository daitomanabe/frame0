#ifndef FRAME0_EXTERNAL_API_H
#define FRAME0_EXTERNAL_API_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#if defined(_WIN32)
#define FRAME0_EXTERNAL_EXPORT __declspec(dllexport)
#else
#define FRAME0_EXTERNAL_EXPORT __attribute__((visibility("default")))
#endif

#define FRAME0_EXTERNAL_API_VERSION 1u

typedef struct frame0_external_context frame0_external_context;
typedef struct frame0_external_node frame0_external_node;

typedef enum frame0_external_result {
    FRAME0_EXTERNAL_OK = 0,
    FRAME0_EXTERNAL_ERROR_UNKNOWN = 1,
    FRAME0_EXTERNAL_ERROR_UNSUPPORTED = 2,
    FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT = 3,
    FRAME0_EXTERNAL_ERROR_API_VERSION_MISMATCH = 4,
    FRAME0_EXTERNAL_ERROR_OUT_OF_MEMORY = 5,
    FRAME0_EXTERNAL_ERROR_PROCESS_FAILED = 6
} frame0_external_result;

typedef struct frame0_external_descriptor {
    uint32_t api_version;
    const char* external_id;
    const char* external_name;
    const char* external_version;
    const char* capabilities_json;
} frame0_external_descriptor;

typedef struct frame0_external_port {
    const char* name;
    const char* resource_type;
    const char* schema_json;
} frame0_external_port;

typedef struct frame0_external_node_config {
    const char* node_id;
    const char* params_json;
    const char* inputs_json;
} frame0_external_node_config;

typedef struct frame0_external_packet {
    const char* resource_type;
    const char* resource_id;
    uint64_t pts_ns;
    uint64_t duration_ns;
    const void* data;
    size_t data_size;
    const char* metadata_json;
} frame0_external_packet;

typedef void (*frame0_external_emit_callback)(
    const char* output_name,
    const frame0_external_packet* packet,
    void* user_data
);

typedef void (*frame0_external_event_callback)(
    const char* event_json,
    void* user_data
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_get_descriptor(
    frame0_external_descriptor* out_descriptor
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_initialize(
    frame0_external_context* context
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_shutdown(void);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_describe_ports(
    frame0_external_port* out_inputs,
    uint32_t input_capacity,
    uint32_t* out_input_count,
    frame0_external_port* out_outputs,
    uint32_t output_capacity,
    uint32_t* out_output_count
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_create_node(
    const frame0_external_node_config* config,
    frame0_external_node** out_node
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_destroy_node(
    frame0_external_node* node
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_set_emit_callback(
    frame0_external_node* node,
    frame0_external_emit_callback callback,
    void* user_data
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_set_event_callback(
    frame0_external_event_callback callback,
    void* user_data
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_process(
    frame0_external_node* node,
    const frame0_external_packet* input_packets,
    uint32_t input_count
);

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_control_json(
    frame0_external_node* node,
    const char* request_json,
    const char** out_response_json
);

FRAME0_EXTERNAL_EXPORT const char* frame0_external_last_error_json(void);

FRAME0_EXTERNAL_EXPORT void frame0_external_free_string(const char* value);

#ifdef __cplusplus
}
#endif

#endif

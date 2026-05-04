#include "frame0_example_external.h"

#include <stdlib.h>
#include <string.h>

struct frame0_external_context {
    int reserved;
};

struct frame0_external_node {
    char node_id[64];
    float gain;
    float bias;
    frame0_external_emit_callback emit_callback;
    void* emit_user_data;
};

static const char* g_last_error = "{\"error\":\"none\"}";
static frame0_external_event_callback g_event_callback = 0;
static void* g_event_user_data = 0;

static char* frame0_example_strdup(const char* value) {
    size_t size = strlen(value) + 1;
    char* copy = (char*)malloc(size);
    if (!copy) {
        return 0;
    }
    memcpy(copy, value, size);
    return copy;
}

float frame0_example_external_process_value(float input, float gain, float bias) {
    float value = input * gain + bias;
    if (value < 0.0f) {
        return 0.0f;
    }
    if (value > 1.0f) {
        return 1.0f;
    }
    return value;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_get_descriptor(
    frame0_external_descriptor* out_descriptor
) {
    if (!out_descriptor) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    out_descriptor->api_version = FRAME0_EXTERNAL_API_VERSION;
    out_descriptor->external_id = "org.example.frame0.c_external";
    out_descriptor->external_name = "FRAME0 C External Template";
    out_descriptor->external_version = "0.1.0";
    out_descriptor->capabilities_json = "[\"external.node\",\"parameter.processor\"]";
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_initialize(
    frame0_external_context* context
) {
    (void)context;
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_shutdown(void) {
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_describe_ports(
    frame0_external_port* out_inputs,
    uint32_t input_capacity,
    uint32_t* out_input_count,
    frame0_external_port* out_outputs,
    uint32_t output_capacity,
    uint32_t* out_output_count
) {
    if (!out_input_count || !out_output_count) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    *out_input_count = 1;
    *out_output_count = 1;
    if (out_inputs && input_capacity >= 1) {
        out_inputs[0].name = "input";
        out_inputs[0].resource_type = "parameter";
        out_inputs[0].schema_json = "{\"type\":\"number\"}";
    }
    if (out_outputs && output_capacity >= 1) {
        out_outputs[0].name = "output";
        out_outputs[0].resource_type = "parameter";
        out_outputs[0].schema_json = "{\"type\":\"number\"}";
    }
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_create_node(
    const frame0_external_node_config* config,
    frame0_external_node** out_node
) {
    if (!config || !out_node) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    frame0_external_node* node = (frame0_external_node*)calloc(1, sizeof(frame0_external_node));
    if (!node) {
        return FRAME0_EXTERNAL_ERROR_OUT_OF_MEMORY;
    }
    strncpy(node->node_id, config->node_id ? config->node_id : "node", sizeof(node->node_id) - 1);
    node->gain = 1.0f;
    node->bias = 0.0f;
    *out_node = node;
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_destroy_node(
    frame0_external_node* node
) {
    free(node);
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_set_emit_callback(
    frame0_external_node* node,
    frame0_external_emit_callback callback,
    void* user_data
) {
    if (!node) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    node->emit_callback = callback;
    node->emit_user_data = user_data;
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_set_event_callback(
    frame0_external_event_callback callback,
    void* user_data
) {
    g_event_callback = callback;
    g_event_user_data = user_data;
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_process(
    frame0_external_node* node,
    const frame0_external_packet* input_packets,
    uint32_t input_count
) {
    if (!node) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    (void)input_packets;
    (void)input_count;
    if (node->emit_callback) {
        const float value = frame0_example_external_process_value(0.5f, node->gain, node->bias);
        frame0_external_packet packet;
        memset(&packet, 0, sizeof(packet));
        packet.resource_type = "parameter";
        packet.resource_id = node->node_id;
        packet.data = &value;
        packet.data_size = sizeof(value);
        packet.metadata_json = "{\"encoding\":\"f32\"}";
        node->emit_callback("output", &packet, node->emit_user_data);
    }
    if (g_event_callback) {
        g_event_callback("{\"event\":\"external.processed\"}", g_event_user_data);
    }
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_control_json(
    frame0_external_node* node,
    const char* request_json,
    const char** out_response_json
) {
    (void)node;
    (void)request_json;
    if (!out_response_json) {
        return FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT;
    }
    *out_response_json = frame0_example_strdup("{\"ok\":true}");
    return *out_response_json ? FRAME0_EXTERNAL_OK : FRAME0_EXTERNAL_ERROR_OUT_OF_MEMORY;
}

FRAME0_EXTERNAL_EXPORT const char* frame0_external_last_error_json(void) {
    return g_last_error;
}

FRAME0_EXTERNAL_EXPORT void frame0_external_free_string(const char* value) {
    free((void*)value);
}

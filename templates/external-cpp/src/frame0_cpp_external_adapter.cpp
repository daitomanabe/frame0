#include "frame0_cpp_external_adapter.hpp"
#include "frame0_external_api.h"

#include <cstdlib>
#include <cstring>
#include <memory>
#include <new>
#include <string>

struct frame0_external_context {
    int reserved;
};

struct frame0_external_node {
    std::unique_ptr<frame0::external::GainBiasExternal> impl;
    frame0_external_emit_callback emit_callback = nullptr;
    void* emit_user_data = nullptr;
};

namespace {

std::string g_last_error = "{\"error\":\"none\"}";
frame0_external_event_callback g_event_callback = nullptr;
void* g_event_user_data = nullptr;

char* copy_string(const std::string& value) {
    char* out = static_cast<char*>(std::malloc(value.size() + 1));
    if (!out) {
        return nullptr;
    }
    std::memcpy(out, value.c_str(), value.size() + 1);
    return out;
}

frame0_external_result set_error(const char* message, frame0_external_result result) {
    g_last_error = std::string("{\"error\":\"") + message + "\"}";
    return result;
}

} // namespace

extern "C" {

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_get_descriptor(
    frame0_external_descriptor* out_descriptor
) {
    if (!out_descriptor) {
        return set_error("descriptor output is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
    }
    out_descriptor->api_version = FRAME0_EXTERNAL_API_VERSION;
    out_descriptor->external_id = "org.example.frame0.cpp_external";
    out_descriptor->external_name = "FRAME0 C++ External Template";
    out_descriptor->external_version = "0.1.0";
    out_descriptor->capabilities_json = "[\"external.node\",\"cxx.adapter\",\"parameter.processor\"]";
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
        return set_error("port count output is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
    }
    *out_input_count = 1;
    *out_output_count = 1;
    if (out_inputs && input_capacity > 0) {
        out_inputs[0] = {"input", "parameter", "{\"type\":\"number\"}"};
    }
    if (out_outputs && output_capacity > 0) {
        out_outputs[0] = {"output", "parameter", "{\"type\":\"number\"}"};
    }
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_create_node(
    const frame0_external_node_config* config,
    frame0_external_node** out_node
) {
    if (!config || !out_node) {
        return set_error("node config or output is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
    }
    try {
        frame0::external::NodeConfig cpp_config;
        cpp_config.node_id = config->node_id ? config->node_id : "cpp_external_node";
        cpp_config.params_json = config->params_json ? config->params_json : "{}";
        auto* node = new frame0_external_node;
        node->impl = std::make_unique<frame0::external::GainBiasExternal>(std::move(cpp_config));
        *out_node = node;
        return FRAME0_EXTERNAL_OK;
    } catch (const std::bad_alloc&) {
        return set_error("allocation failed", FRAME0_EXTERNAL_ERROR_OUT_OF_MEMORY);
    } catch (...) {
        return set_error("unknown C++ exception", FRAME0_EXTERNAL_ERROR_UNKNOWN);
    }
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_destroy_node(
    frame0_external_node* node
) {
    delete node;
    return FRAME0_EXTERNAL_OK;
}

FRAME0_EXTERNAL_EXPORT frame0_external_result frame0_external_set_emit_callback(
    frame0_external_node* node,
    frame0_external_emit_callback callback,
    void* user_data
) {
    if (!node) {
        return set_error("node is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
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
    if (!node || !node->impl) {
        return set_error("node is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
    }
    (void)input_packets;
    (void)input_count;
    if (node->emit_callback) {
        const float value = node->impl->process_value(0.5f);
        const std::string metadata = node->impl->metadata_json();
        frame0_external_packet packet{};
        packet.resource_type = "parameter";
        packet.resource_id = node->impl->node_id().c_str();
        packet.data = &value;
        packet.data_size = sizeof(value);
        packet.metadata_json = metadata.c_str();
        node->emit_callback("output", &packet, node->emit_user_data);
    }
    if (g_event_callback) {
        g_event_callback("{\"event\":\"cpp_external.processed\"}", g_event_user_data);
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
        return set_error("response output is null", FRAME0_EXTERNAL_ERROR_INVALID_ARGUMENT);
    }
    *out_response_json = copy_string("{\"ok\":true,\"language\":\"c++\"}");
    return *out_response_json ? FRAME0_EXTERNAL_OK : FRAME0_EXTERNAL_ERROR_OUT_OF_MEMORY;
}

FRAME0_EXTERNAL_EXPORT const char* frame0_external_last_error_json(void) {
    return g_last_error.c_str();
}

FRAME0_EXTERNAL_EXPORT void frame0_external_free_string(const char* value) {
    std::free(const_cast<char*>(value));
}

}

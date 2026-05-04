#ifndef FRAME0_CPP_EXTERNAL_ADAPTER_HPP
#define FRAME0_CPP_EXTERNAL_ADAPTER_HPP

#include <algorithm>
#include <cstdint>
#include <string>
#include <vector>

namespace frame0::external {

struct PacketView {
    std::string resource_type;
    std::string resource_id;
    std::uint64_t pts_ns = 0;
    std::uint64_t duration_ns = 0;
    const void* data = nullptr;
    std::size_t data_size = 0;
};

struct NodeConfig {
    std::string node_id;
    std::string params_json;
};

class GainBiasExternal {
public:
    explicit GainBiasExternal(NodeConfig config)
        : node_id_(std::move(config.node_id)) {}

    float process_value(float input) const {
        return std::clamp(input * gain_ + bias_, 0.0f, 1.0f);
    }

    std::string metadata_json() const {
        return "{\"implementation\":\"cxx17\",\"encoding\":\"f32\"}";
    }

    const std::string& node_id() const {
        return node_id_;
    }

private:
    std::string node_id_;
    float gain_ = 1.0f;
    float bias_ = 0.0f;
};

} // namespace frame0::external

#endif

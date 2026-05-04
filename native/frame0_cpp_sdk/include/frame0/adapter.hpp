#pragma once

#include <cstdint>
#include <functional>
#include <memory>
#include <string>
#include <vector>

namespace frame0 {

enum class ResultCode {
    Ok,
    Unknown,
    Unsupported,
    DeviceBusy,
    PermissionDenied,
    InvalidArgument,
    TransportFailed,
};

struct Result {
    ResultCode code = ResultCode::Ok;
    std::string message;
    std::string error_json;

    explicit operator bool() const { return code == ResultCode::Ok; }
};

struct DeviceDescriptor {
    std::string id;
    std::string type;
    std::string vendor;
    std::vector<std::string> capabilities;
    std::string modes_json;
    std::string permissions_json;
    std::string vendor_properties_json;
};

struct CaptureConfig {
    std::string device_id;
    std::string mode_id;
    std::string pixel_format;
    std::string options_json;
};

struct FramePacket {
    std::string stream_id;
    std::uint64_t pts_ns = 0;
    std::uint64_t duration_ns = 0;
    std::uint64_t frame_index = 0;
    std::uint32_t width = 0;
    std::uint32_t height = 0;
    std::string pixel_format;
    std::string color_space;
    std::string storage_json;
};

struct AudioPacket {
    std::string stream_id;
    std::uint64_t pts_ns = 0;
    std::uint64_t duration_ns = 0;
    std::uint32_t sample_rate = 0;
    std::uint16_t channels = 0;
    std::uint32_t frames = 0;
    std::string format;
    std::string storage_json;
};

using FrameCallback = std::function<void(const FramePacket&)>;
using AudioCallback = std::function<void(const AudioPacket&)>;
using EventCallback = std::function<void(const std::string& event_json)>;

class VideoInputAdapter {
public:
    virtual ~VideoInputAdapter() = default;
    virtual Result open(const std::string& device_id) = 0;
    virtual Result start(const CaptureConfig& config) = 0;
    virtual Result stop() = 0;
    virtual Result close() = 0;
    virtual void set_frame_callback(FrameCallback callback) = 0;
};

class AudioInputAdapter {
public:
    virtual ~AudioInputAdapter() = default;
    virtual Result open(const std::string& device_id) = 0;
    virtual Result start(const CaptureConfig& config) = 0;
    virtual Result stop() = 0;
    virtual Result close() = 0;
    virtual void set_audio_callback(AudioCallback callback) = 0;
};

class NativeSdkAdapter {
public:
    virtual ~NativeSdkAdapter() = default;
    virtual Result initialize() = 0;
    virtual Result shutdown() = 0;
    virtual std::vector<DeviceDescriptor> enumerate_devices() = 0;
    virtual Result set_event_callback(EventCallback callback) = 0;
};

// The bridge owns exception boundaries. Implementations should translate all
// vendor exceptions and error codes into Result before crossing the C ABI.
class AdapterBridge {
public:
    explicit AdapterBridge(std::unique_ptr<NativeSdkAdapter> adapter)
        : adapter_(std::move(adapter)) {}

    NativeSdkAdapter& adapter() { return *adapter_; }
    const NativeSdkAdapter& adapter() const { return *adapter_; }

private:
    std::unique_ptr<NativeSdkAdapter> adapter_;
};

} // namespace frame0


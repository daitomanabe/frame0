#include "frame0_example_external.h"

#include <algorithm>

namespace frame0_example {
class GainBiasProcessor {
public:
    float process(float input, float gain, float bias) const {
        return std::clamp(input * gain + bias, 0.0f, 1.0f);
    }
};
}

extern "C" float frame0_example_external_cpp_process_value(float input, float gain, float bias) {
    const frame0_example::GainBiasProcessor processor;
    return processor.process(input, gain, bias);
}

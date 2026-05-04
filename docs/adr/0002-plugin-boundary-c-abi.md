# ADR 0002: Plugin Boundary Uses C ABI

Status: accepted

FRAME0 native plugins expose `native/frame0_plugin_c_api/frame0_plugin_api.h` as their stable boundary.

The runtime does not expose C++ ABI, Rust ABI, STL types, exceptions, RTTI, allocator ownership, or vendor SDK types across the plugin boundary.

Consequences:

- C++ SDK adapters must translate vendor types into JSON metadata plus `FramePacket` and `AudioPacket` structs.
- Memory ownership is explicit. Strings returned by plugins must be released through `frame0_plugin_free_string`.
- Exceptions must be caught before returning from exported functions.


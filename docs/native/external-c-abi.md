# Native External C ABI

Native externals are third-party processing nodes that can be loaded by FRAME0 without linking vendor SDK handles into Core. They are different from device plugins:

- Plugins enumerate devices and open streams.
- Externals process packets and emit named node outputs.

The public external skeleton is:

```text
native/frame0_external_c_api/frame0_external_api.h
```

## Required Exports

Every external should export:

- `frame0_external_get_descriptor`
- `frame0_external_initialize`
- `frame0_external_shutdown`
- `frame0_external_describe_ports`
- `frame0_external_create_node`
- `frame0_external_destroy_node`
- `frame0_external_process`
- `frame0_external_last_error_json`
- `frame0_external_free_string`

Optional control is handled through `frame0_external_control_json`. Externals can emit frame, audio, event, channel, table, mesh, texture, point, text, or custom packet resources by calling the emit callback configured with `frame0_external_set_emit_callback`.

## Boundary Rules

- Exported functions must use C ABI and must not throw exceptions.
- All returned strings must be freed with `frame0_external_free_string`.
- Packet data is borrowed for the duration of the call unless metadata says otherwise.
- Node params and control requests are JSON strings so schemas can evolve without changing the ABI.
- Vendor SDK objects stay behind the external boundary.

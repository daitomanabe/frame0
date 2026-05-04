# ADR 0003: Plugin Isolation

Status: accepted

Native plugins run out of process by default. The runtime sees plugin output as resources and event packets, not as in-process object references.

Rationale:

- Vendor SDKs, callbacks, and device drivers can crash or deadlock.
- A plugin crash must produce a structured event and failed resource state without taking down the runtime.
- Process ownership makes permissions and restart policy inspectable.

The v0.1 scaffold models this in manifests and resource ownership. The later native host should implement the process supervisor and IPC transport.


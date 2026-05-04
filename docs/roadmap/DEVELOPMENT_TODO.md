# FRAME0 Development TODO

This checklist is generated from the current API documentation pass. Each item
should be implemented as a small, reviewable change with tests and a push.

## Documentation And Tooling

- [x] Add public API documentation for CLI, schemas, resources, plugins, native ABI, ML, and addons.
- [x] Add a user manual that explains the current development workflow.
- [x] Add a machine-readable documentation index exposed by the CLI.
- [x] Add an example verification script that checks every `examples/*/scene.yaml`.
- [x] Add API compatibility notes for future breaking schema changes.

## Runtime And IPC

- [ ] Add structured IPC between the runtime and plugin host.
- [ ] Forward stream packets from plugin host smoke tests into runtime events/resources.
- [ ] Add runtime resource mutation transport for `frame0 resource set`.
- [ ] Persist structured logs for `frame0 logs`.

## Rendering

- [ ] Add a real Metal command queue behind `frame0_render`.
- [ ] Add shader compilation diagnostics and source mapping.
- [ ] Add texture pool lifecycle metrics.
- [ ] Add GPU timing packets to benchmark output.

## Audio

- [ ] Add adapter-backed CoreAudio input.
- [ ] Add real FFT/analyzer implementation.
- [ ] Add audio clock synchronization tests.
- [ ] Add audio output device selection.

## Machine Learning

- [ ] Add Core ML model loading behind the native ML adapter.
- [ ] Add MPSGraph/ANE execution path behind the same inference packet contract.
- [ ] Add multimodal batching and timestamp alignment tests.
- [ ] Add model asset discovery and checksum validation.

## Apple Native Extensions

- [ ] Add signed Core Media I/O Camera Extension scaffold.
- [ ] Add signed AUv3 extension scaffold.
- [ ] Add ScreenCaptureKit capture adapter.
- [ ] Add Vision request examples that emit overlay and metadata packets.

## Third-Party Addons

- [ ] Add addon package generator command.
- [ ] Add addon registry validation to CI.
- [ ] Add external C/C++ skeleton smoke tests.
- [ ] Add sample addon release checklist.

# Creative Tool Concept Map

FRAME0 borrows durable ideas from established creative coding tools while preserving its own runtime boundaries. The goal is not to clone GUI behavior. The goal is to make familiar creative-coding concepts inspectable, testable, serializable, and controllable through CLI, JSON, schemas, and native adapters.

## Core Translation Rule

| Existing Tool Habit | FRAME0 Equivalent | Primary Contract |
| --- | --- | --- |
| Write code in an app lifecycle | Declare a scene graph and run it with an explicit clock | `scene.yaml`, `runtime_snapshot` |
| Draw immediately to the window | Produce a named texture/mesh/event resource | `visual_node`, `frame_packet` |
| Keep state in global variables | Keep state in graph nodes, packets, parameters, or replay logs | `parameter`, `event_packet`, `timeline` |
| Use host GUI panels | Use schema-backed manifests and CLI inspection first | `schema export`, `inspect`, `graph` |
| Patch third-party libraries directly | Wrap them as addons, plugins, or native externals | `plugin`, `addon`, C ABI |

## Processing

| Processing Concept | FRAME0 Runtime Form | Example |
| --- | --- | --- |
| `setup()` | Static scene manifest plus resource declarations | `examples/creative_primitives/scene.yaml` |
| `draw()` | Clocked graph execution that emits frames/events | `frame0 run --events ndjson` |
| `size()` / surface setup | Output resource and render target parameters | `outputs.preview` |
| `line()`, `rect()`, `ellipse()` | `frame0_creative` primitives and draw-node manifests | `creative_primitives` |
| `pushMatrix()` / `popMatrix()` | Explicit transform stacks or node params | `transform` params |
| `noise()` / easing | Deterministic helpers in `frame0_creative` | `crates/frame0_creative` |
| Mouse/key callbacks | Normalized input event streams | `examples/input_events` |

## openFrameworks

| openFrameworks Concept | FRAME0 Runtime Form | Example |
| --- | --- | --- |
| `ofApp::setup/update/draw` | Scene load, clock step, graph execution, event output | CLI `run` and `runtime_snapshot` |
| `ofParameter` / `ofxGui` | Schema-backed parameters, presets, automation, smoothing | `parameter_automation` |
| `ofFbo` / texture feedback | Texture resource plus feedback visual node | `visual_nodes` |
| `ofMesh` / `ofVboMesh` | Mesh resources and instancing nodes | `cinder_geometry`, `visual_nodes` |
| `ofSoundBuffer` / FFT | Audio packet/resource plus analysis stream | `audio_reactive`, `media_utilities` |
| `ofxOsc`, MIDI, serial, HID | Input resource contracts and event normalization | `input_events` |
| `ofx` addons | FRAME0 addon manifests with capabilities, permissions, examples, and tests | `plugins/`, future `addons/` |

## Cinder

| Cinder Concept | FRAME0 Runtime Form | Example |
| --- | --- | --- |
| `ci::app::App` lifecycle | Runtime supervisor and explicit timebase | `frame0_core`, `frame0_time` |
| `CameraPersp` | Camera node/resource manifest | `cinder_geometry`, `visual_nodes` |
| `geom::Source` / `TriMesh` | Geometry resources and mesh contracts | `frame0_creative::Mesh` |
| `gl::Batch` / `gl::GlslProg` | Render shader/material node with named inputs/outputs | `cinder_geometry` |
| `params::InterfaceGl` | Parameters, presets, automation, and CLI inspection | `parameter_automation` |
| Timeline/keyframes | Timeline schema with clips, cues, and replay | `timeline_sequencing` |

## TouchDesigner

| TouchDesigner Concept | FRAME0 Runtime Form | Example |
| --- | --- | --- |
| TOP | Texture-producing nodes and frame packets | `operator_network`, `visual_nodes` |
| CHOP | Channel streams, automation inputs, beat/control tracks | `operator_network`, `parameter_automation` |
| DAT | Structured table resources and JSON/CSV adapters | `operator_network` |
| SOP | Geometry/mesh resources | `cinder_geometry`, `visual_nodes` |
| COMP | Scene/subgraph manifests and plugin namespaces | `operator_network` |
| Parameters and exports | Schema-backed params with explicit targets | `automation`, `timeline` |
| Movie File In / Audio File In | Media asset contracts | `media_utilities` |
| Record / Perform mode | Capture/playback manifests and deterministic replay | `media_utilities`, `timeline_sequencing` |

## Migration Checklist

1. Identify each external IO dependency as a resource: camera, audio, OSC, MIDI, serial, HID, file, network, ML model, or OS extension.
2. Convert frame-dependent logic into explicit clock policy: manual, monotonic, fixed-step, media PTS, timeline, or external sync.
3. Convert mutable app state into parameters, timeline tracks, event state, or captured replay logs.
4. Convert immediate drawing calls into nodes that produce named texture, mesh, text, point, channel, or event outputs.
5. Put third-party SDK code behind an addon/plugin/external boundary instead of linking it into core runtime crates.
6. Add one CLI-verifiable example per capability: `inspect`, `graph` when acyclic, `schema export`, and plugin/addon verification.

## Boundaries

- FRAME0 Core should not import vendor SDK headers or framework-specific handles.
- Scene manifests describe runtime intent; native plugins/adapters implement transport.
- GUI panels are optional clients. The source of truth is schema-backed data.
- Every non-trivial feature should have a public example and a command that verifies it.

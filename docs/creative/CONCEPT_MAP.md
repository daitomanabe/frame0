# Creative Tool Concept Map

FRAME0 borrows durable ideas from established creative coding tools while preserving its own runtime boundaries.

| Existing Tool Concept | FRAME0 Runtime Form |
| --- | --- |
| Processing `setup()` / `draw()` | Scene manifest plus runtime event stream |
| Processing drawing primitives | `frame0_creative` primitives and render node manifests |
| openFrameworks addons | FRAME0 addon manifests with capabilities and permissions |
| openFrameworks app lifecycle | Runtime supervisor, resources, events, and timebase |
| Cinder geometry/camera/material | Geometry resources, camera manifests, material/shader parameters |
| Cinder params | Parameter resources, presets, smoothing, and automation |
| TouchDesigner operators | Typed graph nodes with inputs, outputs, parameters, channels, and textures |
| TouchDesigner CHOP channels | Channel streams and parameter automation inputs |
| TouchDesigner TOP textures | Texture resources and render/media node outputs |
| TouchDesigner DAT tables | Structured table resources and JSON/CSV adapters |
| TouchDesigner COMP containers | Scene/subgraph manifests and plugin namespaces |

FRAME0 should not duplicate GUI behavior first. The priority is to make these concepts inspectable, testable, serializable, and controllable through CLI and JSON.


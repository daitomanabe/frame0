# FRAME0 Creative Parity Checklist

This checklist tracks FRAME0 v0.2 work inspired by openFrameworks, Processing, Cinder, and TouchDesigner. Each item is intentionally framed as a FRAME0 runtime capability, not as a GUI-first clone of those tools.

Loop rule for this checklist:

1. Pick the first unchecked item.
2. Implement the smallest useful runtime/schema/example surface for that item.
3. Mark only that item complete.
4. Run format, tests, copyright scan, and privacy scan.
5. Commit and push.
6. Repeat until every item is checked.

## Creative Runtime Foundation

- [x] Creative primitives crate: vectors, colors, easing, noise, rects, polylines, meshes, and transforms.
- [x] Processing/openFrameworks-style scene examples for draw primitives, color, transforms, and animation.
- [x] Cinder-style geometry and camera/material manifest examples.
- [x] TouchDesigner-style operator network schema, parameters, channels, and table/texture conventions.
- [x] Parameter automation: LFO, envelope, ramp, smoothing, mapping, and presets.
- [x] Input and event resources: mouse, keyboard, MIDI, OSC, HID, serial, and multitouch contracts.
- [x] Timeline and sequencing: clips, cues, keyframes, transport, beats, and deterministic replay.
- [x] Media utilities: image sequence, movie, audio buffer, texture feedback, and capture/playback manifests.
- [x] Visual node examples: feedback, instancing, particles, shader passes, text, and 2D/3D composition.
- [x] Developer-facing docs that map openFrameworks / Processing / Cinder / TouchDesigner concepts to FRAME0.

## Third-Party Ecosystem

- [x] Addon manifest schema and addon registry convention.
- [x] External C ABI skeleton for native externals.
- [x] Rust addon skeleton with tests.
- [ ] C/C++ external skeleton with header, build notes, and example entry points.
- [ ] Example third-party addon/external packages.
- [ ] Addon authoring guide: packaging, versioning, capabilities, permissions, tests, and examples.
- [ ] CLI helper commands or documented flows for verifying addon/external packages.

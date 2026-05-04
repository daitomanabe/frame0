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
- [ ] TouchDesigner-style operator network schema, parameters, channels, and table/texture conventions.
- [ ] Parameter automation: LFO, envelope, ramp, smoothing, mapping, and presets.
- [ ] Input and event resources: mouse, keyboard, MIDI, OSC, HID, serial, and multitouch contracts.
- [ ] Timeline and sequencing: clips, cues, keyframes, transport, beats, and deterministic replay.
- [ ] Media utilities: image sequence, movie, audio buffer, texture feedback, and capture/playback manifests.
- [ ] Visual node examples: feedback, instancing, particles, shader passes, text, and 2D/3D composition.
- [ ] Developer-facing docs that map openFrameworks / Processing / Cinder / TouchDesigner concepts to FRAME0.

## Third-Party Ecosystem

- [ ] Addon manifest schema and addon registry convention.
- [ ] External C ABI skeleton for native externals.
- [ ] Rust addon skeleton with tests.
- [ ] C/C++ external skeleton with header, build notes, and example entry points.
- [ ] Example third-party addon/external packages.
- [ ] Addon authoring guide: packaging, versioning, capabilities, permissions, tests, and examples.
- [ ] CLI helper commands or documented flows for verifying addon/external packages.

# ADR 0004: Timebase Is Independent Of Render Loop

Status: accepted

FRAME0 treats time as a runtime resource. Render scheduling consumes clock state; it does not define clock state.

The scaffold implements monotonic, manual, and fixed-step clocks. Audio, video, timecode, beat, and external clocks are reserved as explicit adapter-backed clocks.

Consequences:

- Tests can run deterministic headless frame sequences.
- Scene manifests must declare `clock.primary` and `clock.fallback`.
- Dropped frames and late frames are runtime events, not implicit render-loop side effects.


# analog_filter

FRAME0 example manifest based on the `analog-filter` Swift/Metal app. It models
a realtime webcam-to-analog-TV pipeline with microphone-reactive distortion,
VHS/CRT/RF/NTSC artifact groups, temporal feedback, presets, and adaptive render
scale.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/analog_filter/scene.yaml --json
cargo run -p frame0_cli -- graph examples/analog_filter/scene.yaml --json
cargo run -p frame0_cli -- run examples/analog_filter/scene.yaml --dry-run --json
cargo run -p frame0_cli -- examples launch analog_filter --frames 120 --out runs/examples/analog_filter --json
```

This is a FRAME0 contract example. It does not launch the original Swift app,
open an MTKView, request macOS camera/microphone permissions, or compile the
Metal source at runtime. Those native actions remain adapter/runtime work. The
example captures the app's graph shape and parameter surface so agents and
tests can inspect it through FRAME0.

## Source Mapping

| analog-filter module | FRAME0 contract |
| --- | --- |
| `CameraCapture.swift` | `camera` input with AVFoundation-style BGRA camera metadata |
| `AudioMeter.swift` | `audio_meter` node exposing level, peak, transient, and bass |
| `ControlsPanel.swift` | `analog_settings` preset bank and slider metadata |
| `AnalogSettings.swift` | `clean_crt`, `broadcast_ntsc`, `worn_vhs`, `bad_rf`, and `mono_security` presets |
| `MetalRenderer.swift` | `adaptive_render_scale`, feedback history, offscreen pass, and present pass |
| `Shaders.metal` | `analog_tv` shader pass and artifact group contract |

## Modeled Features

- RF reception: snow, impulse noise, multipath echo, AGC pumping, rolling bands.
- Composite/NTSC decode: chroma phase drift, cross-color, dot crawl, chroma
  noise, luma/chroma delay.
- VHS transport: time-base wobble, head switching, creases, scratches,
  dropouts, damaged-tape chroma loss.
- CRT display: scanlines, interlace shimmer, shadow mask, RGB misconvergence,
  bloom, phosphor flicker, persistence, vignette, and tube edge falloff.
- Microphone reactivity: level, peak, transient, and bass mappings into snow,
  bloom, sync hits, horizontal wobble, and dropout intensity.
- Performance controls: render scale, auto optimize, quantized scale changes,
  effect quality, feedback allocation fallback, and texture cache flushing.

## Presets

- `clean_crt`
- `broadcast_ntsc`
- `worn_vhs`
- `bad_rf`
- `mono_security`

Use the launch artifacts to inspect the deterministic FRAME0-side graph:

```bash
cargo run -p frame0_cli -- logs --root runs/examples/analog_filter --tail 5 --json
```

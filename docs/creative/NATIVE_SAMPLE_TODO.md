# FRAME0 Native Sample TODO Checklist

This checklist tracks the next sample-development loop for C++, shader/post-processing, audio, audio-visual interaction, machine learning, and Apple native capabilities.

Loop rule for this checklist:

1. Pick the first unchecked item.
2. Implement a CLI-verifiable doc/schema/example/template surface for that item.
3. Mark only that item complete.
4. Run format, tests, copyright scan, and privacy scan.
5. Commit and push.
6. Repeat until every item is checked.

## Samples And Native Workflows

- [x] C++ integration mechanism and example: C ABI boundary, C++ adapter class, build notes, and scene manifest.
- [ ] Shader and post-processing examples: multipass shader stack, bloom, LUT/color, feedback, and output capture.
- [ ] Audio-focused samples: input/generator/analyzer/mixer/recorder contracts and inspectable routing.
- [ ] Audio-visual relationship samples: beat/onset/FFT/channel mapping to visual params, timeline sync, and replay.
- [ ] Machine learning samples: model registry, preprocessing, inference, postprocess, multimodal overlays, and deterministic mock outputs.
- [ ] Apple Native feature samples: AVFoundation, CoreAudio, Core Media I/O Camera Extension, Metal, Core ML, Vision, Audio Unit, and ScreenCaptureKit contracts.

# input_events

This scene models mouse, keyboard, MIDI, OSC, HID, serial, and multitouch input resources as normalized FRAME0 event streams.

Run:

```bash
cargo run -p frame0_cli -- inspect examples/input_events/scene.yaml --json
cargo run -p frame0_cli -- schema export input_event --json
```

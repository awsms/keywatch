# keywatch

A tiny egui tool to inspect keyboard input in real time.

It shows:
- currently held logical keys
- last pressed key
- last released key (with a release counter)
- text input events
- physical key hints (when available)

Useful for debugging:
- keyboard layouts
- key repeats
- focus issues

## Run

```bash
cargo run --release
````

Click the window, press keys, observe.
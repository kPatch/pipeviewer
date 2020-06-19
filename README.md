# Pipe Viewer
Rust piper visualizer


## How to run

```
# Piping strings
echo "a string" | cargo run
```

## Notices
There's a ```pre-commit``` hook that runs fmt and clippy.

```
# Set environment variable
echo "a string" | PV_SILENT=something cargo run
```

## Commands

```
# Format
cargo fmt
```

```
# Optimization suggestions
cargo clippy
```
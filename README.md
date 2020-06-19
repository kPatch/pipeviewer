# Pipe Viewer
Rust piper visualizer

## Notices
There's a ```pre-commit``` hook that runs fmt and clippy.

```
cargo fmt
exec cargo clippy -- -D warnings
```


## How to run

```
# Piping strings
echo "a string" | cargo run
```

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
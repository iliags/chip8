# Chip 8

A Chip-8 emulator for practice.

## Building

### PC

```cargo build --release```
```cargo run --release```

### WASM

- Install trunk using ```cargo install trunk```
  - This may take a while
- Build using ```trunk serve --release```

### Build Warnings

If ```wasm-bindgen-cli``` was installed separately, but not update, it may print a tool mismatch warning.

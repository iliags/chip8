# Chip 8

A Chip-8 emulator for practice.

## Input Mapping

1234
QWER
ASDF
ZXCV

## Building

### PC

```cargo build --release```
```cargo run --release```

### WASM

- Install trunk using ```cargo install trunk```
  - This may take a while
- Build using ```trunk serve --release```

### Build Warnings

If ```wasm-bindgen-cli``` was installed separately, but not updated, it may print a tool mismatch warning.

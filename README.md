# Chip 8

A Chip-8 emulator for practice.

[Live version here](https://iliags.github.io/chip8/)

## Emulator Info

### Input Mapping

1234

QWER

ASDF

ZXCV

### Included ROMs

- [Chip8 Test Suite by Timendus](https://github.com/Timendus/chip8-test-suite)

### Localization

The user-facing text uses [fluent-rs](https://github.com/projectfluent/fluent-rs). Currently, only `en-US` is available but the capability to add more is there.

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

## Wishlist

Stuff that might be added later.

- Disassembler
- Debugger
- System visualizer
- Save states
- Bind actions to keys instead of hardcoding input

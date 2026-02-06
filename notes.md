# Notes

Notes for later things.

## Profiling

```cargo build --profile profiling```
```samply record ./target/profiling/chip8```

### Device

- Make the device crate usable as a standalone library
  
### Emulator Frontend

The idea is that ROM developers use the studio mode and gamers use the emulator mode. Studio mode should contain everything needed to develop a Chip-8 ROM (i.e. simple sprite editor, audio pattern generator, etc.) while the emulator variation is meant to play games. Controls for the emulator need to be implemented in the device crate while extra tools go in the studio crate.

There is an [egui-macroquad](https://github.com/optozorax/egui-macroquad) crate, but it might be a better idea to re-implement I/O logic in their respective frameworks. Marshalling the frame data and device input might be more effort than it's worth in the long run.

## Octo `options.json`

Egui has a hex to RGB color converter, however it isn't `const`.

Conversion:

```json
{
// cpu_speed
 "tickrate": 20,

 // Foreground1
 "fillColor": "#FFCC00",

 // Foreground2
 "fillColor2": "#FF6600",

 // Blended
 "blendColor": "#662200",

 // Background
 "backgroundColor": "#996600",

 // Buzzer
 "buzzColor": "#FFAA00",

 // Silence
 "quietColor": "#000000",

// vx_shifted_directly
 "shiftQuirks": false,
 
 // i_incremented
 "loadStoreQuirks": false,
 
// Unknown
 "vfOrderQuirks": false,

 // clip_sprites
 "clipQuirks": true,

 // jump_bits
 "jumpQuirks": false,

 // vf_zero
 "logicQuirks": true,

 // v_blank
 "vBlankQuirks": true,

 // N/A
 "screenRotation": 0,
 // N/A
 "maxSize": 3216,
 // N/A
 "touchInputMode": "vip",

// system_font
 "fontStyle": "octo",

 // Unknown if it can be used, it would need a converter
 "displayScale": "6"
}
```

## Wishlist

Stuff that might be added later.

- System visualizer
- Save states
- Touch input
- User facing error messages rather than panicking.
- Use icons over text
- UI color themes
- Assembler
- Disassembler
- Debugger

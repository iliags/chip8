# Notes

Notes for later things.

## Refactoring Stuff

- Get rid of the ad-hoc message system and run the emulator on a background thread
- Reference Octo for the audio issues

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

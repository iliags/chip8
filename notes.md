# Notes

Notes for later things.

## Octo `options.json`

Conversion:

```json
{
// cpu_speed
 "tickrate": 20,

 // Foreground1 color
 "fillColor": "#FFCC00",

 // Foreground2 color
 "fillColor2": "#FF6600",

 // Blended color
 "blendColor": "#662200",

 // Background color
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

## Compiler

- Assembler/Disassembler
- Debugger

## Wishlist

Stuff that might be added later.

- System visualizer
- Save states
- Bind actions to keys instead of hardcoding input
- User facing error messages rather than panicking.

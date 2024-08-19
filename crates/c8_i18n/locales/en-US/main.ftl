# App name
app_name = Chip 8

# Menu bar
open_rom = Open ROM
control_panel = Control Panel
visualizer_panel = Visualizer Panel

## ROM

### Included ROMs
included_roms = Included ROMs
test_roms = Test ROMs
game_roms = Games

### Rom Controls
reload_rom = Reload ROM
unload_rom = Unload ROM

# Controls

under_construction = Under construction
not_implemented = Not implemented

## CPU
cpu_speed = CPU Speed
speed = Speed
speed_hover = The number of instructions executed per frame (the UI updates at 60Hz/16ms)

## Display
display = Display
scale = Scale

## Pixel
pixel_colors = Pixel Colors
color_palette = Color Palette

## Input
keyboard = Keyboard

## Quirks
compatibility_profile = Profile
quirks = Quirks
quirk_vf0 = VF Zero
quirk_vf0_hover = VF is set to 0 during OR, AND, and XOR operations
quirk_i = I Incremented
quirk_i_hover = I is incremented by 1 after storing a ranged memory value
quirk_shift_vx = Shift VX directly
quirk_shift_vx_hover = VX is set to VY directly during shift operations
quirk_v_blank = Display Waiting
quirk_v_blank_hover = The display waits until the vertical blank period to draw
quirk_clip_sprites = Clip Sprites
quirk_clip_sprites_hover = Sprites are clipped to the display area
quirk_jump = Jump
quirk_jump_hover = The 4 high bits of target address determines the offset register instead of V0

## About
about = About
version = Version: {" "}
source = Source code
powered_by = Powered by {" "}
and = {" "} and {" "}

## Emulator
emulator = Emulator
language = Language
font_small = Small Font
font_large = Large Font
font_hover = Font usage may not be supported by all ROMs
default = Default

## Audio
audio_controls = Audio Controls
pitch = Pitch
octave = Octave
volume = Volume
enable_audio = Enable Audio

## Visualizer
memory = Memory
registers = Registers
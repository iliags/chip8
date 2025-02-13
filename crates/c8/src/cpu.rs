use std::vec;

use crate::{
    display::{Display, DisplayResolution},
    fonts::FONT_DATA,
    keypad::{Keypad, KeypadKey, KEYPAD_KEYS},
    memory::Memory,
    message::DeviceMessage,
};

use super::{display, quirks, PROGRAM_START};
use rand::prelude::*;

/// The general purpose registers of the Chip-8
// Dead code is allowed here because:
// A) Removing unused registers would mandate manual register mapping
// B) Unused registers may be used in the future (i.e. for debugging or testing)
#[allow(dead_code, missing_docs)]
#[derive(Debug)]
pub enum Register {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

#[derive(Debug, Clone)]
struct WaitingForKey {
    register: usize,
    key: Option<KeypadKey>,
}

/// The CPU of the Chip-8
#[derive(Debug)]
pub struct CPU {
    /// Index register
    index_register: u16,

    /// Program counter
    program_counter: u16,

    /// General purpose registers
    registers: Vec<u8>,

    // TODO: Make private when timers are implemented
    /// Delay timer
    pub(crate) delay_timer: u8,

    // TODO: Make private when timers are implemented
    /// Sound timer
    pub(crate) sound_timer: u8,

    waiting_for_key: Option<WaitingForKey>,

    // Flags are saved to a file or external storage in some implementations
    saved_registers: Vec<u8>,

    audio_buffer: Vec<u8>,

    buffer_pitch: u8,

    requesting_exit: bool,

    sound_dirty: bool,
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            index_register: 0,
            program_counter: PROGRAM_START,
            registers: vec![0; 16],
            delay_timer: 0,
            sound_timer: 0,
            waiting_for_key: None,
            saved_registers: vec![0; 16],
            audio_buffer: Vec::new(),
            buffer_pitch: 64,
            requesting_exit: false,
            sound_dirty: false,
        }
    }
}

impl CPU {
    /// Check if the CPU is requesting an exit
    pub(crate) fn is_requesting_exit(&self) -> bool {
        self.requesting_exit
    }

    /// Get the audio buffer
    pub fn audio_buffer(&self) -> &Vec<u8> {
        &self.audio_buffer
    }

    /// Clear the audio buffer
    pub fn clear_audio_buffer(&mut self) {
        self.audio_buffer.clear();
    }

    /// Get the buffer pitch
    pub fn buffer_pitch(&self) -> u8 {
        self.buffer_pitch
    }

    /// Get the program counter
    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Get the index register
    pub fn index_register(&self) -> u16 {
        self.index_register
    }

    /// Get the general registers
    pub fn registers(&self) -> &Vec<u8> {
        &self.registers
    }

    pub(crate) fn sound_dirty(&self) -> bool {
        self.sound_dirty
    }

    pub(crate) fn clear_sound_dirty(&mut self) {
        self.sound_dirty = false;
    }

    /// Step the CPU by one instruction
    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: &quirks::Quirks,
        keypad: &Keypad,
    ) -> Option<DeviceMessage> {
        // Note: This feels very hacky and should be refactored
        if let Some(task) = self.waiting_for_key.as_mut() {
            match task.key {
                Some(key) => {
                    if !keypad.is_key_pressed(&key) {
                        self.registers[task.register] = key.key_index() as u8;
                        self.waiting_for_key = None;
                    }
                }
                None => {
                    for key in KEYPAD_KEYS.iter() {
                        if keypad.is_key_pressed(key) {
                            task.key = Some(*key);
                            break;
                        }
                    }
                }
            }
            return None;
        }

        let pc = self.program_counter as usize;
        //println!("Program counter: {:#X}", pc);
        let opcode = (memory.data[pc] as u16) << 8 | (memory.data[pc + 1] as u16);

        // TODO: Move to a UI window
        /*
        println!(
            "Executing opcode: {:#X} from {:#X}, {:#X}",
            opcode,
            (memory.data[pc] as u16) << 8,
            memory.data[pc + 1] as u16
        );
        panic!("Stop");
         */

        self.program_counter += 2;

        let message = self.execute_instruction(opcode, memory, display, stack, *quirks, keypad);

        if let Some(DeviceMessage::WaitingForKey(register)) = message {
            self.waiting_for_key = Some(WaitingForKey {
                register: register.unwrap_or_else(|| {
                    // TODO: Shift to user facing error
                    eprintln!("Register not set");
                    0
                }),
                key: None,
            });
        }

        message
    }

    // Intentionally allowing too many lines
    #[allow(clippy::too_many_lines)]
    fn execute_instruction(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: quirks::Quirks,
        keypad: &Keypad,
    ) -> Option<DeviceMessage> {
        let mut message = None;

        //println!("Executing opcode: {:#X}", opcode);

        // Extract the opcode parts
        let reg_x = ((opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00F0 {
                //NOP
                // 0x0000
                0x0000 => {}

                // Scroll down n lines
                // 0x00CN
                0x00C0 => {
                    display.scroll_down(n);
                }

                // Scroll up n lines
                // 0x00DN
                0x00D0 => {
                    display.scroll_up(n);
                }

                0x00E0 => match opcode & 0xF0FF {
                    // Clear the display
                    // 0x00E0
                    0x00E0 => {
                        for layer in 0..display.plane_count() {
                            if display.active_plane() & (layer + 1) == 0 {
                                continue;
                            }
                            display.clear(layer);
                        }
                    }

                    // Return from a subroutine
                    // 0x00EE
                    0x00EE => {
                        // TODO: Make this more graceful
                        self.program_counter =
                            stack.pop().unwrap_or_else(|| panic!("Stack underflow"));
                    }

                    _ => {
                        message = unknown_opcode(opcode);
                    }
                },

                0x00F0 => match opcode & 0xF0FF {
                    // Scroll right 4 pixels
                    // 0x00FB
                    0x00FB => {
                        display.scroll_right(4);
                    }

                    // Scroll left 4 pixels
                    // 0x00FC
                    0x00FC => {
                        display.scroll_left(4);
                    }

                    // Exit
                    // 0x00FD
                    0x00FD => {
                        // Note: The program counter is decremented by 2 to prevent the program from advancing

                        self.requesting_exit = true;
                        self.program_counter -= 2;
                    }

                    // Enable low-res
                    // 0x00FE
                    0x00FE => {
                        display.set_resolution(DisplayResolution::Low);
                        message = Some(DeviceMessage::ChangeResolution(DisplayResolution::Low));
                    }

                    // Enable high-res
                    // 0x00FF
                    0x00FF => {
                        display.set_resolution(DisplayResolution::High);
                        message = Some(DeviceMessage::ChangeResolution(DisplayResolution::High));
                    }

                    _ => {
                        message = unknown_opcode(opcode);
                    }
                },

                _ => {
                    message = unknown_opcode(opcode);
                }
            },

            // Jump to address nnn
            // 0x1NNN
            0x1000 => {
                self.program_counter = nnn;
            }

            // Call subroutine at nnn
            // 0x2NNN
            0x2000 => {
                stack.push(self.program_counter);
                self.program_counter = nnn;
            }

            // Skip next instruction if Vx == nn
            // 0x3XNN
            0x3000 => {
                if self.registers[reg_x] == nn {
                    self.skip_next_instruction(memory);
                }
            }

            // Skip next instruction if Vx != nn
            // 0x4XNN
            0x4000 => {
                if self.registers[reg_x] != nn {
                    self.skip_next_instruction(memory);
                }
            }

            0x5000 => match opcode & 0xF00F {
                // Skip next instruction if Vx == Vy
                // 0x5XY0
                0x5000 => {
                    if self.registers[reg_x] == self.registers[reg_y] {
                        self.skip_next_instruction(memory);
                    }
                }

                // Save range
                // 0x5XY2
                0x5002 => {
                    let distance = reg_x.abs_diff(reg_y);
                    for z in 0..=distance {
                        let index = (self.index_register + z as u16) as usize;
                        memory.data[index] = if reg_x < reg_y {
                            self.registers[reg_x + z]
                        } else {
                            self.registers[reg_x - z]
                        };
                    }
                }

                // Load range
                // 0x5XY3
                0x5003 => {
                    let distance = reg_x.abs_diff(reg_y);

                    for z in 0..=distance {
                        let index = if reg_x < reg_y { reg_x + z } else { reg_x - z };
                        self.registers[index] =
                            memory.data[(self.index_register + z as u16) as usize];
                    }
                }
                _ => {
                    message = unknown_opcode(opcode);
                }
            },

            // Set Vx = nn
            // 0x6XNN
            0x6000 => {
                self.registers[reg_x] = nn;
            }

            // Set Vx = Vx + nn
            // 0x7XNN
            0x7000 => {
                self.registers[reg_x] = self.registers[reg_x].wrapping_add(nn);
            }

            0x8000 => match opcode & 0xF00F {
                // Set Vx = Vy
                // 0x8XY0
                0x8000 => {
                    self.registers[reg_x] = self.registers[reg_y];
                }

                // Set Vx = Vx OR Vy
                // 0x8XY1
                0x8001 => {
                    self.registers[reg_x] |= self.registers[reg_y];

                    // Quirk: Some programs expect VF to be 0
                    if quirks.vf_zero {
                        self.registers[Register::VF as usize] = 0;
                    }
                }

                // Set Vx = Vx AND Vy
                // 0x8XY2
                0x8002 => {
                    self.registers[reg_x] &= self.registers[reg_y];

                    // Quirk: Some programs expect VF to be 0
                    if quirks.vf_zero {
                        self.registers[Register::VF as usize] = 0;
                    }
                }

                // Set Vx = Vx XOR Vy
                // 0x8XY3
                0x8003 => {
                    self.registers[reg_x] ^= self.registers[reg_y];

                    // Quirk: Some programs expect VF to be 0
                    if quirks.vf_zero {
                        self.registers[Register::VF as usize] = 0;
                    }
                }

                // Set Vx = Vx + Vy, set VF = carry
                // 0x8XY4
                0x8004 => {
                    let (result, overflow) =
                        self.registers[reg_x].overflowing_add(self.registers[reg_y]);
                    self.registers[reg_x] = result;
                    self.registers[Register::VF as usize] = overflow as u8;
                }

                // Set Vx = Vx - Vy, set VF = NOT borrow
                // 0x8XY5
                0x8005 => {
                    let (result, overflow) =
                        self.registers[reg_x].overflowing_sub(self.registers[reg_y]);
                    self.registers[reg_x] = result;
                    self.registers[Register::VF as usize] = !overflow as u8;
                }

                // Vx >>= 1
                // 0x8XY6
                0x8006 => {
                    // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                    let quirk_y = if quirks.vx_shifted_directly {
                        self.registers[reg_x]
                    } else {
                        self.registers[reg_y]
                    };

                    self.registers[reg_x] = quirk_y >> 1;
                    self.registers[Register::VF as usize] = quirk_y & 0x1;
                }

                // Set Vx = Vy - Vx, set VF = NOT borrow
                // 0x8XY7
                0x8007 => {
                    let (result, overflow) =
                        self.registers[reg_y].overflowing_sub(self.registers[reg_x]);
                    self.registers[reg_x] = result;
                    self.registers[Register::VF as usize] = !overflow as u8;
                }

                // Vx <<= 1
                // 0x8XYE
                0x800E => {
                    // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                    let quirk_y = if quirks.vx_shifted_directly {
                        self.registers[reg_x]
                    } else {
                        self.registers[reg_y]
                    };

                    self.registers[reg_x] = quirk_y << 1;
                    self.registers[Register::VF as usize] = quirk_y >> 7;
                }
                _ => {
                    message = unknown_opcode(opcode);
                }
            },

            // Skip next instruction if Vx != Vy
            // 0x9XY0
            0x9000 => {
                if self.registers[reg_x] != self.registers[reg_y] {
                    self.skip_next_instruction(memory);
                }
            }

            // Set I = nnn
            // 0xANNN
            0xA000 => {
                self.index_register = nnn;
            }

            // Jump to location nnn + V0
            // 0xBNNN
            0xB000 => {
                self.program_counter = if quirks.jump_bits {
                    let index = (nnn >> 8) & 0xF;
                    nnn + self.registers[index as usize] as u16
                } else {
                    nnn + self.registers[Register::V0 as usize] as u16
                }
            }

            // Set Vx = random byte AND nn
            // 0xCXNN
            0xC000 => {
                let mut rng = rand::rng();
                self.registers[reg_x] = rng.random::<u8>() & nn;
            }

            // Draw a sprite at position (Vx, Vy) with N bytes of sprite data starting at the address stored in the index register
            // 0xDXYN
            0xD000 => self.draw_sprite(
                display,
                memory,
                self.registers[reg_x] as usize,
                self.registers[reg_y] as usize,
                n as usize,
                quirks.clip_sprites,
            ),

            0xE000 => match opcode & 0xF0FF {
                // Skip next instruction if key with the value of Vx is pressed
                // 0xEX9E
                0xE09E => {
                    let key = self.registers[reg_x] as usize;

                    if keypad.key(&key.into()) != 0 {
                        self.skip_next_instruction(memory);
                    }
                }

                // Skip next instruction if key with the value of Vx is not pressed
                // 0xEXA1
                0xE0A1 => {
                    let key = self.registers[reg_x] as usize;

                    if keypad.key(&key.into()) == 0 {
                        self.skip_next_instruction(memory);
                    }
                }

                _ => {
                    message = unknown_opcode(opcode);
                }
            },

            0xF000 => match opcode & 0xF0FF {
                // Load I extended
                // 0xF000
                0xF000 => {
                    let pc: usize = self.program_counter as usize;
                    let address = (memory.data[pc] as u16) << 8 | (memory.data[pc + 1] as u16);

                    self.index_register = address;
                    self.program_counter += 2;
                }

                // Set active plane from Vx
                // 0xFX01
                0xF001 => display.set_active_plane(reg_x),

                // Audio control
                // 0xFX02
                0xF002 => {
                    self.audio_buffer = vec![0; 16];
                    for offset in 0..16_u16 {
                        let index = (self.index_register + offset) as usize;
                        self.audio_buffer[offset as usize] = memory.data[index];
                    }
                    self.sound_dirty = true;
                }

                // Set Vx to the value of the delay timer
                // 0xFX07
                0xF007 => {
                    self.registers[reg_x] = self.delay_timer;
                }

                // Wait for a key press and store the result in Vx
                // 0xFX0A
                0xF00A => {
                    message = Some(DeviceMessage::WaitingForKey(Some(reg_x)));
                }

                // Set the delay timer to Vx
                // 0xFX15
                0xF015 => {
                    self.delay_timer = self.registers[reg_x];
                }

                // Set the sound timer to Vx
                // 0xFX18
                0xF018 => {
                    self.sound_timer = self.registers[reg_x];

                    if self.sound_timer == 0 {
                        self.audio_buffer.clear();
                    }
                }

                // Add Vx to the index register
                // 0xFX1E
                0xF01E => {
                    self.index_register += self.registers[reg_x] as u16;
                }

                // Set I to the location of the sprite for the character in Vx
                // 0xFX29
                0xF029 => {
                    self.index_register = (self.registers[reg_x] * 5) as u16;
                }

                // Load I with big sprite
                // 0xFX30
                0xF030 => {
                    let block = (self.registers[reg_x] & 0xF) * 10;
                    let font_size = &FONT_DATA[memory.system_font as usize].small_data.len();
                    self.index_register = (block + *font_size as u8) as u16;
                }

                // Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
                // 0xFX33
                0xF033 => {
                    memory.data[self.index_register as usize] = self.registers[reg_x] / 100;
                    memory.data[(self.index_register + 1) as usize] =
                        (self.registers[reg_x] / 10) % 10;
                    memory.data[(self.index_register + 2) as usize] =
                        (self.registers[reg_x] % 100) % 10;
                }

                // Buzz pitch
                // 0xFX3A
                0xF03A => {
                    self.buffer_pitch = self.registers[reg_x];
                }

                // Store V0 to Vx in memory starting at address I
                // 0xFX55
                0xF055 => {
                    // TODO: Check if this is correct
                    for i in 0..=reg_x {
                        memory.data[(self.index_register + i as u16) as usize] = self.registers[i];
                    }

                    // Quirk: Some programs expect I to be incremented
                    if quirks.i_incremented {
                        self.index_register += 1;
                    }
                }

                // Read V0 to Vx from memory starting at address I
                // 0xFX65
                0xF065 => {
                    // TODO: Check if this is correct
                    for i in 0..reg_x + 1 {
                        self.registers[i] = memory.data[(self.index_register + i as u16) as usize];
                    }

                    // Quirk: Some programs expect I to be incremented
                    if quirks.i_incremented {
                        self.index_register += 1;
                    }
                }

                // Save registers
                // 0xFX75
                0xF075 => {
                    for i in 0..=reg_x {
                        self.saved_registers[i] = self.registers[i];
                    }
                }

                // Load registers
                // 0xFX85
                0xF085 => {
                    // Do not clear saved registers after loading
                    for i in 0..=reg_x {
                        self.registers[i] = self.saved_registers[i];
                    }
                }

                _ => {
                    message = unknown_opcode(opcode);
                }
            },

            // Unknown opcode
            _ => {
                message = unknown_opcode(opcode);
            }
        }

        message
    }

    #[inline]
    fn skip_next_instruction(&mut self, memory: &Memory) {
        let pc = self.program_counter as usize;
        let next_op = (memory.data[pc] as u16) << 8 | (memory.data[pc + 1] as u16);

        // Check if the next instruction is an XO instruction
        let result = if next_op == 0xF000 { 4 } else { 2 };

        self.program_counter += result;
    }

    fn draw_sprite(
        &mut self,
        display: &mut Display,
        memory: &mut Memory,
        x: usize,
        y: usize,
        height: usize,
        clip_sprites: bool,
    ) {
        // Note: This is one of the more complex instructions.

        // Quirk: The sprites are limited to 60 per second due to V-blank interrupt waiting.
        // This may be implemented in the future with a toggle.

        let (screen_width, screen_height) = display.screen_size_xy();
        let x = x % screen_width;
        let y = y % screen_height;

        self.registers[Register::VF as usize] = 0;
        let mut collision = 0;

        let mut i = self.index_register as usize;

        // If height is 0, we are drawing a SuperChip 16x16 sprite, otherwise we are drawing an 8xN sprite
        let sprite_width = if height == 0 { 16 } else { 8 };
        let sprite_height = if height == 0 { 16 } else { height };
        let step = if height == 0 { 32 } else { height };

        for layer in 0..display.plane_count() {
            if display.active_plane() & (layer + 1) == 0 {
                continue;
            }

            for a in 0..sprite_height {
                let line: u16 = if height == 0 {
                    let read_index = (2 * a) + i;
                    (memory.data[read_index] as u16) << 8 | memory.data[read_index + 1] as u16
                } else {
                    memory.data[i + a] as u16
                };

                for b in 0..sprite_width {
                    let bit = if height == 0 { 15 - b } else { 7 - b };
                    let mut pixel = (line & (1 << bit)) >> bit;

                    // Quirk: Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen.
                    if clip_sprites && (x + b >= screen_width || y + a >= screen_height) {
                        pixel = 0;
                    }

                    if pixel == 0 {
                        continue;
                    }

                    // Note, something in the previous code causes the sprite index to be past capacity.
                    // This is a temporary fix until the root cause is found.

                    let pos_x = if x + b >= screen_width {
                        (x + b) % screen_width
                    } else {
                        x + b
                    };

                    let pos_y = if y + a >= screen_height {
                        (y + a) % screen_height
                    } else {
                        y + a
                    };

                    if display.set_plane_pixel(layer, pos_x, pos_y) == 1 {
                        collision = 1;
                    }
                }
            }

            i += step;
        }

        self.registers[Register::VF as usize] = collision;
    }
}

fn unknown_opcode(opcode: u16) -> Option<DeviceMessage> {
    eprintln!("Unknown opcode: {:#X}", opcode);
    Some(DeviceMessage::UnknownOpCode(opcode))
}

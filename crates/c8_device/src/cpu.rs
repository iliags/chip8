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

    buffer_pitch: f32,
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
            buffer_pitch: 64.0,
        }
    }
}

impl CPU {
    /// Get the audio buffer
    pub fn get_audio_buffer(&self) -> &Vec<u8> {
        &self.audio_buffer
    }

    /// Clear the audio buffer
    pub fn clear_audio_buffer(&mut self) {
        self.audio_buffer.clear();
    }

    /// Get the buffer pitch
    pub fn get_buffer_pitch(&self) -> f32 {
        self.buffer_pitch
    }

    /// Get the program counter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Get the index register
    pub fn get_index_register(&self) -> u16 {
        self.index_register
    }

    /// Get the general registers
    pub fn get_registers(&self) -> &Vec<u8> {
        &self.registers
    }

    /// Step the CPU by one instruction
    pub fn step(
        &mut self,
        memory: &mut Memory,
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: &quirks::Quirks,
        keypad: &Keypad,
    ) -> Vec<DeviceMessage> {
        // Note: This feels very hacky and should be refactored
        if let Some(task) = self.waiting_for_key.as_mut() {
            match task.key {
                Some(key) => {
                    if !keypad.is_key_pressed(&key) {
                        self.registers[task.register] = key.get_key_index() as u8;
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
            return Vec::new();
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

        let messages = self.execute_instruction(opcode, memory, display, stack, quirks, keypad);

        for message in messages.iter() {
            if let DeviceMessage::WaitingForKey(register) = message {
                self.waiting_for_key = Some(WaitingForKey {
                    register: register.unwrap_or_else(|| {
                        // TODO: Shift to user facing error
                        eprintln!("Register not set");
                        0
                    }),
                    key: None,
                });
            }
        }

        messages
    }

    fn execute_instruction(
        &mut self,
        opcode: u16,
        memory: &mut Memory,
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: &quirks::Quirks,
        keypad: &Keypad,
    ) -> Vec<DeviceMessage> {
        let mut messages: Vec<DeviceMessage> = Vec::new();

        // Extract the opcode parts
        let reg_x = ((opcode & 0x0F00) >> 8) as usize;
        let reg_y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        match (op_1, op_2, op_3, op_4) {
            //NOP
            // 0x0000
            (0, 0, 0, 0) => {}

            // Scroll down n lines
            // 0x00CN
            (0, 0, 0xC, _) => {
                display.scroll_down(n);
            }

            // Scroll up n lines
            // 0x00DN
            (0, 0, 0xD, _) => {
                display.scroll_up(n);
            }

            // Clear the display
            // 0x00E0
            (0, 0, 0xE, 0) => {
                for layer in 0..display.get_plane_count() {
                    if display.get_active_plane() & (layer + 1) == 0 {
                        continue;
                    }
                    display.clear(layer);
                }
            }

            // Return from a subroutine
            // 0x00EE
            (0, 0, 0xE, 0xE) => {
                // TODO: Make this more graceful
                self.program_counter = stack.pop().unwrap_or_else(|| panic!("Stack underflow"));
            }

            // Scroll right 4 pixels
            // 0x00FB
            (0, 0, 0xF, 0xB) => {
                display.scroll_right(4);
            }

            // Scroll left 4 pixels
            // 0x00FC
            (0, 0, 0xF, 0xC) => {
                display.scroll_left(4);
            }

            // Exit
            // 0x00FD
            (0, 0, 0xF, 0xD) => {
                // Note: The program counter is decremented by 2 to prevent the program from advancing

                self.program_counter -= 2;
                messages.push(DeviceMessage::Exit);
            }

            // Enable low-res
            // 0x00FE
            (0, 0, 0xF, 0xE) => {
                messages.push(DeviceMessage::ChangeResolution(DisplayResolution::Low));
            }

            // Enable high-res
            // 0x00FF
            (0, 0, 0xF, 0xF) => {
                messages.push(DeviceMessage::ChangeResolution(DisplayResolution::High));
            }

            // Jump to address nnn
            // 0x1NNN
            (1, _, _, _) => {
                self.program_counter = nnn;
            }

            // Call subroutine at nnn
            // 0x2NNN
            (2, _, _, _) => {
                stack.push(self.program_counter);
                self.program_counter = nnn;
            }

            // Skip next instruction if Vx == nn
            // 0x3XNN
            (3, _, _, _) => {
                if self.registers[reg_x] == nn {
                    self.skip_next_instruction(memory);
                }
            }

            // Skip next instruction if Vx != nn
            // 0x4XNN
            (4, _, _, _) => {
                if self.registers[reg_x] != nn {
                    self.skip_next_instruction(memory);
                }
            }

            // Skip next instruction if Vx == Vy
            // 0x5XY0
            (5, _, _, 0) => {
                if self.registers[reg_x] == self.registers[reg_y] {
                    self.skip_next_instruction(memory);
                }
            }

            // Save range
            // 0x5XY2
            (5, _, _, 2) => {
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
            (5, _, _, 3) => {
                let distance = reg_x.abs_diff(reg_y);

                for z in 0..=distance {
                    let index = if reg_x < reg_y { reg_x + z } else { reg_x - z };
                    self.registers[index] = memory.data[(self.index_register + z as u16) as usize];
                }
            }

            // Set Vx = nn
            // 0x6XNN
            (6, _, _, _) => {
                self.registers[reg_x] = nn;
            }

            // Set Vx = Vx + nn
            // 0x7XNN
            (7, _, _, _) => {
                self.registers[reg_x] = self.registers[reg_x].wrapping_add(nn);
            }

            // Set Vx = Vy
            // 0x8XY0
            (8, _, _, 0) => {
                self.registers[reg_x] = self.registers[reg_y];
            }

            // Set Vx = Vx OR Vy
            // 0x8XY1
            (8, _, _, 1) => {
                self.registers[reg_x] |= self.registers[reg_y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx AND Vy
            // 0x8XY2
            (8, _, _, 2) => {
                self.registers[reg_x] &= self.registers[reg_y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx XOR Vy
            // 0x8XY3
            (8, _, _, 3) => {
                self.registers[reg_x] ^= self.registers[reg_y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx + Vy, set VF = carry
            // 0x8XY4
            (8, _, _, 4) => {
                let (result, overflow) =
                    self.registers[reg_x].overflowing_add(self.registers[reg_y]);
                self.registers[reg_x] = result;
                self.registers[Register::VF as usize] = overflow as u8;
            }

            // Set Vx = Vx - Vy, set VF = NOT borrow
            // 0x8XY5
            (8, _, _, 5) => {
                let (result, overflow) =
                    self.registers[reg_x].overflowing_sub(self.registers[reg_y]);
                self.registers[reg_x] = result;
                self.registers[Register::VF as usize] = !overflow as u8;
            }

            // Vx >>= 1
            // 0x8XY6
            (8, _, _, 6) => {
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
            (8, _, _, 7) => {
                let (result, overflow) =
                    self.registers[reg_y].overflowing_sub(self.registers[reg_x]);
                self.registers[reg_x] = result;
                self.registers[Register::VF as usize] = !overflow as u8;
            }

            // Vx <<= 1
            // 0x8XYE
            (8, _, _, 0xE) => {
                // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                let quirk_y = if quirks.vx_shifted_directly {
                    self.registers[reg_x]
                } else {
                    self.registers[reg_y]
                };

                self.registers[reg_x] = quirk_y << 1;
                self.registers[Register::VF as usize] = quirk_y >> 7;
            }

            // Skip next instruction if Vx != Vy
            // 0x9XY0
            (9, _, _, 0) => {
                if self.registers[reg_x] != self.registers[reg_y] {
                    self.skip_next_instruction(memory);
                }
            }

            // Set I = nnn
            // 0xANNN
            (0xA, _, _, _) => {
                self.index_register = nnn;
            }

            // Jump to location nnn + V0
            // 0xBNNN
            (0xB, _, _, _) => {
                self.program_counter = if quirks.jump_bits {
                    let index = (nnn >> 8) & 0xF;
                    nnn + self.registers[index as usize] as u16
                } else {
                    nnn + self.registers[Register::V0 as usize] as u16
                }
            }

            // Set Vx = random byte AND nn
            // 0xCXNN
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                self.registers[reg_x] = rng.gen::<u8>() & nn;
            }

            // Draw a sprite at position (Vx, Vy) with N bytes of sprite data starting at the address stored in the index register
            // 0xDXYN
            (0xD, _, _, _) => self.draw_sprite(
                display,
                memory,
                self.registers[reg_x] as usize,
                self.registers[reg_y] as usize,
                n as usize,
                quirks.clip_sprites,
            ),

            // Skip next instruction if key with the value of Vx is pressed
            // 0xEX9E
            (0xE, _, 9, 0xE) => {
                let key = self.registers[reg_x] as usize;

                if keypad.get_key(&key.into()) != 0 {
                    self.skip_next_instruction(memory);
                }
            }

            // Skip next instruction if key with the value of Vx is not pressed
            // 0xEXA1
            (0xE, _, 0xA, 1) => {
                let key = self.registers[reg_x] as usize;

                if keypad.get_key(&key.into()) == 0 {
                    self.skip_next_instruction(memory);
                }
            }

            // Load I extended
            // 0xF000
            (0xF, 0, 0, 0) => {
                let pc: usize = self.program_counter as usize;
                let address = (memory.data[pc] as u16) << 8 | (memory.data[pc + 1] as u16);

                self.index_register = address;
                self.program_counter += 2;
            }

            // Set active plane from Vx
            // 0xFX01
            (0xF, _, 0, 1) => display.set_active_plane(reg_x),

            // Audio control
            // 0xFX02
            (0xF, _, 0, 2) => {
                self.audio_buffer = vec![0; 16];
                for z in 0..16_u16 {
                    let index = (self.index_register + z) as usize;
                    self.audio_buffer[z as usize] = memory.data[index];
                }
                messages.push(DeviceMessage::NewAudioBuffer);
            }

            // Set Vx to the value of the delay timer
            // 0xFX07
            (0xF, _, 0, 7) => {
                self.registers[reg_x] = self.delay_timer;
            }

            // Wait for a key press and store the result in Vx
            // 0xFX0A
            (0xF, _, 0, 0xA) => {
                messages.push(DeviceMessage::WaitingForKey(Some(reg_x)));
            }

            // Set the delay timer to Vx
            // 0xFX15
            (0xF, _, 1, 5) => {
                self.delay_timer = self.registers[reg_x];
            }

            // Set the sound timer to Vx
            // 0xFX18
            (0xF, _, 1, 8) => {
                self.sound_timer = self.registers[reg_x];

                if self.sound_timer == 0 {
                    self.audio_buffer.clear();
                }

                messages.push(DeviceMessage::Beep(self.registers[reg_x]));
            }

            // Add Vx to the index register
            // 0xFX1E
            (0xF, _, 1, 0xE) => {
                self.index_register += self.registers[reg_x] as u16;
            }

            // Set I to the location of the sprite for the character in Vx
            // 0xFX29
            (0xF, _, 2, 9) => {
                self.index_register = (self.registers[reg_x] * 5) as u16;
            }

            // Load I with big sprite
            // 0xFX30
            (0xF, _, 3, 0) => {
                let block = (self.registers[reg_x] & 0xF) * 10;
                let font_size = &FONT_DATA[memory.system_font as usize].small_data.len();
                self.index_register = (block + *font_size as u8) as u16;
            }

            // Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
            // 0xFX33
            (0xF, _, 3, 3) => {
                memory.data[self.index_register as usize] = self.registers[reg_x] / 100;
                memory.data[(self.index_register + 1) as usize] = (self.registers[reg_x] / 10) % 10;
                memory.data[(self.index_register + 2) as usize] =
                    (self.registers[reg_x] % 100) % 10;
            }

            // Buzz pitch
            // 0xFX3A
            (0xF, _, 3, 0xA) => {
                self.buffer_pitch = self.registers[reg_x] as f32;
            }

            // Store V0 to Vx in memory starting at address I
            // 0xFX55
            (0xF, _, 5, 5) => {
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
            (0xF, _, 6, 5) => {
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
            (0xF, _, 7, 5) => {
                for i in 0..=reg_x {
                    self.saved_registers[i] = self.registers[i];
                }
            }

            // Load registers
            // 0xFX85
            (0xF, _, 8, 5) => {
                // Do not clear saved registers after loading
                for i in 0..=reg_x {
                    self.registers[i] = self.saved_registers[i];
                }
            }

            // Unknown opcode
            _ => {
                println!("Unknown opcode: {:#X}", opcode);
            }
        }

        messages
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

        let (screen_width, screen_height) = display.get_screen_size_xy();
        let x = x % screen_width;
        let y = y % screen_height;

        self.registers[Register::VF as usize] = 0;
        let mut collision = 0;

        let mut i = self.index_register as usize;

        // If height is 0, we are drawing a SuperChip 16x16 sprite, otherwise we are drawing an 8xN sprite
        let sprite_width = if height == 0 { 16 } else { 8 };
        let sprite_height = if height == 0 { 16 } else { height } as usize;
        let step = if height == 0 { 32 } else { height as usize };

        for layer in 0..display.get_plane_count() {
            crate::profile_scope!("Draw sprite");
            if display.get_active_plane() & (layer + 1) == 0 {
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

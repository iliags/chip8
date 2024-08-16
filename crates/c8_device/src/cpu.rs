use crate::{
    display::DisplayResolution,
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
        }
    }
}

impl CPU {
    /// Get the program counter
    pub fn get_program_counter(&self) -> u16 {
        self.program_counter
    }

    /// Get the index register
    pub fn get_index_register(&self) -> u16 {
        self.index_register
    }

    /// Get the general registers
    pub fn get_registers(&self) -> Vec<u8> {
        self.registers.clone()
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
        if self.waiting_for_key.is_some() {
            let task = self.waiting_for_key.as_mut().unwrap();

            if task.key.is_none() {
                for key in KEYPAD_KEYS.iter() {
                    if keypad.is_key_pressed(key) {
                        task.key = Some(*key);
                        break;
                    }
                }
            } else {
                match task.key {
                    Some(key) => {
                        if !keypad.is_key_pressed(&key) {
                            self.registers[task.register] = key.get_key_index() as u8;
                            self.waiting_for_key = None;
                        }
                    }
                    None => {
                        unreachable!("Key not set");
                    }
                }
            }

            return Vec::new();
        }

        let pc = self.program_counter as usize;
        let opcode = (memory.data[pc] as u16) << 8 | memory.data[pc + 1] as u16;

        // TODO: Move to a UI window
        /* println!(
            "Executing opcode: {:#X} from {:#X}, {:#X}",
            opcode,
            (self.memory[pc] as u16) << SHIFT,
            self.memory[pc + 1] as u16
        ); */

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
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;

        //println!("Executing opcode: {:#X}", opcode);
        //println!("x: {}, y: {}, n: {}, nn: {}, nnn: {}", x, y, n, nn, nnn);

        match (op_1, op_2, op_3, op_4) {
            //NOP
            (0, 0, 0, 0) => {}

            // Scroll down n lines
            (0, 0, 0xC, _) => {
                display.scroll_down(n);
            }

            // Scroll up n lines
            (0, 0, 0xD, _) => {
                display.scroll_up(n);
            }

            // Clear the display
            (0, 0, 0xE, 0) => {
                display.clear();
            }

            // Return from a subroutine
            (0, 0, 0xE, 0xE) => {
                // TODO: Make this more graceful
                self.program_counter = stack.pop().unwrap_or_else(|| panic!("Stack underflow"));
            }

            // Scroll right 4 pixels
            (0, 0, 0xF, 0xB) => {
                display.scroll_right(4);
            }

            // Scroll left 4 pixels
            (0, 0, 0xF, 0xC) => {
                display.scroll_left(4);
            }

            // Exit
            (0, 0, 0xF, 0xD) => {
                // Note: The program counter is decremented by 2 to prevent the program from advancing

                self.program_counter -= 2;
                messages.push(DeviceMessage::Exit);
            }

            // Enable low-res
            (0, 0, 0xF, 0xE) => {
                messages.push(DeviceMessage::ChangeResolution(DisplayResolution::Low));
            }

            // Enable high-res
            (0, 0, 0xF, 0xF) => {
                messages.push(DeviceMessage::ChangeResolution(DisplayResolution::High));
            }

            // Jump to address nnn
            (1, _, _, _) => {
                self.program_counter = nnn;
            }

            // Call subroutine at nnn
            (2, _, _, _) => {
                stack.push(self.program_counter);
                self.program_counter = nnn;
            }

            // Skip next instruction if Vx == nn
            (3, _, _, _) => {
                if self.registers[x] == nn {
                    self.program_counter += 2;
                }
            }

            // Skip next instruction if Vx != nn
            (4, _, _, _) => {
                if self.registers[x] != nn {
                    self.program_counter += 2;
                }
            }

            // Skip next instruction if Vx == Vy
            (5, _, _, 0) => {
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }

            // Save vx through vy
            (5, _, _, 2) => {
                for i in x..=y {
                    memory.data[(self.index_register + i as u16) as usize] = self.registers[i];
                }
            }

            // Load vx through vy from i
            (5, _, _, 3) => {
                for i in x..=y {
                    self.registers[i] = memory.data[(self.index_register + i as u16) as usize];
                }
            }

            // Set Vx = nn
            (6, _, _, _) => {
                self.registers[x] = nn;
            }

            // Set Vx = Vx + nn
            (7, _, _, _) => {
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }

            // Set Vx = Vy
            (8, _, _, 0) => {
                self.registers[x] = self.registers[y];
            }

            // Set Vx = Vx OR Vy
            (8, _, _, 1) => {
                self.registers[x] |= self.registers[y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx AND Vy
            (8, _, _, 2) => {
                self.registers[x] &= self.registers[y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx XOR Vy
            (8, _, _, 3) => {
                self.registers[x] ^= self.registers[y];

                // Quirk: Some programs expect VF to be 0
                if quirks.vf_zero {
                    self.registers[Register::VF as usize] = 0;
                }
            }

            // Set Vx = Vx + Vy, set VF = carry
            (8, _, _, 4) => {
                let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = result;
                self.registers[Register::VF as usize] = overflow as u8;
            }

            // Set Vx = Vx - Vy, set VF = NOT borrow
            (8, _, _, 5) => {
                let (result, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = result;
                self.registers[Register::VF as usize] = !overflow as u8;
            }

            // Vx >>= 1
            (8, _, _, 6) => {
                // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                let quirk_y = if quirks.vx_shifted_directly {
                    self.registers[y]
                } else {
                    self.registers[x]
                };

                self.registers[x] = quirk_y >> 1;
                self.registers[Register::VF as usize] = quirk_y & 0x1;
            }

            // Set Vx = Vy - Vx, set VF = NOT borrow
            (8, _, _, 7) => {
                let (result, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = result;
                self.registers[Register::VF as usize] = !overflow as u8;
            }

            // Vx <<= 1
            (8, _, _, 0xE) => {
                // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                let quirk_y = if quirks.vx_shifted_directly {
                    self.registers[y]
                } else {
                    self.registers[x]
                };

                self.registers[x] = quirk_y << 1;
                self.registers[Register::VF as usize] = quirk_y >> 7;
            }

            // Skip next instruction if Vx != Vy
            (9, _, _, 0) => {
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }

            // Set I = nnn
            (0xA, _, _, _) => {
                self.index_register = nnn;
            }

            // Jump to location nnn + V0
            (0xB, _, _, _) => {
                self.program_counter = nnn + self.registers[Register::V0 as usize] as u16;
            }

            // Set Vx = random byte AND nn
            (0xC, _, _, _) => {
                let mut rng = rand::thread_rng();
                self.registers[x] = rng.gen::<u8>() & nn;
            }

            // Draw a sprite at position (Vx, Vy) with N bytes of sprite data starting at the address stored in the index register
            (0xD, _, _, _) => {
                // Note: This is one of the more complex instructions

                // Quirk: The sprites are limited to 60 per second due to V-blank interrupt waiting.
                // This may be implemented in the future with a toggle.

                //self.registers[Register::VF as usize] = 0;

                let (display_width, display_height) = display.get_screen_size_xy();

                let x = self.registers[x] as usize % display_width;
                let y = self.registers[y] as usize % display_height;

                // If height is 0, we are drawing a SuperChip 16x16 sprite, otherwise we are drawing an 8xN sprite
                let height = n;

                let mut i = self.index_register as usize;

                let sprite_width = if height == 0 { 16 } else { 8 };
                let sprite_height = if height == 0 { 16 } else { height } as usize;

                for plane in 0..1 {
                    //display.get_plane_count() {
                    // Note: In Octo, the layers

                    let mut collision = 0;
                    for row in 0..sprite_height {
                        let line: u16 = if height == 0 {
                            let read = 2 * row;
                            (memory.data[read + i] as u16) << 8 | memory.data[read + i + 1] as u16
                        } else {
                            memory.data[i + row] as u16
                        };

                        for column in 0..sprite_width {
                            let bit = if height == 0 { 15 - column } else { 7 - column };
                            let pixel = (line & (1 << bit)) >> bit;

                            if pixel == 0 {
                                continue;
                            }

                            let pos_x = x + column;
                            let pos_y = y + row;

                            if display.set_plane_pixel(plane, pos_x, pos_y) == 1 {
                                collision = 1;
                            }
                        }

                        //self.registers[Register::VF as usize] = collision;
                    }

                    i += if height == 0 { 32 } else { height as usize };
                    self.registers[Register::VF as usize] = collision;
                }
            }

            // Skip next instruction if key with the value of Vx is pressed
            (0xE, _, 9, 0xE) => {
                let key = self.registers[x] as usize;

                if keypad.get_key(&key.into()) != 0 {
                    self.program_counter += 2;
                }
            }

            // Skip next instruction if key with the value of Vx is not pressed
            (0xE, _, 0xA, 1) => {
                let key = self.registers[x] as usize;

                if keypad.get_key(&key.into()) == 0 {
                    self.program_counter += 2;
                }
            }

            // Load I extended
            (0xF, _, 0, 0) => {
                // TODO: Check if this is correct
                let pc = self.program_counter as usize;
                let address = (memory.data[pc] as u16) << 8 | (memory.data[pc + 1] as u16);

                self.index_register = address;
                self.program_counter += 2;
            }

            // Set active plane from Vx
            (0xF, _, 0, 1) => display.set_active_plane(x),

            // Audio control
            (0xF, _, 0, 2) => {
                // Note: Playback rate needs to be 4000*2^((vx-64)/48) Hz
                todo!("Audio control")
            }

            // Set Vx to the value of the delay timer
            (0xF, _, 0, 7) => {
                self.registers[x] = self.delay_timer;
            }

            // Wait for a key press and store the result in Vx
            (0xF, _, 0, 0xA) => {
                messages.push(DeviceMessage::WaitingForKey(Some(x)));
            }

            // Set the delay timer to Vx
            (0xF, _, 1, 5) => {
                self.delay_timer = self.registers[x];
            }

            // Set the sound timer to Vx
            (0xF, _, 1, 8) => {
                self.sound_timer = self.registers[x];
            }

            // Add Vx to the index register
            (0xF, _, 1, 0xE) => {
                self.index_register += self.registers[x] as u16;
            }

            // Set I to the location of the sprite for the character in Vx
            (0xF, _, 2, 9) => {
                self.index_register = (self.registers[x] * 5) as u16;
                // TODO: Check if this is correct
                //self.index_register = ((self.registers[x] & 0xF) * 5) as u16;
            }

            // Load I with big sprite
            (0xF, _, 3, 0) => {
                let block = (self.registers[x] & 0xF) * 10;
                let font_size = &FONT_DATA[memory.system_font as usize].small_data.len();
                self.index_register = (block + *font_size as u8) as u16;
            }

            // Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
            (0xF, _, 3, 3) => {
                memory.data[self.index_register as usize] = self.registers[x] / 100;
                memory.data[(self.index_register + 1) as usize] = (self.registers[x] / 10) % 10;
                memory.data[(self.index_register + 2) as usize] = (self.registers[x] % 100) % 10;
            }

            // Store V0 to Vx in memory starting at address I
            (0xF, _, 5, 5) => {
                for i in 0..x + 1 {
                    memory.data[(self.index_register + i as u16) as usize] = self.registers[i];
                }

                // Quirk: Some programs expect I to be incremented
                if quirks.i_incremented {
                    self.index_register += 1;
                }
            }

            // Read V0 to Vx from memory starting at address I
            (0xF, _, 6, 5) => {
                for i in 0..x + 1 {
                    self.registers[i] = memory.data[(self.index_register + i as u16) as usize];
                }

                // Quirk: Some programs expect I to be incremented
                if quirks.i_incremented {
                    self.index_register += 1;
                }
            }

            // Save registers
            (0xF, _, 7, 5) => {
                self.saved_registers = self.registers.clone();
            }

            // Load registers
            (0xF, _, 8, 5) => {
                self.registers = self.saved_registers.clone();
            }

            // Unknown opcode
            _ => {
                println!("Unknown opcode: {:#X}", opcode);
                //messages.push(DeviceMessage::UnknownOpcode(opcode));
            }
        }

        messages
    }
}

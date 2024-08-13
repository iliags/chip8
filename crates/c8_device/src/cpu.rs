use crate::keypad::Keypad;

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
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            index_register: 0,
            program_counter: PROGRAM_START,
            registers: vec![0; 16],
            delay_timer: 0,
            sound_timer: 0,
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
        memory: &mut [u8],
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: &quirks::Quirks,
        keypad: &Keypad,
    ) {
        const SHIFT: u8 = 8;

        let pc = self.program_counter as usize;
        let opcode = (memory[pc] as u16) << SHIFT | memory[pc + 1] as u16;

        // TODO: Move to a UI window
        /* println!(
            "Executing opcode: {:#X} from {:#X}, {:#X}",
            opcode,
            (self.memory[pc] as u16) << SHIFT,
            self.memory[pc + 1] as u16
        ); */

        self.program_counter += 2;

        self.execute_instruction(opcode, memory, display, stack, quirks, keypad);
    }

    fn execute_instruction(
        &mut self,
        opcode: u16,
        memory: &mut [u8],
        display: &mut display::Display,
        stack: &mut Vec<u16>,
        quirks: &quirks::Quirks,
        keypad: &Keypad,
    ) {
        // Extract the opcode parts
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        // Decode the opcode
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x0000 => {
                        // NOP
                    }
                    0x00C0 => {
                        // TODO: Scroll down n lines
                        todo!("Scroll down n lines")
                    }
                    0x00D0 => {
                        // TODO: Scroll up n lines
                        todo!("Scroll up n lines")
                    }
                    0x00E0 => {
                        // Clear the display
                        display.clear();
                    }
                    0x00EE => {
                        // Return from a subroutine
                        // TODO: Make this more graceful
                        self.program_counter =
                            stack.pop().unwrap_or_else(|| panic!("Stack underflow"));
                    }
                    0x00FB => {
                        // TODO: Scroll right 4 pixels
                        todo!("Scroll right 4 pixels")
                    }
                    0x00FC => {
                        // TODO: Scroll left 4 pixels
                        todo!("Scroll left 4 pixels")
                    }
                    0x00FD => {
                        // TODO: Exit
                        todo!("Exit")
                    }
                    0x00FE => {
                        // TODO: Enable low-res
                        todo!("Enable low-res")
                    }
                    0x00FF => {
                        // TODO: Enable high-res
                        todo!("Enable high-res")
                    }
                    _ => {
                        println!("Unknown 0x0000 opcode: {:#X}", opcode);
                    }
                }
            }
            0x1000 => {
                // Jump to address nnn
                self.program_counter = nnn;
            }
            0x2000 => {
                // Call subroutine at nnn
                stack.push(self.program_counter);
                self.program_counter = nnn;
            }
            0x3000 => {
                // Skip next instruction if Vx == nn
                if self.registers[x] == nn {
                    self.program_counter += 2;
                }
            }
            0x4000 => {
                // Skip next instruction if Vx != nn
                if self.registers[x] != nn {
                    self.program_counter += 2;
                }
            }
            0x5000 => {
                // Skip next instruction if Vx == Vy
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            }
            0x6000 => {
                // Set Vx = nn
                self.registers[x] = nn;
            }
            0x7000 => {
                // Set Vx = Vx + nn
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }
            0x8000 => {
                match opcode & 0xF {
                    0x0 => {
                        // Set Vx = Vy
                        self.registers[x] = self.registers[y];
                    }
                    0x1 => {
                        // Set Vx = Vx OR Vy
                        self.registers[x] |= self.registers[y];

                        // Quirk: Some programs expect VF to be 0
                        if quirks.vf_zero {
                            self.registers[Register::VF as usize] = 0;
                        }
                    }
                    0x2 => {
                        // Set Vx = Vx AND Vy
                        self.registers[x] &= self.registers[y];

                        // Quirk: Some programs expect VF to be 0
                        if quirks.vf_zero {
                            self.registers[Register::VF as usize] = 0;
                        }
                    }
                    0x3 => {
                        // Set Vx = Vx XOR Vy
                        self.registers[x] ^= self.registers[y];

                        // Quirk: Some programs expect VF to be 0
                        if quirks.vf_zero {
                            self.registers[Register::VF as usize] = 0;
                        }
                    }
                    0x4 => {
                        // Set Vx = Vx + Vy, set VF = carry
                        let (result, overflow) =
                            self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = overflow as u8;
                    }
                    0x5 => {
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                        let (result, overflow) =
                            self.registers[x].overflowing_sub(self.registers[y]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = !overflow as u8;
                    }
                    0x6 => {
                        // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                        if quirks.vx_shifted_directly {
                            self.registers[x] = self.registers[y];
                        }

                        // Set Vx = Vx SHR 1
                        self.registers[Register::VF as usize] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                    }
                    0x7 => {
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                        let (result, overflow) =
                            self.registers[y].overflowing_sub(self.registers[x]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = !overflow as u8;
                    }
                    0xE => {
                        // Quirk: Some programs expect Vx to be shifted directly without assigning VY
                        if quirks.vx_shifted_directly {
                            self.registers[x] = self.registers[y];
                        }

                        // Set Vx = Vy SHL 1
                        self.registers[Register::VF as usize] = self.registers[x] >> 7;
                        self.registers[x] <<= 1;
                    }
                    _ => {
                        println!("Unknown 0x8000 opcode: {:#X}", opcode);
                    }
                }
            }
            0x9000 => {
                // Skip next instruction if Vx != Vy
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            }
            0xA000 => {
                // Set I = nnn
                self.index_register = nnn;
            }
            0xB000 => {
                // Jump to location nnn + V0
                self.program_counter = nnn + self.registers[Register::V0 as usize] as u16;
            }
            0xC000 => {
                // Set Vx = random byte AND nn
                let mut rng = rand::thread_rng();
                self.registers[x] = rng.gen::<u8>() & nn;
            }
            0xD000 => {
                // Quirk: The sprites are limited to 60 per second due to V-blank interrupt waiting.
                // This may be implemented in the future with a toggle.

                // Draw a sprite at position (Vx, Vy) with N bytes of sprite data starting at the address stored in the index register
                let x = self.registers[x] as i32;
                let y = self.registers[y] as i32;
                let height = opcode & 0x000F;

                self.registers[0xF] = 0;

                for row in 0..height {
                    let pixel = memory[(self.index_register + row) as usize];

                    for col in 0..8 {
                        if (pixel & (0x80 >> col)) != 0
                            && display.set_pixel(x + col, y + row as i32) == 0
                        {
                            self.registers[0xF] = 1;
                        }
                    }
                }
            }
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                        let key = self.registers[x] as usize;

                        if keypad.get_key(&key.into()) != 0 {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                        let key = self.registers[x] as usize;

                        if keypad.get_key(&key.into()) == 0 {
                            self.program_counter += 2;
                        }
                    }
                    _ => {
                        println!("Unknown 0xE000 opcode: {:#X}", opcode);
                    }
                }
            }
            0xF000 => {
                match opcode & 0xFF {
                    0x00 => {
                        // TODO: Load I extended
                        todo!("Load I extended")
                    }
                    0x01 => {
                        // TODO: Plane control
                        todo!("Plane control")
                    }
                    0x02 => {
                        // TODO: Audio control
                        todo!("Audio control")
                    }
                    0x07 => {
                        // Set Vx to the value of the delay timer
                        self.registers[x] = self.delay_timer;
                    }
                    0x0A => {
                        // Note: This doesn't feel right

                        let mut key_pressed = false;

                        for (i, key) in keypad.get_keys().iter().enumerate() {
                            if *key != 0 {
                                key_pressed = true;
                                self.registers[x] = i as u8;
                                break;
                            }
                        }

                        if !key_pressed {
                            self.program_counter -= 2;
                        }
                    }
                    0x15 => {
                        // Set the delay timer to Vx
                        self.delay_timer = self.registers[x];
                    }
                    0x18 => {
                        // Set the sound timer to Vx
                        self.sound_timer = self.registers[x];
                    }
                    0x1E => {
                        // Add Vx to the index register
                        self.index_register += self.registers[x] as u16;
                    }
                    0x29 => {
                        // Set I to the location of the sprite for the character in Vx
                        self.index_register = (self.registers[x] * 5) as u16;
                    }
                    0x30 => {
                        // TODO: Load I with big sprite
                        todo!("Load I with big sprite")
                    }
                    0x33 => {
                        // Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
                        memory[self.index_register as usize] = self.registers[x] / 100;
                        memory[(self.index_register + 1) as usize] = (self.registers[x] / 10) % 10;
                        memory[(self.index_register + 2) as usize] = (self.registers[x] % 100) % 10;
                    }
                    0x55 => {
                        // Store V0 to Vx in memory starting at address I
                        for i in 0..x + 1 {
                            memory[(self.index_register + i as u16) as usize] = self.registers[i];
                        }

                        // Quirk: Some programs expect I to be incremented
                        if quirks.i_incremented {
                            self.index_register += 1;
                        }
                    }
                    0x65 => {
                        // Read V0 to Vx from memory starting at address I
                        for i in 0..x + 1 {
                            self.registers[i] = memory[(self.index_register + i as u16) as usize];
                        }

                        // Quirk: Some programs expect I to be incremented
                        if quirks.i_incremented {
                            self.index_register += 1;
                        }
                    }
                    0x75 => {
                        // TODO: Save flags
                        todo!("Save flags")
                    }
                    0x85 => {
                        // TODO: Load flags
                        todo!("Load flags")
                    }
                    _ => {
                        println!("Unknown 0xF000 opcode: {:#X}", opcode);
                    }
                }
            }
            _ => {
                println!("Unknown opcode: {:#X}", opcode);
            }
        }
    }
}

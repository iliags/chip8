use egui::Key;
use rand::prelude::*;

use crate::app::keyboard::get_key_index;

/// Chip-8 Device
#[derive(Debug)]
pub struct C8 {
    /// The RAM (4kb)
    pub memory: Vec<u8>,

    /// The display of the device (64x32)
    pub display: Vec<u8>,

    /// Index register
    pub index_register: u16,

    /// Program counter
    pub program_counter: u16,

    /// Stack memory
    pub stack: Vec<u16>,

    /// Delay timer
    pub delay_timer: u8,

    /// Sound timer
    pub sound_timer: u8,

    /// General purpose registers
    pub registers: Vec<u8>,

    /// Whether the device is running
    pub is_running: bool,

    /// Keyboard state
    keyboard: [u8; 16],

    /// Quirks
    pub quirks: Quirks,
}

/// Quirks for the Chip-8 device
#[derive(Debug, Default, Clone, Copy)]
pub struct Quirks {
    /// Quirk: Some programs expect VF to be 0
    pub vf_zero: bool,

    /// Quirk: Some programs expect I to be incremented
    pub i_incremented: bool,

    /// Quirk: Some programs expect VX to be shifted directly without assigning VY
    pub vx_shifted_directly: bool,
    // TODO
    //display_waiting: bool,

    // TODO
    //clip_sprites: bool,
}

// Dead code is allowed here because:
// A) Removing unused registers would mandate manual register mapping
// B) Unused registers may be used in the future (i.e. for debugging or testing)
#[allow(dead_code)]
#[derive(Debug)]
enum Register {
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

/// Font data
const FONT: &'static [u8] = &[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// Screen width constant
pub const SCREEN_WIDTH: i32 = 64;

/// Screen height constant
pub const SCREEN_HEIGHT: i32 = 32;

/// Screen size constant
pub const SCREEN_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

impl Default for C8 {
    fn default() -> Self {
        Self {
            // 4kb of memory
            memory: vec![0; 4096],

            // 64x32 display
            display: vec![0; SCREEN_SIZE],
            index_register: 0,
            program_counter: 0x200,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            registers: vec![0; 16],
            is_running: false,
            keyboard: [0; 16],
            quirks: Quirks {
                vf_zero: true,
                i_incremented: true,
                vx_shifted_directly: true,
            },
        }
    }
}

impl C8 {
    /// Load font in the first 512 bytes of memory
    fn load_font(&mut self) {
        for (i, &byte) in FONT.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    /// Resets the device, loads ROM and font data into memory, and starts the device
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset_device();
        self.load_font();
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
        self.is_running = true;
    }

    /// Resets the device
    fn reset_device(&mut self) {
        self.memory = vec![0; 4096];
        self.display = vec![0; SCREEN_SIZE];
        self.index_register = 0;
        self.program_counter = 0x200;
        self.stack = vec![];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.registers = vec![0; 16];
        self.is_running = false;
        self.keyboard = [0; 16];
    }

    /// Uses i32 for x and y to allow for wrapping around the screen
    fn set_pixel(&mut self, x: i32, y: i32) -> u8 {
        // Quirk: Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen.
        // This may be implemented in the future with a toggle.

        // If the pixels are out of bounds, wrap them around
        let x = x % SCREEN_WIDTH;
        let y = y % SCREEN_HEIGHT;

        // Set the pixel
        let index = (y * SCREEN_WIDTH + x) as usize;

        // Pixels are XORed on the display
        self.display[index] ^= 1;

        // Return the pixel value
        self.display[index]
    }

    /// Set the state of a key
    pub fn set_key(&mut self, key: &Key, pressed: bool) {
        let key_index = match get_key_index(key) {
            Some(index) => index,
            None => {
                println!("Unknown key: {:?}", key);
                return;
            }
        };

        self.keyboard[key_index as usize] = pressed as u8;
    }

    /// Get the state of a key
    pub fn get_key(&self, key: &Key) -> bool {
        let key_index = match get_key_index(key) {
            Some(index) => index,
            None => {
                println!("Unknown key: {:?}", key);
                return false;
            }
        };

        self.keyboard[key_index as usize] == 1
    }

    /// Clear the screen, resets all pixels to 0.
    fn clear_screen(&mut self) {
        self.display = vec![0; SCREEN_SIZE];
    }

    /// Step the device
    pub fn step(&mut self, cpu_speed: u32) {
        if self.is_running {
            // Update timers
            if self.delay_timer > 0 {
                self.delay_timer = self.delay_timer.saturating_sub(1);
            }

            if self.sound_timer > 0 {
                self.sound_timer = self.sound_timer.saturating_sub(1);

                // TODO: Play sound
            }

            // Execute instructions
            for _ in 0..cpu_speed {
                const SHIFT: u8 = 8;

                let pc = self.program_counter as usize;
                let opcode = (self.memory[pc] as u16) << SHIFT | self.memory[pc + 1] as u16;

                // TODO: Move to a UI window
                /* println!(
                    "Executing opcode: {:#X} from {:#X}, {:#X}",
                    opcode,
                    (self.memory[pc] as u16) << SHIFT,
                    self.memory[pc + 1] as u16
                ); */

                self.program_counter += 2;

                self.execute_instruction(opcode);
            }
        }
    }

    fn execute_instruction(&mut self, opcode: u16) {
        // Extract the opcode parts
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        // Decode the opcode
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        // Clear the display
                        self.clear_screen();
                    }
                    0x00EE => {
                        // Return from a subroutine
                        // TODO: Make this more graceful
                        self.program_counter = self
                            .stack
                            .pop()
                            .unwrap_or_else(|| panic!("Stack underflow"));
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
                self.stack.push(self.program_counter);
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
                        if self.quirks.vf_zero {
                            self.registers[Register::VF as usize] = 0;
                        }
                    }
                    0x2 => {
                        // Set Vx = Vx AND Vy
                        self.registers[x] &= self.registers[y];

                        // Quirk: Some programs expect VF to be 0
                        if self.quirks.vf_zero {
                            self.registers[Register::VF as usize] = 0;
                        }
                    }
                    0x3 => {
                        // Set Vx = Vx XOR Vy
                        self.registers[x] ^= self.registers[y];

                        // Quirk: Some programs expect VF to be 0
                        if self.quirks.vf_zero {
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
                        if self.quirks.vx_shifted_directly {
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
                        if self.quirks.vx_shifted_directly {
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
                    let pixel = self.memory[(self.index_register + row) as usize];

                    for col in 0..8 {
                        if (pixel & (0x80 >> col)) != 0 {
                            if self.set_pixel(x + col, y + row as i32) == 0 {
                                self.registers[0xF] = 1;
                            }
                        }
                    }
                }
            }
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                        let key = self.registers[x] as usize;

                        if self.keyboard[key] != 0 {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                        let key = self.registers[x] as usize;

                        if self.keyboard[key] == 0 {
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
                    0x07 => {
                        // Set Vx to the value of the delay timer
                        self.registers[x] = self.delay_timer;
                    }
                    0x0A => {
                        // Note: This doesn't feel right

                        let mut key_pressed = false;

                        for i in 0..16 {
                            if self.keyboard[i] != 0 {
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
                    0x33 => {
                        // Store the binary-coded decimal representation of Vx at the addresses I, I+1, and I+2
                        self.memory[self.index_register as usize] = self.registers[x] / 100;
                        self.memory[(self.index_register + 1) as usize] =
                            (self.registers[x] / 10) % 10;
                        self.memory[(self.index_register + 2) as usize] =
                            (self.registers[x] % 100) % 10;
                    }
                    0x55 => {
                        // Store V0 to Vx in memory starting at address I
                        for i in 0..x + 1 {
                            self.memory[(self.index_register + i as u16) as usize] =
                                self.registers[i];
                        }

                        // Quirk: Some programs expect I to be incremented
                        if self.quirks.i_incremented {
                            self.index_register += 1;
                        }
                    }
                    0x65 => {
                        // Read V0 to Vx from memory starting at address I
                        for i in 0..x + 1 {
                            self.registers[i] =
                                self.memory[(self.index_register + i as u16) as usize];
                        }

                        // Quirk: Some programs expect I to be incremented
                        if self.quirks.i_incremented {
                            self.index_register += 1;
                        }
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

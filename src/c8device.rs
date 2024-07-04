use bevy::prelude::*;
use rand::prelude::*;


// Implementation notes:
// - Using an array for the stack with a stack pointer is faster, but using a Vec is more flexible.

pub struct C8DevicePlugin;

const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;
const SCREEN_SIZE: usize = (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize) as usize;

pub struct C8Device {
    registers: [u8; 16],
    memory: [u8; 4096],
    display: [u8; SCREEN_SIZE],
    stack: Vec<u16>,
    index_register: u16,
    program_counter: u16,
    delay_timer: u8,
    sound_timer: u8,
}

// Dead code is allowed here because:
// A) Removing unused registers would mandate manual register mapping
// B) Unused registers may be used in the future (i.e. for debugging or testing)
#[allow(dead_code)]
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

// Font data
static FONT: &'static [u8] = &[
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

impl Plugin for C8DevicePlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, ui_example_system);
    }
}

impl Default for C8Device {
    fn default() -> Self {
        let mut memory = [0; 4096];

        // Load font into memory (first 512 bytes)
        for i in 0..FONT.len() {
            memory[i] = FONT[i];
        }

        C8Device {
            registers: [0; 16],
            memory,
            display: [0; SCREEN_SIZE],
            stack: Vec::new(),
            index_register: 0,
            program_counter: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }
}

impl C8Device
{
    // XORs the pixel at the given coordinates
    pub fn set_pixel(&mut self, x: i32, y: i32) { //-> u8 {
        // Wrap around the screen
        let x = x % SCREEN_WIDTH;
        let y = y % SCREEN_HEIGHT;

        // Set the pixel
        let index = (y * SCREEN_WIDTH + x) as usize;

        // XOR the pixel
        self.memory[index] ^= 1;

        // Return the pixel value
        //self.memory[index]
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            // TODO: Play sound

            if self.sound_timer == 0 {
                // TODO: Stop sound
            }
        }
    }

    pub fn execute_opcode(&mut self, opcode: u16) {
        // Extract the opcode parts
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        // Decode the opcode
        // TODO: Implement all opcodes
        match opcode & 0xF000 {
            0x0000 => {
                match opcode {
                    0x00E0 => {
                        // Clear the display
                        self.display = [0; SCREEN_SIZE];
                    }
                    0x00EE => {
                        // Return from a subroutine
                        // TODO: Make this more graceful
                        self.program_counter = self.stack.pop().unwrap_or_else(|| panic!("Stack underflow"));
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
                    }
                    0x2 => {
                        // Set Vx = Vx AND Vy
                        self.registers[x] &= self.registers[y];
                    }
                    0x3 => {
                        // Set Vx = Vx XOR Vy
                        self.registers[x] ^= self.registers[y];
                    }
                    // TODO: 0x4, 0x5, 0x6, 0x7, and 0xE have quirks associated with them
                    0x4 => {
                        // Set Vx = Vx + Vy, set VF = carry
                        let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = overflow as u8;
                    }
                    0x5 => {
                        // Set Vx = Vx - Vy, set VF = NOT borrow
                        let (result, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = !overflow as u8;
                    }
                    0x6 => {
                        // Set Vx = Vx SHR 1
                        self.registers[Register::VF as usize] = self.registers[x] & 0x1;
                        self.registers[x] >>= 1;
                    }
                    0x7 => {
                        // Set Vx = Vy - Vx, set VF = NOT borrow
                        let (result, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                        self.registers[x] = result;
                        self.registers[Register::VF as usize] = !overflow as u8;
                    }
                    0xE => {
                        // Set Vx = Vx SHL 1
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
                // Draw a sprite at position (Vx, Vy) with N bytes of sprite data starting at the address stored in the index register
                self.registers[Register::VF as usize] = 0;

                let x = self.registers[x] as i32;
                let y = self.registers[y] as i32;

                let sprite_height = n;

                for yline in 0..sprite_height {
                    let pixel = self.memory[(self.index_register + yline as u16) as usize];
                    
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.display[(x + xline + ((y + yline as i32) * 64)) as usize] == 1 {
                                self.registers[Register::VF as usize] = 1;
                            }

                            /*
                            if self.set_pixel(x + xline, y + yline as i32) == 0 {
                                self.registers[Register::VF as usize] = 1;
                            }*/
                        }
                    }
                }
            }
            0xE000 => {
                match opcode & 0xFF {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed
                        let key = self.registers[x] as usize;

                        if key == 0 {
                            self.program_counter += 2;
                        }
                    }
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed
                        let key = self.registers[x] as usize;

                        if key != 0 {
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
                        // TODO
                        // Wait for a key press, store the value of the key in Vx
                        //self.is_running = false;

                        // TODO: Wait for key press
                        println!("Waiting for key press (not implemented yet)");

                        //self.registers[x] = KEY_PRESSED;

                        //self.is_running = true;
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
                    }
                    0x65 => {
                        // Read V0 to Vx from memory starting at address I
                        for i in 0..x + 1 {
                            self.registers[i] =
                                self.memory[(self.index_register + i as u16) as usize];
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
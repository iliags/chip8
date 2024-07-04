use bevy_tasks::futures_lite::future;
use egui::{Color32, TextureOptions, Vec2};
use rand::prelude::*;
use rfd::AsyncFileDialog;
use std::sync::Arc;

pub struct App {
    memory: Vec<u8>,
    display: Vec<u8>,
    index_register: u16,
    program_counter: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: Vec<u8>,

    display_image: egui::ColorImage,
    display_handle: Option<egui::TextureHandle>,
    step_counter: f32,
    cpu_speed: u32,

    rom_file: Option<Vec<u8>>,
    is_running: bool,
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

const SCREEN_WIDTH: i32 = 64;
const SCREEN_HEIGHT: i32 = 32;
const STEP_INTERVAL: f32 = 0.0167; // 60 FPS

impl Default for App {
    fn default() -> Self {
        Self {
            // 4kb of memory
            memory: vec![0; 4096],

            // 64x32 display
            display: vec![0; 64 * 32],
            index_register: 0,
            program_counter: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            registers: vec![0; 16],
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::WHITE,
            ),
            display_handle: None,
            step_counter: 0.0,
            rom_file: None,
            cpu_speed: 500,
            is_running: false,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // Load font in the first 512 bytes of memory
    fn load_font(&mut self) {
        for (i, &byte) in FONT.iter().enumerate() {
            self.memory[i] = byte;
        }
    }

    // Loads ROM and font data into memory
    fn load_rom(&mut self, rom: Vec<u8>) {
        self.load_font();
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
    }

    // Using i32 for x and y to allow for wrapping around the screen
    fn set_pixel(&mut self, x: i32, y: i32) {
        // If the pixels are out of bounds, wrap them around
        let x = x % SCREEN_WIDTH;
        let y = y % SCREEN_HEIGHT;

        // Set the pixel
        let index = (y * SCREEN_WIDTH + x) as usize;

        // Pixels are XORed on the display
        self.display[index] ^= 1;
    }

    fn clear_screen(&mut self) {
        self.display = vec![0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
    }

    fn test_display(&mut self) {
        self.set_pixel(0, 0);
        self.set_pixel(10, 10);
        self.set_pixel(20, 20);

        self.update_display_image();
    }

    fn update_display_image(&mut self) {
        // Clear image
        //self.display_image.pixels = vec![Color32::BLACK; 64 * 32];

        for (i, &pixel) in self.display.iter().enumerate() {
            let color = if pixel == 1 {
                Color32::WHITE
            } else {
                Color32::BLACK
            };

            self.display_image.pixels[i] = color;
        }
    }

    fn step(&mut self, delta_time: f32) {
        if self.is_running {
            // Update timers
            if self.delay_timer > 0 {
                self.delay_timer = self.delay_timer.saturating_sub(1);

                if self.delay_timer == 0 {
                    println!("Delay timer at 0");
                }
            }

            if self.sound_timer > 0 {
                self.sound_timer = self.sound_timer.saturating_sub(1);

                if self.sound_timer == 0 {
                    println!("Sound timer at 0");
                }
            }

            // Execute instructions
            for _ in 0..self.cpu_speed {
                const SHIFT: u8 = 8;

                let pc = self.program_counter as usize;
                let opcode = (self.memory[pc] as u16) << SHIFT | self.memory[pc + 1] as u16;

                self.execute_instruction(opcode);
            }
        }

        // Update the image texture from the display data
        self.update_display_image();
    }

    fn execute_instruction(&mut self, opcode: u16) {
        // Extract the opcode parts
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let n = (opcode & 0x000F) as u8;
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
                        // TODO: Wait for a key press, store the value of the key in Vx
                        println!("Waiting for key press (not implemented yet)");

                        //self.program_counter -= 2;
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

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Step the emulator
            let delta_time = ctx.input(|i| i.stable_dt);
            self.step(delta_time);

            // Draw the UI
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");

                if !is_web {
                    ui.menu_button("File", |ui| {
                        // No File->Quit on web pages
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
                if ui.button("Open ROM").clicked() {
                    // TODO: Open a ROM file

                    self.rom_file = future::block_on(async {
                        let file = AsyncFileDialog::new()
                            .add_filter("Chip8", &["ch8"])
                            .set_directory("/")
                            .pick_file()
                            .await;

                        ui.close_menu();

                        if let Some(file) = file {
                            let file = file.read().await;
                            Some(file)
                        } else {
                            None
                        }
                    });

                    if self.rom_file.is_some() {
                        println!("ROM file loaded");
                        self.load_rom(self.rom_file.as_ref().unwrap().clone());
                        self.is_running = true;

                        //println!("Memory {:?}", self.memory);
                    }
                }
                if ui.button("Test Display").clicked() {
                    self.test_display();
                }
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Display")
                .resizable(true)
                .show(ctx, |ui| {
                    self.display_handle = Some(ctx.load_texture(
                        "DisplayTexture",
                        egui::ImageData::Color(Arc::new(self.display_image.clone())),
                        TextureOptions {
                            magnification: egui::TextureFilter::Nearest,
                            minification: egui::TextureFilter::Nearest,
                            wrap_mode: egui::TextureWrapMode::ClampToEdge,
                        },
                    ));

                    let image = egui::Image::new(self.display_handle.as_ref().unwrap())
                        .fit_to_exact_size(Vec2::new(512.0, 256.0));

                    ui.add(image);
                });

            // "Powered by" text
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        // TODO: Only request a repaint if a ROM is loaded.
        // Refresh the UI
        ctx.request_repaint();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

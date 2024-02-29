use egui::{Color32, TextureOptions, Vec2};
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
}

enum Registers {
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
            stack: vec![0],
            delay_timer: 0,
            sound_timer: 0,
            registers: vec![0; 16],
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            display_handle: None,
            step_counter: 0.0,
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

    // Load ROM into memory after the font data
    fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, &byte) in rom.iter().enumerate() {
            self.memory[i + 512] = byte;
        }
    }

    // Using i32 for x and y to allow for wrapping around the screen
    fn set_pixel(&mut self, x: i32, y: i32) {
        // If the pixels are out of bounds, wrap them around
        let mut pos_x = x;
        let mut pos_y = y;

        if x > SCREEN_WIDTH {
            pos_x -= SCREEN_WIDTH;
        } else if x < 0 {
            pos_x += SCREEN_WIDTH;
        }

        if y > SCREEN_HEIGHT {
            pos_y -= SCREEN_HEIGHT;
        } else if y < 0 {
            pos_y += SCREEN_HEIGHT;
        }

        let index = (pos_x + pos_y * 64) as usize;

        // Pixels are XORed on the display
        self.display[index] ^= 1;
    }

    fn clear_screen(&mut self) {
        self.display = vec![0; 64 * 32];
    }

    fn test_display(&mut self) {
        use rand::Rng;
        for _ in 0..100 {
            let x = rand::thread_rng().gen_range(0..64);
            let y = rand::thread_rng().gen_range(0..32);
            self.set_pixel(x, y);
        }
    }

    fn update_display_image(&mut self) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = (x + y * 64) as usize;
                let color = if self.display[index] == 1 {
                    Color32::WHITE
                } else {
                    Color32::BLACK
                };

                self.display_image.pixels[index] = color;
            }
        }
    }

    fn step(&mut self, delta_time: f32) {
        self.step_counter += delta_time;

        if self.step_counter >= STEP_INTERVAL {
            self.step_counter = 0.0;

            // Do stuff
        }
        // Update the image texture from the display data
        self.test_display();
        self.update_display_image();
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

                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM").clicked() {
                        // TODO: Open a ROM file
                    }

                    // No File->Quit on web pages
                    if !is_web {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Window::new("Display")
                .resizable(true)
                .show(ctx, |ui| {
                    // Create the display texture handle if it doesn't exist
                    if self.display_handle.is_none() {
                        self.display_handle = Some(ctx.load_texture(
                            "DisplayTexture",
                            egui::ImageData::Color(Arc::new(self.display_image.clone())),
                            TextureOptions {
                                magnification: egui::TextureFilter::Nearest,
                                minification: egui::TextureFilter::Nearest,
                                wrap_mode: egui::TextureWrapMode::ClampToEdge,
                            },
                        ));
                    }

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

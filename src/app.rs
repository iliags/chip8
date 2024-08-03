use crate::c8::*;
use bevy_tasks::futures_lite::future;
use egui::{Color32, Key, TextureOptions, Vec2};
use rfd::AsyncFileDialog;
use std::{default, sync::Arc};

const DEFAULT_CPU_SPEED: u32 = 50;

pub struct App {
    display_image: egui::ColorImage,
    display_handle: Option<egui::TextureHandle>,
    cpu_speed: u32,

    rom_file: Option<Vec<u8>>,

    c8_device: C8,

    pixel_colors: PixelColors,
}

#[derive(Debug)]
struct PixelColors {
    on: Color32,
    off: Color32,
}

impl default::Default for PixelColors {
    fn default() -> Self {
        Self {
            on: Color32::WHITE,
            off: Color32::BLACK,
        }
    }
}

#[allow(dead_code)]
impl PixelColors {
    fn get_color(&self, pixel: u8) -> Color32 {
        if pixel == 1 {
            self.on
        } else {
            self.off
        }
    }

    fn set_on_color(&mut self, color: Color32) {
        self.on = color;
    }

    fn set_off_color(&mut self, color: Color32) {
        self.off = color;
    }
}

static KEYBOARD: &[Key] = &[
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Q,
    Key::W,
    Key::E,
    Key::R,
    Key::A,
    Key::S,
    Key::D,
    Key::F,
    Key::Z,
    Key::X,
    Key::C,
    Key::V,
];

impl Default for App {
    fn default() -> Self {
        Self {
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            display_handle: None,
            rom_file: None,
            cpu_speed: DEFAULT_CPU_SPEED,
            c8_device: C8::default(),
            pixel_colors: PixelColors::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    /// Update the display image with the current display buffer
    fn update_display_image(&mut self) {
        for (i, &pixel) in self.c8_device.display.iter().enumerate() {
            self.display_image.pixels[i] = self.pixel_colors.get_color(pixel);
        }
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Step the emulator
            self.c8_device.step(self.cpu_speed);
            self.update_display_image();

            // Process input
            for key in KEYBOARD {
                ctx.input(|i| self.c8_device.set_key(key, i.key_down(*key)));
            }

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

                ui.separator();

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
                        self.c8_device
                            .load_rom(self.rom_file.as_ref().unwrap().clone());

                        println!("ROM loaded");
                    }
                }

                ui.separator();

                // TODO: Disable if no rom is loaded
                if ui.button("Reload ROM").clicked() {
                    if self.rom_file.is_some() {
                        self.c8_device
                            .load_rom(self.rom_file.as_ref().unwrap().clone());

                        println!("ROM reloaded");
                    }
                }

                ui.separator();

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

            egui::Window::new("Controls").show(ctx, |ui| {
                ui.label("CPU Speed");
                ui.add(egui::Slider::new(&mut self.cpu_speed, 1..=100).text("Speed"));

                if ui.button("Default Speed").clicked() {
                    self.cpu_speed = DEFAULT_CPU_SPEED;
                }

                ui.separator();

                ui.label("Colors");

                // TODO: Add color picker

                ui.separator();

                ui.label("Keyboard");

                egui::Grid::new("keyboard_grid")
                    //.spacing(Vec2::new(20.0, 3.0))
                    .show(ui, |ui| {
                        // TODO: Change into a grid with button highlighting
                        for i in 0..KEYBOARD.len() {
                            let key = KEYBOARD[i];
                            let key_down = self.c8_device.get_key(&key);
                            // Slight hack because spacing doesn't work as expected
                            let key_down = if key_down { "Down" } else { "Up      " };
                            ui.label(format!("{:?}: {}", key, key_down));

                            if i % 4 == 3 {
                                ui.end_row();
                            }
                        }
                    });
            });

            // "Powered by" text
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

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

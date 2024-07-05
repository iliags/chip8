use crate::c8::*;
use bevy_tasks::futures_lite::future;
use egui::{Color32, TextureOptions, Vec2};
use rfd::AsyncFileDialog;
use std::sync::Arc;

pub struct App {
    display_image: egui::ColorImage,
    display_handle: Option<egui::TextureHandle>,
    cpu_speed: u32,

    rom_file: Option<Vec<u8>>,

    c8_device: C8,
}

impl Default for App {
    fn default() -> Self {
        Self {
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            display_handle: None,
            rom_file: None,
            cpu_speed: 50,
            c8_device: C8::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    fn update_display_image(&mut self) {
        // Clear image
        for (i, &pixel) in self.c8_device.display.iter().enumerate() {
            let color = if pixel == 1 {
                Color32::WHITE
            } else {
                Color32::BLACK
            };

            self.display_image.pixels[i] = color;
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
                        self.c8_device
                            .load_rom(self.rom_file.as_ref().unwrap().clone());

                        println!("ROM loaded");
                    }
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

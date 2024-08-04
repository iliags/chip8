use crate::{
    c8::*,
    localization::{LANG, LOCALES},
    roms::TEST_ROMS,
};
use egui::{color_picker::color_picker_color32, Color32, TextureOptions, Vec2};
use fluent_templates::Loader;
use rfd::AsyncFileDialog;
use std::sync::Arc;

use super::{
    keyboard::{get_key_name, KEYBOARD},
    pixel_color::PixelColors,
};

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

#[cfg(not(target_arch = "wasm32"))]
use bevy_tasks::futures_lite::future;

const DEFAULT_CPU_SPEED: u32 = 50;

const DEFAULT_DISPLAY_SIZE: Vec2 = Vec2::new(512.0, 256.0);

const DEFAULT_DISPLAY_SCALE: f32 = 1.0;

/// The application state
pub struct App {
    /// The image used to display the video memory
    display_image: egui::ColorImage,

    /// The handle to the display texture
    display_handle: Option<egui::TextureHandle>,

    /// The CPU speed
    cpu_speed: u32,

    /// The ROM file
    rom_file: Option<Vec<u8>>,

    /// The Chip8 device
    c8_device: C8,

    /// The pixel colors
    pixel_colors: PixelColors,

    /// The display scale
    display_scale: f32,

    // TODO: Use this for both web and native
    #[cfg(target_arch = "wasm32")]
    file_data: Rc<RefCell<Option<Vec<u8>>>>,
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
            cpu_speed: DEFAULT_CPU_SPEED,
            c8_device: C8::default(),
            pixel_colors: PixelColors::default(),
            display_scale: DEFAULT_DISPLAY_SCALE,

            #[cfg(target_arch = "wasm32")]
            file_data: Rc::new(RefCell::new(None)),
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
            self.display_image.pixels[i] = self.pixel_colors.get_color(pixel).clone();
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
                ui.menu_button(LOCALES.lookup(&LANG, "test_roms"), |ui| {
                    for rom in TEST_ROMS.iter() {
                        if ui.button(rom.get_name()).clicked() {
                            self.rom_file = Some(rom.get_data().to_vec());
                            self.c8_device
                                .load_rom(self.rom_file.as_ref().unwrap().clone());
                            println!("ROM loaded: {}", rom.get_name());
                            ui.close_menu();
                            break;
                        }
                    }
                });

                ui.separator();

                if ui.button(LOCALES.lookup(&LANG, "open_rom")).clicked() {
                    let task = AsyncFileDialog::new()
                        .add_filter("Chip8", &["ch8"])
                        .set_directory("/")
                        .pick_file();

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        self.rom_file = future::block_on(async {
                            let file = task.await;

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
                        } else {
                            println!("No ROM selected");
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        // Clone the file data for the async block
                        let data_clone = Rc::clone(&self.file_data.clone());

                        wasm_bindgen_futures::spawn_local(async move {
                            let file = task.await;

                            let file_data = match file {
                                Some(file) => {
                                    let file = file.read().await;
                                    Some(file)
                                }
                                None => None,
                            };

                            // Update the shared state
                            *data_clone.borrow_mut() = file_data;
                        });
                    }

                    ui.close_menu();
                }

                #[cfg(target_arch = "wasm32")]
                {
                    match self.file_data.take() {
                        Some(file_data) => {
                            self.rom_file = Some(file_data);
                            self.c8_device
                                .load_rom(self.rom_file.as_ref().unwrap().clone());
                            self.file_data = Rc::new(RefCell::new(None));
                        }
                        None => {}
                    }
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.rom_file.is_some(),
                        egui::Button::new(LOCALES.lookup(&LANG, "reload_rom")),
                    )
                    .clicked()
                {
                    self.c8_device
                        .load_rom(self.rom_file.as_ref().unwrap().clone());
                    println!("ROM reloaded");
                }

                ui.separator();

                egui::widgets::global_dark_light_mode_buttons(ui);

                ui.separator();
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "LeftPanel").show(ctx, |ui| {
            ui.add_space(5.0);

            egui::CollapsingHeader::new(LOCALES.lookup(&LANG, "cpu_speed")).show(ui, |ui| {
                ui.add(
                    egui::Slider::new(&mut self.cpu_speed, 1..=100)
                        .text(LOCALES.lookup(&LANG, "speed")),
                );

                if ui.button(LOCALES.lookup(&LANG, "default_speed")).clicked() {
                    self.cpu_speed = DEFAULT_CPU_SPEED;
                }
            });

            ui.separator();

            egui::CollapsingHeader::new(LOCALES.lookup(&LANG, "display")).show(ui, |ui| {
                ui.add(
                    egui::Slider::new(&mut self.display_scale, 0.5..=2.0)
                        .text(LOCALES.lookup(&LANG, "scale")),
                );

                if ui.button(LOCALES.lookup(&LANG, "default_scale")).clicked() {
                    self.display_scale = DEFAULT_DISPLAY_SCALE;
                }
            });

            ui.separator();

            egui::CollapsingHeader::new(LOCALES.lookup(&LANG, "pixel_colors")).show(ui, |ui| {
                // TODO: Make this look nicer
                if ui.button(LOCALES.lookup(&LANG, "default_colors")).clicked() {
                    self.pixel_colors = PixelColors::default();
                }

                ui.label(LOCALES.lookup(&LANG, "pixel_on"));

                color_picker_color32(
                    ui,
                    &mut self.pixel_colors.get_on_color_mut(),
                    egui::color_picker::Alpha::Opaque,
                );

                ui.separator();

                ui.label(LOCALES.lookup(&LANG, "pixel_off"));
                color_picker_color32(
                    ui,
                    &mut self.pixel_colors.get_off_color_mut(),
                    egui::color_picker::Alpha::Opaque,
                );
            });

            ui.separator();

            egui::CollapsingHeader::new(LOCALES.lookup(&LANG, "keyboard")).show(ui, |ui| {
                egui::Grid::new("keyboard_grid").show(ui, |ui| {
                    for i in 0..KEYBOARD.len() {
                        let key = KEYBOARD[i];
                        let key_down = self.c8_device.get_key(&key);
                        let key_name = get_key_name(&key);
                        let text = format!("{}", key_name);

                        if key_down {
                            let background_color = if ui.ctx().style().visuals.dark_mode {
                                Color32::DARK_GRAY
                            } else {
                                Color32::LIGHT_GRAY
                            };

                            ui.label(egui::RichText::new(text).background_color(background_color));
                        } else {
                            ui.label(text);
                        }

                        if i % 4 == 3 {
                            ui.end_row();
                        }
                    }
                });
            });

            ui.separator();

            egui::CollapsingHeader::new(LOCALES.lookup(&LANG, "quirks")).show(ui, |ui| {
                ui.checkbox(
                    &mut self.c8_device.quirks.vf_zero,
                    LOCALES.lookup(&LANG, "quirk_vf0"),
                )
                .on_hover_text(LOCALES.lookup(&LANG, "quirk_vf0_hover"));
                ui.checkbox(
                    &mut self.c8_device.quirks.i_incremented,
                    LOCALES.lookup(&LANG, "quirk_i"),
                )
                .on_hover_text(LOCALES.lookup(&LANG, "quirk_i_hover"));
                ui.checkbox(
                    &mut self.c8_device.quirks.vx_shifted_directly,
                    LOCALES.lookup(&LANG, "quirk_set_vxvy"),
                )
                .on_hover_text(LOCALES.lookup(&LANG, "quirk_set_vxvy_hover"));
            });

            // "Powered by" text
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.hyperlink_to(
                    LOCALES.lookup(&LANG, "source"),
                    "https://github.com/iliags/chip8",
                );
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Window::new(LOCALES.lookup(&LANG, "display"))
                .resizable(true)
                .show(ctx, |ui| {
                    // Note: This is hacky
                    let image_data = egui::ImageData::Color(Arc::new(self.display_image.clone()));

                    let texture_options = TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                        wrap_mode: egui::TextureWrapMode::ClampToEdge,
                    };

                    match &mut self.display_handle {
                        Some(handle) => {
                            handle.set(image_data, texture_options);
                        }
                        None => {
                            self.display_handle = Some(ctx.load_texture(
                                "DisplayTexture",
                                image_data,
                                texture_options,
                            ));
                        }
                    }

                    let image = egui::Image::new(self.display_handle.as_ref().unwrap())
                        .fit_to_exact_size(DEFAULT_DISPLAY_SIZE * self.display_scale);

                    ui.add(image);
                });
        });

        // Refresh the UI
        ctx.request_repaint();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        ui.label(LOCALES.lookup(&LANG, "powered_by"));
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");

        ui.label(LOCALES.lookup(&LANG, "and"));
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

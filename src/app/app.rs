use super::{
    keyboard::{get_key_name, KEYBOARD},
    pixel_color::PixelColors,
};
use crate::{
    device::{
        c8::*,
        display::{SCREEN_HEIGHT, SCREEN_WIDTH},
    },
    localization::{Languages, LANGUAGE_LIST, LOCALES},
    roms::TEST_ROMS,
};
use egui::{color_picker::color_picker_color32, Color32, TextureOptions, Vec2};
use fluent_templates::Loader;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use unic_langid::LanguageIdentifier;

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
    rom_file: Vec<u8>,

    /// The Chip8 device
    c8_device: C8,

    /// The pixel colors
    pixel_colors: PixelColors,

    /// The display scale
    display_scale: f32,

    /// File data used when loading the ROM
    file_data: Rc<RefCell<Option<Vec<u8>>>>,

    /// The current language the app is using
    current_language: Languages,
}

impl Default for App {
    fn default() -> Self {
        Self {
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            display_handle: None,
            rom_file: Vec::new(),
            cpu_speed: DEFAULT_CPU_SPEED,
            c8_device: C8::default(),
            pixel_colors: PixelColors::default(),
            display_scale: DEFAULT_DISPLAY_SCALE,
            file_data: Rc::new(RefCell::new(None)),

            // Current language
            current_language: Languages::English,
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
        for (i, &pixel) in self.c8_device.display.get_pixels().iter().enumerate() {
            self.display_image.pixels[i] = self.pixel_colors.get_color(pixel).clone();
        }
    }

    fn load_rom(&mut self, rom_data: Vec<u8>) {
        if rom_data.is_empty() {
            println!("ROM data is empty");
            return;
        }

        // Assign the rom data to the rom file copy
        self.rom_file = rom_data.clone();

        self.c8_device.load_rom(self.rom_file.clone());
    }

    fn reload_rom(&mut self) {
        self.c8_device.load_rom(self.rom_file.clone());
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
                ui.menu_button(
                    LOCALES.lookup(&self.current_language.value(), "test_roms"),
                    |ui| {
                        for rom in TEST_ROMS.iter() {
                            if ui.button(rom.get_name()).clicked() {
                                self.load_rom(rom.get_data().to_vec());

                                println!("ROM loaded: {}", rom.get_name());

                                // Close the menu
                                ui.close_menu();

                                // Break out of the loop
                                break;
                            }
                        }
                    },
                );

                ui.separator();

                if ui
                    .button(LOCALES.lookup(&self.current_language.value(), "open_rom"))
                    .clicked()
                {
                    // Clone the file data reference
                    let data_clone = Rc::clone(&self.file_data.clone());

                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        use bevy_tasks::futures_lite::future;

                        future::block_on(async move {
                            let file_data = load_file().await;

                            // Update the shared state
                            *data_clone.borrow_mut() = file_data;
                        });
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        wasm_bindgen_futures::spawn_local(async move {
                            let file_data = load_file().await;

                            // Update the shared state
                            *data_clone.borrow_mut() = file_data;
                        });
                    }
                }

                // Check if the file data has been updated
                match self.file_data.take() {
                    Some(file_data) => {
                        // Load the ROM
                        self.load_rom(file_data);

                        // Reset the file data
                        self.file_data = Rc::new(RefCell::new(None));
                    }
                    None => {}
                }

                ui.separator();

                if ui
                    .add_enabled(
                        self.rom_file.len() > 0,
                        egui::Button::new(
                            LOCALES.lookup(&self.current_language.value(), "reload_rom"),
                        ),
                    )
                    .clicked()
                {
                    self.reload_rom();
                }

                ui.separator();

                egui::widgets::global_dark_light_mode_buttons(ui);

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    ui.menu_button(
                        LOCALES.lookup(&self.current_language.value(), "about"),
                        |ui| {
                            let version_label = format!(
                                "{}{}",
                                LOCALES.lookup(&self.current_language.value(), "version"),
                                env!("CARGO_PKG_VERSION")
                            );
                            ui.label(version_label);

                            ui.separator();

                            ui.hyperlink_to(
                                LOCALES.lookup(&self.current_language.value(), "source"),
                                "https://github.com/iliags/chip8",
                            );

                            ui.separator();

                            powered_by_egui_and_eframe(ui, &self.current_language.value());

                            #[cfg(debug_assertions)]
                            {
                                ui.separator();

                                egui::warn_if_debug_build(ui);
                            }
                        },
                    );
                });
            });
        });

        egui::SidePanel::new(egui::panel::Side::Left, "LeftPanel").show(ctx, |ui| {
            ui.add_space(5.0);

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "cpu_speed"),
                )
                .show(ui, |ui| {
                    ui.add(
                        egui::Slider::new(&mut self.cpu_speed, 1..=100)
                            .text(LOCALES.lookup(&self.current_language.value(), "speed")),
                    );

                    if ui
                        .button(LOCALES.lookup(&self.current_language.value(), "default_speed"))
                        .clicked()
                    {
                        self.cpu_speed = DEFAULT_CPU_SPEED;
                    }
                });

                ui.separator();

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "display"),
                )
                .show(ui, |ui| {
                    ui.add(
                        egui::Slider::new(&mut self.display_scale, 0.5..=2.0)
                            .text(LOCALES.lookup(&self.current_language.value(), "scale")),
                    );

                    if ui
                        .button(LOCALES.lookup(&self.current_language.value(), "default_scale"))
                        .clicked()
                    {
                        self.display_scale = DEFAULT_DISPLAY_SCALE;
                    }
                });

                ui.separator();

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "pixel_colors"),
                )
                .show(ui, |ui| {
                    // TODO: Make this look nicer
                    if ui
                        .button(LOCALES.lookup(&self.current_language.value(), "default_colors"))
                        .clicked()
                    {
                        self.pixel_colors = PixelColors::default();
                    }

                    ui.label(LOCALES.lookup(&self.current_language.value(), "pixel_on"));

                    color_picker_color32(
                        ui,
                        &mut self.pixel_colors.get_on_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.separator();

                    ui.label(LOCALES.lookup(&self.current_language.value(), "pixel_off"));
                    color_picker_color32(
                        ui,
                        &mut self.pixel_colors.get_off_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.separator();

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "keyboard"),
                )
                .show(ui, |ui| {
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

                                ui.label(
                                    egui::RichText::new(text).background_color(background_color),
                                );
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

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "quirks"),
                )
                .show(ui, |ui| {
                    ui.checkbox(
                        &mut self.c8_device.quirks.vf_zero,
                        LOCALES.lookup(&self.current_language.value(), "quirk_vf0"),
                    )
                    .on_hover_text(
                        LOCALES.lookup(&self.current_language.value(), "quirk_vf0_hover"),
                    );
                    ui.checkbox(
                        &mut self.c8_device.quirks.i_incremented,
                        LOCALES.lookup(&self.current_language.value(), "quirk_i"),
                    )
                    .on_hover_text(LOCALES.lookup(&self.current_language.value(), "quirk_i_hover"));
                    ui.checkbox(
                        &mut self.c8_device.quirks.vx_shifted_directly,
                        LOCALES.lookup(&self.current_language.value(), "quirk_set_vxvy"),
                    )
                    .on_hover_text(
                        LOCALES.lookup(&self.current_language.value(), "quirk_set_vxvy_hover"),
                    );
                });

                ui.separator();

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "emulator"),
                )
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        egui::ComboBox::from_label(
                            LOCALES.lookup(&self.current_language.value(), "language"),
                        )
                        .selected_text(self.current_language.as_str())
                        .show_ui(ui, |ui| {
                            for language in LANGUAGE_LIST {
                                ui.selectable_value(
                                    &mut self.current_language,
                                    language.clone(),
                                    language.as_str(),
                                );
                            }
                        });
                    });
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |_ui| {
            egui::Window::new(LOCALES.lookup(&self.current_language.value(), "display"))
                .resizable(true)
                .show(ctx, |ui| {
                    // Note: This is hacky
                    // TODO: Figure out how to do this without cloning the image
                    let image_data = egui::ImageData::Color(Arc::new(self.display_image.clone()));

                    const TEXTURE_OPTIONS: TextureOptions = TextureOptions {
                        magnification: egui::TextureFilter::Nearest,
                        minification: egui::TextureFilter::Nearest,
                        wrap_mode: egui::TextureWrapMode::ClampToEdge,
                    };

                    match &mut self.display_handle {
                        Some(handle) => {
                            handle.set(image_data, TEXTURE_OPTIONS);
                        }
                        None => {
                            self.display_handle = Some(ctx.load_texture(
                                "DisplayTexture",
                                image_data,
                                TEXTURE_OPTIONS,
                            ));
                        }
                    }

                    let image = egui::Image::new(self.display_handle.as_ref().unwrap())
                        .fit_to_exact_size(DEFAULT_DISPLAY_SIZE * self.display_scale);

                    ui.add(image);
                });
        });

        // By default, egui will only repaint if input is detected. This isn't
        // ideal for this application, so we request a repaint every frame.
        ctx.request_repaint();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui, language: &LanguageIdentifier) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        ui.label(LOCALES.lookup(&language, "powered_by"));
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");

        ui.label(LOCALES.lookup(&language, "and"));
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

async fn load_file() -> Option<Vec<u8>> {
    let file = AsyncFileDialog::new()
        .add_filter("Chip8", &["ch8"])
        .set_directory("/")
        .pick_file()
        .await;

    match file {
        Some(file) => {
            let file = file.read().await;
            Some(file)
        }
        None => None,
    }
}

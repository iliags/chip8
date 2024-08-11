use crate::roms::TEST_ROMS;

use super::{
    keyboard::{get_key_mapping, KEYBOARD},
    pixel_color::PixelColors,
};
use c8_device::{
    device::C8,
    display::{SCREEN_HEIGHT, SCREEN_WIDTH},
};
use c8_i18n::localization::{Languages, LANGUAGE_LIST, LOCALES};
use egui::{color_picker::color_picker_color32, Color32, TextureOptions, Vec2};
use fluent_templates::Loader;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use unic_langid::LanguageIdentifier;

const DEFAULT_CPU_SPEED: u32 = 60;

const DEFAULT_DISPLAY_SIZE: Vec2 = Vec2::new(512.0, 256.0);

const DEFAULT_DISPLAY_SCALE: f32 = 1.0;

/// The application state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppUI {
    // The image used to display the video memory
    #[serde(skip)]
    display_image: egui::ColorImage,

    // The handle to the display texture
    #[serde(skip)]
    display_handle: Option<egui::TextureHandle>,

    // File data used when loading the ROM
    //
    // This uses a RefCell to allow the async file dialog code to work on both
    // native and web platforms.
    #[serde(skip)]
    file_data: Rc<RefCell<Option<Vec<u8>>>>,

    // The ROM file
    #[serde(skip)]
    rom_file: Vec<u8>,

    // The Chip8 device
    #[serde(skip)]
    c8_device: C8,

    // The CPU speed
    cpu_speed: u32,

    // The pixel colors
    pixel_colors: PixelColors,

    // The display scale
    display_scale: f32,

    // The current language the app is using
    current_language: Languages,

    // Whether the control panel is expanded
    control_panel_expanded: bool,

    visualizer_panel_expanded: bool,
}

impl Default for AppUI {
    fn default() -> Self {
        Self {
            display_image: egui::ColorImage::new(
                [SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize],
                Color32::BLACK,
            ),
            display_handle: None,
            rom_file: Vec::new(),
            c8_device: C8::default(),
            cpu_speed: DEFAULT_CPU_SPEED,
            pixel_colors: PixelColors::default(),
            display_scale: DEFAULT_DISPLAY_SCALE,
            file_data: Rc::new(RefCell::new(None)),

            // Current language
            current_language: Languages::English,

            control_panel_expanded: true,
            visualizer_panel_expanded: false,
        }
    }
}

impl eframe::App for AppUI {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Step the emulator
            self.c8_device.step(self.cpu_speed);

            // Update the display image with the current display buffer
            for (i, &pixel) in self.c8_device.get_display().get_pixels().iter().enumerate() {
                self.display_image.pixels[i] = *self.pixel_colors.get_color(pixel);
            }

            // Process input
            for key in KEYBOARD {
                ctx.input(|i| {
                    let current_key = &get_key_mapping(key)
                        .unwrap_or_else(|| panic!("Key mapping not found for key: {:?}", key));

                    self.c8_device
                        .get_keypad_mut()
                        .set_key(current_key, i.key_down(*key))
                });
            }

            // Menu bar
            egui::menu::bar(ui, |ui| {
                ui.toggle_value(
                    &mut self.control_panel_expanded,
                    LOCALES.lookup(&self.current_language.value(), "control_panel"),
                );

                ui.toggle_value(
                    &mut self.visualizer_panel_expanded,
                    LOCALES.lookup(&self.current_language.value(), "visualizer_panel"),
                );

                ui.separator();

                // Test rom menu
                self.menu_test_roms(ui);

                ui.separator();

                // Open ROM button
                self.menu_open_rom(ui);

                // Check if the file data has been updated
                if let Some(file_data) = self.file_data.take() {
                    // Load the ROM
                    self.load_rom(file_data);

                    // Reset the file data
                    self.file_data = Rc::new(RefCell::new(None));
                }

                ui.separator();

                if ui
                    .add_enabled(
                        !self.rom_file.is_empty(),
                        egui::Button::new(
                            LOCALES.lookup(&self.current_language.value(), "reload_rom"),
                        ),
                    )
                    .clicked()
                {
                    self.reload_rom();
                }

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    // Global dark/light mode buttons
                    egui::widgets::global_dark_light_mode_buttons(ui);

                    ui.separator();

                    self.menu_about(ui);
                });
            });
        });

        // Control panel
        egui::SidePanel::new(egui::panel::Side::Left, "ControlPanel").show_animated(
            ctx,
            self.control_panel_expanded,
            |ui| {
                ui.add_space(5.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.controls_cpu_speed(ui);

                    ui.separator();

                    self.controls_display_scale(ui);

                    ui.separator();

                    self.controls_pixel_color(ui);

                    ui.separator();

                    self.controls_keyboard_grid(ui);

                    ui.separator();

                    self.controls_quirks(ui);

                    ui.separator();

                    self.controls_emulator(ui);

                    ui.separator();

                    self.controls_audio(ui);
                });
            },
        );

        // Central panel with display window
        egui::CentralPanel::default().show(ctx, |_ui| {
            self.update_display_window(ctx);
        });

        egui::SidePanel::new(egui::panel::Side::Right, "VisualizerPanel").show_animated(
            ctx,
            self.visualizer_panel_expanded,
            |ui| {
                self.visualizer_memory(ui);
                self.visualizer_registers(ui);
            },
        );

        // By default, egui will only repaint if input is detected. This isn't
        // ideal for this application, so we request a repaint every frame if running.
        if self.c8_device.get_is_running() {
            ctx.request_repaint();
        }
    }
}

impl AppUI {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where app-wide settings can be set, such as fonts and visuals.
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn update_display_window(&mut self, ctx: &egui::Context) {
        egui::Window::new(LOCALES.lookup(&self.current_language.value(), "display"))
            .resizable(false)
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
                        self.display_handle =
                            Some(ctx.load_texture("DisplayTexture", image_data, TEXTURE_OPTIONS));
                    }
                }

                let image = match &self.display_handle {
                    Some(handle) => egui::Image::new(handle),
                    None => {
                        panic!("Display handle is None, this should never happen");
                    }
                };

                ui.add(image.fit_to_exact_size(DEFAULT_DISPLAY_SIZE * self.display_scale));
            });
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

    fn menu_test_roms(&mut self, ui: &mut egui::Ui) {
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
    }

    fn menu_open_rom(&mut self, ui: &mut egui::Ui) {
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
                    let file_data = Self::load_file().await;

                    // Update the shared state
                    *data_clone.borrow_mut() = file_data;
                });
            }

            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::spawn_local(async move {
                    let file_data = Self::load_file().await;

                    // Update the shared state
                    *data_clone.borrow_mut() = file_data;
                });
            }
        }
    }

    fn menu_about(&self, ui: &mut egui::Ui) {
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

                Self::powered_by_egui_and_eframe(ui, &self.current_language.value());

                #[cfg(debug_assertions)]
                {
                    ui.separator();

                    egui::warn_if_debug_build(ui);
                }
            },
        );
    }

    fn controls_cpu_speed(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "cpu_speed"))
            .show(ui, |ui| {
                ui.add(
                    egui::Slider::new(&mut self.cpu_speed, 1..=240)
                        .text(LOCALES.lookup(&self.current_language.value(), "speed")),
                )
                .on_hover_text(LOCALES.lookup(&self.current_language.value(), "speed_hover"));

                if ui
                    .button(LOCALES.lookup(&self.current_language.value(), "default_speed"))
                    .clicked()
                {
                    self.cpu_speed = DEFAULT_CPU_SPEED;
                }
            });
    }

    fn controls_display_scale(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "display"))
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
    }

    fn controls_pixel_color(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "pixel_colors"))
            .show(ui, |ui| {
                // TODO: Make this look nicer
                if ui
                    .button(LOCALES.lookup(&self.current_language.value(), "default_colors"))
                    .clicked()
                {
                    self.pixel_colors = PixelColors::default();
                }

                //ui.label(LOCALES.lookup(&self.current_language.value(), "pixel_on"));
                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "pixel_on"),
                )
                .show(ui, |ui| {
                    color_picker_color32(
                        ui,
                        self.pixel_colors.get_on_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.separator();

                //ui.label(LOCALES.lookup(&self.current_language.value(), "pixel_off"));

                egui::CollapsingHeader::new(
                    LOCALES.lookup(&self.current_language.value(), "pixel_off"),
                )
                .show(ui, |ui| {
                    color_picker_color32(
                        ui,
                        self.pixel_colors.get_off_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );
                });
            });
    }

    fn controls_keyboard_grid(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "keyboard"))
            .show(ui, |ui| {
                egui::Grid::new("keyboard_grid").show(ui, |ui| {
                    for (i, key) in KEYBOARD.iter().enumerate() {
                        let key_down = self.c8_device.get_keypad().is_key_pressed(
                            &get_key_mapping(key).unwrap_or_else(|| {
                                panic!("Key mapping not found for key: {:?}", key)
                            }),
                        );

                        let key_name = match get_key_mapping(key) {
                            Some(key_pad) => key_pad.get_name().to_owned(),
                            None => "Unknown".to_owned(),
                        };

                        if key_down {
                            let background_color = if ui.ctx().style().visuals.dark_mode {
                                Color32::DARK_GRAY
                            } else {
                                Color32::LIGHT_GRAY
                            };

                            ui.label(
                                egui::RichText::new(key_name.to_string())
                                    .background_color(background_color),
                            );
                        } else {
                            ui.label(key_name.to_string());
                        }

                        if i % 4 == 3 {
                            ui.end_row();
                        }
                    }
                });
            });
    }

    fn controls_quirks(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "quirks")).show(
            ui,
            |ui| {
                ui.checkbox(
                    &mut self.c8_device.get_quirks_mut().vf_zero,
                    LOCALES.lookup(&self.current_language.value(), "quirk_vf0"),
                )
                .on_hover_text(LOCALES.lookup(&self.current_language.value(), "quirk_vf0_hover"));
                ui.checkbox(
                    &mut self.c8_device.get_quirks_mut().i_incremented,
                    LOCALES.lookup(&self.current_language.value(), "quirk_i"),
                )
                .on_hover_text(LOCALES.lookup(&self.current_language.value(), "quirk_i_hover"));
                ui.checkbox(
                    &mut self.c8_device.get_quirks_mut().vx_shifted_directly,
                    LOCALES.lookup(&self.current_language.value(), "quirk_set_vxvy"),
                )
                .on_hover_text(
                    LOCALES.lookup(&self.current_language.value(), "quirk_set_vxvy_hover"),
                );
            },
        );
    }

    fn controls_emulator(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "emulator"))
            .show(ui, |ui| {
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
    }

    fn controls_audio(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(
            LOCALES.lookup(&self.current_language.value(), "audio_controls"),
        )
        .show(ui, |ui| {
            #[cfg(target_arch = "wasm32")]
            ui.label(LOCALES.lookup(&self.current_language.value(), "under_construction"));

            // Disable on WASM for now
            #[cfg(not(target_arch = "wasm32"))]
            {
                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        self.c8_device.beeper.play();
                    }

                    if ui.button("Pause").clicked() {
                        self.c8_device.beeper.pause();
                    }

                    /*
                    if ui.button("Stop").clicked() {
                        self.c8_device.beeper.stop();
                    }
                     */
                });

                ui.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.c8_device.beeper.settings.volume, 0.0..=1.0)
                            .text(LOCALES.lookup(&self.current_language.value(), "volume")),
                    )
                    .on_hover_text(
                        LOCALES.lookup(&self.current_language.value(), "not_implemented"),
                    );

                    ui.add(
                        egui::Slider::new(
                            &mut self.c8_device.beeper.settings.pitch,
                            20.0..=20000.0,
                        )
                        .text(LOCALES.lookup(&self.current_language.value(), "pitch")),
                    )
                    .on_hover_text(
                        LOCALES.lookup(&self.current_language.value(), "not_implemented"),
                    );

                    ui.add(
                        egui::Slider::new(&mut self.c8_device.beeper.settings.octave, 1.0..=4.0)
                            .text(LOCALES.lookup(&self.current_language.value(), "octave")),
                    )
                    .on_hover_text(
                        LOCALES.lookup(&self.current_language.value(), "not_implemented"),
                    );
                });
            }
        });
    }

    fn visualizer_memory(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "memory")).show(
            ui,
            |ui| {
                ui.label(LOCALES.lookup(&self.current_language.value(), "under_construction"));
            },
        );
    }

    fn visualizer_registers(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(LOCALES.lookup(&self.current_language.value(), "registers"))
            .show(ui, |ui| {
                ui.label(LOCALES.lookup(&self.current_language.value(), "under_construction"));
            });
    }

    fn powered_by_egui_and_eframe(ui: &mut egui::Ui, language: &LanguageIdentifier) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            ui.label(LOCALES.lookup(language, "powered_by"));
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");

            ui.label(LOCALES.lookup(language, "and"));
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
    }

    async fn load_file() -> Option<Vec<u8>> {
        let file_task = AsyncFileDialog::new()
            .add_filter("Chip8", &["ch8"])
            .set_directory("/")
            .pick_file()
            .await;

        match file_task {
            Some(file) => {
                let file = file.read().await;
                Some(file)
            }
            None => None,
        }
    }
}

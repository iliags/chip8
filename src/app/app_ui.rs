use crate::roms::{GAME_ROMS, ROM, TEST_ROMS};

use super::{
    keyboard::{get_key_mapping, KEYBOARD},
    pixel_color::PixelColors,
};
use c8_device::{device::C8, display::DisplayResolution, fonts::FONT_DATA, message::DeviceMessage};
use c8_i18n::{
    locale_text::LocaleText,
    localization::{LANGUAGE_LIST, LOCALES},
};
use egui::{color_picker::color_picker_color32, Color32, TextureOptions, Vec2};
use fluent_templates::Loader;
use rfd::AsyncFileDialog;
use std::sync::Arc;
use std::{cell::RefCell, rc::Rc};
use unic_langid::LanguageIdentifier;

// 60 seems to be a good default, Octo uses 20
const DEFAULT_CPU_SPEED: u32 = 60;

const DEFAULT_DISPLAY_SIZE: Vec2 = Vec2::new(512.0, 256.0);

const DEFAULT_DISPLAY_SCALE: f32 = 1.0;

/// The application state
// TODO: Add rom-specific settings
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

    #[serde(skip)]
    file_name: Rc<RefCell<Option<String>>>,

    // The ROM file
    #[serde(skip)]
    rom_file: Vec<u8>,

    #[serde(skip)]
    rom_name: String,

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
    language: LocaleText,

    // Whether the control panel is expanded
    control_panel_expanded: bool,

    // Whether the visualizer panel is expanded
    visualizer_panel_expanded: bool,

    test_loaded: bool,
}

impl Default for AppUI {
    fn default() -> Self {
        let (width, height) = DisplayResolution::Low.get_resolution_size_xy();
        Self {
            display_image: egui::ColorImage::new([width, height], Color32::BLACK),
            display_handle: None,
            rom_file: Vec::new(),
            rom_name: String::new(),
            c8_device: C8::default(),
            cpu_speed: DEFAULT_CPU_SPEED,
            pixel_colors: PixelColors::default(),
            display_scale: DEFAULT_DISPLAY_SCALE,
            file_data: Rc::new(RefCell::new(None)),
            file_name: Rc::new(RefCell::new(None)),

            // Current language
            language: LocaleText::default(),

            control_panel_expanded: true,
            visualizer_panel_expanded: false,
            test_loaded: false,
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
            let messages = self.c8_device.step(self.cpu_speed);

            // Process messages
            for message in messages.iter() {
                match message {
                    DeviceMessage::ChangeResolution(_) => {
                        self.update_resolution();
                    }
                    DeviceMessage::Exit => {
                        println!("Exiting device");
                        self.unload_rom();
                    }
                    DeviceMessage::UnknownOpCode(_opcode) => {
                        // TODO: Push to a list
                        //println!("Unknown OpCode: {:#06X}", op_code);
                    }
                    _ => {}
                }
            }

            // Update the display image with the current display buffer
            // TODO: Handle planes and colors
            self.display_image.pixels = self
                .c8_device
                .get_display()
                .get_plane_pixels(0)
                .iter()
                .map(|&pixel| *self.pixel_colors.get_color(pixel))
                .collect();

            // Process input
            for key in KEYBOARD {
                ctx.input(|i| {
                    let current_key = &get_key_mapping(key)
                        .unwrap_or_else(|| panic!("Key mapping not found for key: {:?}", key));

                    // Temporary debug code
                    /*
                    #[cfg(debug_assertions)]
                    {

                        if i.key_pressed(egui::Key::Space) {
                            self.load_rom(TEST_ROMS[7].get_data().to_vec());
                            self.c8_device.get_memory_mut().data[0x1FF] = 3;
                        }

                    }
                    */

                    self.c8_device
                        .get_keypad_mut()
                        .set_key(current_key, i.key_down(*key))
                });
            }

            // Menu bar
            egui::menu::bar(ui, |ui| {
                ui.toggle_value(
                    &mut self.control_panel_expanded,
                    self.language.get_locale_string("control_panel"),
                );

                ui.toggle_value(
                    &mut self.visualizer_panel_expanded,
                    self.language.get_locale_string("visualizer_panel"),
                );

                ui.separator();

                self.menu_roms(ui);

                ui.separator();

                // Open ROM button
                self.menu_open_rom(ui);

                // Check if the file data has been updated
                if let Some(file_data) = self.file_data.take() {
                    // Update the file name
                    if let Some(file_name) = self.file_name.take() {
                        let name = file_name.strip_suffix(".ch8").unwrap_or(&file_name);
                        self.rom_name = name.to_string();

                        // Reset the file name
                        self.file_name = Rc::new(RefCell::new(None));
                    }

                    // Load the ROM
                    self.load_rom(file_data);

                    // Reset the file data
                    self.file_data = Rc::new(RefCell::new(None));
                }

                if ui
                    .add_enabled(
                        !self.rom_file.is_empty(),
                        egui::Button::new(self.language.get_locale_string("reload_rom")),
                    )
                    .clicked()
                {
                    self.reload_rom();
                }

                if ui
                    .add_enabled(
                        self.c8_device.get_is_running(),
                        egui::Button::new(self.language.get_locale_string("unload_rom")),
                    )
                    .clicked()
                {
                    self.unload_rom();
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

    fn update_resolution(&mut self) {
        let (width, height) = self
            .c8_device
            .get_display()
            .get_resolution()
            .get_resolution_size_xy();
        self.display_image = egui::ColorImage::new([width, height], Color32::BLACK);
    }

    fn update_display_window(&mut self, ctx: &egui::Context) {
        let display_title = if self.rom_name.is_empty() {
            self.language.get_locale_string("display")
        } else {
            self.rom_name.clone()
        };

        egui::Window::new(display_title)
            .resizable(false)
            .id("display_window".into())
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

    fn unload_rom(&mut self) {
        self.c8_device.reset_device();
    }

    fn menu_roms(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.language.get_locale_string("included_roms"), |ui| {
            // Test rom menu
            ui.menu_button(self.language.get_locale_string("test_roms"), |ui| {
                for rom in TEST_ROMS.iter() {
                    if self.menu_rom_button(ui, rom) {
                        break;
                    }
                }
            });

            ui.menu_button(self.language.get_locale_string("game_roms"), |ui| {
                for rom in GAME_ROMS.iter() {
                    if self.menu_rom_button(ui, rom) {
                        break;
                    }
                }
            });
        });
    }

    // Returns true if a ROM was selected
    fn menu_rom_button(&mut self, ui: &mut egui::Ui, rom: &ROM) -> bool {
        if ui.button(rom.get_name()).clicked() {
            self.load_rom(rom.get_data().to_vec());
            self.rom_name = rom.get_name().to_string();

            println!("ROM loaded: {}", rom.get_name());

            // Close the menu
            ui.close_menu();

            return true;
        }

        false
    }

    fn menu_open_rom(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(self.language.get_locale_string("open_rom"))
            .clicked()
        {
            // Clone the file data reference
            let data_clone = Rc::clone(&self.file_data.clone());
            let name_clone = Rc::clone(&self.file_name.clone());

            #[cfg(not(target_arch = "wasm32"))]
            {
                use bevy_tasks::futures_lite::future;

                future::block_on(async move {
                    let (file_data, file_name) = Self::load_file().await;

                    // Update the shared state
                    *data_clone.borrow_mut() = file_data;
                    *name_clone.borrow_mut() = file_name;
                });
            }

            #[cfg(target_arch = "wasm32")]
            {
                wasm_bindgen_futures::spawn_local(async move {
                    let (file_data, file_name) = Self::load_file().await;

                    // Update the shared state
                    *data_clone.borrow_mut() = file_data;
                    *name_clone.borrow_mut() = file_name;
                });
            }
        }
    }

    fn menu_about(&self, ui: &mut egui::Ui) {
        ui.menu_button(self.language.get_locale_string("about"), |ui| {
            let version_label = format!(
                "{}{}",
                self.language.get_locale_string("version"),
                env!("CARGO_PKG_VERSION")
            );
            ui.label(version_label);

            ui.separator();

            ui.hyperlink_to(
                self.language.get_locale_string("source"),
                "https://github.com/iliags/chip8",
            );

            ui.separator();

            Self::powered_by_egui_and_eframe(ui, &self.language.get_language().value());

            #[cfg(debug_assertions)]
            {
                ui.separator();

                egui::warn_if_debug_build(ui);
            }
        });
    }

    fn controls_cpu_speed(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("cpu_speed")).show(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut self.cpu_speed, 1..=240)
                    .text(self.language.get_locale_string("speed")),
            )
            .on_hover_text(self.language.get_locale_string("speed_hover"));

            if ui
                .button(self.language.get_locale_string("default_speed"))
                .clicked()
            {
                self.cpu_speed = DEFAULT_CPU_SPEED;
            }
        });
    }

    fn controls_display_scale(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("display")).show(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut self.display_scale, 0.5..=3.0)
                    .text(self.language.get_locale_string("scale")),
            );

            if ui
                .button(self.language.get_locale_string("default_scale"))
                .clicked()
            {
                self.display_scale = DEFAULT_DISPLAY_SCALE;
            }
        });
    }

    fn controls_pixel_color(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("pixel_colors")).show(
            ui,
            |ui| {
                // TODO: Make this look nicer
                if ui
                    .button(self.language.get_locale_string("default_colors"))
                    .clicked()
                {
                    self.pixel_colors = PixelColors::default();
                }

                //ui.label(self.language.get_locale_string( "pixel_on"));
                egui::CollapsingHeader::new(self.language.get_locale_string("pixel_on")).show(
                    ui,
                    |ui| {
                        color_picker_color32(
                            ui,
                            self.pixel_colors.get_on_color_mut(),
                            egui::color_picker::Alpha::Opaque,
                        );
                    },
                );

                ui.separator();

                //ui.label(self.language.get_locale_string( "pixel_off"));

                egui::CollapsingHeader::new(self.language.get_locale_string("pixel_off")).show(
                    ui,
                    |ui| {
                        color_picker_color32(
                            ui,
                            self.pixel_colors.get_off_color_mut(),
                            egui::color_picker::Alpha::Opaque,
                        );
                    },
                );
            },
        );
    }

    fn controls_keyboard_grid(&self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("keyboard")).show(ui, |ui| {
            egui::Grid::new("keyboard_grid").show(ui, |ui| {
                for (i, key) in KEYBOARD.iter().enumerate() {
                    let key_down = self.c8_device.get_keypad().is_key_pressed(
                        &get_key_mapping(key)
                            .unwrap_or_else(|| panic!("Key mapping not found for key: {:?}", key)),
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
        egui::CollapsingHeader::new(self.language.get_locale_string("quirks")).show(ui, |ui| {
            ui.checkbox(
                &mut self.c8_device.get_quirks_mut().vf_zero,
                self.language.get_locale_string("quirk_vf0"),
            )
            .on_hover_text(self.language.get_locale_string("quirk_vf0_hover"));
            ui.checkbox(
                &mut self.c8_device.get_quirks_mut().i_incremented,
                self.language.get_locale_string("quirk_i"),
            )
            .on_hover_text(self.language.get_locale_string("quirk_i_hover"));
            ui.checkbox(
                &mut self.c8_device.get_quirks_mut().vx_shifted_directly,
                self.language.get_locale_string("quirk_set_vxvy"),
            )
            .on_hover_text(self.language.get_locale_string("quirk_set_vxvy_hover"));
        });
    }

    fn controls_emulator(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("emulator")).show(ui, |ui| {
            // Emulator language
            egui::ComboBox::from_label(self.language.get_locale_string("language"))
                .selected_text(self.language.get_language().as_str())
                .show_ui(ui, |ui| {
                    for language in LANGUAGE_LIST {
                        ui.selectable_value(
                            &mut self.language.get_language_mut(),
                            &mut language.clone(),
                            language.as_str(),
                        );
                    }
                });

            // Emulator font
            // TODO: Move this to emulator settings
            let current_font_name: String = self.c8_device.get_memory().system_font.into();
            egui::ComboBox::from_label(self.language.get_locale_string("font_small"))
                .selected_text(current_font_name)
                .show_ui(ui, |ui| {
                    for font in FONT_DATA {
                        if font.small_data.is_empty() {
                            continue;
                        }

                        let font_string: String = font.name.into();

                        ui.selectable_label(
                            self.c8_device.get_memory_mut().system_font == font.name,
                            font_string,
                        )
                        .on_hover_text(self.language.get_locale_string("font_hover"))
                        .clicked()
                        .then(|| {
                            self.c8_device
                                .get_memory_mut()
                                .load_font_small(font.clone());
                        });
                    }
                });
        });
    }

    fn controls_audio(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("audio_controls")).show(
            ui,
            |ui| {
                //#[cfg(target_arch = "wasm32")]
                //ui.label(self.language.get_locale_string("under_construction"));

                #[cfg(not(debug_assertions))]
                ui.label(self.language.get_locale_string("under_construction"));

                // Disable for now
                //#[cfg(not(target_arch = "wasm32"))]
                #[cfg(debug_assertions)]
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
                            egui::Slider::new(
                                &mut self.c8_device.beeper.settings.volume,
                                0.0..=1.0,
                            )
                            .text(self.language.get_locale_string("volume")),
                        )
                        .on_hover_text(self.language.get_locale_string("not_implemented"));

                        ui.add(
                            egui::Slider::new(
                                &mut self.c8_device.beeper.settings.pitch,
                                20.0..=20000.0,
                            )
                            .text(self.language.get_locale_string("pitch")),
                        )
                        .on_hover_text(self.language.get_locale_string("not_implemented"));

                        ui.add(
                            egui::Slider::new(
                                &mut self.c8_device.beeper.settings.octave,
                                1.0..=4.0,
                            )
                            .text(self.language.get_locale_string("octave")),
                        )
                        .on_hover_text(self.language.get_locale_string("not_implemented"));
                    });
                }
            },
        );
    }

    fn visualizer_memory(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("memory")).show(ui, |ui| {
            ui.label(self.language.get_locale_string("under_construction"));
        });
    }

    fn visualizer_registers(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.get_locale_string("registers")).show(ui, |ui| {
            ui.label(self.language.get_locale_string("under_construction"));
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

    async fn load_file() -> (Option<Vec<u8>>, Option<String>) {
        let file_task = AsyncFileDialog::new()
            .add_filter("Chip8", &["ch8"])
            .set_directory("/")
            .pick_file()
            .await;

        // TODO: Check for an `options.json` file in the same directory and convert it to compatible settings

        match file_task {
            Some(file) => {
                let name = file.file_name();

                let file = file.read().await;
                (Some(file), Some(name))
            }
            None => (None, None),
        }
    }
}

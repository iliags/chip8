use crate::roms::{GAME_ROMS, ROM, TEST_ROMS};

use super::{
    is_mobile,
    keyboard::{KEY_MAPPINGS, KEYBOARD, KeyboardMapping},
    pixel_color::{PALETTES, PixelColors},
};

use c8::{
    audio::audio_settings::AudioSettings,
    device::C8,
    display::DisplayResolution,
    fonts::FONT_DATA,
    keypad::KEYPAD_KEYS,
    message::DeviceMessage,
    quirks::{COMPATIBILITY_PROFILES, CompatibilityProfile, Quirks},
};
use c8_i18n::{
    locale_text::LocaleText,
    localization::{LANGUAGE_LIST, LOCALES},
};
use egui::{Color32, TextureOptions, Vec2};
use fluent_templates::Loader;
use rfd::AsyncFileDialog;
use std::{cell::RefCell, rc::Rc};
use unic_langid::LanguageIdentifier;

// 60 seems to be a good default, Octo uses 20
const DEFAULT_CPU_SPEED: u32 = 20;

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

    #[serde(skip)]
    debug_window: bool,

    language: LocaleText,

    settings: Settings,
}

impl Default for AppUI {
    fn default() -> Self {
        let (width, height) = DisplayResolution::Low.resolution_size_xy();
        Self {
            display_image: egui::ColorImage::filled([width, height], Color32::BLACK),
            display_handle: None,
            rom_file: Vec::new(),
            rom_name: String::new(),
            c8_device: C8::default(),

            file_data: Rc::new(RefCell::new(None)),
            file_name: Rc::new(RefCell::new(None)),

            debug_window: false,

            language: LocaleText::default(),
            settings: Settings::default(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Settings {
    // The CPU speed
    cpu_speed: u32,

    // The pixel colors
    pixel_colors: PixelColors,

    // The display scale
    display_scale: f32,

    // Whether the control panel is expanded
    control_panel_expanded: bool,

    // Whether the visualizer panel is expanded
    visualizer_panel_expanded: bool,

    // Quirk settings
    quirk_settings: Quirks,

    // Display in fullscreen
    display_fullscreen: bool,

    // Display is drawn under the side panels
    draw_display_underneath: bool,

    key_mapping: KeyboardMapping,

    audio_settings: AudioSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            cpu_speed: DEFAULT_CPU_SPEED,
            pixel_colors: PixelColors::default(),
            display_scale: DEFAULT_DISPLAY_SCALE,

            control_panel_expanded: true,
            visualizer_panel_expanded: false,
            quirk_settings: Quirks::default(),

            display_fullscreen: false,
            draw_display_underneath: false,

            key_mapping: KeyboardMapping::default(),

            audio_settings: AudioSettings::default(),
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
        // Step the emulator
        let messages = self.c8_device.step(self.settings.cpu_speed);

        // Process messages
        for message in messages.iter() {
            match message {
                DeviceMessage::ChangeResolution(_) => {
                    self.update_resolution();
                }
                DeviceMessage::UnknownOpCode(_opcode) => {
                    // TODO: Push to a list
                    //println!("Unknown OpCode: {:#06X}", op_code);
                }
                _ => {}
            }
        }

        // Process debug input
        #[cfg(debug_assertions)]
        {
            ctx.input(|i| {
                // Load ROM shortcut for testing
                if i.key_pressed(egui::Key::Tab) {
                    // Chip-8 Logo
                    self.load_rom(TEST_ROMS[0].data().to_vec());

                    // Skyward
                    //self.load_rom(GAME_ROMS[8].data().to_vec());

                    // Music player 1
                    //self.load_rom(GAME_ROMS[9].data().to_vec());

                    // Music player 2
                    //self.load_rom(GAME_ROMS[10].data().to_vec());

                    // Nyancat
                    //self.load_rom(GAME_ROMS[11].data().to_vec());

                    // Beep
                    //self.load_rom(TEST_ROMS[6].data().to_vec());
                }
            });
        }

        // By default, egui will only repaint if input is detected. This isn't
        // ideal for this application, so we request a repaint every frame if running.
        if self.c8_device.is_running() {
            ctx.request_repaint();
        }

        if is_mobile(ctx) {
            // TODO: Portrait and landscape
            self.ui_mobile_portrait(ctx);
        } else {
            self.ui_desktop(ctx);
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

        let mut new_self = Self::default();
        new_self.settings.control_panel_expanded = !is_mobile(&cc.egui_ctx);

        new_self
    }

    fn update_resolution(&mut self) {
        let (width, height) = self.c8_device.display().resolution().resolution_size_xy();
        let bg_color = self.settings.pixel_colors.background_color();
        self.display_image = egui::ColorImage::filled([width, height], *bg_color);
    }

    fn reset_display(&mut self) {
        let (width, height) = DisplayResolution::Low.resolution_size_xy();
        let bg_color = self.settings.pixel_colors.background_color();
        self.display_image = egui::ColorImage::filled([width, height], *bg_color);
    }

    fn update_display_window(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Update the display image with the current display buffer
        // TODO: There is some minor color blending issues with the display, probably needs a buffer
        self.display_image.pixels = self
            .c8_device
            .display()
            .zipped_iterator()
            .map(|(&p0, &p1)| {
                let result = (p0 << 1) | p1;
                *self.settings.pixel_colors.pixel_color(result.into())
            })
            .collect();

        // Note: This is hacky
        const TEXTURE_OPTIONS: TextureOptions = TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            wrap_mode: egui::TextureWrapMode::ClampToEdge,
            mipmap_mode: None,
        };

        match &mut self.display_handle {
            Some(handle) => {
                handle.set(self.display_image.clone(), TEXTURE_OPTIONS);
            }
            None => {
                self.display_handle = Some(ctx.load_texture(
                    "DisplayTexture",
                    self.display_image.clone(),
                    TEXTURE_OPTIONS,
                ));
            }
        }

        let image = match &self.display_handle {
            Some(handle) => egui::Image::new(handle),
            None => {
                panic!("Display handle is None, this should never happen");
            }
        };

        if self.settings.display_fullscreen || is_mobile(ctx) {
            ui.add(image.fit_to_exact_size(ui.available_size()));
        } else {
            let display_title = if self.rom_name.is_empty() {
                self.language.locale_string("display")
            } else {
                self.rom_name.clone()
            };
            egui::Window::new(display_title)
                .resizable(false)
                .id("display_window".into())
                .show(ctx, |ui| {
                    ui.add(
                        image.fit_to_exact_size(DEFAULT_DISPLAY_SIZE * self.settings.display_scale),
                    );
                });
        }
    }

    fn load_rom(&mut self, rom_data: Vec<u8>) {
        if rom_data.is_empty() {
            eprintln!("ROM data is empty");
            return;
        }

        self.reset_display();
        self.c8_device
            .audio_device
            .set_audio_settings(self.settings.audio_settings);

        // Assign the rom data to the rom file copy
        self.rom_file = rom_data.clone();

        self.c8_device.load_rom(&self.rom_file.clone());
    }

    fn reload_rom(&mut self) {
        self.reset_display();
        self.c8_device.load_rom(&self.rom_file.clone());
    }

    fn unload_rom(&mut self) {
        self.reset_display();
        self.c8_device.reset_device();
    }

    pub fn ui_mobile_portrait(&mut self, ctx: &egui::Context) {
        /*
           Screen
        */
        egui::TopBottomPanel::top("display_mobile")
            .min_height(250.0)
            .show(ctx, |ui| {
                self.update_display_window(ctx, ui);
            });

        /*
           Bottom menu
        */
        egui::TopBottomPanel::bottom("bottom_menu").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // let toggle = egui::SelectableLabel::new(
                //     self.settings.control_panel_expanded,
                //     self.language.locale_string("control_panel"),
                // );
                let toggle = egui::Button::new(self.language.locale_string("control_panel"))
                    .selected(self.settings.control_panel_expanded);

                let response = ui.add_sized([100.0, 50.0], toggle);

                if response.clicked() {
                    self.settings.control_panel_expanded = !self.settings.control_panel_expanded;
                }

                self.menu_roms(ui);
            });
        });

        self.side_panel_controls(ctx);

        /*
           Keyboard buttons
        */
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(50.0);
            egui::Grid::new("keyboard_grid")
                .num_columns(4)
                .show(ui, |ui| {
                    for (i, key) in KEYPAD_KEYS.iter().enumerate() {
                        let key_name = key.name();

                        let button = ui.add_sized([90.0, 90.0], egui::Button::new(key_name));

                        self.c8_device
                            .keypad_mut()
                            .set_key(key, button.is_pointer_button_down_on());

                        if i % 4 == 3 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    pub fn ui_desktop(&mut self, ctx: &egui::Context) {
        // Process input
        for key in KEYBOARD {
            ctx.input(|i| {
                let current_key = &self
                    .settings
                    .key_mapping
                    .key_from_mapping(key)
                    .unwrap_or_else(|| panic!("Key mapping not found for key: {:?}", key));

                if self.settings.key_mapping.is_extra_key(key) {
                    let regular_key = self
                        .settings
                        .key_mapping
                        .regular_key_from_extra_key(key)
                        .unwrap_or_else(|| panic!("No regular key found for key: {:?}", key));

                    let is_down = i.key_down(regular_key) || i.key_down(*key);
                    self.c8_device.keypad_mut().set_key(current_key, is_down);
                } else {
                    self.c8_device
                        .keypad_mut()
                        .set_key(current_key, i.key_down(*key))
                }
            });
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // Menu bar
            egui::MenuBar::new().ui(ui, |ui| {
                ui.toggle_value(
                    &mut self.settings.control_panel_expanded,
                    self.language.locale_string("control_panel"),
                );

                #[cfg(debug_assertions)]
                ui.toggle_value(
                    &mut self.settings.visualizer_panel_expanded,
                    self.language.locale_string("visualizer_panel"),
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
                        egui::Button::new(self.language.locale_string("reload_rom")),
                    )
                    .clicked()
                {
                    self.reload_rom();
                }

                if ui
                    .add_enabled(
                        self.c8_device.is_running(),
                        egui::Button::new(self.language.locale_string("unload_rom")),
                    )
                    .clicked()
                {
                    self.unload_rom();
                }

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                    // Global dark/light mode buttons
                    egui::widgets::global_theme_preference_switch(ui);

                    ui.separator();

                    self.menu_about(ui);

                    ui.separator();

                    if ui.button("Reset").clicked() {
                        ctx.memory_mut(|mem| *mem = Default::default());
                    }
                });
            });
        });

        if self.settings.draw_display_underneath {
            // Central panel with display window
            egui::CentralPanel::default().show(ctx, |ui| {
                self.update_display_window(ctx, ui);
            });

            self.side_panel_visualizer(ctx);
            self.side_panel_controls(ctx);
        } else {
            self.side_panel_controls(ctx);
            self.side_panel_visualizer(ctx);

            // Central panel with display window
            egui::CentralPanel::default().show(ctx, |ui| {
                self.update_display_window(ctx, ui);
            });
        }
    }

    fn menu_roms(&mut self, ui: &mut egui::Ui) {
        ui.menu_button(self.language.locale_string("included_roms"), |ui| {
            // Test rom menu
            ui.menu_button(self.language.locale_string("test_roms"), |ui| {
                for rom in TEST_ROMS.iter() {
                    if self.menu_rom_button(ui, rom) {
                        break;
                    }
                }
            });

            ui.menu_button(self.language.locale_string("game_roms"), |ui| {
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
        if ui.button(rom.name()).clicked() {
            self.load_rom(rom.data().to_vec());
            self.rom_name = rom.name().to_string();

            eprintln!("ROM loaded: {}", rom.name());

            // Close the menu
            ui.close();

            return true;
        }

        false
    }

    fn menu_open_rom(&mut self, ui: &mut egui::Ui) {
        if ui.button(self.language.locale_string("open_rom")).clicked() {
            // Clone the file data reference
            let data_clone = Rc::clone(&self.file_data.clone());
            let name_clone = Rc::clone(&self.file_name.clone());

            #[cfg(not(target_arch = "wasm32"))]
            {
                futures::executor::block_on(async move {
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
        ui.menu_button(self.language.locale_string("about"), |ui| {
            let version_label = format!(
                "{}{}",
                self.language.locale_string("version"),
                env!("CARGO_PKG_VERSION")
            );
            ui.label(version_label);

            ui.separator();

            ui.hyperlink_to(
                self.language.locale_string("source"),
                "https://github.com/iliags/chip8",
            );

            ui.separator();

            Self::powered_by_egui_and_eframe(ui, &self.language.language().value());

            #[cfg(debug_assertions)]
            {
                ui.separator();

                egui::warn_if_debug_build(ui);
            }
        });
    }

    pub fn side_panel_controls(&mut self, ctx: &egui::Context) {
        // Control panel
        egui::SidePanel::new(egui::panel::Side::Left, "ControlPanel").show_animated(
            ctx,
            self.settings.control_panel_expanded,
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

                    #[cfg(debug_assertions)]
                    {
                        //ui.separator();
                    }
                });
            },
        );
    }

    fn controls_cpu_speed(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("cpu_speed")).show(ui, |ui| {
            ui.add(
                egui::Slider::new(&mut self.settings.cpu_speed, 1..=240)
                    .clamping(egui::SliderClamping::Never)
                    .text(self.language.locale_string("speed")),
            )
            .on_hover_text(self.language.locale_string("speed_hover"));

            ui.horizontal(|ui| {
                if ui.button(self.language.locale_string("default")).clicked() {
                    self.settings.cpu_speed = DEFAULT_CPU_SPEED;
                }
                for speed in (500..=2000).step_by(500) {
                    if ui.button(speed.to_string()).clicked() {
                        self.settings.cpu_speed = speed;
                    }
                }
            });
        });
    }

    fn controls_display_scale(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("display")).show(ui, |ui| {
            ui.checkbox(
                &mut self.settings.display_fullscreen,
                self.language.locale_string("display_fullscreen"),
            );

            ui.separator();

            if self.settings.display_fullscreen {
                ui.add(egui::Checkbox::new(
                    &mut self.settings.draw_display_underneath,
                    self.language.locale_string("display_underneath"),
                ))
                .on_hover_text(self.language.locale_string("display_underneath_hover"));
            } else {
                ui.add(
                    egui::Slider::new(&mut self.settings.display_scale, 0.5..=3.0)
                        .text(self.language.locale_string("scale")),
                );

                if ui.button(self.language.locale_string("default")).clicked() {
                    self.settings.display_scale = DEFAULT_DISPLAY_SCALE;
                }
            }

            #[cfg(debug_assertions)]
            {
                ui.separator();

                ui.checkbox(&mut self.debug_window, "Debug window");

                if self.debug_window {
                    let pixels0 = format!("{:?}", self.c8_device.display().plane_pixels(0));
                    let pixels1 = format!("{:?}", self.c8_device.display().plane_pixels(0));

                    ui.label(pixels0);
                    ui.label(pixels1);
                }
            }
        });
    }

    fn controls_pixel_color(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("pixel_colors")).show(ui, |ui| {
            let selected_text = self
                .language
                .locale_string(self.settings.pixel_colors.name_key());

            egui::ComboBox::from_label(self.language.locale_string("color_palette"))
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    for palette in PALETTES.iter() {
                        let palette_name = self.language.locale_string(palette.name_key());
                        ui.selectable_value(
                            &mut self.settings.pixel_colors,
                            *palette,
                            palette_name,
                        );
                    }
                });

            /* TODO: Custom color palette
            if ui
                .button(self.language.get_locale_string("default"))
                .clicked()
            {
                self.settings.pixel_colors = PixelColors::default();
            }
            egui::CollapsingHeader::new(self.language.get_locale_string("pixel_on")).show(
                ui,
                |ui| {
                    color_picker_color32(
                        ui,
                        self.settings.pixel_colors.get_on_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );
                },
            );

            ui.separator();

            egui::CollapsingHeader::new(self.language.get_locale_string("pixel_off")).show(
                ui,
                |ui| {
                    color_picker_color32(
                        ui,
                        self.settings.pixel_colors.get_off_color_mut(),
                        egui::color_picker::Alpha::Opaque,
                    );
                },
            );
             */
        });
    }

    fn controls_keyboard_grid(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("keyboard")).show(ui, |ui| {
            egui::ComboBox::from_label(self.language.locale_string("mapping"))
                .selected_text(self.settings.key_mapping.key_mapping_name())
                .show_ui(ui, |ui| {
                    for key_mapping in KEY_MAPPINGS {
                        ui.selectable_value(
                            self.settings.key_mapping.key_mapping_mut(),
                            *key_mapping,
                            key_mapping.name().to_owned(),
                        );
                    }
                });

            ui.separator();

            egui::Grid::new("keyboard_grid").show(ui, |ui| {
                for (i, key) in KEYPAD_KEYS.iter().enumerate() {
                    let key_down = self.c8_device.keypad().is_key_pressed(key);

                    let key_name = key.name();

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
        egui::CollapsingHeader::new(self.language.locale_string("quirks")).show(ui, |ui| {
            ui.checkbox(
                &mut self.settings.quirk_settings.vf_zero,
                self.language.locale_string("quirk_vf0"),
            )
            .on_hover_text(self.language.locale_string("quirk_vf0_hover"));
            ui.checkbox(
                &mut self.settings.quirk_settings.i_incremented,
                self.language.locale_string("quirk_i"),
            )
            .on_hover_text(self.language.locale_string("quirk_i_hover"));
            ui.checkbox(
                &mut self.settings.quirk_settings.vx_shifted_directly,
                self.language.locale_string("quirk_shift_vx"),
            )
            .on_hover_text(self.language.locale_string("quirk_shift_vx_hover"));
            /*
               ui.checkbox(
                   &mut self.settings.quirk_settings.v_blank,
                   self.language.get_locale_string("quirk_v_blank"),
               )
               .on_hover_text(self.language.get_locale_string("quirk_v_blank_hover"));
            */
            ui.checkbox(
                &mut self.settings.quirk_settings.clip_sprites,
                self.language.locale_string("quirk_clip_sprites"),
            )
            .on_hover_text(self.language.locale_string("quirk_clip_sprites_hover"));

            ui.checkbox(
                &mut self.settings.quirk_settings.jump_bits,
                self.language.locale_string("quirk_jump"),
            )
            .on_hover_text(self.language.locale_string("quirk_jump_hover"));

            let profile_name =
                CompatibilityProfile::find_profile_name_key(self.settings.quirk_settings);

            egui::ComboBox::from_label(self.language.locale_string("compatibility_profile"))
                .selected_text(self.language.locale_string(profile_name))
                .show_ui(ui, |ui| {
                    for profile in COMPATIBILITY_PROFILES.iter() {
                        ui.selectable_value(
                            &mut self.settings.quirk_settings,
                            profile.quirks,
                            self.language.locale_string(profile.name_key()),
                        );
                    }
                });
        });

        self.c8_device.set_quirks(self.settings.quirk_settings);
    }

    fn controls_emulator(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("emulator")).show(ui, |ui| {
            // Emulator language
            egui::ComboBox::from_label(self.language.locale_string("language"))
                .selected_text(self.language.language().as_str())
                .show_ui(ui, |ui| {
                    for language in LANGUAGE_LIST {
                        ui.selectable_value(
                            &mut self.language.language_mut(),
                            &mut language.clone(),
                            language.as_str(),
                        );
                    }
                });

            // Emulator font
            // TODO: Move this to emulator settings
            let current_font_name: String = self.c8_device.memory().system_font().into();
            egui::ComboBox::from_label(self.language.locale_string("font_small"))
                .selected_text(current_font_name)
                .show_ui(ui, |ui| {
                    for font in FONT_DATA {
                        if font.small_data.is_empty() {
                            continue;
                        }

                        let font_string: String = font.name.into();

                        ui.selectable_label(
                            self.c8_device.memory_mut().system_font() == font.name,
                            font_string,
                        )
                        .on_hover_text(self.language.locale_string("font_hover"))
                        .clicked()
                        .then(|| {
                            self.c8_device.memory_mut().load_font_small(font);
                        });
                    }
                });
        });
    }

    fn controls_audio(&mut self, ui: &mut egui::Ui) {
        // TODO: Add audio settings to the settings struct
        egui::CollapsingHeader::new(self.language.locale_string("audio_controls")).show(ui, |ui| {
            ui.label(self.language.locale_string("under_construction"));

            ui.separator();

            ui.checkbox(
                &mut self.settings.audio_settings.enabled,
                self.language.locale_string("enable_audio"),
            );

            ui.separator();

            ui.vertical(|ui| {
                ui.add(
                    egui::Slider::new(&mut self.settings.audio_settings.volume, 0.0..=1.0)
                        .text(self.language.locale_string("volume")),
                );

                /*
                   ui.add(
                       egui::Slider::new(&mut self.settings.audio_settings.frequency, 50.0..=150.0)
                           .text(self.language.locale_string("pitch")),
                   );
                */
            });

            if ui.button(self.language.locale_string("default")).clicked() {
                self.settings.audio_settings = AudioSettings::default();
            }

            self.c8_device
                .audio_device
                .set_audio_settings(self.settings.audio_settings);

            #[cfg(debug_assertions)]
            {
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Play").clicked() {
                        self.c8_device.audio_device.play_beep();
                    }

                    if ui.button("Pause").clicked() {
                        self.c8_device.audio_device.pause();
                    }

                    /*
                    if ui.button("Stop").clicked() {
                        self.c8_device.beeper.stop();
                    }
                     */
                });
            }
        });
    }

    pub fn side_panel_visualizer(&mut self, ctx: &egui::Context) {
        egui::SidePanel::new(egui::panel::Side::Right, "VisualizerPanel").show_animated(
            ctx,
            self.settings.visualizer_panel_expanded,
            |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.visualizer_memory(ui);
                    self.visualizer_registers(ui);
                });
            },
        );
    }

    fn visualizer_memory(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("memory")).show(ui, |ui| {
            ui.label(self.language.locale_string("under_construction"));
        });
    }

    fn visualizer_registers(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(self.language.locale_string("registers")).show(ui, |ui| {
            ui.label(self.language.locale_string("under_construction"));
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

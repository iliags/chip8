#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use c8::device::C8;
//use c8_player::settings::PlayerSettings;
use macroquad::prelude::*;

#[cfg(feature = "profiling")]
use macroquad_profiler::ProfilerParams;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip-8 Player".to_owned(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        ..Default::default()
    }
}

const DEFAULT_WIDTH: u32 = 64;
const DEFAULT_HEIGHT: u32 = 32;

#[macroquad::main(window_conf)]
async fn main() {
    let mut c8_device = C8::default();

    //let mut settings = PlayerSettings::new();

    let test_rom = include_bytes!("../../../assets/test_roms/1-chip8-logo.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/2-ibm-logo.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/3-corax+.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/4-flags.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/5-quirks.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/6-keypad.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/7-beep.ch8");
    //let test_rom = include_bytes!("../../../assets/test_roms/8-scrolling.ch8");
    //let test_rom = include_bytes!("../../../assets/games/octo-sample/octo-sample.ch8");

    let mut current_frame: Option<Texture2D> = None;

    loop {
        clear_background(BLACK);

        let _ = c8_device.step(20);

        if is_key_pressed(KeyCode::Tab) {
            c8_device.load_rom(test_rom.to_vec());
            println!("Loading rom");
        }

        // TODO: Fix this
        // TODO: Scale display texture properly, this is temporary
        let pixels = c8_device.display().plane_pixels(0);
        let rgba8_pixels = convert_to_rgba8(pixels);

        // TODO: This should be screen center minus (chip 8 display size multiplied by half of the current scaling)
        let x = (screen_width() / 2.0) - (DEFAULT_WIDTH as f32 * 2.0);
        let y = (screen_height() / 2.0) - (DEFAULT_HEIGHT as f32 * 2.0);
        match &current_frame {
            Some(frame) => {
                frame.update_from_bytes(DEFAULT_WIDTH, DEFAULT_HEIGHT, &rgba8_pixels);
                //draw_texture(frame, x, y, WHITE);
                // TODO: Move this to player settings
                let params = DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: (DEFAULT_WIDTH * 4) as f32,
                        y: (DEFAULT_HEIGHT * 4) as f32,
                    }),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };
                draw_texture_ex(frame, x, y, WHITE, params);
            }
            None => {
                let new_texture = Texture2D::from_rgba8(
                    DEFAULT_WIDTH as u16,
                    DEFAULT_HEIGHT as u16,
                    &rgba8_pixels,
                );
                new_texture.set_filter(FilterMode::Nearest);
                //draw_texture(&new_texture, x, y, WHITE);
                let params = DrawTextureParams {
                    dest_size: Some(Vec2 {
                        x: (DEFAULT_WIDTH * 4) as f32,
                        y: (DEFAULT_HEIGHT * 4) as f32,
                    }),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                };
                draw_texture_ex(&new_texture, x, y, WHITE, params);
                current_frame = Some(new_texture);
            }
        }

        #[cfg(feature = "profiling")]
        macroquad_profiler::profiler(ProfilerParams {
            fps_counter_pos: Vec2::new(10.0, -35.0),
        });

        next_frame().await
    }
}

fn convert_to_rgba8(pixels: &[u8]) -> Vec<u8> {
    // Define RGBA8 values for on and off pixels
    let on_pixel = [255, 255, 255, 255]; // White
    let off_pixel = [0, 0, 0, 255]; // Black

    // Map each pixel to its corresponding RGBA8 value
    pixels
        .iter()
        .flat_map(|&pixel| {
            if pixel == 1 {
                on_pixel.to_vec()
            } else {
                off_pixel.to_vec()
            }
        })
        .collect()
}

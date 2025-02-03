#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use c8::device::C8;
use macroquad::prelude::*;
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

    let test_rom = include_bytes!("../../../assets/test_roms/1-chip8-logo.ch8");
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
        let pixels = c8_device.display().plane_pixels(0);
        let rgba8_pixels = convert_to_rgba8(&pixels);
        let x = (screen_width() / 2.0) - (DEFAULT_WIDTH as f32 / 4.0);
        let y = (screen_height() / 2.0) - (DEFAULT_HEIGHT as f32 / 4.0);
        match &current_frame {
            Some(frame) => {
                frame.update_from_bytes(DEFAULT_WIDTH as u32, DEFAULT_HEIGHT as u32, &rgba8_pixels);
                draw_texture(frame, x, y, WHITE);
            }
            None => {
                let new_texture = Texture2D::from_rgba8(
                    DEFAULT_WIDTH as u16,
                    DEFAULT_HEIGHT as u16,
                    &rgba8_pixels,
                );
                draw_texture(&new_texture, x, y, WHITE);
                current_frame = Some(new_texture);
            }
        }

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

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
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

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        macroquad_profiler::profiler(ProfilerParams {
            fps_counter_pos: Vec2::new(10.0, -35.0),
        });

        next_frame().await
    }
}

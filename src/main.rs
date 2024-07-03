// Disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use winit::window::Icon;
use std::io::Cursor;

// TODO: Once the project is more complete, look into https://bevy-cheatbook.github.io/setup/perf.html

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chip8".to_string(),
                // Set the window size to match the HTML canvas size
                resolution: (800., 600.).into(),
                // Bind to canvas included in `index.html`
                canvas: Some("#bevy".to_owned()),
                // Tells wasm not to override default event handling, like F5 and Ctrl+R
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)

        // Remove before release
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, set_window_icon)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
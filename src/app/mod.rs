/// Application
pub mod app_ui;

/// Pixel color option
pub mod pixel_color;

/// Keyboard information
pub mod keyboard;

pub fn is_mobile(ctx: &egui::Context) -> bool {
    let screen_size = ctx.screen_rect().size();
    screen_size.x < 550.0
}

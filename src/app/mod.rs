/// Application
pub mod app_ui;

/// Pixel color option
pub mod pixel_color;

/// Keyboard information
pub mod keyboard;

#[allow(unused_variables)]
pub fn is_mobile(ctx: &egui::Context) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        // TODO: Portrait and landscape
        let screen_size = ctx.content_rect().size();
        screen_size.x < 550.0
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}

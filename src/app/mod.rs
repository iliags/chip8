/// Application
pub mod app_ui;

/// Pixel color option
pub mod pixel_color;

/// Keyboard information
pub mod keyboard;

#[allow(unused_variables)]
pub fn is_mobile(ui: &mut egui::Ui) -> bool {
    cfg_select! {
        target_arch = "wasm32" => {
           let screen_size = ui.content_rect().size();
           screen_size.x < 550.0
        }
        _ => {
            false
        }
    }
}

#[allow(unused_variables)]
pub fn is_mobile_ctx(ctx: &egui::Context) -> bool {
    cfg_select! {
        target_arch = "wasm32" => {
           let screen_size = ctx.content_rect().size();
           screen_size.x < 550.0
        }
        _ => {
            false
        }
    }
}

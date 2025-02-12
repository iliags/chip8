use macroquad::{math::Vec2, texture::DrawTextureParams};

#[derive(Debug, Default)]
pub struct PlayerSettings {
    texture_params: DrawTextureParams,
    display_scale: f32,
    display_size: Vec2,
}

impl PlayerSettings {
    pub fn new() -> Self {
        let mut new = Self {
            texture_params: DrawTextureParams::default(),
            display_scale: 4.0,
            ..Default::default()
        };
        new.update_texture_params();
        new
    }

    pub fn set_display_scale(&mut self, new_scale: f32) {
        self.display_scale = new_scale;
        self.update_texture_params();
    }

    /// Updates the texture draw params scaling
    pub fn update_texture_params(&mut self) {
        self.texture_params.dest_size = Some(self.display_size * self.display_scale);
    }
}

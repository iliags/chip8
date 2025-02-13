use egui::Color32;
use std::default;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ColorLayer {
    Background,
    Foreground1,
    Foreground2,
    Blended,
}

impl From<u8> for ColorLayer {
    fn from(layer: u8) -> Self {
        match layer {
            0 => ColorLayer::Background,
            1 => ColorLayer::Foreground2,
            2 => ColorLayer::Foreground1,
            3 => ColorLayer::Blended,
            _ => ColorLayer::Background,
        }
    }
}

/// The colors used to display the pixels
#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct PixelColors {
    pub(crate) palette: Palette,

    /// Pixel off/background color
    background: Color32,

    /// Pixel on/plane 1 color
    foreground1: Color32,

    // Plane 2 color
    foreground2: Color32,

    // Both planes color
    blended: Color32,

    // Note: I believe these are used for background flash colors (if implemented)
    buzzer: Color32,
    silence: Color32,
}

impl default::Default for PixelColors {
    fn default() -> Self {
        PALETTE_DEFAULT
    }
}

impl From<&PixelColors> for [Color32; 6] {
    fn from(colors: &PixelColors) -> Self {
        [
            colors.background,
            colors.foreground1,
            colors.foreground2,
            colors.blended,
            colors.buzzer,
            colors.silence,
        ]
    }
}

impl PixelColors {
    /// Get the color of a pixel
    pub fn pixel_color(&self, layer: ColorLayer) -> &Color32 {
        match layer {
            ColorLayer::Background => &self.background,
            ColorLayer::Foreground1 => &self.foreground1,
            ColorLayer::Foreground2 => &self.foreground2,
            ColorLayer::Blended => &self.blended,
        }
    }

    pub fn name_key(&self) -> &str {
        self.palette.name_key()
    }

    pub fn background_color(&self) -> &Color32 {
        &self.background
    }

    pub fn foreground1_color(&self) -> &Color32 {
        &self.foreground1
    }

    pub fn foreground2_color(&self) -> &Color32 {
        &self.foreground2
    }

    pub fn blended_color(&self) -> &Color32 {
        &self.blended
    }

    /// Get a mutable reference to the color of an active pixel
    pub fn on_color_mut(&mut self) -> &mut Color32 {
        &mut self.foreground1
    }

    /// Get a reference to the color of an active pixel
    pub fn on_color(&self) -> &Color32 {
        &self.foreground1
    }

    /// Set the color of an active pixel
    pub fn set_on_color(&mut self, color: Color32) {
        self.foreground1 = color;
    }

    /// Get a mutable reference to the color of an inactive pixel
    pub fn off_color_mut(&mut self) -> &mut Color32 {
        &mut self.background
    }

    /// Get a reference to the color of an inactive pixel
    pub fn off_color(&self) -> &Color32 {
        &self.background
    }

    /// Set the color of an inactive pixel
    pub fn set_off_color(&mut self, color: Color32) {
        self.background = color;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Palette {
    Default,
    Octo,
    LCD,
    Grey,
}

impl From<Palette> for PixelColors {
    fn from(palette: Palette) -> Self {
        match palette {
            Palette::Default => PALETTE_DEFAULT,
            Palette::Octo => PALETTE_OCTO,
            Palette::LCD => PALETTE_LCD,
            Palette::Grey => PALETTE_GREY,
        }
    }
}

impl Palette {
    /// Get the locale string key
    pub fn name_key(&self) -> &str {
        match self {
            Palette::Default => "default",
            Palette::Octo => "octo",
            Palette::LCD => "lcd",
            Palette::Grey => "grey",
        }
    }
}

pub const PALETTES: &[PixelColors] = &[PALETTE_DEFAULT, PALETTE_OCTO, PALETTE_LCD, PALETTE_GREY];

// TODO: Make this a dark theme since GREY is a light theme
const PALETTE_DEFAULT: PixelColors = PixelColors {
    palette: Palette::Default,
    background: Color32::BLACK,
    foreground1: Color32::WHITE,
    foreground2: Color32::LIGHT_GREEN,
    blended: Color32::DARK_GREEN,
    buzzer: Color32::BLACK,
    silence: Color32::BLACK,
};

const PALETTE_OCTO: PixelColors = PixelColors {
    palette: Palette::Octo,
    background: Color32::from_rgb(153, 102, 0),
    foreground1: Color32::from_rgb(255, 204, 0),
    foreground2: Color32::from_rgb(255, 102, 0),
    blended: Color32::from_rgb(102, 34, 0),
    buzzer: Color32::from_rgb(255, 170, 0),
    silence: Color32::BLACK,
};

const PALETTE_LCD: PixelColors = PixelColors {
    palette: Palette::LCD,
    background: Color32::from_rgb(249, 255, 179),
    foreground1: Color32::from_rgb(61, 128, 38),
    foreground2: Color32::from_rgb(174, 204, 71),
    blended: Color32::from_rgb(0, 19, 26),
    buzzer: Color32::from_rgb(249, 255, 179),
    silence: Color32::BLACK,
};

const PALETTE_GREY: PixelColors = PixelColors {
    palette: Palette::Grey,
    background: Color32::from_rgb(170, 170, 170),
    foreground1: Color32::BLACK,
    foreground2: Color32::WHITE,
    blended: Color32::from_rgb(102, 102, 102),
    buzzer: Color32::from_rgb(102, 102, 102),
    silence: Color32::BLACK,
};

#[cfg(test)]
mod tests {
    //use super::*;
}

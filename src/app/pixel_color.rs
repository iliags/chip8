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
#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct PixelColors {
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
        Self::from(PALETTE_DEFAULT)
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
    pub fn get_pixel_color(&self, layer: ColorLayer) -> &Color32 {
        match layer {
            ColorLayer::Background => &self.background,
            ColorLayer::Foreground1 => &self.foreground1,
            ColorLayer::Foreground2 => &self.foreground2,
            ColorLayer::Blended => &self.blended,
        }
    }

    pub fn get_background_color(&self) -> &Color32 {
        &self.background
    }

    pub fn get_foreground1_color(&self) -> &Color32 {
        &self.foreground1
    }

    pub fn get_foreground2_color(&self) -> &Color32 {
        &self.foreground2
    }

    pub fn get_blended_color(&self) -> &Color32 {
        &self.blended
    }

    /// Get a mutable reference to the color of an active pixel
    pub fn get_on_color_mut(&mut self) -> &mut Color32 {
        &mut self.foreground1
    }

    /// Get a reference to the color of an active pixel
    pub fn get_on_color(&self) -> &Color32 {
        &self.foreground1
    }

    /// Set the color of an active pixel
    pub fn set_on_color(&mut self, color: Color32) {
        self.foreground1 = color;
    }

    /// Get a mutable reference to the color of an inactive pixel
    pub fn get_off_color_mut(&mut self) -> &mut Color32 {
        &mut self.background
    }

    /// Get a reference to the color of an inactive pixel
    pub fn get_off_color(&self) -> &Color32 {
        &self.background
    }

    /// Set the color of an inactive pixel
    pub fn set_off_color(&mut self, color: Color32) {
        self.background = color;
    }
}

pub const PALETTES: &[PixelColors] = &[PALETTE_DEFAULT, PALETTE_OCTO, PALETTE_LCD, PALETTE_GREY];

// TODO: Make this a dark theme since GREY is a light theme
const PALETTE_DEFAULT: PixelColors = PixelColors {
    background: Color32::BLACK,
    foreground1: Color32::WHITE,
    foreground2: Color32::LIGHT_RED,
    blended: Color32::DARK_RED,
    buzzer: Color32::BLACK,
    silence: Color32::BLACK,
};

const PALETTE_OCTO: PixelColors = PixelColors {
    background: Color32::from_rgb(153, 102, 0),
    foreground1: Color32::from_rgb(255, 204, 0),
    foreground2: Color32::from_rgb(255, 102, 0),
    blended: Color32::from_rgb(102, 34, 0),
    buzzer: Color32::from_rgb(255, 170, 0),
    silence: Color32::BLACK,
};

const PALETTE_LCD: PixelColors = PixelColors {
    background: Color32::from_rgb(249, 255, 179),
    foreground1: Color32::from_rgb(61, 128, 38),
    foreground2: Color32::from_rgb(174, 204, 71),
    blended: Color32::from_rgb(0, 19, 26),
    buzzer: Color32::from_rgb(249, 255, 179),
    silence: Color32::BLACK,
};

const PALETTE_GREY: PixelColors = PixelColors {
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

    /*
    #[test]
    fn test_get_color() {
        let colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        assert_eq!(colors.get_color(0), &Color32::BLACK);
        assert_eq!(colors.get_color(1), &Color32::WHITE);
    }

    #[test]
    fn test_get_on_color_mut() {
        let mut colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        *colors.get_on_color_mut() = Color32::RED;

        assert_eq!(colors.get_on_color(), &Color32::RED);
    }

    #[test]
    fn test_get_on_color() {
        let colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        assert_eq!(colors.get_on_color(), &Color32::WHITE);
    }

    #[test]
    fn test_set_on_color() {
        let mut colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        colors.set_on_color(Color32::RED);

        assert_eq!(colors.get_on_color(), &Color32::RED);
    }

    #[test]
    fn test_get_off_color_mut() {
        let mut colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        *colors.get_off_color_mut() = Color32::RED;

        assert_eq!(colors.get_off_color(), &Color32::RED);
    }

    #[test]
    fn test_get_off_color() {
        let colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        assert_eq!(colors.get_off_color(), &Color32::BLACK);
    }

    #[test]
    fn test_set_off_color() {
        let mut colors = PixelColors {
            on: Color32::WHITE,
            off: Color32::BLACK,
        };

        colors.set_off_color(Color32::RED);

        assert_eq!(colors.get_off_color(), &Color32::RED);
    }
     */
}

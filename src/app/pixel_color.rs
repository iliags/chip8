use egui::Color32;
use std::default;

/// The colors used to display the pixels
#[derive(Debug, Clone, Copy)]
pub struct PixelColors {
    /// The color of a pixel that is on
    on: Color32,

    /// The color of a pixel that is off
    off: Color32,
}

impl default::Default for PixelColors {
    fn default() -> Self {
        Self {
            on: Color32::WHITE,
            off: Color32::BLACK,
        }
    }
}

impl PixelColors {
    /// Get the color of a pixel
    pub fn get_color(&self, pixel: u8) -> &Color32 {
        if pixel == 1 {
            self.get_on_color()
        } else {
            self.get_off_color()
        }
    }

    /// Get a mutable reference to the color of an active pixel
    pub fn get_on_color_mut(&mut self) -> &mut Color32 {
        &mut self.on
    }

    /// Get a reference to the color of an active pixel
    pub fn get_on_color(&self) -> &Color32 {
        &self.on
    }

    /// Set the color of an active pixel
    pub fn set_on_color(&mut self, color: Color32) {
        self.on = color;
    }

    /// Get a mutable reference to the color of an inactive pixel
    pub fn get_off_color_mut(&mut self) -> &mut Color32 {
        &mut self.off
    }

    /// Get a reference to the color of an inactive pixel
    pub fn get_off_color(&self) -> &Color32 {
        &self.off
    }

    /// Set the color of an inactive pixel
    pub fn set_off_color(&mut self, color: Color32) {
        self.off = color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

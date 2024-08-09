/// Screen width constant
pub const SCREEN_WIDTH: i32 = 64;

/// Screen height constant
pub const SCREEN_HEIGHT: i32 = 32;

/// Screen size constant
pub const SCREEN_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

/// Struct representing the display of the Chip-8
#[derive(Debug)]
pub struct Display {
    // Reserved for future use
    _width: u32,

    // Reserved for future use
    _height: u32,

    // The pixels in the display
    pixels: Vec<u8>,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            _width: SCREEN_WIDTH as u32,
            _height: SCREEN_HEIGHT as u32,
            pixels: vec![0; SCREEN_SIZE],
        }
    }
}

impl Display {
    /// Get the pixels container of the display
    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.pixels
    }

    /// Get the pixel at the given x and y coordinates
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        // Get the pixel index
        let index = self.get_pixel_index(x, y);

        // Return the pixel value
        self.pixels[index]
    }

    /// Get if a pixel is on or off at the given x and y coordinates
    pub fn get_pixel_state(&self, x: i32, y: i32) -> bool {
        self.get_pixel(x, y) == 1
    }

    /// Clear the display
    pub fn clear(&mut self) {
        self.pixels = vec![0; SCREEN_SIZE];
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns the value of the pixel after toggling
    pub fn set_pixel(&mut self, x: i32, y: i32) -> u8 {
        let index = self.get_pixel_index(x, y);

        // Pixels are XORed on the display
        self.pixels[index] ^= 1;

        // Return the pixel value
        self.pixels[index]
    }

    const fn get_pixel_index(&self, x: i32, y: i32) -> usize {
        // Quirk: Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen.
        // This may be implemented in the future with a toggle.

        // If the pixels are out of bounds, wrap them around
        let x = x % SCREEN_WIDTH;
        let y = y % SCREEN_HEIGHT;

        // Get the pixel index
        (y * SCREEN_WIDTH + x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_default() {
        let display = Display::default();

        assert_eq!(display._width, SCREEN_WIDTH as u32);
        assert_eq!(display._height, SCREEN_HEIGHT as u32);
        assert_eq!(display.pixels.len(), SCREEN_SIZE);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::default();

        display.pixels = vec![1; SCREEN_SIZE];

        display.clear();

        assert_eq!(display.pixels, vec![0; SCREEN_SIZE]);
    }

    #[test]
    fn test_display_set_pixel() {
        let mut display = Display::default();

        display.set_pixel(0, 0);
        assert_eq!(display.pixels[0], 1);

        display.set_pixel(0, 0);
        assert_eq!(display.pixels[0], 0);

        display.set_pixel(SCREEN_WIDTH, SCREEN_HEIGHT);
        assert_eq!(display.pixels[0], 1);
    }
}

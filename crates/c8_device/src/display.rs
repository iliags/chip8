/// Screen width constant
pub(crate) const DEFAULT_SCREEN_WIDTH: usize = 64;

/// Screen height constant
pub(crate) const DEFAULT_SCREEN_HEIGHT: usize = 32;

/// Low resolution screen size constant
pub(crate) const SCREEN_SIZE_LOW: usize = DEFAULT_SCREEN_WIDTH * DEFAULT_SCREEN_HEIGHT;

/// High resolution screen size constant
pub(crate) const SCREEN_SIZE_HIGH: usize = (DEFAULT_SCREEN_WIDTH * 2) * (DEFAULT_SCREEN_HEIGHT * 2);

/// Low resolution screen size XY constant
pub(crate) const SCREEN_SIZE_LOW_XY: (usize, usize) = (DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT);

/// High resolution screen size XY constant
pub(crate) const SCREEN_SIZE_HIGH_XY: (usize, usize) =
    (DEFAULT_SCREEN_WIDTH * 2, DEFAULT_SCREEN_HEIGHT * 2);

// Note: There are two planes, plane 0 and 1; drawing to plane 2 (i.e. plane 3) draws to both planes.
// When retrieving pixels, plane 1 is superimposed on plane 0, allowing for more colors. Plane 1 is
// used for regular monochrome displays (default chip-8 behavior).

/// Display resolution
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum DisplayResolution {
    Low,
    High,
}

impl DisplayResolution {
    /// Get the resolution size
    pub const fn get_resolution_size(&self) -> usize {
        match self {
            DisplayResolution::Low => SCREEN_SIZE_LOW,
            DisplayResolution::High => SCREEN_SIZE_HIGH,
        }
    }

    /// Get the resolution size XY
    pub const fn get_resolution_size_xy(&self) -> (usize, usize) {
        match self {
            DisplayResolution::Low => SCREEN_SIZE_LOW_XY,
            DisplayResolution::High => SCREEN_SIZE_HIGH_XY,
        }
    }

    /// Get the resolution as a string
    pub const fn get_resolution_str(&self) -> &str {
        match self {
            DisplayResolution::Low => "Low",
            DisplayResolution::High => "High",
        }
    }
}

/// Struct representing the display of the Chip-8
#[derive(Debug)]
pub struct Display {
    // The pixels in the display
    planes: Vec<Plane>,

    // The resolution of the display
    resolution: DisplayResolution,

    // Which plane is active for drawing
    active_plane: usize,
}

/// Struct for a plane of pixels on the display
#[derive(Debug, Clone)]
pub struct Plane {
    pixels: Vec<u8>,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            pixels: vec![0; SCREEN_SIZE_LOW],
        }
    }
}

impl Default for Display {
    fn default() -> Self {
        Self {
            planes: vec![Plane::default(); 2],
            resolution: DisplayResolution::Low,
            active_plane: 1,
        }
    }
}

impl Display {
    /// Get the display resolution
    pub fn get_resolution(&self) -> DisplayResolution {
        self.resolution
    }

    /// Get the display resolution as a string
    pub fn get_resolution_str(&self) -> &str {
        self.resolution.get_resolution_str()
    }

    /// Set the display resolution
    pub fn set_resolution(&mut self, resolution: DisplayResolution) {
        self.resolution = resolution;
        self.clear_all();
    }

    /// Get the screen size XY
    pub const fn get_screen_size_xy(&self) -> (usize, usize) {
        self.resolution.get_resolution_size_xy()
    }

    /// Get the screen size
    pub const fn get_screen_size(&self) -> usize {
        self.resolution.get_resolution_size()
    }

    /// Get the pixels of a plane
    pub fn get_plane_pixels(&self, plane: usize) -> &Vec<u8> {
        let plane = self.clamp_plane_value(plane);
        &self.planes[plane].pixels
    }

    /// Get the pixel at the given x and y coordinates for a plane
    pub fn get_plane_pixel(&self, plane: usize, x: usize, y: usize) -> u8 {
        let plane = self.clamp_plane_value(plane);
        // Get the pixel index
        let index = self.get_pixel_index(x, y);

        // Return the pixel value
        self.planes[plane].pixels[index]
    }

    /// Get if a pixel is on or off at the given x and y coordinates
    pub fn get_plane_pixel_state(&self, plane: usize, x: usize, y: usize) -> bool {
        self.get_plane_pixel(plane, x, y) == 1
    }

    /// Clear the display
    pub(crate) fn clear(&mut self, plane: usize) {
        self.planes[plane].pixels = vec![0; self.get_screen_size()];
    }

    fn clear_all(&mut self) {
        for plane in 0..self.get_plane_count() {
            self.clear(plane);
        }
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns if a collision occurred
    pub(crate) fn set_plane_pixel(&mut self, plane: usize, x: usize, y: usize) -> u8 {
        // TODO: If plane is 2, draw to both planes
        // TODO: Implement colors

        let plane = self.clamp_plane_value(plane);
        let index = self.get_pixel_index(x, y);

        let mut result = false;

        // Pixels are XORed on the display

        if self.planes[plane].pixels[index] == 0 {
            self.planes[plane].pixels[index] = 1;
        } else {
            self.planes[plane].pixels[index] = 0;
            result = true;
        }

        // Return if a collision occurred
        result as u8
    }

    /// Set the active plane
    pub(crate) fn set_active_plane(&mut self, plane: usize) {
        let plane = self.clamp_plane_value(plane);
        self.active_plane = plane;
    }

    const fn clamp_plane_value(&self, value: usize) -> usize {
        value & 0x3
    }

    pub(crate) fn get_plane_count(&self) -> usize {
        self.planes.len()
    }

    /// Get the active plane
    pub fn get_active_plane(&self) -> usize {
        self.active_plane
    }

    /// Scroll planes left by the given number of pixels
    pub(crate) fn scroll_left(&mut self, pixels: u8) {
        let row_size = self.get_screen_size_xy().0;

        for layer in 0..self.get_plane_count() {
            if self.get_active_plane() & (layer + 1) == 0 {
                continue;
            }

            for a in (0..self.planes[layer].pixels.len()).step_by(row_size) {
                for b in 0..row_size {
                    let index = a + b;
                    self.planes[layer].pixels[index] = if b < row_size - pixels as usize {
                        self.planes[layer].pixels[index + pixels as usize]
                    } else {
                        0
                    };
                }
            }
        }
    }

    /// Scroll planes right by the given number of pixels
    pub(crate) fn scroll_right(&mut self, pixels: u8) {
        let row_size = self.get_screen_size_xy().0;

        for layer in 0..self.get_plane_count() {
            if self.get_active_plane() & (layer + 1) == 0 {
                continue;
            }

            for a in (0..self.planes[layer].pixels.len()).step_by(row_size) {
                for b in (0..row_size).rev() {
                    let index = a + b;
                    self.planes[layer].pixels[index] = if b >= pixels as usize {
                        self.planes[layer].pixels[index - pixels as usize]
                    } else {
                        0
                    };
                }
            }
        }
    }

    /// Scroll planes up by the given number of pixels
    pub(crate) fn scroll_up(&mut self, pixels: u8) {
        let row_size = self.get_screen_size_xy().0;
        let buffer_size = self.get_screen_size();

        for layer in 0..self.get_plane_count() {
            if self.get_active_plane() & (layer + 1) == 0 {
                continue;
            }

            for z in 0..buffer_size {
                let condition = z < (buffer_size - row_size * pixels as usize);
                self.planes[layer].pixels[z] = if condition {
                    self.planes[layer].pixels[z + (row_size * pixels as usize)]
                } else {
                    0
                };
            }
        }
    }

    /// Scroll planes down by the given number of pixels
    pub(crate) fn scroll_down(&mut self, pixels: u8) {
        let row_size = self.get_screen_size_xy().0;

        for layer in 0..self.get_plane_count() {
            if self.get_active_plane() & (layer + 1) == 0 {
                continue;
            }

            for z in (0..self.planes[layer].pixels.len()).rev() {
                let condition = z >= row_size * pixels as usize;
                self.planes[layer].pixels[z] = if condition {
                    self.planes[layer].pixels[z - (row_size * pixels as usize)]
                } else {
                    0
                };
            }
        }
    }

    const fn get_pixel_index(&self, x: usize, y: usize) -> usize {
        // Quirk: Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen.
        // This may be implemented in the future with a toggle.

        // If the pixels are out of bounds, wrap them around
        let (width, height) = self.get_screen_size_xy();

        let x = x % width;
        let y = y % height;

        // Get the pixel index
        y * width + x
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_display_default() {
        let display = Display::default();

        for plane in display.planes.iter() {
            assert_eq!(plane.pixels, vec![0; SCREEN_SIZE_LOW]);
        }
    }

    #[test]
    fn test_display_resolution() {
        let mut display = Display::default();

        display.set_resolution(DisplayResolution::High);
        assert_eq!(display.get_resolution(), DisplayResolution::High);
        assert_eq!(display.get_screen_size(), SCREEN_SIZE_HIGH);

        display.set_resolution(DisplayResolution::Low);
        assert_eq!(display.get_resolution(), DisplayResolution::Low);
        assert_eq!(display.get_screen_size(), SCREEN_SIZE_LOW);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::default();
        let screen_size = display.get_screen_size();

        let active_plane = display.get_active_plane();

        display.planes[active_plane].pixels = vec![1; screen_size];

        display.clear(active_plane);

        assert_eq!(display.planes[active_plane].pixels, vec![0; screen_size]);
    }

    #[test]
    fn test_display_set_pixel() {
        let mut display = Display::default();

        let mut rng = rand::thread_rng();
        let range = 0..display.get_screen_size();

        for plane in 0..2 {
            let (x, y) = (rng.gen_range(range.clone()), rng.gen_range(range.clone()));
            let pixel_index = display.get_pixel_index(x, y);

            display.set_plane_pixel(plane, x, y);
            assert_eq!(display.planes[plane].pixels[pixel_index], 1);

            display.set_plane_pixel(plane, x, y);
            assert_eq!(display.planes[plane].pixels[pixel_index], 0);

            let (width, height) = display.get_screen_size_xy();

            display.set_plane_pixel(plane, width, height);
            assert_eq!(
                display.planes[plane].pixels[display.get_pixel_index(width, height)],
                1
            );
        }
    }
}

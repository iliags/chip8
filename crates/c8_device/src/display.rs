/// Screen width constant
pub const DEFAULT_SCREEN_WIDTH: i32 = 64;

/// Screen height constant
pub const DEFAULT_SCREEN_HEIGHT: i32 = 32;

/// Low resolution screen size constant
pub const SCREEN_SIZE_LOW: usize = (DEFAULT_SCREEN_WIDTH * DEFAULT_SCREEN_HEIGHT) as usize;

/// High resolution screen size constant
pub const SCREEN_SIZE_HIGH: usize = SCREEN_SIZE_LOW * 2;

// Note: There are two planes, plane 0 and 1; drawing to plane 2 (i.e. plane 3) draws to both planes.
// When retrieving pixels, plane 1 is superimposed on plane 0, allowing for more colors. Plane 0 is
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
    pub const fn get_resolution_size(&self) -> (i32, i32) {
        match self {
            DisplayResolution::Low => (DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT),
            DisplayResolution::High => (DEFAULT_SCREEN_WIDTH * 2, DEFAULT_SCREEN_HEIGHT * 2),
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
        self.clear();
    }

    /// Get the screen size XY
    pub const fn get_screen_size_xy(&self) -> (i32, i32) {
        self.resolution.get_resolution_size()
    }

    /// Get the screen size
    pub const fn get_screen_size(&self) -> usize {
        match self.resolution {
            DisplayResolution::Low => SCREEN_SIZE_LOW,
            DisplayResolution::High => SCREEN_SIZE_HIGH,
        }
    }

    /// Get the pixels container of the display
    #[deprecated(note = "Use the version with planes instead")]
    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.planes[0].pixels
    }

    /// Get the pixels of a plane
    pub fn get_plane_pixels(&self, plane: usize) -> &Vec<u8> {
        let plane = plane % self.planes.len();
        &self.planes[plane].pixels
    }

    /// Get the pixel at the given x and y coordinates
    #[deprecated(note = "Use the version with planes instead")]
    pub fn get_pixel(&self, x: i32, y: i32) -> u8 {
        // Get the pixel index
        let index = self.get_pixel_index(x, y);

        // Return the pixel value
        self.planes[0].pixels[index]
    }

    /// Get the pixel at the given x and y coordinates for a plane
    pub fn get_plane_pixel(&self, plane: usize, x: i32, y: i32) -> u8 {
        let plane = plane % self.planes.len();
        // Get the pixel index
        let index = self.get_pixel_index(x, y);

        // Return the pixel value
        self.planes[plane].pixels[index]
    }

    /// Get if a pixel is on or off at the given x and y coordinates
    pub fn get_plane_pixel_state(&self, plane: usize, x: i32, y: i32) -> bool {
        self.get_plane_pixel(plane, x, y) == 1
    }

    /// Clear the display
    pub fn clear(&mut self) {
        let screen_size = match self.resolution {
            DisplayResolution::Low => SCREEN_SIZE_LOW,
            DisplayResolution::High => SCREEN_SIZE_HIGH,
        };

        for plane in self.planes.iter_mut() {
            plane.pixels = vec![0; screen_size];
        }
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns the value of the pixel after toggling
    #[deprecated(note = "Use the version with planes instead")]
    pub fn set_pixel(&mut self, x: i32, y: i32) -> u8 {
        let index = self.get_pixel_index(x, y);

        // Pixels are XORed on the display
        self.planes[0].pixels[index] ^= 1;

        // Return the pixel value
        self.planes[0].pixels[index]
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns the value of the pixel after toggling
    pub fn set_plane_pixel(&mut self, plane: usize, x: i32, y: i32) -> u8 {
        // TODO: If plane is 2, draw to both planes
        let plane = plane.clamp(0, 2);
        let index = self.get_pixel_index(x, y);

        // Pixels are XORed on the display
        self.planes[plane].pixels[index] ^= 1;

        // Return the pixel value
        self.planes[plane].pixels[index]
    }

    const fn get_pixel_index(&self, x: i32, y: i32) -> usize {
        // Quirk: Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen.
        // This may be implemented in the future with a toggle.

        // If the pixels are out of bounds, wrap them around
        let (width, height) = self.get_screen_size_xy();

        let x = x % width;
        let y = y % height;

        // Get the pixel index
        (y * width + x) as usize
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

        display.planes[0].pixels = vec![1; screen_size];

        display.clear();

        assert_eq!(display.planes[0].pixels, vec![0; screen_size]);
    }

    #[test]
    fn test_display_set_pixel() {
        let mut display = Display::default();

        let mut rng = rand::thread_rng();
        let range = 0..display.get_screen_size();

        for plane in 0..2 {
            let (x, y) = (
                rng.gen_range(range.clone()) as i32,
                rng.gen_range(range.clone()) as i32,
            );
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

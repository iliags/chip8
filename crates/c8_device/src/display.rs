// Temporary
#![allow(dead_code)]

/// Screen width constant
pub(crate) const DEFAULT_SCREEN_WIDTH: usize = 64;

/// Screen height constant
pub(crate) const DEFAULT_SCREEN_HEIGHT: usize = 32;

/// Low resolution screen size constant
pub(crate) const SCREEN_SIZE_LOW: usize = DEFAULT_SCREEN_WIDTH * DEFAULT_SCREEN_HEIGHT;

/// High resolution screen size constant
pub(crate) const SCREEN_SIZE_HIGH: usize = SCREEN_SIZE_LOW * 2;

/// Low resolution screen size XY constant
pub(crate) const SCREEN_SIZE_LOW_XY: (usize, usize) = (DEFAULT_SCREEN_WIDTH, DEFAULT_SCREEN_HEIGHT);

/// High resolution screen size XY constant
pub(crate) const SCREEN_SIZE_HIGH_XY: (usize, usize) =
    (DEFAULT_SCREEN_WIDTH * 2, DEFAULT_SCREEN_HEIGHT * 2);

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
            active_plane: 0,
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
        //println!("Display resolution size {}", self.get_screen_size());
    }

    /// Get the screen size XY
    pub const fn get_screen_size_xy(&self) -> (usize, usize) {
        self.resolution.get_resolution_size_xy()
    }

    /// Get the screen size
    pub const fn get_screen_size(&self) -> usize {
        self.resolution.get_resolution_size()
    }

    /// Get the pixels container of the display
    #[deprecated(note = "Use the version with planes instead")]
    pub fn get_pixels(&self) -> &Vec<u8> {
        self.get_plane_pixels(0)
    }

    /// Get the pixels of a plane
    pub fn get_plane_pixels(&self, plane: usize) -> &Vec<u8> {
        let plane = plane.clamp(0, 2);
        &self.planes[plane].pixels
    }

    /// Get the pixel at the given x and y coordinates
    #[deprecated(note = "Use the version with planes instead")]
    pub fn get_pixel(&self, x: usize, y: usize) -> u8 {
        self.get_plane_pixel(0, x, y)
    }

    /// Get the pixel at the given x and y coordinates for a plane
    pub fn get_plane_pixel(&self, plane: usize, x: usize, y: usize) -> u8 {
        let plane = plane.clamp(0, 2);
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
    pub(crate) fn clear(&mut self) {
        for plane in self.planes.iter_mut() {
            plane.pixels = vec![0; self.resolution.get_resolution_size()];
        }
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns the value of the pixel after toggling
    #[deprecated(note = "Use the version with planes instead")]
    pub fn set_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.set_plane_pixel(0, x, y)
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns the value of the pixel after toggling
    pub(crate) fn set_plane_pixel(&mut self, plane: usize, x: usize, y: usize) -> u8 {
        // TODO: If plane is 2, draw to both planes
        let plane = plane.clamp(0, 2);
        let index = self.get_pixel_index(x, y);

        // Pixels are XORed on the display
        self.planes[plane].pixels[index] ^= 1;

        // Return the pixel value
        self.planes[plane].pixels[index]
    }

    /// Set the active plane
    pub(crate) fn set_active_plane(&mut self, plane: usize) {
        #[cfg(debug_assertions)]
        {
            println!("Setting active plane to {}", plane);
        }

        self.active_plane = plane;
    }

    pub(crate) fn get_plane_count(&self) -> usize {
        self.planes.len()
    }

    /// Get the active plane
    pub(crate) fn get_active_plane(&self) -> usize {
        self.active_plane
    }

    /// Toggle a pixel at the given x and y coordinates for the active plane
    pub(crate) fn set_active_plane_pixel(&mut self, x: usize, y: usize) -> u8 {
        self.set_plane_pixel(self.active_plane, x, y)
    }

    /// Scroll planes left by the given number of pixels
    pub(crate) fn scroll_left(&mut self, pixels: u8) {
        let pixels = -(pixels as isize);
        self.scroll_planes(pixels, 0);
    }

    /// Scroll planes right by the given number of pixels
    pub(crate) fn scroll_right(&mut self, pixels: u8) {
        self.scroll_planes(pixels as isize, 0);
    }

    /// Scroll planes up by the given number of pixels
    pub(crate) fn scroll_up(&mut self, pixels: u8) {
        let pixels = -(pixels as isize);
        self.scroll_planes(0, pixels);
    }

    /// Scroll planes down by the given number of pixels
    pub(crate) fn scroll_down(&mut self, pixels: u8) {
        self.scroll_planes(0, pixels as isize);
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

    /// Scroll the planes by the given number of pixels in the x and y directions
    fn scroll_planes(&mut self, pixels_x: isize, pixels_y: isize) {
        for i in 0..self.get_plane_count() {
            let mut new_pixels = vec![0; self.resolution.get_resolution_size()];

            let (width, height) = self.get_screen_size_xy();
            for y in 0..height {
                for x in 0..width {
                    // Get the pixel index
                    let index = self.get_pixel_index(x, y);

                    // Calculate the new x and y coordinates
                    // TODO: Check if wrapping is needed
                    let new_x = (x as isize + pixels_x) as usize; // % width;
                    let new_y = (y as isize + pixels_y) as usize; // % height;

                    // Get the new pixel index
                    let new_index = self.get_pixel_index(new_x, new_y);

                    if new_index >= 4096 {
                        println!("Index out of bounds: {}", new_index);
                    }

                    // Copy the pixel to the new position
                    new_pixels[new_index] = self.planes[i].pixels[index];
                }
            }

            // Update the pixels
            self.planes[i].pixels = new_pixels;
        }
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

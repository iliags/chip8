use crate::profile_function;

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
    pub const fn resolution_size(&self) -> usize {
        match self {
            DisplayResolution::Low => SCREEN_SIZE_LOW,
            DisplayResolution::High => SCREEN_SIZE_HIGH,
        }
    }

    /// Get the resolution size XY
    pub const fn resolution_size_xy(&self) -> (usize, usize) {
        match self {
            DisplayResolution::Low => SCREEN_SIZE_LOW_XY,
            DisplayResolution::High => SCREEN_SIZE_HIGH_XY,
        }
    }

    /// Get the resolution as a string
    pub const fn resolution_str(&self) -> &str {
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
    /// Get the planes for iteration
    pub fn zipped_iterator(&self) -> impl Iterator<Item = (&u8, &u8)> {
        self.plane_pixels(0).iter().zip(self.plane_pixels(1).iter())
    }

    /// Get the display resolution
    pub fn resolution(&self) -> DisplayResolution {
        self.resolution
    }

    /// Set the display resolution
    pub fn set_resolution(&mut self, resolution: DisplayResolution) {
        self.resolution = resolution;
        self.clear_all();
    }

    /// Get the screen size XY
    pub(crate) const fn screen_size_xy(&self) -> (usize, usize) {
        self.resolution.resolution_size_xy()
    }

    /// Get the screen size
    pub(crate) const fn screen_size(&self) -> usize {
        self.resolution.resolution_size()
    }

    /// Get the pixels of a plane
    pub fn plane_pixels(&self, plane: usize) -> &Vec<u8> {
        &self.planes[plane].pixels
    }

    /// Clear the display
    pub(crate) fn clear(&mut self, plane: usize) {
        self.planes[plane].pixels = vec![0; self.screen_size()];
    }

    fn clear_all(&mut self) {
        for plane in 0..self.plane_count() {
            self.clear(plane);
        }
    }

    /// Toggle a pixel at the given x and y coordinates
    ///
    /// Returns if a collision occurred
    pub(crate) fn set_plane_pixel(&mut self, plane: usize, x: usize, y: usize) -> u8 {
        profile_function!();
        let index = self.pixel_index(x, y);

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
        self.active_plane = plane & 0x3;
    }

    pub(crate) fn plane_count(&self) -> usize {
        self.planes.len()
    }

    /// Get the active plane
    pub fn active_plane(&self) -> usize {
        self.active_plane
    }

    /// Scroll planes left by the given number of pixels
    pub(crate) fn scroll_left(&mut self, pixels: u8) {
        profile_function!();
        let row_size = self.screen_size_xy().0;

        for layer in 0..self.plane_count() {
            if self.active_plane() & (layer + 1) == 0 {
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
        profile_function!();
        let row_size = self.screen_size_xy().0;

        for layer in 0..self.plane_count() {
            if self.active_plane() & (layer + 1) == 0 {
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
        profile_function!();

        let size = self.screen_size_xy().0 * pixels as usize;
        let buffer_size = self.screen_size();

        for layer in 0..self.plane_count() {
            if self.active_plane() & (layer + 1) == 0 {
                continue;
            }

            self.planes[layer].pixels = self.planes[layer]
                .pixels
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    if i < buffer_size - size {
                        self.planes[layer].pixels[i + size]
                    } else {
                        0
                    }
                })
                .collect();
        }
    }

    /// Scroll planes down by the given number of pixels
    pub(crate) fn scroll_down(&mut self, pixels: u8) {
        profile_function!();

        let size = self.screen_size_xy().0 * pixels as usize;

        for layer in 0..self.plane_count() {
            if self.active_plane() & (layer + 1) == 0 {
                continue;
            }

            self.planes[layer].pixels = self.planes[layer]
                .pixels
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    if i < size {
                        0
                    } else {
                        self.planes[layer].pixels[i - size]
                    }
                })
                .collect();
        }
    }

    #[inline]
    const fn pixel_index(&self, x: usize, y: usize) -> usize {
        let (width, _) = self.screen_size_xy();

        //let x = x % width;
        //let y = y % height;

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
        assert_eq!(display.resolution(), DisplayResolution::High);
        assert_eq!(display.screen_size(), SCREEN_SIZE_HIGH);

        display.set_resolution(DisplayResolution::Low);
        assert_eq!(display.resolution(), DisplayResolution::Low);
        assert_eq!(display.screen_size(), SCREEN_SIZE_LOW);
    }

    #[test]
    fn test_display_clear() {
        let mut display = Display::default();
        let screen_size = display.screen_size();

        let active_plane = display.active_plane();

        display.planes[active_plane].pixels = vec![1; screen_size];

        display.clear(active_plane);

        assert_eq!(display.planes[active_plane].pixels, vec![0; screen_size]);
    }

    #[test]
    fn test_display_set_pixel() {
        let mut display = Display::default();

        let mut rng = rand::rng();
        let range = 0..display.screen_size();

        for plane in 0..2 {
            let (x, y) = (
                rng.random_range(range.clone()),
                rng.random_range(range.clone()),
            );
            let pixel_index = display.pixel_index(x, y);

            display.set_plane_pixel(plane, x, y);
            assert_eq!(display.planes[plane].pixels[pixel_index], 1);

            display.set_plane_pixel(plane, x, y);
            assert_eq!(display.planes[plane].pixels[pixel_index], 0);

            let (width, height) = display.screen_size_xy();

            display.set_plane_pixel(plane, width, height);
            assert_eq!(
                display.planes[plane].pixels[display.pixel_index(width, height)],
                1
            );
        }
    }
}

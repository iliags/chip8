/// Custom audio settings
#[derive(Debug, Clone, Copy)]
pub struct AudioSettings {
    /// Whether audio is enabled
    enabled: bool,

    /// Tone pitch
    pub frequency: f32,

    /// Tone volume
    pub volume: f32,
}
impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            enabled: true,
            frequency: 440.0,
            volume: 0.05,
        }
    }
}

impl AudioSettings {
    /// Create a new audio settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Is audio enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled || self.volume > 0.0
    }

    /// Get the frequency
    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    /// Set the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    /// Get the volume
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    /// Set the volume
    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

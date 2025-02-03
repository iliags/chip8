/// Custom audio settings
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AudioSettings {
    /// Whether audio is enabled
    pub enabled: bool,

    /// Tone pitch
    // Marked public for egui access, find a better way to do this
    pub frequency: f32,

    /// Tone volume
    // Marked public for egui access, find a better way to do this
    pub volume: f32,
}
impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            enabled: true,
            frequency: 100.0,
            volume: 0.25,
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
        self.enabled && self.volume > 0.0
    }

    /// Get the frequency
    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    /// Get the volume
    pub fn volume(&self) -> f32 {
        self.volume
    }
}

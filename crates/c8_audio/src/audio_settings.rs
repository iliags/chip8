/// Custom audio settings
#[derive(Debug, Clone, Copy)]
pub struct AudioSettings {
    /// Tone pitch
    pub frequency: f32,

    /// Tone volume
    pub volume: f32,
}
impl Default for AudioSettings {
    fn default() -> Self {
        AudioSettings {
            frequency: 440.0,
            volume: 0.05,
        }
    }
}

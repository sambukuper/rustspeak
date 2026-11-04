//! Audio state — push-to-talk and voice activity, modeled on TS3 behavior.
//!
//! The real voice pipeline (Opus encode/decode, jitter buffer, comfort
//! noise) lives behind the `AudioBackend` trait so this crate ships with
//! zero audio dependencies. The CLI ships a null backend; plugging in
//! cpal + opus is a one-trait implementation.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PttMode {
    Continuous,
    PushToTalk,
    VoiceActivity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioState {
    pub mode: PttMode,
    pub is_muted: bool,
    pub is_deaf: bool,
    pub is_transmitting: bool,
    pub is_voice_active: bool,
    pub level: u8,
    pub recording: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        AudioState {
            mode: PttMode::PushToTalk,
            is_muted: false,
            is_deaf: false,
            is_transmitting: false,
            is_voice_active: false,
            level: 0,
            recording: false,
        }
    }
}

impl AudioState {
    pub fn toggle_mute(&mut self) {
        self.is_muted = !self.is_muted;
        if self.is_muted {
            self.is_transmitting = false;
        }
    }
    pub fn toggle_deaf(&mut self) {
        self.is_deaf = !self.is_deaf;
        if self.is_deaf {
            self.is_muted = true;
        }
    }
    pub fn set_ptt_pressed(&mut self, pressed: bool) {
        if self.mode != PttMode::PushToTalk {
            return;
        }
        self.is_transmitting = pressed && !self.is_muted && !self.is_deaf;
    }
    pub fn on_audio_level(&mut self, sample: u8) {
        self.level = sample;
        self.is_voice_active = sample > 30;
    }
}

pub trait AudioBackend {
    fn start(&mut self) -> Result<(), String>;
    fn stop(&mut self);
    fn is_running(&self) -> bool;
}

pub struct NullBackend;
impl AudioBackend for NullBackend {
    fn start(&mut self) -> Result<(), String> { Ok(()) }
    fn stop(&mut self) {}
    fn is_running(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mute_blocks_ptt() {
        let mut a = AudioState::default();
        a.toggle_mute();
        a.set_ptt_pressed(true);
        assert!(!a.is_transmitting);
    }
    #[test]
    fn deaf_implies_mute() {
        let mut a = AudioState::default();
        a.toggle_deaf();
        assert!(a.is_muted);
    }
}

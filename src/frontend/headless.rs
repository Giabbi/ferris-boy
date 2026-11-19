//! Created by Giancarlo Ambrosino
//!
//! Headless module for frontend to perform tests
//! during development
//!
use super::Frontend;
use crate::emulator::{HEIGHT, WIDTH};

/// A headless frontend for automated testing without opening a window.
pub struct HeadlessFrontend {
    max_frames: usize,
    frames_drawn: usize,
    display: Vec<bool>,
    scripted_keys: Vec<(usize, [bool; 16])>,
    beep_count: usize,
}

impl HeadlessFrontend {
    /// Creates a new headless frontend that stops after max_frames drawn frames.
    pub fn new(max_frames: usize) -> Self {
        Self {
            max_frames,
            frames_drawn: 0,
            display: vec![false; WIDTH * HEIGHT],
            scripted_keys: Vec::new(),
            beep_count: 0,
        }
    }

    /// Attaches scripted (frame_number, key_state) inputs to replay during poll_keys.
    pub fn with_keys(mut self, scripted_keys: Vec<(usize, [bool; 16])>) -> Self {
        self.scripted_keys = scripted_keys;
        self
    }

    /// Returns the pixel at (x, y) from the most recently drawn frame (false if out of bounds).
    pub fn pixel(&self, x: usize, y: usize) -> bool {
        if x >= WIDTH || y >= HEIGHT {
            return false;
        }
        self.display.get(y * WIDTH + x).copied().unwrap_or(false)
    }

    /// Returns the full framebuffer from the most recently drawn frame.
    pub fn display(&self) -> &[bool] {
        &self.display
    }

    /// Returns how many frames have been drawn so far.
    pub fn frames_drawn(&self) -> usize {
        self.frames_drawn
    }

    /// Returns how many frames had the sound timer active.
    pub fn beep_count(&self) -> usize {
        self.beep_count
    }
}

impl Frontend for HeadlessFrontend {
    fn is_open(&self) -> bool {
        self.frames_drawn < self.max_frames
    }

    fn draw(&mut self, display: &[bool]) {
        self.display = display.to_vec();
        self.frames_drawn += 1;
    }

    fn poll_keys(&mut self) -> [bool; 16] {
        for &(frame, keys) in &self.scripted_keys {
            if frame == self.frames_drawn {
                return keys;
            }
        }
        [false; 16]
    }

    fn beep(&mut self, on: bool) {
        if on {
            self.beep_count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulator::Ferris8;
    use crate::frontend::run;

    #[test]
    fn runs_empty_rom_for_n_frames_and_exits() {
        let mut emu = Ferris8::new();
        emu.load_rom(&[]).unwrap();

        let mut frontend = HeadlessFrontend::new(10);
        run(&mut emu, &mut frontend, 10);

        assert!(!frontend.is_open());
        assert_eq!(frontend.frames_drawn(), 10);
        assert_eq!(frontend.display().len(), WIDTH * HEIGHT);
        assert!(!frontend.pixel(0, 0));
    }

    #[test]
    fn replays_scripted_keys_on_matching_frame() {
        let mut key_state = [false; 16];
        key_state[0x5] = true;

        let mut frontend = HeadlessFrontend::new(3).with_keys(vec![(1, key_state)]);

        // Frame 0: no scripted key
        assert_eq!(frontend.poll_keys(), [false; 16]);
        frontend.draw(&[false; WIDTH * HEIGHT]);

        // Frame 1: key 0x5 should be pressed
        assert_eq!(frontend.poll_keys(), key_state);
        frontend.draw(&[false; WIDTH * HEIGHT]);

        // Frame 2: back to all released
        assert_eq!(frontend.poll_keys(), [false; 16]);
    }
}

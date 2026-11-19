//! Created by Giancarlo Ambrosino
//!
//! --- ADD A DESCRIPTION OF THE FILE LATER
//!
//!
use super::Frontend;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

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
            display: vec![false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
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
        if x >= DISPLAY_WIDTH || y >= DISPLAY_HEIGHT {
            return false;
        }
        self.display
            .get(y * DISPLAY_WIDTH + x)
            .copied()
            .unwrap_or(false)
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
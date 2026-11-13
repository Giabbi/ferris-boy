//! Created by Giancarlo Ambrosino
//!
//! --- ADD A DESCRIPTION OF THE FILE LATER
//!
//!

// Modules
pub mod headless;
pub mod screenshot;
pub mod window;

// Packages
use crate::emulator::Ferris8;

// Frontend
///
pub trait Frontend {
    fn is_open(&self) -> bool;
    fn draw(&mut self, display: &[bool]);
    fn poll_keys(&mut self) -> [bool; 16];
    fn beep(&mut self, on: bool);
}

// Run
/// A single loop iteration (one 60hz frame)
pub fn run<F: Frontend>(ferris8: &mut Ferris8, frontend: &mut F, cycles_per_frame: u32) {
    while frontend.is_open() {
        // update keys
        ferris8.set_keys(frontend.poll_keys());

        // run 'cycles_per_frame' instructions
        for _ in 0..cycles_per_frame {
            ferris8.step();
        }

        // tick timers
        ferris8.tick_timers();

        // play sound if needed
        frontend.beep(ferris8.is_sound_active());

        // draw the ferris8 display
        frontend.draw(ferris8.get_display());
    }
}

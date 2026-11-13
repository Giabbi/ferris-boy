//! Created by Zachary Lebak
//!
//! --- ADD A DESCRIPTION OF THE FILE LATER
//!
//!

// Random Source Trait
/// returns a random byte to the caller
pub trait RandomSource {
    fn next_byte(&mut self) -> u8;
}

// System RNG
/// completely random, uses rand
pub struct SystemRNG;

impl RandomSource for SystemRNG {
    fn next_byte(&mut self) -> u8 {
        rand::random()
    }
}

// Fixed RNG
/// returns the same number, used for testing
pub struct FixedRNG;

impl RandomSource for FixedRNG {
    fn next_byte(&mut self) -> u8 {
        0u8
    }
}

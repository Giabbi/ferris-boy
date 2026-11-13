//! Created by Zachary Lebak
//! 
//! --- ADD A DESCRIPTION OF THE FILE LATER
//! 
//! 

#![allow(dead_code)] // REMOVE THIS WHEN CPU USES EVERYTHING

// Modules
pub mod font;
pub mod instruction;
pub mod quirks;
pub mod savestate;

// Packages
use crate::rng::{RandomSource, SystemRNG};
use quirks::Quirks;


/// The width of the screen
pub const WIDTH: usize = 64;

/// The height of the screen
pub const HEIGHT: usize = 32;

/// The amount of memory chip8 has
pub const MEMORY_SIZE: usize = 4096;

/// The start of a program, by convention
pub const PROGRAM_START: usize = 0x200;


// Ferris8
/// Contains all of the CPU and other fields needed to run the emulator
pub struct Ferris8 {
    memory: [u8; MEMORY_SIZE],
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    display: [bool; WIDTH * HEIGHT],
    keys: [bool; 16],
    quirks: Quirks,
    rng: Box<dyn RandomSource>,
}


// Impl Ferris8
/// implements functions needed to access Ferris8 fields
impl Ferris8 {
    // New
    /// Creates a new Ferris8 instance using the default Quirk Profile and SystemRNG
    pub fn new() -> Self {
        Self::with_parts(Quirks::default(), Box::new(SystemRNG))
    }


    // With Parts
    /// Creates a new Ferris8 instance using a sepcific Quirk Profile and RandomSource
    pub fn with_parts(quirks: Quirks, rng: Box<dyn RandomSource>) -> Self {
        // copy fonts into memory
        let mut memory = [0u8; MEMORY_SIZE];
        let start = font::FONT_START;
        memory[start..start + font::FONT.len()].copy_from_slice(&font::FONT);

        // create new Ferris8 instance
        Ferris8 {
            memory,
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START as u16,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            display: [false; WIDTH * HEIGHT],
            keys: [false; 16],
            quirks,
            rng,
        }
    }


    // Load Rom
    /// Copies a ROM into memory, if the ROM exists
    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), String> {
        // check if ROM can fit in memory
        if data.len() > MEMORY_SIZE - PROGRAM_START {
            return Err(format!("ROM is too big to fit in memory: {} bytes", data.len()));
        }

        // copy rom into memory
        self.memory[PROGRAM_START..PROGRAM_START + data.len()].copy_from_slice(data);
        Ok(())
    }


    // Fetch
    /// Fetches the next opcode instruction (big-endian)
    /// 
    /// Only used by step()
    fn fetch(&mut self) -> u16 {
        // get both bytes
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc as usize) + 1] as u16;
        
        // increment pc
        self.pc += 2;

        // merge the bytes
        (hi << 8) | lo
    }


    // Step
    /// Execute the next instruction
    pub fn step(&mut self) {
        /* IMPLEMENT THIS */
    }


    // Tick Timers
    /// Ticks the delay and sound timer
    /// 
    /// Should be called once per frame (60hz)
    pub fn tick_timers(&mut self) {
        // sat sub avoids overflow
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }
    

    // Get Display
    /// Returns the current display of the instance
    pub fn get_display(&self) -> &[bool] {
        &self.display
    }


    // Set Keys
    /// Sets the keys of the instance
    pub fn set_keys(&mut self, keys: [bool; 16]) {
        self.keys = keys;
    }


    // Is Sound Active
    /// Returns true if the sound timer is > 0, false otherwise
    pub fn is_sound_active(&self) -> bool {
        self.sound_timer > 0
    }

    
    // Get PC
    /// Gets the current PC
    /// 
    /// Should only be used in tests to inspect the current state
    pub fn get_pc(&self) -> u16 { self.pc }


    // Get Index
    /// Gets the current index
    /// 
    /// Should only be used in tests to inspect the current state
    pub fn get_index(&self) -> u16 { self.i }


    // Get V
    /// Gets the current state of register 'x'
    /// 
    /// Should only be used in tests to inspect the current state
    pub fn get_v(&self, x: usize) -> u8 { self.v[x] }


    //  TEMPORARY FN TO TEST PIXELS UNTIL DXYN WORKS
    /// TEMPORARY FN TO TEST PIXELS UNTIL DXYN WORKS
    pub fn debug_fill_checkerboard(&mut self) {
        for (idx, px) in self.display.iter_mut().enumerate() {
            *px = (idx % WIDTH + idx / WIDTH) % 2 == 0;
        }
    }

}


// Default
/// Creates a new Ferris8 instance using the default Quirk Profile and SystemRNG
impl Default for Ferris8 {
    fn default() -> Self {
        Self::new()
    }
}
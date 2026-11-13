//! Created by Zachary Lebak and Giancarlo Ambrosino
//! 
//! --- ADD A DESCRIPTION OF THE FILE LATER
//! 
//! 

use ferris8::Ferris8;

// Main
/// Creates a new Ferris8 instance, and boots up a ROM
fn main() {
    // get path
    let path = std::env::args().nth(1).expect("Could not read rom path. Expects chip8 <rom>, with a valid rom.");

    // try to read the path
    let rom = std::fs::read(&path).expect("Could not read the ROM.");

    // create a new Ferris8 instance
    let mut ferris8 = Ferris8::new();
    ferris8.load_rom(&rom).expect("Could not load the ROM.");

    // DEBUG: print that rom's bytes were loaded
    println!("Loaded {} bytes", rom.len());
}

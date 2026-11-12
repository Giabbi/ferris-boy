const MEMORY_SIZE: usize = 4096;
const PROGRAM_START: usize = 0x200;

// Ferris8 Struct
/// holds the memory and pc of a Ferris8 instance
pub struct Ferris8 {
  memory: [u8; MEMORY_SIZE],
  pc: u16,
}

// Ferris8 Methods
/// actually do something with a Ferris8 instance
impl Ferris8 {
    // New
    /// creates a new Ferris8 instance
    pub fn new() -> Self {
        Ferris8 {
            memory: [0; MEMORY_SIZE],
            pc: PROGRAM_START as u16,
        }
    }

    // Load Rom
    /// loads a new rom into the Ferris8 instance
    pub fn load_rom(&mut self, data: &[u8]) -> Result<(), String> {
        // rom is too big to fit in memory
        if data.len() > MEMORY_SIZE - PROGRAM_START {
            return Err(format!("ROM too big to be loaded: {} bytes", data.len()));
        }

        // copy the rom data into memory, starting at program start
        self.memory[PROGRAM_START..PROGRAM_START + data.len()].copy_from_slice(data);
        Ok(())
    }

    // Fetch Instruction
    /// fetches 2 bytes from memory, and merges them to get the next instruction
    /// (instructions are stored in big-endian)
    pub fn fetch_instr(&mut self) -> u16 {
        // get both bytes
        let hi = self.memory[self.pc as usize] as u16;
        let lo = self.memory[(self.pc as usize) + 1] as u16;

        // increment pc
        self.pc += 2;

        // combine the two bytes
        (hi << 8) | lo
    }
}

// Default Impl
/// default just creates a new Ferris8 instance
impl Default for Ferris8 {
    fn default() -> Self {
        Self::new()
    }
}
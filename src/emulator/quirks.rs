//! Created by Zachary Lebak
//!
//! --- ADD A DESCRIPTION OF THE FILE LATER
//!
//!

// Quirk Profile
/// the different chip8 interpreters
#[derive(Debug, Clone, Copy)]
pub enum QuirkProfile {
    Vip,
    Chip48,
    SuperChip,
}

// Quirks
/// possible quirks each profile can have
#[derive(Debug, Clone, Copy)]
pub struct Quirks {
    pub shift_uses_vy: bool,
    pub load_store_increments_i: bool,
    pub jump_uses_vx: bool,
    pub sprite_wrap: bool,
}

// Default
/// default quirk profile is Vip
impl Default for Quirks {
    fn default() -> Self {
        Quirks::from_profile(QuirkProfile::Vip)
    }
}

// Impl Quirks
/// create a new quirk struct from a specific profile
impl Quirks {
    pub fn from_profile(_profile: QuirkProfile) -> Self {
        /* FILL THESE IN PER PROFILE LATER */
        Quirks {
            shift_uses_vy: true,
            load_store_increments_i: true,
            jump_uses_vx: true,
            sprite_wrap: true,
        }
    }
}

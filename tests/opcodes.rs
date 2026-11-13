//! Created by Zachary Lebak and Giancarlo Ambrosino
//!
//! --- ADD A DESCRIPTION OF THE FILE LATER
//!
//!

use ferris8::Ferris8;

#[test]
/// New Ferris8 instance should start its pc at 0x200
fn new_starts_at_0x200() {
    assert_eq!(Ferris8::new().get_pc(), 0x200);
}

/* ADD MORE TESTS */

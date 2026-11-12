# ROMs

A few test ROMs and programs used to develop/verify the emulator.
All are from Timendus on GitHub.

[text](https://github.com/Timendus/chip8-test-suite#available-tests)


## Included ROM - Chip8 Logo
'1-chip8-logo.ch8'

    Tests opcodes:
        00E0 - Clear the screen
        6xnn - Load normal register with immediate value
        Annn - Load index register with immediate value
        Dxyn - Draw sprite to screen (only aligned)
    If run for more than 39 cycles, tests:
        1nnn - Jump


## Included ROM - IBM Logo
'2-ibm-logo.ch8'

    Tests opcodes:
        00E0 - Clear the screen
        6xnn - Load normal register with immediate value
        Annn - Load index register with immediate value
        7xnn - Add immediate value to normal register
        Dxyn - Draw sprite to screen (un-aligned)
    If run for more than 20 cycles, tests:
        1nnn - Jump


## Included ROM - Corax+
'3-corax+.ch8'

    Tests opcodes:
        3xnn - if vX == nn, skip next opcode
        4xnn - if vX != nn, skip next opcode
        5xy0 - if vX == vY, skip next opcode,
        7xnn - vX += nn
        9xy0 - if vX != vY, skip next opcode
        1nnn - jump nnn (goto)
        2nnn - call nnn (subroutine)
        00EE - return from subroutine
        8xy0 - vX = vY
        8xy1 - vX |= vY
        8xy2 - vX &= vY
        8xy3 - vX ^= vY
        8xy4 - vX += vY
        8xy5 - vX -= vY
        8xy7 - vX = vY - vX
        8xy6 - vX = vY >> 1 or vX = vX >> 1 depending on quirks
        8xyE - vX = vY << 1 or vX = vX << 1 depending on quirks
        Fx65 - load registers v0 - vX from memory starting at i
        Fx55 - save registers v0 - vX to memory starting at i
        Fx33 - store binary-coded decimal representation of vX to memory at i, i + 1 and i + 2
        Fx1E - i += vX
        Registers - The v0 - vF registers should be 8 bits wide. 
            This tests to see if it can overflow your registers.

## Included ROM - Flags
'4-flags.ch8'

    Top (HAPPY) tests opcodes:
        8xy1 - vX |= vY
        8xy2 - vX &= vY
        8xy3 - vX ^= vY
        8xy4 - vX += vY
        8xy5 - vX -= vY
        8xy6 - vX = vY >> 1 or vX = vX >> 1 depending on quirks
        8xy7 - vX = vY - vX
        8xyE - vX = vY << 1 or vX = vX << 1 depending on quirks
    Bottom (CARRY) tests opcodes:
        8xy4 - vX += vY
        8xy5 - vX -= vY
        8xy6 - vX = vY >> 1 or vX = vX >> 1 depending on quirks
        8xy7 - vX = vY - vX
        8xyE - vX = vY << 1 or vX = vX << 1 depending on quirks


## Included ROM - Quirks
'5-quirks.ch8'

    Tests:
        vF reset - The AND, OR and XOR opcodes (8xy1, 8xy2 and 8xy3) reset the flags register to zero. Test will show ERR1 if the AND and OR tests don't behave the same and ERR2 if the AND and XOR tests don't behave the same.
        Memory - The save and load opcodes (Fx55 and Fx65) increment the index register. More information here and here. Test will show ERR1 if reading and writing don't behave the same.
        Display wait - Drawing sprites to the display waits for the vertical blank interrupt, limiting their speed to max 60 sprites per second. More information here. Test will show SLOW if the number of cycles per frame is too low for the test to be deterministic (this is not necessarily an error, but a suggestion to rerun the test with a higher number of cycles per frame).
        Clipping - Sprites drawn at the bottom edge of the screen get clipped instead of wrapping around to the top of the screen. When clipping is off, the test checks if sprites get rendered at the right coordinates on the other side of the screen. This also tests that sprites drawn at coordinates of x > 63 and/or y > 31 wrap around to x % 64 and y % 32. More information here. Test will show ERR1 if the clipping is inconsistent in different dimensions or wrapping to the wrong coordinates and ERR2 if sprites don't wrap around as expected.
        Shifting - The shift opcodes (8xy6 and 8xyE) only operate on vX instead of storing the shifted version of vY in vX (more information here). Test will show ERR1 if the shift opcodes behave differently.
        Jumping - The "jump to some address plus v0" instruction (Bnnn) doesn't use v0, but vX instead where X is the highest nibble of nnn (more information here)


## Included ROM - Keypad
'6-keypad.ch8'
    
    You can press any of the numbers 1 to 3 on the CHIP-8 keypad to jump to the corresponding test.

    Alternatively, you can move the cursor up and down with CHIP-8 keys E and F and select an item with A. This feature mainly exists so people implementing interpreters for platforms with limited input devices (like a game controller) can map their buttons to those CHIP-8 keys and have an intuitive interface too.

    If you want to repeat a test often or even automate them, having to use the graphical menu just gets in the way. In that case, you can force this ROM to select a specific platform by loading a value between 1 and 3 into memory at the address 0x1FF (512).


## Included ROM - Beep
'7-beep.ch8'
    
    This test allows you to test if your buzzer is working. It will beep SOS in morse code and flash a speaker icon on the display in the same pattern. If you press the CHIP-8 button B it will give you manual control over the buzzer. Press B to beep.


## Included ROM - Scrolling
'8-scrolling.ch8'
    
    You can press any of the numbers 1 or 2 on the CHIP-8 keypad to select the corresponding entry.

    Alternatively, you can move the cursor up and down with CHIP-8 keys E and F and select an item with A. This feature mainly exists so people implementing interpreters for platforms with limited input devices (like a game controller) can map their buttons to those CHIP-8 keys and have an intuitive interface too.

    If you want to repeat a test often or even automate them, having to use the graphical menu just gets in the way. In that case, you can force this ROM to select a specific platform by loading a value between 1 and 5 into memory at the address 0x1FF (512).

    1 - SUPER-CHIP lores with modern behaviour
    2 - SUPER-CHIP lores with legacy behaviour
    3 - SUPER-CHIP hires
    4 - XO-CHIP lores
    5 - XO-CHIP hires


## How to Get the ROMs


Download each ROM from the GitHub repository linked above.


## Not Included ROMs

Comerical or unlicensed ROMs are not present in this repository.
To run other programs using a CHIP-8 architecture, download them yourself
and pass the path:

    cargo run -- path/to/game.ch8
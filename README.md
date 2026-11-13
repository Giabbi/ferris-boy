chip8/
├── Cargo.toml
├── README.md              # build steps, every CLI flag, config format, controls
├── config.example.toml    # documented sample config
├── roms/                  # only ROMs you're allowed to redistribute
│   └── README.md          # sources and licenses
├── src/
│   ├── lib.rs             # declares modules, re-exports public API
│   ├── main.rs            # entry point: parse CLI, load config, run loop
│   ├── cli.rs             # clap argument definitions
│   ├── config.rs          # serde config struct, defaults, loading
│   ├── error.rs           # EmuError enum, From impls
│   ├── emulator/
│   │   ├── mod.rs         # Chip8 struct, new(), load_rom(), step(), tick_timers()
│   │   ├── instruction.rs # enum Instruction, From<u16>, Display
│   │   ├── quirks.rs      # Quirks struct/profiles, FromStr
│   │   ├── display.rs     # framebuffer type and DRW logic
│   │   ├── keypad.rs      # key state
│   │   ├── font.rs        # font sprite constant
│   │   └── savestate.rs   # serde save/load of machine state
│   ├── frontend/
│   │   ├── mod.rs         # Frontend trait
│   │   ├── minifb.rs      # MinifbFrontend (window, input, audio)
│   │   ├── headless.rs    # HeadlessFrontend (for tests)
│   │   └── screenshot.rs  # PNG export using the image crate
│   ├── rng.rs             # RandomSource trait, SystemRng, FixedRng
│   └── bin/
│       └── disasm.rs      # optional: second entry point, prints a ROM disassembly
└── tests/
    ├── opcodes.rs         # unit-style tests per instruction using FixedRng
    └── rom_tests.rs       # runs Timendus ROMs headlessly, checks the framebuffer

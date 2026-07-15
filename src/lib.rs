#![no_std]

pub mod pid;
pub mod registers;
pub mod spi;

/// TMC4671 expected chip identification value.
/// ASCII encoding of "4671".
pub const CHIP_ID: u32 = 0x3436_3731;

/// TMC4671 external clock frequency (Hz).
/// Both OpenFFBoard and Ouroboros use a 25 MHz oscillator.
pub const TMC_CLOCK_HZ: u32 = 25_000_000;

#![no_std]

pub mod current_limit;
pub mod pid;
pub mod registers;
pub mod spi;

/// TMC4671 expected chip identification value.
/// ASCII encoding of "4671".
pub const CHIP_ID: u32 = 0x3436_3731;

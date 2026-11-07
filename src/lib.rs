//! Driver for the TMC4671 FOC servo controller: register map, typed PID gain
//! and current-limit domains, and the SPI access layer.

#![no_std]
// Lints that fire only in test code; production code is still checked for them.
#![cfg_attr(
    test,
    allow(clippy::decimal_bitwise_operands, clippy::unused_async_trait_impl)
)]
#![warn(missing_docs)]

pub mod current_limit;
pub mod pid;
pub mod registers;
pub mod spi;

/// TMC4671 expected chip identification value.
/// ASCII encoding of "4671".
pub const CHIP_ID: u32 = 0x3436_3731;

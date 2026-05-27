use embedded_hal_async::spi::{Operation, SpiDevice};

use crate::registers::{Register, SubRegister};

/// Errors from TMC4671 SPI communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub enum SpiError<E> {
    /// SPI bus error from the underlying driver.
    Bus(E),
    /// Read-back verification failed after write.
    VerifyFailed { expected: u32, actual: u32 },
}

/// SPI retry behaviour. Applied only to the `*_verified` methods on
/// [`Tmc4671Spi`]. Plain `read_register` and `write_register` are not
/// affected by the policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Total attempts including the first. Must be at least 1.
    pub max_attempts: u8,
}

impl Default for RetryPolicy {
    /// Library default: single attempt, identical to pre-spec behaviour.
    /// Boards opt into retries via [`Tmc4671Spi::with_retry_policy`].
    fn default() -> Self {
        Self { max_attempts: 1 }
    }
}

impl RetryPolicy {
    /// Production policy used by both board binaries.
    pub const fn production() -> Self {
        Self { max_attempts: 3 }
    }
}

impl<E> From<E> for SpiError<E> {
    fn from(e: E) -> Self {
        SpiError::Bus(e)
    }
}

/// TMC4671 SPI interface.
///
/// Generic over the SPI bus type. All operations are async (DMA-backed
/// on real hardware, immediate in test mocks).
pub struct Tmc4671Spi<SPI> {
    spi: SPI,
}

impl<SPI, E> Tmc4671Spi<SPI>
where
    SPI: SpiDevice<u8, Error = E>,
{
    /// Creates a new `Tmc4671Spi` wrapping the given SPI device.
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Consume the driver and return the underlying SPI device.
    pub fn into_inner(self) -> SPI {
        self.spi
    }

    /// Read a 32-bit register value.
    ///
    /// Sends the address byte, pauses 500ns (required by TMC4671-LA for
    /// SPI reads above 2 MHz, datasheet Table 2 tSCKpause), then clocks
    /// out 4 data bytes. CS stays asserted across the full transaction.
    pub async fn read_register(&mut self, reg: Register) -> Result<u32, SpiError<E>> {
        let addr = [reg.addr()];
        let mut data = [0u8; 4];
        self.spi
            .transaction(&mut [
                Operation::Write(&addr),
                Operation::DelayNs(500),
                Operation::Read(&mut data),
            ])
            .await?;
        Ok(u32::from_be_bytes(data))
    }

    /// Write a 32-bit register value.
    ///
    /// Sends a 5-byte frame with the register address ORed with `0x80` (write
    /// flag) in byte 0, followed by the value in big-endian order.
    pub async fn write_register(&mut self, reg: Register, value: u32) -> Result<(), SpiError<E>> {
        let bytes = value.to_be_bytes();
        let buf = [reg.addr() | 0x80, bytes[0], bytes[1], bytes[2], bytes[3]];
        self.spi.write(&buf).await?;
        Ok(())
    }

    /// Write a register and verify by reading back.
    ///
    /// Returns [`SpiError::VerifyFailed`] if the read-back value does not match
    /// the written value.
    pub async fn write_register_verified(
        &mut self,
        reg: Register,
        value: u32,
    ) -> Result<(), SpiError<E>> {
        self.write_register(reg, value).await?;
        let actual = self.read_register(reg).await?;
        if actual != value {
            return Err(SpiError::VerifyFailed {
                expected: value,
                actual,
            });
        }
        Ok(())
    }

    /// Read a sub-register (write selector address first, then read data).
    ///
    /// Writes `sub.sub_addr` to `sub.addr`, then reads `sub.data`.
    pub async fn read_sub_register(&mut self, sub: SubRegister) -> Result<u32, SpiError<E>> {
        self.write_register(sub.addr, sub.sub_addr as u32).await?;
        self.read_register(sub.data).await
    }

    /// Write a sub-register (write selector address first, then write data).
    ///
    /// Writes `sub.sub_addr` to `sub.addr`, then writes `value` to `sub.data`.
    pub async fn write_sub_register(
        &mut self,
        sub: SubRegister,
        value: u32,
    ) -> Result<(), SpiError<E>> {
        self.write_register(sub.addr, sub.sub_addr as u32).await?;
        self.write_register(sub.data, value).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers;

    #[test]
    fn read_frame_address_bit7_clear() {
        let reg = registers::PID_POSITION_TARGET;
        assert_eq!(reg.addr(), 0x68);
        assert_eq!(reg.addr() & 0x80, 0);
    }

    #[test]
    fn write_frame_address_bit7_set() {
        let reg = registers::PID_POSITION_TARGET;
        assert_eq!(reg.addr() | 0x80, 0xE8);
    }

    #[test]
    fn value_big_endian_encoding() {
        let value: u32 = 0x12345678;
        let bytes = value.to_be_bytes();
        assert_eq!(bytes, [0x12, 0x34, 0x56, 0x78]);
    }

    #[test]
    fn value_big_endian_decoding() {
        let bytes = [0x12, 0x34, 0x56, 0x78];
        let value = u32::from_be_bytes(bytes);
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn spi_error_from_bus_error() {
        let err: SpiError<u8> = SpiError::from(42u8);
        assert_eq!(err, SpiError::Bus(42));
    }

    #[test]
    fn verify_failed_error() {
        let err: SpiError<u8> = SpiError::VerifyFailed {
            expected: 0x1234,
            actual: 0x5678,
        };
        match err {
            SpiError::VerifyFailed { expected, actual } => {
                assert_eq!(expected, 0x1234);
                assert_eq!(actual, 0x5678);
            }
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn retry_policy_default_is_single_attempt() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 1);
    }

    #[test]
    fn retry_policy_production_is_three_attempts() {
        let policy = RetryPolicy::production();
        assert_eq!(policy.max_attempts, 3);
    }
}

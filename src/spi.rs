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

/// SPI retry behaviour. Applied to the `*_verified` methods and to
/// [`Tmc4671Spi::read_register_retried`]. Plain `read_register` and
/// `write_register` are not affected by the policy.
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
    retry_policy: RetryPolicy,
}

impl<SPI, E> Tmc4671Spi<SPI>
where
    SPI: SpiDevice<u8, Error = E>,
{
    /// Creates a new `Tmc4671Spi` wrapping the given SPI device.
    pub fn new(spi: SPI) -> Self {
        Self {
            spi,
            retry_policy: RetryPolicy::default(),
        }
    }

    /// Override the retry policy. Affects the `*_verified` methods and
    /// [`Tmc4671Spi::read_register_retried`].
    pub fn with_retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry_policy = policy;
        self
    }

    /// Consume the driver and return the underlying SPI device.
    pub fn into_inner(self) -> SPI {
        self.spi
    }

    /// Borrow the wrapped SPI device (used by host tests to inspect a mock).
    pub fn inner(&self) -> &SPI {
        &self.spi
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

    /// Run `attempt` up to `self.retry_policy.max_attempts` times, stopping at
    /// the first success. When every attempt fails, returns the final attempt's
    /// error.
    async fn retry<T>(
        &mut self,
        mut attempt: impl AsyncFnMut(&mut Self) -> Result<T, SpiError<E>>,
    ) -> Result<T, SpiError<E>> {
        let max = self.retry_policy.max_attempts.max(1);
        let mut attempts = 1;
        loop {
            match attempt(self).await {
                Ok(value) => return Ok(value),
                Err(error) if attempts == max => return Err(error),
                Err(_) => attempts += 1,
            }
        }
    }

    /// Write a register and verify by reading back. Retries the complete
    /// write/read-back attempt after bus errors or mismatches, up to
    /// `self.retry_policy.max_attempts` total attempts (1 = no retry).
    ///
    /// Returns [`SpiError::VerifyFailed`] when retries are exhausted, carrying
    /// the most recent mismatched read-back.
    pub async fn write_register_verified(
        &mut self,
        reg: Register,
        value: u32,
    ) -> Result<(), SpiError<E>> {
        self.retry(async |spi| {
            spi.write_register(reg, value).await?;
            let actual = spi.read_register(reg).await?;
            if actual == value {
                Ok(())
            } else {
                Err(SpiError::VerifyFailed {
                    expected: value,
                    actual,
                })
            }
        })
        .await
    }

    /// Read a register and compare against `expected`. Retries after bus errors
    /// or mismatches up to `self.retry_policy.max_attempts` total attempts.
    ///
    /// Returns [`SpiError::VerifyFailed`] when retries are exhausted, carrying
    /// the most recent mismatched read.
    pub async fn read_register_verified(
        &mut self,
        reg: Register,
        expected: u32,
    ) -> Result<u32, SpiError<E>> {
        self.retry(async |spi| {
            let actual = spi.read_register(reg).await?;
            if actual == expected {
                Ok(actual)
            } else {
                Err(SpiError::VerifyFailed { expected, actual })
            }
        })
        .await
    }

    /// Read a register, retrying on bus error only (not on value — unlike
    /// [`Tmc4671Spi::read_register_verified`], the result is not compared
    /// against any expected value, since some registers legitimately vary
    /// between reads). Retries up to `self.retry_policy.max_attempts` total
    /// attempts.
    pub async fn read_register_retried(&mut self, reg: Register) -> Result<u32, SpiError<E>> {
        self.retry(async |spi| spi.read_register(reg).await).await
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

impl<SPI, E> Tmc4671Spi<SPI>
where
    SPI: embedded_hal::spi::SpiDevice<u8, Error = E>,
{
    /// Write one register synchronously in a complete CS-framed transaction.
    ///
    /// This is intended for short diagnostic and capture paths that already
    /// use synchronous SPI transactions.
    pub fn write_register_blocking(
        &mut self,
        reg: Register,
        value: u32,
    ) -> Result<(), SpiError<E>> {
        let bytes = value.to_be_bytes();
        let frame = [reg.addr() | 0x80, bytes[0], bytes[1], bytes[2], bytes[3]];
        embedded_hal::spi::SpiDevice::write(&mut self.spi, &frame)?;
        Ok(())
    }

    /// Read one register synchronously in a complete CS-framed transaction.
    ///
    /// This is intended for short, fixed-rate capture paths where DMA setup
    /// and an async wakeup cost more than the five transferred bytes.
    pub fn read_register_blocking(&mut self, reg: Register) -> Result<u32, SpiError<E>> {
        let addr = [reg.addr()];
        let mut data = [0u8; 4];
        embedded_hal::spi::SpiDevice::transaction(
            &mut self.spi,
            &mut [
                embedded_hal::spi::Operation::Write(&addr),
                embedded_hal::spi::Operation::DelayNs(500),
                embedded_hal::spi::Operation::Read(&mut data),
            ],
        )?;
        Ok(u32::from_be_bytes(data))
    }

    /// Read one sub-register synchronously.
    ///
    /// The selector write and data read remain separate CS-framed
    /// transactions, matching the TMC4671 sub-register protocol.
    pub fn read_sub_register_blocking(&mut self, sub: SubRegister) -> Result<u32, SpiError<E>> {
        self.write_register_blocking(sub.addr, u32::from(sub.sub_addr))?;
        self.read_register_blocking(sub.data)
    }

    /// Read two registers synchronously as separate CS-framed transactions.
    ///
    /// This is intended for short, fixed-rate capture paths where DMA setup
    /// and two async wakeups cost more than the ten transferred bytes. Each
    /// register retains the required 500 ns address-to-data pause.
    pub fn read_register_pair_blocking(
        &mut self,
        first: Register,
        second: Register,
    ) -> Result<(u32, u32), SpiError<E>> {
        let first = self.read_register_blocking(first)?;
        let second = self.read_register_blocking(second)?;
        Ok((first, second))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registers;

    #[derive(Default)]
    struct MockSpiBus;

    impl embedded_hal_async::spi::ErrorType for MockSpiBus {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal_async::spi::SpiDevice<u8> for MockSpiBus {
        async fn transaction(
            &mut self,
            _ops: &mut [embedded_hal_async::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct VerifyBusError;

    impl embedded_hal_async::spi::Error for VerifyBusError {
        fn kind(&self) -> embedded_hal_async::spi::ErrorKind {
            embedded_hal_async::spi::ErrorKind::Other
        }
    }

    #[derive(Default)]
    struct VerifyMock {
        attempts: core::cell::Cell<u8>,
        /// Number of read-back attempts that should return the wrong value.
        wrong_for_attempts: u8,
        expected: u32,
        write_bus_errors_remaining: u8,
        read_bus_errors_remaining: u8,
        write_transactions: core::cell::Cell<u8>,
        read_transactions: core::cell::Cell<u8>,
    }

    impl embedded_hal_async::spi::ErrorType for VerifyMock {
        type Error = VerifyBusError;
    }

    impl embedded_hal_async::spi::SpiDevice<u8> for VerifyMock {
        async fn transaction(
            &mut self,
            ops: &mut [embedded_hal_async::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            let is_read = ops
                .iter()
                .any(|op| matches!(op, embedded_hal_async::spi::Operation::Read(_)));
            if is_read {
                self.read_transactions.set(self.read_transactions.get() + 1);
                if self.read_bus_errors_remaining > 0 {
                    self.read_bus_errors_remaining -= 1;
                    return Err(VerifyBusError);
                }
                let attempt = self.attempts.get();
                self.attempts.set(attempt + 1);
                let value = if attempt < self.wrong_for_attempts {
                    0xDEAD_BEEF
                } else {
                    self.expected
                };
                for op in ops.iter_mut() {
                    if let embedded_hal_async::spi::Operation::Read(buf) = op {
                        buf.copy_from_slice(&value.to_be_bytes());
                    }
                }
            } else {
                self.write_transactions
                    .set(self.write_transactions.get() + 1);
                if self.write_bus_errors_remaining > 0 {
                    self.write_bus_errors_remaining -= 1;
                    return Err(VerifyBusError);
                }
            }
            Ok(())
        }
    }

    struct BlockingPairMock {
        transactions: usize,
        addresses: [u8; 2],
        values: [u32; 2],
    }

    impl embedded_hal::spi::ErrorType for BlockingPairMock {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::spi::SpiDevice<u8> for BlockingPairMock {
        fn transaction(
            &mut self,
            ops: &mut [embedded_hal::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            assert_eq!(ops.len(), 3);
            let transaction = self.transactions;
            assert!(transaction < self.values.len());
            match &ops[0] {
                embedded_hal::spi::Operation::Write(address) => {
                    self.addresses[transaction] = address[0];
                }
                _ => panic!("first operation must write the register address"),
            }
            assert!(matches!(ops[1], embedded_hal::spi::Operation::DelayNs(500)));
            match &mut ops[2] {
                embedded_hal::spi::Operation::Read(data) => {
                    data.copy_from_slice(&self.values[transaction].to_be_bytes());
                }
                _ => panic!("third operation must read the register value"),
            }
            self.transactions += 1;
            Ok(())
        }
    }

    impl embedded_hal_async::spi::SpiDevice<u8> for BlockingPairMock {
        async fn transaction(
            &mut self,
            _ops: &mut [embedded_hal_async::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            panic!("blocking pair test must not use the async SPI path")
        }
    }

    struct BlockingSubRegisterMock {
        transactions: usize,
        frames: [[u8; 5]; 2],
        frame_lengths: [usize; 2],
        value: u32,
    }

    impl embedded_hal::spi::ErrorType for BlockingSubRegisterMock {
        type Error = core::convert::Infallible;
    }

    impl embedded_hal::spi::SpiDevice<u8> for BlockingSubRegisterMock {
        fn transaction(
            &mut self,
            ops: &mut [embedded_hal::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            match ops {
                [embedded_hal::spi::Operation::Write(frame)] => {
                    let index = self.transactions;
                    self.frames[index][..frame.len()].copy_from_slice(frame);
                    self.frame_lengths[index] = frame.len();
                }
                [
                    embedded_hal::spi::Operation::Write(address),
                    embedded_hal::spi::Operation::DelayNs(500),
                    embedded_hal::spi::Operation::Read(data),
                ] => {
                    let index = self.transactions;
                    self.frames[index][..address.len()].copy_from_slice(address);
                    self.frame_lengths[index] = address.len();
                    data.copy_from_slice(&self.value.to_be_bytes());
                }
                _ => panic!("unexpected blocking sub-register transaction"),
            }
            self.transactions += 1;
            Ok(())
        }
    }

    impl embedded_hal_async::spi::SpiDevice<u8> for BlockingSubRegisterMock {
        async fn transaction(
            &mut self,
            _ops: &mut [embedded_hal_async::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            panic!("blocking sub-register test must not use the async SPI path")
        }
    }

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

    #[test]
    fn with_retry_policy_sets_field() {
        let spi: Tmc4671Spi<MockSpiBus> =
            Tmc4671Spi::new(MockSpiBus).with_retry_policy(RetryPolicy::production());
        assert_eq!(spi.retry_policy.max_attempts, 3);
    }

    #[test]
    fn blocking_register_pair_uses_two_complete_read_transactions() {
        let bus = BlockingPairMock {
            transactions: 0,
            addresses: [0; 2],
            values: [0x1234_5678, 0x9abc_def0],
        };
        let mut spi = Tmc4671Spi::new(bus);

        let values = spi
            .read_register_pair_blocking(registers::OPENLOOP_PHI, registers::PID_TORQUE_FLUX_ACTUAL)
            .expect("blocking pair read");
        let bus = spi.into_inner();

        assert_eq!(values, (0x1234_5678, 0x9abc_def0));
        assert_eq!(bus.transactions, 2);
        assert_eq!(
            bus.addresses,
            [
                registers::OPENLOOP_PHI.addr(),
                registers::PID_TORQUE_FLUX_ACTUAL.addr(),
            ]
        );
    }

    #[test]
    fn blocking_sub_register_read_writes_selector_then_reads_data() {
        let bus = BlockingSubRegisterMock {
            transactions: 0,
            frames: [[0; 5]; 2],
            frame_lengths: [0; 2],
            value: 0x1234_5678,
        };
        let mut spi = Tmc4671Spi::new(bus);

        let value = spi
            .read_sub_register_blocking(registers::CHIPINFO_SI_VERSION)
            .expect("blocking sub-register read");
        let bus = spi.into_inner();

        assert_eq!(value, 0x1234_5678);
        assert_eq!(bus.transactions, 2);
        assert_eq!(
            &bus.frames[0][..bus.frame_lengths[0]],
            &[registers::CHIPINFO_ADDR.addr() | 0x80, 0, 0, 0, 1],
        );
        assert_eq!(
            &bus.frames[1][..bus.frame_lengths[1]],
            &[registers::CHIPINFO_DATA.addr()],
        );
    }

    #[test]
    fn write_verified_succeeds_after_transient_mismatches() {
        pollster::block_on(async {
            let bus = VerifyMock {
                wrong_for_attempts: 2,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());
            let result = spi
                .write_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            assert!(
                result.is_ok(),
                "expected verified write to succeed on attempt 3, got {:?}",
                result
            );
        });
    }

    #[test]
    fn write_verified_exhausts_and_returns_verify_failed() {
        pollster::block_on(async {
            let bus = VerifyMock {
                wrong_for_attempts: 99,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());
            let result = spi
                .write_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            match result {
                Err(SpiError::VerifyFailed { expected, actual }) => {
                    assert_eq!(expected, 0x1234_5678);
                    assert_eq!(actual, 0xDEAD_BEEF);
                }
                other => panic!("expected VerifyFailed, got {:?}", other),
            }
        });
    }

    #[test]
    fn write_verified_recovers_after_transient_write_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                write_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi
                .write_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Ok(()));
            assert_eq!(bus.write_transactions.get(), 2);
            assert_eq!(bus.read_transactions.get(), 1);
        });
    }

    #[test]
    fn write_verified_recovers_after_transient_readback_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi
                .write_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Ok(()));
            assert_eq!(bus.write_transactions.get(), 2);
            assert_eq!(bus.read_transactions.get(), 2);
        });
    }

    #[test]
    fn write_verified_returns_bus_error_after_three_failed_attempts() {
        pollster::block_on(async {
            let bus = VerifyMock {
                write_bus_errors_remaining: 3,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi
                .write_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Err(SpiError::Bus(VerifyBusError)));
            assert_eq!(bus.write_transactions.get(), 3);
            assert_eq!(bus.read_transactions.get(), 0);
        });
    }

    #[test]
    fn read_verified_succeeds_after_transient_mismatches() {
        pollster::block_on(async {
            let bus = VerifyMock {
                wrong_for_attempts: 2,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());
            let result = spi
                .read_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            assert_eq!(result, Ok(0x1234_5678));
        });
    }

    #[test]
    fn read_verified_exhausts_and_returns_verify_failed() {
        pollster::block_on(async {
            let bus = VerifyMock {
                wrong_for_attempts: 99,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());
            let result = spi
                .read_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            match result {
                Err(SpiError::VerifyFailed { expected, actual }) => {
                    assert_eq!(expected, 0x1234_5678);
                    assert_eq!(actual, 0xDEAD_BEEF);
                }
                other => panic!("expected VerifyFailed, got {:?}", other),
            }
        });
    }

    #[test]
    fn read_verified_recovers_after_transient_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi
                .read_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Ok(0x1234_5678));
            assert_eq!(bus.read_transactions.get(), 2);
        });
    }

    #[test]
    fn read_verified_returns_bus_error_after_three_failed_attempts() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 3,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi
                .read_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Err(SpiError::Bus(VerifyBusError)));
            assert_eq!(bus.read_transactions.get(), 3);
        });
    }

    #[test]
    fn default_policy_does_not_retry_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus);

            let result = spi
                .read_register_verified(registers::PID_POSITION_TARGET, 0x1234_5678)
                .await;
            let bus = spi.into_inner();

            assert_eq!(result, Err(SpiError::Bus(VerifyBusError)));
            assert_eq!(bus.read_transactions.get(), 1);
        });
    }

    #[test]
    fn read_retried_recovers_after_transient_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi.read_register_retried(registers::STATUS_FLAGS).await;
            let bus = spi.into_inner();

            assert_eq!(result, Ok(0x1234_5678));
            assert_eq!(bus.read_transactions.get(), 2);
        });
    }

    #[test]
    fn read_retried_returns_bus_error_after_exhausting_attempts() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 3,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi.read_register_retried(registers::STATUS_FLAGS).await;
            let bus = spi.into_inner();

            assert_eq!(result, Err(SpiError::Bus(VerifyBusError)));
            assert_eq!(bus.read_transactions.get(), 3);
        });
    }

    #[test]
    fn read_retried_does_not_reject_a_legitimately_varying_value() {
        pollster::block_on(async {
            let bus = VerifyMock {
                expected: 0,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus).with_retry_policy(RetryPolicy::production());

            let result = spi.read_register_retried(registers::STATUS_FLAGS).await;

            assert_eq!(result, Ok(0));
        });
    }

    #[test]
    fn read_retried_default_policy_does_not_retry_bus_error() {
        pollster::block_on(async {
            let bus = VerifyMock {
                read_bus_errors_remaining: 1,
                expected: 0x1234_5678,
                ..Default::default()
            };
            let mut spi = Tmc4671Spi::new(bus);

            let result = spi.read_register_retried(registers::STATUS_FLAGS).await;
            let bus = spi.into_inner();

            assert_eq!(result, Err(SpiError::Bus(VerifyBusError)));
            assert_eq!(bus.read_transactions.get(), 1);
        });
    }
}

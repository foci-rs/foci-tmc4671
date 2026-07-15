//! TMC4671 torque/flux current-limit domain.

/// Scalar current limit shared by the TMC4671 torque and flux controllers.
///
/// The chip accepts values from `0..=0x7fff` in bits 15:0 of
/// `PID_TORQUE_FLUX_LIMITS`. Bits 31:16 remain clear when this type is
/// serialized with [`Self::register_value`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidTorqueFluxLimit(u16);

impl PidTorqueFluxLimit {
    /// Disabled current limit.
    pub const ZERO: Self = Self(0);
    /// Largest documented TMC4671 torque/flux current limit.
    pub const MAX: Self = Self(0x7fff);

    /// Validate a raw current-limit register value.
    ///
    /// # Errors
    ///
    /// Returns [`PidTorqueFluxLimitOutOfRange`] when the value exceeds the
    /// documented low-half domain or sets any upper register bits.
    pub const fn new(raw: u32) -> Result<Self, PidTorqueFluxLimitOutOfRange> {
        if raw <= Self::MAX.0 as u32 {
            Ok(Self(raw as u16))
        } else {
            Err(PidTorqueFluxLimitOutOfRange { raw })
        }
    }

    /// Return the validated scalar current-limit count.
    pub const fn raw(self) -> u16 {
        self.0
    }

    /// Encode the complete register word with bits 31:16 clear.
    pub const fn register_value(self) -> u32 {
        self.0 as u32
    }
}

impl TryFrom<u32> for PidTorqueFluxLimit {
    type Error = PidTorqueFluxLimitOutOfRange;

    fn try_from(raw: u32) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl From<PidTorqueFluxLimit> for u16 {
    fn from(limit: PidTorqueFluxLimit) -> Self {
        limit.raw()
    }
}

impl From<PidTorqueFluxLimit> for u32 {
    fn from(limit: PidTorqueFluxLimit) -> Self {
        limit.register_value()
    }
}

/// Raw value outside the TMC4671 torque/flux current-limit domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidTorqueFluxLimitOutOfRange {
    raw: u32,
}

impl PidTorqueFluxLimitOutOfRange {
    /// Return the rejected raw register value.
    pub const fn raw(self) -> u32 {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_limit_accepts_exact_domain_boundaries() {
        assert_eq!(
            PidTorqueFluxLimit::new(0).unwrap(),
            PidTorqueFluxLimit::ZERO
        );
        assert_eq!(
            PidTorqueFluxLimit::new(0x7fff).unwrap(),
            PidTorqueFluxLimit::MAX
        );
        assert_eq!(PidTorqueFluxLimit::MAX.raw(), 0x7fff);
        assert_eq!(PidTorqueFluxLimit::MAX.register_value(), 0x0000_7fff);
    }

    #[test]
    fn current_limit_rejects_out_of_range_and_reserved_bits() {
        for raw in [0x8000, 0x1_0000, u32::MAX] {
            assert_eq!(PidTorqueFluxLimit::new(raw).unwrap_err().raw(), raw);
        }
    }

    #[test]
    fn current_limit_checked_and_infallible_conversions_preserve_raw_value() {
        let limit = PidTorqueFluxLimit::try_from(0x1234).unwrap();

        assert_eq!(u16::from(limit), 0x1234);
        assert_eq!(u32::from(limit), 0x0000_1234);
    }
}

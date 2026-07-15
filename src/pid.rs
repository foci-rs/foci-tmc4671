//! PID gain domain for TMC4671 control loops.

/// Non-negative TMC4671 PID gain accepted by the signed 16-bit gain fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidGain(u16);

impl PidGain {
    /// Disabled gain.
    pub const ZERO: Self = Self(0);
    /// Largest non-negative gain accepted by the TMC4671.
    pub const MAX: Self = Self(0x7fff);

    /// Validate one raw PID gain.
    ///
    /// # Errors
    ///
    /// Returns [`PidGainOutOfRange`] when the raw value would encode a
    /// negative signed 16-bit gain.
    pub const fn new(raw: u16) -> Result<Self, PidGainOutOfRange> {
        if raw <= Self::MAX.0 {
            Ok(Self(raw))
        } else {
            Err(PidGainOutOfRange { raw })
        }
    }

    /// Return the validated raw register value.
    pub const fn raw(self) -> u16 {
        self.0
    }
}

impl TryFrom<u16> for PidGain {
    type Error = PidGainOutOfRange;

    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        Self::new(raw)
    }
}

impl From<PidGain> for u16 {
    fn from(gain: PidGain) -> Self {
        gain.raw()
    }
}

impl From<PidGain> for u32 {
    fn from(gain: PidGain) -> Self {
        u32::from(gain.raw())
    }
}

/// Raw value that cannot represent a non-negative TMC4671 PID gain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidGainOutOfRange {
    raw: u16,
}

impl PidGainOutOfRange {
    /// Return the rejected raw value.
    pub const fn raw(self) -> u16 {
        self.raw
    }
}

/// Validated P/I pair for the shared TMC4671 PID register layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidGainPair {
    /// Proportional gain stored in bits 31:16.
    pub p: PidGain,
    /// Integral gain stored in bits 15:0.
    pub i: PidGain,
}

impl PidGainPair {
    /// Construct a pair from validated gains.
    pub const fn new(p: PidGain, i: PidGain) -> Self {
        Self { p, i }
    }

    /// Pack the pair into the TMC4671 PID register layout.
    pub const fn register_value(self) -> u32 {
        ((self.p.raw() as u32) << 16) | self.i.raw() as u32
    }

    /// Validate and unpack one raw TMC4671 PID register value.
    ///
    /// # Errors
    ///
    /// Returns the first invalid half, checking P before I.
    pub const fn try_from_register(raw: u32) -> Result<Self, PidGainOutOfRange> {
        let p = match PidGain::new((raw >> 16) as u16) {
            Ok(p) => p,
            Err(error) => return Err(error),
        };
        let i = match PidGain::new(raw as u16) {
            Ok(i) => i,
            Err(error) => return Err(error),
        };
        Ok(Self { p, i })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gain_accepts_exact_domain_boundaries() {
        assert_eq!(PidGain::new(0).unwrap(), PidGain::ZERO);
        assert_eq!(PidGain::new(0x7fff).unwrap(), PidGain::MAX);
        assert_eq!(PidGain::MAX.raw(), 0x7fff);
    }

    #[test]
    fn gain_rejects_negative_s16_encodings() {
        assert_eq!(PidGain::new(0x8000).unwrap_err().raw(), 0x8000);
        assert_eq!(PidGain::new(0xffff).unwrap_err().raw(), 0xffff);
    }

    #[test]
    fn pair_preserves_tmc_half_word_layout() {
        let pair = PidGainPair::new(PidGain::new(0x1234).unwrap(), PidGain::new(0x5678).unwrap());

        assert_eq!(pair.register_value(), 0x1234_5678);
        assert_eq!(PidGainPair::try_from_register(0x1234_5678).unwrap(), pair);
    }

    #[test]
    fn pair_rejects_negative_readback_halves() {
        assert!(PidGainPair::try_from_register(0x8000_0001).is_err());
        assert!(PidGainPair::try_from_register(0x0001_ffff).is_err());
    }
}

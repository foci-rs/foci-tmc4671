//! PID gain domain for TMC4671 control loops.

const fn validate_nonnegative_gain(raw: u16) -> Result<u16, PidGainOutOfRange> {
    if raw <= 0x7fff {
        Ok(raw)
    } else {
        Err(PidGainOutOfRange { raw })
    }
}

/// Common shape of a validated non-negative TMC4671 PID gain register.
pub trait RegisterGain: Copy {
    /// Largest non-negative gain accepted by the TMC4671 for this field.
    const MAX: Self;
    /// Return the validated raw register value.
    fn raw(self) -> u16;
    /// Validate one raw register value.
    ///
    /// # Errors
    ///
    /// Returns [`PidGainOutOfRange`] when the raw value would encode a
    /// negative signed 16-bit gain.
    fn new(raw: u16) -> Result<Self, PidGainOutOfRange>;
}

macro_rules! register_gain {
    ($(#[$meta:meta])* $name:ident, $format:literal, $role:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
        pub struct $name(u16);

        impl $name {
            #[doc = concat!("Disabled ", $role, " gain.")]
            pub const ZERO: Self = Self(0);
            #[doc = concat!("Largest non-negative ", $role, " gain accepted by the TMC4671.")]
            pub const MAX: Self = Self(0x7fff);

            #[doc = concat!("Validate one raw ", $format, " ", $role, " gain.")]
            ///
            /// # Errors
            ///
            /// Returns [`PidGainOutOfRange`] when the raw value would encode a
            /// negative signed 16-bit gain.
            pub const fn new(raw: u16) -> Result<Self, PidGainOutOfRange> {
                match validate_nonnegative_gain(raw) {
                    Ok(raw) => Ok(Self(raw)),
                    Err(error) => Err(error),
                }
            }

            /// Return the validated raw register value.
            pub const fn raw(self) -> u16 {
                self.0
            }
        }

        impl TryFrom<u16> for $name {
            type Error = PidGainOutOfRange;

            fn try_from(raw: u16) -> Result<Self, Self::Error> {
                Self::new(raw)
            }
        }

        impl From<$name> for u16 {
            fn from(gain: $name) -> Self {
                gain.raw()
            }
        }

        impl From<$name> for u32 {
            fn from(gain: $name) -> Self {
                u32::from(gain.raw())
            }
        }

        impl RegisterGain for $name {
            const MAX: Self = Self::MAX;

            fn raw(self) -> u16 {
                self.raw()
            }

            fn new(raw: u16) -> Result<Self, PidGainOutOfRange> {
                Self::new(raw)
            }
        }
    };
}

register_gain!(
    /// Non-negative Q8.8 proportional gain accepted by a TMC4671 PID P field.
    PidPGain,
    "Q8.8",
    "proportional"
);
register_gain!(
    /// Non-negative Q4.12 integral gain accepted by a TMC4671 PID I field.
    PidIGain,
    "Q4.12",
    "integral"
);

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

/// Validated Q8.8 P/Q4.12 I pair for the shared TMC4671 PID register layout.
///
/// Used for registers PID_FLUX_P_FLUX_I (0x54), PID_TORQUE_P_TORQUE_I (0x56),
/// PID_VELOCITY_P_VELOCITY_I (0x58), and PID_POSITION_P_POSITION_I (0x5A),
/// under the fixed `CONFIG_ADVANCED_PI_REPRESENT=0x15` contract.
///
/// The distinct field types prevent swapping or duplicating gain domains:
///
/// ```compile_fail
/// use foci_tmc4671::pid::{PidGainPair, PidPGain};
///
/// let p = PidPGain::new(1).unwrap();
/// let _ = PidGainPair::new(p, p);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct PidGainPair {
    /// Q8.8 proportional gain stored in bits 31:16.
    pub p: PidPGain,
    /// Q4.12 integral gain stored in bits 15:0.
    pub i: PidIGain,
}

impl PidGainPair {
    /// Construct a pair from validated gains.
    pub const fn new(p: PidPGain, i: PidIGain) -> Self {
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
        let p = match PidPGain::new((raw >> 16) as u16) {
            Ok(p) => p,
            Err(error) => return Err(error),
        };
        let i = match PidIGain::new(raw as u16) {
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
    fn typed_gains_accept_exact_domain_boundaries() {
        assert_eq!(PidPGain::new(0).unwrap(), PidPGain::ZERO);
        assert_eq!(PidPGain::new(0x7fff).unwrap(), PidPGain::MAX);
        assert_eq!(PidIGain::new(0).unwrap(), PidIGain::ZERO);
        assert_eq!(PidIGain::new(0x7fff).unwrap(), PidIGain::MAX);
    }

    #[test]
    fn typed_gains_reject_negative_signed_encodings() {
        assert_eq!(PidPGain::new(0x8000).unwrap_err().raw(), 0x8000);
        assert_eq!(PidIGain::new(0xffff).unwrap_err().raw(), 0xffff);
    }

    #[test]
    fn pair_requires_p_then_i_and_preserves_layout() {
        let pair = PidGainPair::new(
            PidPGain::new(0x1234).unwrap(),
            PidIGain::new(0x5678).unwrap(),
        );

        assert_eq!(pair.register_value(), 0x1234_5678);
        assert_eq!(PidGainPair::try_from_register(0x1234_5678).unwrap(), pair);
    }

    #[test]
    fn pair_rejects_negative_readback_halves() {
        assert_eq!(
            PidGainPair::try_from_register(0x8000_0001)
                .unwrap_err()
                .raw(),
            0x8000
        );
        assert_eq!(
            PidGainPair::try_from_register(0x0001_ffff)
                .unwrap_err()
                .raw(),
            0xffff
        );
    }

    #[test]
    fn register_gain_trait_matches_inherent_methods_for_both_gain_types() {
        fn round_trip<G: RegisterGain>(raw: u16) -> u16 {
            G::new(raw).unwrap().raw()
        }
        assert_eq!(round_trip::<PidPGain>(512), 512);
        assert_eq!(round_trip::<PidIGain>(512), 512);
        assert_eq!(PidPGain::MAX.raw(), <PidPGain as RegisterGain>::MAX.raw());
        assert_eq!(PidIGain::MAX.raw(), <PidIGain as RegisterGain>::MAX.raw());
        assert!(<PidPGain as RegisterGain>::new(0x8000).is_err());
    }
}

/// TMC4671 register address.
///
/// Wraps a 7-bit register address. Bit 7 is the read/write flag
/// (set by the SPI codec, not the caller).
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct Register(u8);

impl Register {
    /// Creates a new register address, masking off the read/write flag bit.
    pub const fn new(addr: u8) -> Self {
        Self(addr & 0x7F)
    }

    /// Returns the 7-bit register address.
    pub const fn addr(self) -> u8 {
        self.0
    }
}

/// A sub-register accessed via an address/data register pair.
///
/// Write `sub_addr` to the ADDR register, then read/write the DATA register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
pub struct SubRegister {
    /// The DATA register that holds the sub-register value.
    pub data: Register,
    /// The ADDR register that selects which sub-register to access.
    pub addr: Register,
    /// The sub-register selector value written to `addr`.
    pub sub_addr: u8,
}

impl SubRegister {
    /// Creates a new sub-register descriptor.
    ///
    /// # Parameters
    /// - `data_addr`: Address of the DATA register.
    /// - `addr_addr`: Address of the ADDR (selector) register.
    /// - `sub_addr`: Sub-register selector value.
    pub const fn new(data_addr: u8, addr_addr: u8, sub_addr: u8) -> Self {
        Self {
            data: Register::new(data_addr),
            addr: Register::new(addr_addr),
            sub_addr,
        }
    }
}

// ---- Direct register address constants ----

// Chip identification (sub-register multiplexed via 0x00/0x01)
/// CHIPINFO data register.
pub const CHIPINFO_DATA: Register = Register::new(0x00);
/// CHIPINFO address/selector register.
pub const CHIPINFO_ADDR: Register = Register::new(0x01);

// ADC raw data (sub-register multiplexed via 0x02/0x03)
/// ADC raw data register.
pub const ADC_RAW_DATA: Register = Register::new(0x02);
/// ADC raw address/selector register.
pub const ADC_RAW_ADDR: Register = Register::new(0x03);

// Delta-sigma ADC configuration
/// Delta-sigma ADC motor config B and A.
pub const DSADC_MCFG_B_MCFG_A: Register = Register::new(0x04);
/// Delta-sigma ADC motor clock A.
pub const DSADC_MCLK_A: Register = Register::new(0x05);
/// Delta-sigma ADC motor clock B.
pub const DSADC_MCLK_B: Register = Register::new(0x06);
/// Delta-sigma ADC decimation ratio B and A.
pub const DSADC_MDEC_B_MDEC_A: Register = Register::new(0x07);

// ADC scale and offset
/// ADC I1 scale and offset.
pub const ADC_I1_SCALE_OFFSET: Register = Register::new(0x08);
/// ADC I0 scale and offset.
pub const ADC_I0_SCALE_OFFSET: Register = Register::new(0x09);
/// ADC current channel selection.
pub const ADC_I_SELECT: Register = Register::new(0x0A);
/// ADC I1 and I0 external inputs.
pub const ADC_I1_I0_EXT: Register = Register::new(0x0B);
/// Delta-sigma analog input stage configuration.
pub const DS_ANALOG_INPUT_STAGE_CFG: Register = Register::new(0x0C);

// Analog encoder scale and offset
/// Analog encoder channel 0 scale and offset.
pub const AENC_0_SCALE_OFFSET: Register = Register::new(0x0D);
/// Analog encoder channel 1 scale and offset.
pub const AENC_1_SCALE_OFFSET: Register = Register::new(0x0E);
/// Analog encoder channel 2 scale and offset.
pub const AENC_2_SCALE_OFFSET: Register = Register::new(0x0F);
/// Analog encoder channel selection.
pub const AENC_SELECT: Register = Register::new(0x11);

// ADC monitor
/// ADC IWY and IUX values.
pub const ADC_IWY_IUX: Register = Register::new(0x12);
/// ADC IV value.
pub const ADC_IV: Register = Register::new(0x13);
/// Analog encoder WY and UX values.
pub const AENC_WY_UX: Register = Register::new(0x15);
/// Analog encoder VN value.
pub const AENC_VN: Register = Register::new(0x16);

// PWM configuration
/// PWM output polarities.
pub const PWM_POLARITIES: Register = Register::new(0x17);
/// PWM maximum count value.
pub const PWM_MAXCNT: Register = Register::new(0x18);
/// PWM break-before-make high and low times.
pub const PWM_BBM_H_BBM_L: Register = Register::new(0x19);
/// PWM space-vector chop configuration.
pub const PWM_SV_CHOP: Register = Register::new(0x1A);

// Motor configuration
/// Motor type and number of pole pairs.
pub const MOTOR_TYPE_N_POLE_PAIRS: Register = Register::new(0x1B);
/// External electrical angle PHI_E.
pub const PHI_E_EXT: Register = Register::new(0x1C);

// Open-loop
/// Open-loop mode configuration.
pub const OPENLOOP_MODE: Register = Register::new(0x1F);
/// Open-loop acceleration.
pub const OPENLOOP_ACCELERATION: Register = Register::new(0x20);
/// Open-loop velocity target.
pub const OPENLOOP_VELOCITY_TARGET: Register = Register::new(0x21);
/// Open-loop velocity actual.
pub const OPENLOOP_VELOCITY_ACTUAL: Register = Register::new(0x22);
/// Open-loop electrical angle.
pub const OPENLOOP_PHI: Register = Register::new(0x23);

// External UQ/UD
/// External UQ and UD voltage targets.
pub const UQ_UD_EXT: Register = Register::new(0x24);

// ABN encoder
/// ABN decoder mode configuration.
pub const ABN_DECODER_MODE: Register = Register::new(0x25);
/// ABN decoder pulses per revolution.
pub const ABN_DECODER_PPR: Register = Register::new(0x26);
/// ABN decoder count value.
pub const ABN_DECODER_COUNT: Register = Register::new(0x27);
/// ABN decoder count at N pulse.
pub const ABN_DECODER_COUNT_N: Register = Register::new(0x28);
/// ABN decoder PHI_E and PHI_M offsets.
pub const ABN_DECODER_PHI_E_PHI_M_OFFSET: Register = Register::new(0x29);
/// ABN decoder PHI_E and PHI_M actual values.
pub const ABN_DECODER_PHI_E_PHI_M: Register = Register::new(0x2A);

// ABN 2 encoder
/// ABN2 decoder mode configuration.
pub const ABN_2_DECODER_MODE: Register = Register::new(0x2C);
/// ABN2 decoder pulses per revolution.
pub const ABN_2_DECODER_PPR: Register = Register::new(0x2D);
/// ABN2 decoder count value.
pub const ABN_2_DECODER_COUNT: Register = Register::new(0x2E);
/// ABN2 decoder count at N pulse.
pub const ABN_2_DECODER_COUNT_N: Register = Register::new(0x2F);
/// ABN2 decoder PHI_M offset.
pub const ABN_2_DECODER_PHI_M_OFFSET: Register = Register::new(0x30);
/// ABN2 decoder PHI_M actual value.
pub const ABN_2_DECODER_PHI_M: Register = Register::new(0x31);

// Hall sensor
/// Hall sensor mode configuration.
pub const HALL_MODE: Register = Register::new(0x33);
/// Hall sensor positions for 60 and 0 degree sectors.
pub const HALL_POSITION_060_000: Register = Register::new(0x34);
/// Hall sensor positions for 180 and 120 degree sectors.
pub const HALL_POSITION_180_120: Register = Register::new(0x35);
/// Hall sensor positions for 300 and 240 degree sectors.
pub const HALL_POSITION_300_240: Register = Register::new(0x36);
/// Hall sensor PHI_E and PHI_M offsets.
pub const HALL_PHI_E_PHI_M_OFFSET: Register = Register::new(0x37);
/// Hall sensor maximum delta-phi.
pub const HALL_DPHI_MAX: Register = Register::new(0x38);
/// Hall sensor interpolated and raw PHI_E values.
pub const HALL_PHI_E_INTERPOLATED_PHI_E: Register = Register::new(0x39);
/// Hall sensor PHI_M value.
pub const HALL_PHI_M: Register = Register::new(0x3A);

// Analog encoder decoder
/// Analog encoder decoder mode configuration.
pub const AENC_DECODER_MODE: Register = Register::new(0x3B);
/// Analog encoder decoder N threshold.
pub const AENC_DECODER_N_THRESHOLD: Register = Register::new(0x3C);
/// Analog encoder decoder raw PHI_A value.
pub const AENC_DECODER_PHI_A_RAW: Register = Register::new(0x3D);
/// Analog encoder decoder PHI_A offset.
pub const AENC_DECODER_PHI_A_OFFSET: Register = Register::new(0x3E);
/// Analog encoder decoder PHI_A value.
pub const AENC_DECODER_PHI_A: Register = Register::new(0x3F);
/// Analog encoder decoder pulses per revolution.
pub const AENC_DECODER_PPR: Register = Register::new(0x40);
/// Analog encoder decoder count value.
pub const AENC_DECODER_COUNT: Register = Register::new(0x41);
/// Analog encoder decoder count at N pulse.
pub const AENC_DECODER_COUNT_N: Register = Register::new(0x42);
/// Analog encoder decoder PHI_E and PHI_M offsets.
pub const AENC_DECODER_PHI_E_PHI_M_OFFSET: Register = Register::new(0x45);
/// Analog encoder decoder PHI_E and PHI_M actual values.
pub const AENC_DECODER_PHI_E_PHI_M: Register = Register::new(0x46);

// Config data (sub-register multiplexed via 0x4D/0x4E)
/// Configuration data register.
pub const CONFIG_DATA: Register = Register::new(0x4D);
/// Configuration address/selector register.
pub const CONFIG_ADDR: Register = Register::new(0x4E);

// Selection registers
/// Velocity feedback source selection.
pub const VELOCITY_SELECTION: Register = Register::new(0x50);
/// Position feedback source selection.
pub const POSITION_SELECTION: Register = Register::new(0x51);
/// PHI_E source selection.
pub const PHI_E_SELECTION: Register = Register::new(0x52);
/// Electrical angle PHI_E actual value.
pub const PHI_E: Register = Register::new(0x53);

// PID gains
/// PID flux P and I gains.
pub const PID_FLUX_P_FLUX_I: Register = Register::new(0x54);
/// PID torque P and I gains.
pub const PID_TORQUE_P_TORQUE_I: Register = Register::new(0x56);
/// PID velocity P and I gains.
pub const PID_VELOCITY_P_VELOCITY_I: Register = Register::new(0x58);
/// PID position P and I gains.
pub const PID_POSITION_P_POSITION_I: Register = Register::new(0x5A);

// PID limits
/// PID torque and flux target derivative limits.
pub const PID_TORQUE_FLUX_TARGET_DDT_LIMITS: Register = Register::new(0x5C);
/// PID output UQ and UD limits.
pub const PIDOUT_UQ_UD_LIMITS: Register = Register::new(0x5D);
/// PID torque and flux limits.
pub const PID_TORQUE_FLUX_LIMITS: Register = Register::new(0x5E);
/// PID acceleration limit.
pub const PID_ACCELERATION_LIMIT: Register = Register::new(0x5F);
/// PID velocity limit.
pub const PID_VELOCITY_LIMIT: Register = Register::new(0x60);
/// PID position lower limit.
pub const PID_POSITION_LIMIT_LOW: Register = Register::new(0x61);
/// PID position upper limit.
pub const PID_POSITION_LIMIT_HIGH: Register = Register::new(0x62);

// Mode and motion
/// Ramp mode and motion mode configuration.
pub const MODE_RAMP_MODE_MOTION: Register = Register::new(0x63);

// PID targets and actuals
/// PID torque and flux targets.
pub const PID_TORQUE_FLUX_TARGET: Register = Register::new(0x64);
/// PID torque and flux offsets.
pub const PID_TORQUE_FLUX_OFFSET: Register = Register::new(0x65);
/// PID velocity target.
pub const PID_VELOCITY_TARGET: Register = Register::new(0x66);
/// PID velocity offset.
pub const PID_VELOCITY_OFFSET: Register = Register::new(0x67);
/// PID position target.
pub const PID_POSITION_TARGET: Register = Register::new(0x68);
/// PID torque and flux actual values.
pub const PID_TORQUE_FLUX_ACTUAL: Register = Register::new(0x69);
/// PID velocity actual value.
pub const PID_VELOCITY_ACTUAL: Register = Register::new(0x6A);
/// PID position actual value.
pub const PID_POSITION_ACTUAL: Register = Register::new(0x6B);

// Diagnostic (sub-register multiplexed)
/// PID error data register.
pub const PID_ERROR_DATA: Register = Register::new(0x6C);
/// PID error address/selector register.
pub const PID_ERROR_ADDR: Register = Register::new(0x6D);
/// Interim data register.
pub const INTERIM_DATA: Register = Register::new(0x6E);
/// Interim address/selector register.
pub const INTERIM_ADDR: Register = Register::new(0x6F);

// Voltage monitoring
/// ADC VM (supply voltage) limits.
pub const ADC_VM_LIMITS: Register = Register::new(0x75);

// I/O
/// Raw digital input states.
pub const TMC4671_INPUTS_RAW: Register = Register::new(0x76);
/// Raw digital output states.
pub const TMC4671_OUTPUTS_RAW: Register = Register::new(0x77);

// Step width
/// Step width for step/dir interface.
pub const STEP_WIDTH: Register = Register::new(0x78);

// UART
/// UART baud rate configuration.
pub const UART_BPS: Register = Register::new(0x79);

// GPIO
/// GPIO and delta-sigma ADC input configuration.
pub const GPIO_DSADCI_CONFIG: Register = Register::new(0x7B);

// Status
/// Status flags register.
pub const STATUS_FLAGS: Register = Register::new(0x7C);
/// Status flags mask register.
pub const STATUS_MASK: Register = Register::new(0x7D);

// ---- Sub-register constants ----

// CHIPINFO sub-registers (data at 0x00, selector at 0x01)
/// Chip type identifier.
pub const CHIPINFO_SI_TYPE: SubRegister = SubRegister::new(0x00, 0x01, 0);
/// Chip silicon version.
pub const CHIPINFO_SI_VERSION: SubRegister = SubRegister::new(0x00, 0x01, 1);
/// Chip silicon date.
pub const CHIPINFO_SI_DATE: SubRegister = SubRegister::new(0x00, 0x01, 2);
/// Chip silicon time.
pub const CHIPINFO_SI_TIME: SubRegister = SubRegister::new(0x00, 0x01, 3);
/// Chip silicon variant.
pub const CHIPINFO_SI_VARIANT: SubRegister = SubRegister::new(0x00, 0x01, 4);
/// Chip silicon build number.
pub const CHIPINFO_SI_BUILD: SubRegister = SubRegister::new(0x00, 0x01, 5);

// ADC_RAW sub-registers (data at 0x02, selector at 0x03)
/// Raw ADC I1 and I0 current samples.
pub const ADC_I1_RAW_ADC_I0_RAW: SubRegister = SubRegister::new(0x02, 0x03, 0);
/// Raw ADC general purpose input A and VM voltage.
pub const ADC_AGPI_A_RAW_ADC_VM_RAW: SubRegister = SubRegister::new(0x02, 0x03, 1);
/// Raw ADC analog encoder UX and general purpose input B.
pub const ADC_AENC_UX_RAW_ADC_AGPI_B_RAW: SubRegister = SubRegister::new(0x02, 0x03, 2);
/// Raw ADC analog encoder WY and VN.
pub const ADC_AENC_WY_RAW_ADC_AENC_VN_RAW: SubRegister = SubRegister::new(0x02, 0x03, 3);

// CONFIG sub-registers (data at 0x4D, selector at 0x4E)
// Biquad filter coefficients — position loop (X)
/// Biquad position filter A1 coefficient.
pub const CONFIG_BIQUAD_X_A_1: SubRegister = SubRegister::new(0x4D, 0x4E, 1);
/// Biquad position filter A2 coefficient.
pub const CONFIG_BIQUAD_X_A_2: SubRegister = SubRegister::new(0x4D, 0x4E, 2);
/// Biquad position filter B0 coefficient.
pub const CONFIG_BIQUAD_X_B_0: SubRegister = SubRegister::new(0x4D, 0x4E, 4);
/// Biquad position filter B1 coefficient.
pub const CONFIG_BIQUAD_X_B_1: SubRegister = SubRegister::new(0x4D, 0x4E, 5);
/// Biquad position filter B2 coefficient.
pub const CONFIG_BIQUAD_X_B_2: SubRegister = SubRegister::new(0x4D, 0x4E, 6);
/// Biquad position filter enable flag.
pub const CONFIG_BIQUAD_X_ENABLE: SubRegister = SubRegister::new(0x4D, 0x4E, 7);
// Biquad filter coefficients — velocity loop (V)
/// Biquad velocity filter A1 coefficient.
pub const CONFIG_BIQUAD_V_A_1: SubRegister = SubRegister::new(0x4D, 0x4E, 9);
/// Biquad velocity filter A2 coefficient.
pub const CONFIG_BIQUAD_V_A_2: SubRegister = SubRegister::new(0x4D, 0x4E, 10);
/// Biquad velocity filter B0 coefficient.
pub const CONFIG_BIQUAD_V_B_0: SubRegister = SubRegister::new(0x4D, 0x4E, 12);
/// Biquad velocity filter B1 coefficient.
pub const CONFIG_BIQUAD_V_B_1: SubRegister = SubRegister::new(0x4D, 0x4E, 13);
/// Biquad velocity filter B2 coefficient.
pub const CONFIG_BIQUAD_V_B_2: SubRegister = SubRegister::new(0x4D, 0x4E, 14);
/// Biquad velocity filter enable flag.
pub const CONFIG_BIQUAD_V_ENABLE: SubRegister = SubRegister::new(0x4D, 0x4E, 15);
// Biquad filter coefficients — torque loop (T)
/// Biquad torque filter A1 coefficient.
pub const CONFIG_BIQUAD_T_A_1: SubRegister = SubRegister::new(0x4D, 0x4E, 17);
/// Biquad torque filter A2 coefficient.
pub const CONFIG_BIQUAD_T_A_2: SubRegister = SubRegister::new(0x4D, 0x4E, 18);
/// Biquad torque filter B0 coefficient.
pub const CONFIG_BIQUAD_T_B_0: SubRegister = SubRegister::new(0x4D, 0x4E, 20);
/// Biquad torque filter B1 coefficient.
pub const CONFIG_BIQUAD_T_B_1: SubRegister = SubRegister::new(0x4D, 0x4E, 21);
/// Biquad torque filter B2 coefficient.
pub const CONFIG_BIQUAD_T_B_2: SubRegister = SubRegister::new(0x4D, 0x4E, 22);
/// Biquad torque filter enable flag.
pub const CONFIG_BIQUAD_T_ENABLE: SubRegister = SubRegister::new(0x4D, 0x4E, 23);
// Biquad filter coefficients — flux loop (F)
/// Biquad flux filter A1 coefficient.
pub const CONFIG_BIQUAD_F_A_1: SubRegister = SubRegister::new(0x4D, 0x4E, 25);
/// Biquad flux filter A2 coefficient.
pub const CONFIG_BIQUAD_F_A_2: SubRegister = SubRegister::new(0x4D, 0x4E, 26);
/// Biquad flux filter B0 coefficient.
pub const CONFIG_BIQUAD_F_B_0: SubRegister = SubRegister::new(0x4D, 0x4E, 28);
/// Biquad flux filter B1 coefficient.
pub const CONFIG_BIQUAD_F_B_1: SubRegister = SubRegister::new(0x4D, 0x4E, 29);
/// Biquad flux filter B2 coefficient.
pub const CONFIG_BIQUAD_F_B_2: SubRegister = SubRegister::new(0x4D, 0x4E, 30);
/// Biquad flux filter enable flag.
pub const CONFIG_BIQUAD_F_ENABLE: SubRegister = SubRegister::new(0x4D, 0x4E, 31);
// Feedforward
/// Feedforward velocity gain.
pub const CONFIG_FEED_FORWARD_VELOCITY_GAIN: SubRegister = SubRegister::new(0x4D, 0x4E, 40);
/// Feedforward velocity filter time constant.
pub const CONFIG_FEED_FORWARD_VELOCITY_FILTER_CONSTANT: SubRegister =
    SubRegister::new(0x4D, 0x4E, 41);
/// Feedforward torque gain.
pub const CONFIG_FEED_FORWARD_TORQUE_GAIN: SubRegister = SubRegister::new(0x4D, 0x4E, 42);
/// Feedforward torque filter time constant.
pub const CONFIG_FEED_FORWARD_TORQUE_FILTER_CONSTANT: SubRegister =
    SubRegister::new(0x4D, 0x4E, 43);
// Reference switch
/// Reference switch configuration.
pub const CONFIG_REF_SWITCH_CONFIG: SubRegister = SubRegister::new(0x4D, 0x4E, 51);
// Single pin interface
/// Single-pin interface status configuration.
pub const CONFIG_SINGLE_PIN_IF_STATUS_CFG: SubRegister = SubRegister::new(0x4D, 0x4E, 60);
/// Single-pin interface scale and offset.
pub const CONFIG_SINGLE_PIN_IF_SCALE_OFFSET: SubRegister = SubRegister::new(0x4D, 0x4E, 61);
// Advanced PI representation
/// Advanced PI controller representation mode.
pub const CONFIG_ADVANCED_PI_REPRESENT: SubRegister = SubRegister::new(0x4D, 0x4E, 62);

// PID_ERROR sub-registers (data at 0x6C, selector at 0x6D)
/// PID torque error value.
pub const PID_ERROR_PID_TORQUE_ERROR: SubRegister = SubRegister::new(0x6C, 0x6D, 0);
/// PID flux error value.
pub const PID_ERROR_PID_FLUX_ERROR: SubRegister = SubRegister::new(0x6C, 0x6D, 1);
/// PID velocity error value.
pub const PID_ERROR_PID_VELOCITY_ERROR: SubRegister = SubRegister::new(0x6C, 0x6D, 2);
/// PID position error value.
pub const PID_ERROR_PID_POSITION_ERROR: SubRegister = SubRegister::new(0x6C, 0x6D, 3);
/// PID torque integrator error sum.
pub const PID_ERROR_PID_TORQUE_ERROR_SUM: SubRegister = SubRegister::new(0x6C, 0x6D, 4);
/// PID flux integrator error sum.
pub const PID_ERROR_PID_FLUX_ERROR_SUM: SubRegister = SubRegister::new(0x6C, 0x6D, 5);
/// PID velocity integrator error sum.
pub const PID_ERROR_PID_VELOCITY_ERROR_SUM: SubRegister = SubRegister::new(0x6C, 0x6D, 6);
/// PID position integrator error sum.
pub const PID_ERROR_PID_POSITION_ERROR_SUM: SubRegister = SubRegister::new(0x6C, 0x6D, 7);

// INTERIM sub-registers (data at 0x6E, selector at 0x6F)
/// PID input target torque.
pub const INTERIM_PIDIN_TARGET_TORQUE: SubRegister = SubRegister::new(0x6E, 0x6F, 0);
/// PID input target flux.
pub const INTERIM_PIDIN_TARGET_FLUX: SubRegister = SubRegister::new(0x6E, 0x6F, 1);
/// PID input target velocity.
pub const INTERIM_PIDIN_TARGET_VELOCITY: SubRegister = SubRegister::new(0x6E, 0x6F, 2);
/// PID input target position.
pub const INTERIM_PIDIN_TARGET_POSITION: SubRegister = SubRegister::new(0x6E, 0x6F, 3);
/// PID output target torque.
pub const INTERIM_PIDOUT_TARGET_TORQUE: SubRegister = SubRegister::new(0x6E, 0x6F, 4);
/// PID output target flux.
pub const INTERIM_PIDOUT_TARGET_FLUX: SubRegister = SubRegister::new(0x6E, 0x6F, 5);
/// PID output target velocity.
pub const INTERIM_PIDOUT_TARGET_VELOCITY: SubRegister = SubRegister::new(0x6E, 0x6F, 6);
/// PID output target position.
pub const INTERIM_PIDOUT_TARGET_POSITION: SubRegister = SubRegister::new(0x6E, 0x6F, 7);
/// FOC phase currents IWY and IUX.
pub const INTERIM_FOC_IWY_IUX: SubRegister = SubRegister::new(0x6E, 0x6F, 8);
/// FOC phase current IV.
pub const INTERIM_FOC_IV: SubRegister = SubRegister::new(0x6E, 0x6F, 9);
/// FOC Clarke transform outputs IB and IA.
pub const INTERIM_FOC_IB_IA: SubRegister = SubRegister::new(0x6E, 0x6F, 10);
/// FOC Park transform outputs IQ and ID.
pub const INTERIM_FOC_IQ_ID: SubRegister = SubRegister::new(0x6E, 0x6F, 11);
/// FOC voltage outputs UQ and UD.
pub const INTERIM_FOC_UQ_UD: SubRegister = SubRegister::new(0x6E, 0x6F, 12);
/// FOC voltage outputs UQ and UD after limiting.
pub const INTERIM_FOC_UQ_UD_LIMITED: SubRegister = SubRegister::new(0x6E, 0x6F, 13);
/// FOC inverse Park outputs UB and UA.
pub const INTERIM_FOC_UB_UA: SubRegister = SubRegister::new(0x6E, 0x6F, 14);
/// FOC phase voltages UWY and UUX.
pub const INTERIM_FOC_UWY_UUX: SubRegister = SubRegister::new(0x6E, 0x6F, 15);
/// FOC phase voltage UV.
pub const INTERIM_FOC_UV: SubRegister = SubRegister::new(0x6E, 0x6F, 16);
/// PWM duty cycles for WY and UX phases.
pub const INTERIM_PWM_WY_UX: SubRegister = SubRegister::new(0x6E, 0x6F, 17);
/// PWM duty cycle for UV phase.
pub const INTERIM_PWM_UV: SubRegister = SubRegister::new(0x6E, 0x6F, 18);
/// ADC current samples I1 and I0.
pub const INTERIM_ADC_I1_I0: SubRegister = SubRegister::new(0x6E, 0x6F, 19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_addr_masking() {
        let r = Register::new(0xFF);
        assert_eq!(r.addr(), 0x7F);
    }

    #[test]
    fn register_preserves_valid_addr() {
        let r = Register::new(0x68);
        assert_eq!(r.addr(), 0x68);
    }

    #[test]
    fn known_register_addresses() {
        assert_eq!(CHIPINFO_DATA.addr(), 0x00);
        assert_eq!(STATUS_FLAGS.addr(), 0x7C);
        assert_eq!(STATUS_MASK.addr(), 0x7D);
        assert_eq!(PID_POSITION_TARGET.addr(), 0x68);
        assert_eq!(MODE_RAMP_MODE_MOTION.addr(), 0x63);
        assert_eq!(MOTOR_TYPE_N_POLE_PAIRS.addr(), 0x1B);
        assert_eq!(PWM_MAXCNT.addr(), 0x18);
        assert_eq!(ABN_DECODER_PPR.addr(), 0x26);
    }

    #[test]
    fn sub_register_construction() {
        assert_eq!(CHIPINFO_SI_TYPE.data.addr(), 0x00);
        assert_eq!(CHIPINFO_SI_TYPE.addr.addr(), 0x01);
        assert_eq!(CHIPINFO_SI_TYPE.sub_addr, 0);
    }

    #[test]
    fn config_sub_registers_share_data_addr() {
        assert_eq!(CONFIG_BIQUAD_X_A_1.data.addr(), 0x4D);
        assert_eq!(CONFIG_BIQUAD_V_A_1.data.addr(), 0x4D);
        assert_eq!(CONFIG_ADVANCED_PI_REPRESENT.data.addr(), 0x4D);
        // All use the same selector register
        assert_eq!(CONFIG_BIQUAD_X_A_1.addr.addr(), 0x4E);
        assert_eq!(CONFIG_ADVANCED_PI_REPRESENT.addr.addr(), 0x4E);
    }

    #[test]
    fn pid_error_sub_registers() {
        assert_eq!(PID_ERROR_PID_TORQUE_ERROR.data.addr(), 0x6C);
        assert_eq!(PID_ERROR_PID_TORQUE_ERROR.addr.addr(), 0x6D);
        assert_eq!(PID_ERROR_PID_TORQUE_ERROR.sub_addr, 0);
        assert_eq!(PID_ERROR_PID_POSITION_ERROR_SUM.sub_addr, 7);
    }

    #[test]
    fn interim_sub_registers() {
        assert_eq!(INTERIM_PIDIN_TARGET_TORQUE.data.addr(), 0x6E);
        assert_eq!(INTERIM_PIDIN_TARGET_TORQUE.addr.addr(), 0x6F);
        assert_eq!(INTERIM_PIDIN_TARGET_TORQUE.sub_addr, 0);
        assert_eq!(INTERIM_ADC_I1_I0.sub_addr, 19);
    }
}

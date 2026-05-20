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

// ---- Motion mode and motor type enums ----

/// TMC4671 motion modes (MODE_RAMP_MODE_MOTION register, bits `[7:0]`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum MotionMode {
    /// Motor stopped; no output drive.
    Stopped = 0,
    /// Torque control mode.
    Torque = 1,
    /// Velocity control mode.
    Velocity = 2,
    /// Position control mode.
    Position = 3,
    /// PRBS flux excitation (identification).
    PrbsFlux = 4,
    /// PRBS torque excitation (identification).
    PrbsTorque = 5,
    /// PRBS velocity excitation (identification).
    PrbsVelocity = 6,
    /// PRBS position excitation (identification).
    PrbsPosition = 7,
    /// External UQ/UD voltage drive mode.
    UqUdExt = 8,
}

/// TMC4671 motor types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, defmt::Format)]
#[repr(u8)]
pub enum MotorType {
    /// No motor configured.
    None = 0,
    /// DC motor.
    Dc = 1,
    /// Two-phase stepper motor.
    TwoPhase = 2,
    /// Three-phase BLDC/PMSM motor.
    ThreePhase = 3,
}

// ---- Bitfield struct definitions ----

use bitfield_struct::bitfield;

/// MOTOR_TYPE_N_POLE_PAIRS register (0x1B).
///
/// Configures the motor type and number of electrical pole pairs.
#[bitfield(u32)]
pub struct MotorTypeNPolePairs {
    /// Number of electrical pole pairs.
    #[bits(16)]
    pub n_pole_pairs: u16,
    /// Motor type selection. Cast to/from [`MotorType`].
    #[bits(8)]
    pub motor_type: u8,
    #[bits(8)]
    _reserved: u8,
}

/// PWM_BBM_H_BBM_L register (0x19).
///
/// Break-before-make dead-time for high-side and low-side gate drivers.
#[bitfield(u32)]
pub struct PwmBbmHBbmL {
    /// Low-side BBM dead-time in PWM clock cycles.
    #[bits(8)]
    pub pwm_bbm_l: u8,
    /// High-side BBM dead-time in PWM clock cycles.
    #[bits(8)]
    pub pwm_bbm_h: u8,
    #[bits(16)]
    _reserved: u16,
}

/// PWM_SV_CHOP register (0x1A).
///
/// Space-vector and chopper PWM configuration.
#[bitfield(u32)]
pub struct PwmSvChop {
    /// Chopper mode selection.
    #[bits(8)]
    pub pwm_chop: u8,
    /// Space-vector PWM enable.
    #[bits(1)]
    pub pwm_sv: bool,
    #[bits(23)]
    _reserved: u32,
}

/// ADC_I0_SCALE_OFFSET register (0x09).
///
/// Current sense amplifier I0 calibration: offset and gain scale.
#[bitfield(u32)]
pub struct AdcI0ScaleOffset {
    /// ADC I0 offset correction (unsigned, subtracted before scaling).
    #[bits(16)]
    pub offset: u16,
    /// ADC I0 gain scale factor (signed Q1.15 format).
    #[bits(16)]
    pub scale: i16,
}

/// ADC_I1_SCALE_OFFSET register (0x08).
///
/// Current sense amplifier I1 calibration: offset and gain scale.
#[bitfield(u32)]
pub struct AdcI1ScaleOffset {
    /// ADC I1 offset correction (unsigned, subtracted before scaling).
    #[bits(16)]
    pub offset: u16,
    /// ADC I1 gain scale factor (signed Q1.15 format).
    #[bits(16)]
    pub scale: i16,
}

/// MODE_RAMP_MODE_MOTION register (0x63).
///
/// Selects the active motion mode and PID sampling configuration.
#[bitfield(u32)]
pub struct ModeRampModeMotion {
    /// Active motion mode. Cast to/from [`MotionMode`].
    #[bits(8)]
    pub mode_motion: u8,
    #[bits(8)]
    _reserved1: u8,
    /// Feed-forward mode selection.
    #[bits(8)]
    pub mode_ff: u8,
    /// PID sample rate divider (0 = every cycle).
    #[bits(7)]
    pub mode_pid_smpl: u8,
    /// PID type: false = standard, true = advanced.
    #[bits(1)]
    pub mode_pid_type: bool,
}

/// ABN_DECODER_MODE register (0x25).
///
/// ABN incremental encoder interface configuration.
#[bitfield(u32)]
pub struct AbnDecoderMode {
    /// Invert A channel polarity.
    #[bits(1)]
    pub abn_apol: bool,
    /// Invert B channel polarity.
    #[bits(1)]
    pub abn_bpol: bool,
    /// Invert N (index) channel polarity.
    #[bits(1)]
    pub abn_npol: bool,
    /// Use ABN count as N signal.
    #[bits(1)]
    pub abn_use_abn_as_n: bool,
    #[bits(4)]
    _reserved1: u8,
    /// Clear encoder count on N pulse.
    #[bits(1)]
    pub abn_cln: bool,
    #[bits(3)]
    _reserved2: u8,
    /// Reverse counting direction.
    #[bits(1)]
    pub abn_direction: bool,
    #[bits(19)]
    _reserved3: u32,
}

/// PID gain register layout — shared by flux, torque, velocity, and position loops.
///
/// Used for registers PID_FLUX_P_FLUX_I (0x54), PID_TORQUE_P_TORQUE_I (0x56),
/// PID_VELOCITY_P_VELOCITY_I (0x58), and PID_POSITION_P_POSITION_I (0x5A).
#[bitfield(u32)]
pub struct PidGains {
    /// Integral gain.
    #[bits(16)]
    pub i_gain: u16,
    /// Proportional gain.
    #[bits(16)]
    pub p_gain: u16,
}

/// STATUS_FLAGS (0x7C) and STATUS_MASK (0x7D) register layout.
///
/// Both registers share this bit layout. Read STATUS_FLAGS for current fault state;
/// write STATUS_MASK to enable interrupt generation per flag.
#[bitfield(u32)]
pub struct StatusFlags {
    /// Position PID target limit reached.
    #[bits(1)]
    pub pid_x_target_limit: bool,
    #[bits(1)]
    _reserved0: bool,
    /// Position PID error sum limit reached.
    #[bits(1)]
    pub pid_x_errsum_limit: bool,
    /// Position PID output limit reached.
    #[bits(1)]
    pub pid_x_output_limit: bool,
    /// Velocity PID target limit reached.
    #[bits(1)]
    pub pid_v_target_limit: bool,
    #[bits(1)]
    _reserved1: bool,
    /// Velocity PID error sum limit reached.
    #[bits(1)]
    pub pid_v_errsum_limit: bool,
    /// Velocity PID output limit reached.
    #[bits(1)]
    pub pid_v_output_limit: bool,
    /// Flux PID target limit reached.
    #[bits(1)]
    pub pid_id_target_limit: bool,
    #[bits(1)]
    _reserved2: bool,
    /// Flux PID error sum limit reached.
    #[bits(1)]
    pub pid_id_errsum_limit: bool,
    /// Flux PID output limit reached.
    #[bits(1)]
    pub pid_id_output_limit: bool,
    /// Torque PID target limit reached.
    #[bits(1)]
    pub pid_iq_target_limit: bool,
    #[bits(1)]
    _reserved3: bool,
    /// Torque PID error sum limit reached.
    #[bits(1)]
    pub pid_iq_errsum_limit: bool,
    /// Torque PID output limit reached.
    #[bits(1)]
    pub pid_iq_output_limit: bool,
    /// Inverse Park UD circular limit active.
    #[bits(1)]
    pub ipark_cirlim_limit_u_d: bool,
    /// Inverse Park UQ circular limit active.
    #[bits(1)]
    pub ipark_cirlim_limit_u_q: bool,
    /// Inverse Park UR circular limit active.
    #[bits(1)]
    pub ipark_cirlim_limit_u_r: bool,
    /// Reserved by TMC4671-LA Rev 1, 02/2024 Table 30.
    #[bits(1)]
    _reserved4: bool,
    /// Reference switch R triggered.
    #[bits(1)]
    pub ref_sw_r: bool,
    /// Reference switch H triggered.
    #[bits(1)]
    pub ref_sw_h: bool,
    /// Reference switch L triggered.
    #[bits(1)]
    pub ref_sw_l: bool,
    #[bits(1)]
    _reserved5: bool,
    /// PWM duty cycle at minimum.
    #[bits(1)]
    pub pwm_min: bool,
    /// PWM duty cycle at maximum.
    #[bits(1)]
    pub pwm_max: bool,
    /// ADC current measurement clipped.
    #[bits(1)]
    pub adc_i_clipped: bool,
    /// Analog encoder input clipped.
    #[bits(1)]
    pub aenc_clipped: bool,
    /// Encoder N pulse detected.
    #[bits(1)]
    pub enc_n: bool,
    /// Encoder 2 N pulse detected.
    #[bits(1)]
    pub enc_2_n: bool,
    /// Analog encoder N signal detected.
    #[bits(1)]
    pub aenc_n: bool,
    #[bits(1)]
    _reserved6: bool,
}

/// VELOCITY_SELECTION register (0x50).
///
/// Selects which feedback source drives velocity control and the velocity meter.
#[bitfield(u32)]
pub struct VelocitySelection {
    /// Velocity feedback source for PID control.
    #[bits(8)]
    pub velocity_selection: u8,
    /// Velocity meter source for monitoring.
    #[bits(8)]
    pub velocity_meter_selection: u8,
    #[bits(16)]
    _reserved: u16,
}

/// UQ_UD_EXT register (0x24).
///
/// External voltage vector applied in UQ_UD_EXT motion mode.
#[bitfield(u32)]
pub struct UqUdExt {
    /// D-axis voltage component (signed).
    #[bits(16)]
    pub ud_ext: i16,
    /// Q-axis voltage component (signed).
    #[bits(16)]
    pub uq_ext: i16,
}

/// PID_TORQUE_FLUX_TARGET register (0x64).
///
/// Target setpoints for torque and flux current controllers.
#[bitfield(u32)]
pub struct PidTorqueFluxTarget {
    /// Flux (D-axis) current target (signed).
    #[bits(16)]
    pub pid_flux_target: i16,
    /// Torque (Q-axis) current target (signed).
    #[bits(16)]
    pub pid_torque_target: i16,
}

/// PID_TORQUE_FLUX_ACTUAL register (0x69).
///
/// Measured torque and flux current feedback values.
#[bitfield(u32)]
pub struct PidTorqueFluxActual {
    /// Flux (D-axis) current actual value (signed).
    #[bits(16)]
    pub pid_flux_actual: i16,
    /// Torque (Q-axis) current actual value (signed).
    #[bits(16)]
    pub pid_torque_actual: i16,
}

/// ABN_DECODER_PHI_E_PHI_M_OFFSET register (0x29).
///
/// Angular offset corrections applied to ABN encoder mechanical and electrical angles.
#[bitfield(u32)]
pub struct AbnDecoderPhiOffset {
    /// Mechanical angle offset (signed, applied to PHI_M).
    #[bits(16)]
    pub phi_m_offset: i16,
    /// Electrical angle offset (signed, applied to PHI_E).
    #[bits(16)]
    pub phi_e_offset: i16,
}

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

    #[test]
    fn motor_type_pole_pairs_round_trip() {
        let reg = MotorTypeNPolePairs::new()
            .with_motor_type(MotorType::TwoPhase as u8)
            .with_n_pole_pairs(50);
        let raw: u32 = reg.into();
        let decoded = MotorTypeNPolePairs::from(raw);
        assert_eq!(decoded.motor_type(), MotorType::TwoPhase as u8);
        assert_eq!(decoded.n_pole_pairs(), 50);
    }

    #[test]
    fn status_flags_adc_clipped() {
        let flags = StatusFlags::from(1 << 26);
        assert!(flags.adc_i_clipped());
        assert!(!flags.pwm_max());
        assert!(!flags.pid_x_target_limit());
    }

    #[test]
    fn status_flags_bit19_is_reserved() {
        let flags = StatusFlags::from(1 << 19);
        assert!(!flags.adc_i_clipped());
        assert!(!flags.aenc_clipped());
    }

    #[test]
    fn pid_gains_round_trip() {
        let gains = PidGains::new().with_p_gain(1234).with_i_gain(5678);
        let raw: u32 = gains.into();
        let decoded = PidGains::from(raw);
        assert_eq!(decoded.p_gain(), 1234);
        assert_eq!(decoded.i_gain(), 5678);
    }

    #[test]
    fn mode_motion_fields() {
        let mode = ModeRampModeMotion::new()
            .with_mode_motion(MotionMode::Position as u8)
            .with_mode_pid_type(true);
        let raw: u32 = mode.into();
        let decoded = ModeRampModeMotion::from(raw);
        assert_eq!(decoded.mode_motion(), MotionMode::Position as u8);
        assert!(decoded.mode_pid_type());
        assert_eq!(decoded.mode_ff(), 0);
    }

    #[test]
    fn abn_decoder_mode_direction() {
        let mode = AbnDecoderMode::new()
            .with_abn_direction(true)
            .with_abn_apol(true);
        let raw: u32 = mode.into();
        let decoded = AbnDecoderMode::from(raw);
        assert!(decoded.abn_direction());
        assert!(decoded.abn_apol());
        assert!(!decoded.abn_bpol());
    }

    #[test]
    fn uq_ud_ext_signed() {
        let ext = UqUdExt::new().with_uq_ext(-1000).with_ud_ext(500);
        let raw: u32 = ext.into();
        let decoded = UqUdExt::from(raw);
        assert_eq!(decoded.uq_ext(), -1000);
        assert_eq!(decoded.ud_ext(), 500);
    }

    #[test]
    fn pid_torque_flux_target_signed() {
        let target = PidTorqueFluxTarget::new()
            .with_pid_torque_target(-2000)
            .with_pid_flux_target(100);
        let raw: u32 = target.into();
        let decoded = PidTorqueFluxTarget::from(raw);
        assert_eq!(decoded.pid_torque_target(), -2000);
        assert_eq!(decoded.pid_flux_target(), 100);
    }

    #[test]
    fn pid_torque_flux_target_flux_is_low_half() {
        // Datasheet 0x64: PID_FLUX_TARGET = s16(15:0), PID_TORQUE_TARGET = s16(31:16)
        // For D-axis alignment, flux target must be in bits[15:0].
        let target = PidTorqueFluxTarget::new()
            .with_pid_flux_target(1000)
            .with_pid_torque_target(0);
        let raw: u32 = target.into();
        // Flux in low 16 bits, torque (0) in high 16 bits
        assert_eq!(raw & 0xFFFF, 1000u32, "flux target must be in bits[15:0]");
        assert_eq!(raw >> 16, 0, "torque target must be in bits[31:16]");

        // Compare with UQ_UD_EXT layout — UD is also in bits[15:0]
        let uq_ud = UqUdExt::new().with_ud_ext(1000).with_uq_ext(0);
        let uq_ud_raw: u32 = uq_ud.into();
        assert_eq!(
            raw & 0xFFFF,
            uq_ud_raw & 0xFFFF,
            "flux target and UD_EXT must occupy the same bit position (D-axis)"
        );
    }
}

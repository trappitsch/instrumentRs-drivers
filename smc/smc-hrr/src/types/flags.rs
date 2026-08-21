//! New types with status and alarm flags.

use std::fmt;

use instrumentrs::InstrumentError;

use crate::InstrumentParameter;

/// Status flags.
#[derive(Debug)]
pub struct StatusFlag {
    /// Stop: false, Run: true;
    run_flag: bool,
    /// Did an operation continued alarm given off status occur.
    op_stop_alarm_flag: bool,
    /// Was an operation continued alarm given off?
    op_cont_alarm_flag: bool,
    /// Other than serial: false, Serial mode: true
    remote_status_flag: bool,
    /// Completion of preparation (TEMP READY) status is formed?
    temp_ready_flag: bool,
    /// Is temperature range monitoring condition (TEMP OUT) established?
    temp_out_flag: bool,
    /// Does the chiller need maintenance?
    maintenance_flag: bool,
}

impl StatusFlag {
    /// Is the remote status set to serial?
    pub fn is_remote_mode_serial(&self) -> bool {
        self.remote_status_flag
    }

    /// Is the chiller running?
    pub fn is_running(&self) -> bool {
        self.run_flag
    }

    /// Is temperature range monitoring (TEMP OUT) status condition formed?
    pub fn is_temp_out(&self) -> bool {
        self.temp_out_flag
    }

    /// Is completion of preparation (TEMP READY) status condition formed?
    pub fn is_temp_ready(&self) -> bool {
        self.temp_ready_flag
    }

    /// Was an operation continued alarm been given off?
    pub fn has_op_cont_alarm(&self) -> bool {
        self.op_cont_alarm_flag
    }

    /// Was an operation stop alarm been given off?
    pub fn has_op_stop_alarm(&self) -> bool {
        self.op_stop_alarm_flag
    }

    /// Does the chiller need maintenance?
    pub fn needs_maintenance(&self) -> bool {
        self.maintenance_flag
    }
}

impl InstrumentParameter<Vec<u8>> for StatusFlag {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, InstrumentError> {
        if val.len() != 2 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }

        let mask = u16::from_be_bytes([val[0], val[1]]);

        let run_flag = bit_u16_is_set(mask, 0);
        let op_stop_alarm_flag = bit_u16_is_set(mask, 1);
        let op_cont_alarm_flag = bit_u16_is_set(mask, 2);
        let remote_status_flag = bit_u16_is_set(mask, 5);
        let temp_ready_flag = bit_u16_is_set(mask, 9);
        let temp_out_flag = bit_u16_is_set(mask, 10);
        let maintenance_flag = bit_u16_is_set(mask, 11);

        Ok(Self {
            run_flag,
            op_stop_alarm_flag,
            op_cont_alarm_flag,
            remote_status_flag,
            temp_ready_flag,
            temp_out_flag,
            maintenance_flag,
        })
    }
}

/// Alarm flags.
///
/// Holds the status of all the alarm flags. If the alarm flags are triggered, we store `true` in
/// the flag, otherwise `false`. For details on the meaning of these flags, see the communication
/// functions operational manual, section 3.16.6. Not all chillers will have all flags available, so
/// make sure to double check with flags are meaningful for your exact model.
///
/// All alarm flag names have first the official flag name followed by the description.
#[derive(Debug)]
pub struct AlarmFlags {
    pub al01_low_level_in_tank_abnormal: bool,
    pub al02_low_level_in_tag: bool,
    pub al03_phase_loss_phase_reverse_error: bool,
    pub al04_water_leakage: bool,
    pub al05_pump_inverter_error: bool,
    pub al06_internal_fan_stop: bool,
    pub al07_fan_inverter_error: bool,
    pub al09_circulating_fluid_discharge_pressure_rise: bool,
    pub al10_flow_rate_reduction: bool,
    pub al11_outside_ambient_temperature_range: bool,
    pub al12_electric_conductivity_rise: bool,
    pub al13_not_temp_ready: bool,
    pub al14_circulating_fluid_temperature_range_rise: bool,
    pub al15_circulating_fluid_temperature_range_drop: bool,
    pub al17_flow_rate_failure: bool,
    pub al18_high_circulating_fluid_discharge_temperature: bool,
    pub al19_high_circulating_fluid_return_temp: bool,
    pub al21_high_circulating_fluid_discharge_pressure: bool,
    pub al22_low_circulating_fluid_discharge_pressure: bool,
    pub al24_memory_abnormal: bool,
    pub al25_contact_input_1_signal_detection: bool,
    pub al26_contact_input_2_signal_detection: bool,
    pub al27_forced_stop: bool,
    pub al28_maintenance_notice: bool,
    pub al29_communication_error: bool,
    pub al30_refrigerant_circuit_abnormal: bool,
    pub al31_sensor_abnormal: bool,
    pub al32_controller_abnormal: bool,
}

impl fmt::Display for AlarmFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.has_alarms() {
            return write!(f, "No alarms");
        }

        // List all the alarms in a nice list if we have any
        writeln!(f, "The following alarm(s) occurred:")?;

        if self.al01_low_level_in_tank_abnormal {
            writeln!(f, "- Low level in tank abnormal.")?;
        }
        if self.al02_low_level_in_tag {
            writeln!(f, "- Low level in tag.")?;
        }
        if self.al03_phase_loss_phase_reverse_error {
            writeln!(f, "- Phase loss / phase reverse error.")?;
        }
        if self.al04_water_leakage {
            writeln!(f, "- Water leakage.")?;
        }
        if self.al05_pump_inverter_error {
            writeln!(f, "- Pump inverter error.")?;
        }
        if self.al06_internal_fan_stop {
            writeln!(f, "- Internal fan stop.")?;
        }
        if self.al07_fan_inverter_error {
            writeln!(f, "- Fan inverter error.")?;
        }
        if self.al09_circulating_fluid_discharge_pressure_rise {
            writeln!(f, "- Circulating fluid discharge pressure rise.")?;
        }
        if self.al10_flow_rate_reduction {
            writeln!(f, "- Flow rate reduction.")?;
        }
        if self.al11_outside_ambient_temperature_range {
            writeln!(f, "- Outside ambient temperature range.")?;
        }
        if self.al12_electric_conductivity_rise {
            writeln!(f, "- Electric conductivity rise.")?;
        }
        if self.al13_not_temp_ready {
            writeln!(f, "- NOT TEMP READY.")?;
        }
        if self.al14_circulating_fluid_temperature_range_rise {
            writeln!(f, "- Circulating fluid temperature rise.")?;
        }
        if self.al15_circulating_fluid_temperature_range_drop {
            writeln!(f, "- Circulating fluid temperature drop.")?;
        }
        if self.al17_flow_rate_failure {
            writeln!(f, "- Flow rate failure.")?;
        }
        if self.al18_high_circulating_fluid_discharge_temperature {
            writeln!(f, "- High circulating fluid discharge temperature.")?;
        }
        if self.al19_high_circulating_fluid_return_temp {
            writeln!(f, "- High circulating fluid return temperature.")?;
        }
        if self.al21_high_circulating_fluid_discharge_pressure {
            writeln!(f, "- High circulating fluid discharge pressure.")?;
        }
        if self.al22_low_circulating_fluid_discharge_pressure {
            writeln!(f, "- Low circulating fluid discharge pressure.")?;
        }
        if self.al24_memory_abnormal {
            writeln!(f, "- Memory abnormal.")?;
        }
        if self.al25_contact_input_1_signal_detection {
            writeln!(f, "- Contact input 1 signal detection.")?;
        }
        if self.al26_contact_input_2_signal_detection {
            writeln!(f, "- Contact input 2 signal detection.")?;
        }
        if self.al27_forced_stop {
            writeln!(f, "- Forced stop.")?;
        }
        if self.al28_maintenance_notice {
            writeln!(f, "- Maintenance notice.")?;
        }
        if self.al29_communication_error {
            writeln!(f, "- Communication error.")?;
        }
        if self.al30_refrigerant_circuit_abnormal {
            writeln!(f, "- Refrigerant circuit abnormal.")?;
        }
        if self.al31_sensor_abnormal {
            writeln!(f, "- Sensor abnormality.")?;
        }
        if self.al32_controller_abnormal {
            writeln!(f, "- Controller abnormality.")?;
        }

        Ok(())
    }
}

impl AlarmFlags {
    /// Are there any alarms?
    pub fn has_alarms(&self) -> bool {
        self.al01_low_level_in_tank_abnormal
            || self.al02_low_level_in_tag
            || self.al03_phase_loss_phase_reverse_error
            || self.al04_water_leakage
            || self.al05_pump_inverter_error
            || self.al06_internal_fan_stop
            || self.al07_fan_inverter_error
            || self.al09_circulating_fluid_discharge_pressure_rise
            || self.al10_flow_rate_reduction
            || self.al11_outside_ambient_temperature_range
            || self.al12_electric_conductivity_rise
            || self.al13_not_temp_ready
            || self.al14_circulating_fluid_temperature_range_rise
            || self.al15_circulating_fluid_temperature_range_drop
            || self.al17_flow_rate_failure
            || self.al18_high_circulating_fluid_discharge_temperature
            || self.al19_high_circulating_fluid_return_temp
            || self.al21_high_circulating_fluid_discharge_pressure
            || self.al22_low_circulating_fluid_discharge_pressure
            || self.al24_memory_abnormal
            || self.al25_contact_input_1_signal_detection
            || self.al26_contact_input_2_signal_detection
            || self.al27_forced_stop
            || self.al28_maintenance_notice
            || self.al29_communication_error
            || self.al30_refrigerant_circuit_abnormal
            || self.al31_sensor_abnormal
            || self.al32_controller_abnormal
    }
}

impl InstrumentParameter<Vec<u8>> for AlarmFlags {
    fn to_writable(&self) -> Vec<u8> {
        unreachable!("This is a read only variable. To writable should never be called.");
    }

    fn try_from_writable(val: Vec<u8>) -> Result<Self, InstrumentError> {
        if val.len() != 4 {
            return Err(instrumentrs::InstrumentError::BadInstrumentResponseVecU8 { msg: val });
        }

        let al1 = u16::from_be_bytes([val[0], val[1]]);
        let al2 = u16::from_be_bytes([val[2], val[3]]);

        Ok(Self {
            al01_low_level_in_tank_abnormal: bit_u16_is_set(al1, 0),
            al02_low_level_in_tag: bit_u16_is_set(al1, 1),
            al03_phase_loss_phase_reverse_error: bit_u16_is_set(al1, 2),
            al04_water_leakage: bit_u16_is_set(al1, 3),
            al05_pump_inverter_error: bit_u16_is_set(al1, 4),
            al06_internal_fan_stop: bit_u16_is_set(al1, 5),
            al07_fan_inverter_error: bit_u16_is_set(al1, 6),
            al09_circulating_fluid_discharge_pressure_rise: bit_u16_is_set(al1, 8),
            al10_flow_rate_reduction: bit_u16_is_set(al1, 9),
            al11_outside_ambient_temperature_range: bit_u16_is_set(al1, 10),
            al12_electric_conductivity_rise: bit_u16_is_set(al1, 11),
            al13_not_temp_ready: bit_u16_is_set(al1, 12),
            al14_circulating_fluid_temperature_range_rise: bit_u16_is_set(al1, 13),
            al15_circulating_fluid_temperature_range_drop: bit_u16_is_set(al1, 14),

            al17_flow_rate_failure: bit_u16_is_set(al2, 0),
            al18_high_circulating_fluid_discharge_temperature: bit_u16_is_set(al2, 1),
            al19_high_circulating_fluid_return_temp: bit_u16_is_set(al2, 2),
            al21_high_circulating_fluid_discharge_pressure: bit_u16_is_set(al2, 4),
            al22_low_circulating_fluid_discharge_pressure: bit_u16_is_set(al2, 5),
            al24_memory_abnormal: bit_u16_is_set(al2, 7),
            al25_contact_input_1_signal_detection: bit_u16_is_set(al2, 8),
            al26_contact_input_2_signal_detection: bit_u16_is_set(al2, 9),
            al27_forced_stop: bit_u16_is_set(al2, 10),
            al28_maintenance_notice: bit_u16_is_set(al2, 11),
            al29_communication_error: bit_u16_is_set(al2, 12),
            al30_refrigerant_circuit_abnormal: bit_u16_is_set(al2, 13),
            al31_sensor_abnormal: bit_u16_is_set(al2, 14),
            al32_controller_abnormal: bit_u16_is_set(al2, 15),
        })
    }
}

/// Is a bit in a u16 `mask` at a given `offset` set?.
fn bit_u16_is_set(mask: u16, offset: u16) -> bool {
    mask & (1_u16 << offset) != 0
}

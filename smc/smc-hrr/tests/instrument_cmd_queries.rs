//! Tests to ensure the instrument queries work as expected.

use instrumentrs::{smock, u};

use measurements::{Pressure, Temperature};
use smc_hrr::{
    InstrumentError, SmcHrr,
    types::{OperationStatus, SerialRemoteInstruction},
};

mod helpers;
use helpers::extend_with_crc;

#[test]
fn get_discharge_pressure() {
    let mut exp_writes = [vec![0x01, 0x04, 0x00, 0x02, 0x00, 0x01]];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0xAA], // answer chiller is on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let val: Pressure = u!(inst.get_discharge_pressure()).into();
    assert_eq!(val.as_kilopascals(), 170.0);
}

#[test]
fn get_discharge_temperature() {
    let mut exp_writes = [vec![0x01, 0x04, 0x00, 0x00, 0x00, 0x01]];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0xAA], // answer chiller is on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let val: Temperature = u!(inst.get_discharge_temperature()).into();
    assert_eq!(val.as_celsius(), 17.0);
}

#[test]
fn get_electric_conductivity() {
    let mut exp_writes = [vec![0x01, 0x04, 0x00, 0x03, 0x00, 0x01]];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0xAA], // answer chiller is on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let val = u!(inst.get_electric_conductivity());
    assert_eq!(val.as_microsievert_per_cm(), 17.0);
}

#[test]
fn get_flow_rate() {
    let mut exp_writes = [vec![0x01, 0x04, 0x00, 0x01, 0x00, 0x01]];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0xAA], // answer chiller is on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let val = u!(inst.get_flow_rate());
    assert_eq!(val.as_lpm(), 17.0);
}

#[test]
fn set_and_get_operation_status() {
    let mut exp_writes = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // turn chiller on
        vec![0x01, 0x04, 0x00, 0x0C, 0x00, 0x01], // read back chiller is on
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // answer to turning chiller on
        vec![0x01, 0x04, 0x02, 0x00, 0x01],       // answer chiller is on
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    u!(inst.set_operation_status(OperationStatus::Run));

    let op_st_rec = u!(inst.get_operation_status());
    assert_eq!(op_st_rec, OperationStatus::Run);
}

#[test]
fn set_and_get_setpoint_temperature() {
    let mut exp_writes = [
        vec![0x01, 0x06, 0x00, 0x0B, 0x00, 0x0A], // set temperature to 1 C
        vec![0x01, 0x04, 0x00, 0x0B, 0x00, 0x01], // read back temperature
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x06, 0x00, 0x0B, 0x00, 0x0A], // read back from setting temperature.
        vec![0x01, 0x04, 0x02, 0xFF, 0xF6],       // set point -1 C
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    u!(inst.set_set_temperature(Temperature::from_celsius(1.0).into()));

    let set_temp: Temperature = u!(inst.get_set_temperature()).into();
    assert_eq!(set_temp.as_celsius(), -1.0);
}

#[test]
fn get_serial_remote_instructions() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x0C, 0x00, 0x01], // get remote
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0x30], // return comms set to remote
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let smi_rec = u!(inst.get_serial_remote_instructions());
    assert_eq!(smi_rec, SerialRemoteInstruction::Serial);
}

#[test]
fn set_serial_remote_instructions_chiller_stopped() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x04, 0x00, 0x01],
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x30], // set to remote
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0x00],
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x30], // answer to set to remote
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    u!(inst.set_serial_remote_instructions(SerialRemoteInstruction::Serial));
}

#[test]
fn set_serial_remote_instructions_chiller_running() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x04, 0x00, 0x01],
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x31], // set to remote
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0x01],
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x30], // answer to set to remote
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    u!(inst.set_serial_remote_instructions(SerialRemoteInstruction::Serial));
}

#[test]
fn get_alarm_flags_no_alarms() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x05, 0x00, 0x02], // request alarm flags
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [vec![0x01, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00]]; // all flags on
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let alarms = inst.get_alarm_flags().unwrap();
    assert!(!alarms.has_alarms());
}

#[test]
fn get_alarm_flags_internal_fan_stop_and_forced_stop() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x05, 0x00, 0x02], // request alarm flags
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [vec![0x01, 0x04, 0x04, 0x00, 0b0010_0000, 0b0000_0100, 0x00]]; // all flags on
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let alarms = inst.get_alarm_flags().unwrap();
    assert!(alarms.has_alarms());
    assert!(alarms.al06_internal_fan_stop);
    assert!(alarms.al27_forced_stop);
}

#[test]
fn get_status_flags() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x04, 0x00, 0x01], // request status flags
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [vec![0x01, 0x04, 0x02, 0xFF, 0xFF]]; // all flags on
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let st = inst.get_status().unwrap();

    assert!(st.is_running());
    assert!(st.has_op_stop_alarm());
    assert!(st.has_op_cont_alarm());
    assert!(st.is_remote_mode_serial());
    assert!(st.is_temp_out());
    assert!(st.is_temp_ready());
    assert!(st.needs_maintenance());
}

// BAD REPLY HANDLING //

#[test]
fn set_command_answers_with_invalid_crc() {
    let mut exp_writes = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // turn chiller on
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // answer to turning chiller on
    ];
    exp_reads[0].extend_from_slice(&[0x00, 0x00]); // add an invlalid crc

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let sendcmd_res = inst.set_operation_status(OperationStatus::Run).unwrap_err();
    std::assert_matches!(sendcmd_res, InstrumentError::ChecksumError);
}

#[test]
fn query_command_answers_with_invalid_crc() {
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x0C, 0x00, 0x01], // get remote
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x04, 0x02, 0x00, 0x30], // return comms set to remote
    ];
    exp_reads[0].extend_from_slice(&[0x00, 0x00]);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let sendcmd_res = inst.get_serial_remote_instructions().unwrap_err();
    std::assert_matches!(sendcmd_res, InstrumentError::ChecksumError);
}

#[test]
fn set_command_returns_negative_answer() {
    // Note that the read here is correct, so this is not a real example.
    let mut exp_writes = [
        vec![0x01, 0x06, 0x00, 0x0C, 0x00, 0x01], // turn chiller on
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x86, 0x02], // negative answer
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let sendcmd_res = inst.set_operation_status(OperationStatus::Run).unwrap_err();
    std::assert_matches!(sendcmd_res, InstrumentError::NegativeResponse { .. });
}

#[test]
fn get_command_returns_negative_answer() {
    // Note that the read here is correct, so this is not a real example.
    let mut exp_writes = [
        vec![0x01, 0x04, 0x00, 0x0C, 0x00, 0x01], // get remote
    ];
    extend_with_crc(&mut exp_writes);

    let mut exp_reads = [
        vec![0x01, 0x84, 0x02], // negative answer
    ];
    extend_with_crc(&mut exp_reads);

    let mut inst = smock!(SmcHrr, exp_reads, exp_writes);

    let sendcmd_res = inst.get_serial_remote_instructions().unwrap_err();
    std::assert_matches!(sendcmd_res, InstrumentError::NegativeResponse { .. });
}

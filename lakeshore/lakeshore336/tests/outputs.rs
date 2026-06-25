//! Test the outputs.

use instrumentrs2::{smock, u};

use lakeshore336::{
    Input, Lakeshore336, Output,
    types::{
        HeaterMaxOutputCurrent, HeaterOutputDisplay, HeaterResistance, HeaterSetup, OnPowerup,
        OutputMode, OutputModeSetup, heater_setup::HeaterRange,
    },
};
use measurements::{Current, Temperature, test_utils::assert_almost_eq};

pub static TERM: &str = "\n";

#[test]
fn heater_setup_command_query() {
    let expected_writes = ["HTRSET? 1", "HTRSET? 2"];
    let expected_reads = ["1,1,+1.234,1", "2,0,+0.312,2"];

    let expected = HeaterSetup::try_new(
        HeaterResistance::R25Ohm,
        lakeshore336::types::HeaterMaxOutputCurrent::C0p707A,
        HeaterOutputDisplay::Current,
    )
    .unwrap();

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);
    let received = u!(inst.output(Output::Out1).get_heater_setup());

    assert_eq!(received, expected);

    let expected = HeaterSetup::try_new(
        HeaterResistance::R50Ohm,
        HeaterMaxOutputCurrent::User(Current::from_amperes(0.312)),
        HeaterOutputDisplay::Power,
    )
    .unwrap();

    let received = u!(inst.output(Output::Out2).get_heater_setup());
    assert_eq!(received, expected);
}

#[test]
fn heater_setup_set() {
    let expected_writes = ["HTRSET 1,1,2,+1.000,2"];
    let expected_reads: [&str; 0] = [];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    let heater_setup = HeaterSetup::try_new(
        HeaterResistance::R25Ohm,
        HeaterMaxOutputCurrent::C1A,
        HeaterOutputDisplay::Power,
    )
    .unwrap();

    u!(inst.output(Output::Out1).set_heater_setup(heater_setup));
}

#[test]
fn output_mode_get() {
    let expected_writes = ["OUTMODE? 1"];
    let expected_reads = ["2,3,0"];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    let expected = OutputModeSetup::new(OutputMode::Zone, Some(Input::InC), OnPowerup::Disabled);
    let received = u!(inst.output(Output::Out1).get_output_mode());
    assert_eq!(received, expected);
}

#[test]
fn output_mode_set() {
    let expected_writes = ["OUTMODE 2,1,1,1"];
    let expected_reads: [&str; 0] = [];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    let output_mode_setup = OutputModeSetup::new(
        OutputMode::ClosedLoopPID,
        Some(Input::InA),
        OnPowerup::Enabled,
    );
    u!(inst.output(Output::Out2).set_output_mode(output_mode_setup));
}

#[test]
fn setpoint_temperature_get() {
    let expected_writes = ["SETP? 2"];
    let expected_reads = ["273.4"];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);

    let expected = Temperature::from_kelvin(273.4);
    let received = u!(inst.output(Output::Out2).get_setpoint());

    assert_almost_eq(received.as_kelvin(), expected.as_kelvin());
}

#[test]
fn setpoint_temperature_set() {
    let expected_writes = ["SETP 1,273.1"];
    let expected_reads: [&str; 0] = [];

    let temp_to_set = Temperature::from_kelvin(273.1);

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);
    u!(inst.output(Output::Out1).set_setpoint(temp_to_set));
}

#[test]
fn range_get() {
    let expected_writes = ["RANGE? 3"];
    let expected_reads = ["1"];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);
    let received = u!(inst.output(Output::Out3).get_range());

    assert_eq!(received, HeaterRange::LowOrOn);
}

#[test]
fn range_set() {
    let expected_writes = ["RANGE 1,2"];
    let expected_reads: [&str; 0] = [];

    let mut inst = smock!(Lakeshore336, expected_reads, expected_writes, TERM);
    u!(inst.output(Output::Out1).set_range(HeaterRange::Medium));
}
